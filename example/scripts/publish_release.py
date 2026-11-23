#!/usr/bin/env python3
"""Publish one verified native/web release through the per-file page-artifact API."""
import argparse
import hashlib
import http.client
import json
import os
from pathlib import Path, PurePosixPath
import re
import sys
import time
from urllib.parse import urljoin, urlsplit
from typing import Optional

MANIFEST = 'euv-release.json'
ATTEMPTS = 9
DEFAULT_ORG = 'euv-dev'
DEFAULT_REPO = 'euv'
FILE_DIGEST_HEADER = 'X-File-SHA256'
SLUG_PATTERN = r'[a-z0-9](?:[a-z0-9-]{0,61}[a-z0-9])?'
MAX_FILE_BYTES = 8 * 1024 * 1024
MAX_FILE_COUNT = 8192
MAX_TOTAL_BYTES = 512 * 1024 * 1024


def sleep(seconds):
    """Indirection so the retry backoff can be exercised without real waiting."""
    time.sleep(seconds)


def version(value):
    if not re.fullmatch(r'(0|[1-9][0-9]*)\.(0|[1-9][0-9]*)\.(0|[1-9][0-9]*)', value):
        raise ValueError('Invalid release version')
    return tuple(map(int, value.split('.')))


def inventory(data, expected=None):
    result = json.loads(data)
    if result['format'] != 1 or (expected is not None and result['version'] != expected):
        raise ValueError('Wrong release format/version')
    version(result['version'])
    paths = set()
    for item in result['files']:
        path = item['path']
        if (not path or path.startswith('/') or any(p in ('', '.', '..') for p in path.split('/'))
                or any(c in path for c in '\\:%?#') or any(ord(c) < 32 for c in path)
                or path in paths or MANIFEST in PurePosixPath(path).parts
                or not re.fullmatch('[a-f0-9]{64}', item['sha256'])
                or not isinstance(item['size'], int) or item['size'] < 1):
            raise ValueError('Unsafe or duplicate release inventory')
        paths.add(path)
    if 'index.html' not in paths or not any(p.endswith('.wasm') for p in paths):
        raise ValueError('Incomplete release inventory')
    return result


def verify_file(item, data):
    if len(data) != item['size'] or hashlib.sha256(data).hexdigest() != item['sha256']:
        raise ValueError('Release integrity mismatch: ' + item['path'])
    if item['path'].endswith('.html') and b'__euv_reload' in data:
        raise ValueError('Development reload bootstrap: ' + item['path'])


def validate_directory(directory, expected=None):
    descriptor = (directory / MANIFEST).read_bytes()
    release = inventory(descriptor, expected)
    listed = {item['path'] for item in release['files']}
    actual = set()
    for path in directory.rglob('*'):
        if path.is_symlink():
            raise ValueError('Release symlinks are forbidden')
        if path.is_file() and path.relative_to(directory).as_posix() != MANIFEST:
            actual.add(path.relative_to(directory).as_posix())
    if actual != listed:
        raise ValueError('Release inventory differs from the complete file set')
    for item in release['files']:
        verify_file(item, (directory / item['path']).read_bytes())
    for theme in (directory / 'themes').iterdir():
        if theme.is_dir():
            prefix = theme.relative_to(directory).as_posix() + '/'
            if prefix+'index.html' not in listed or not any(p.startswith(prefix) and p.endswith('.wasm') for p in listed):
                raise ValueError('Incomplete theme: ' + theme.name)
    return descriptor, release


def transport_detail(error, token=None):
    """Render a transport failure for the log without ever exposing the token."""
    message = str(error)
    if token:
        message = message.replace(token, '[REDACTED]')
    detail = type(error).__name__
    return detail + (': ' + message if message else '')


def request(method, url, data=None, token=None, headers=None, allow_missing=False) -> Optional[bytes]:
    headers = dict(headers or {})
    if token:
        headers['Authorization'] = 'Bearer ' + token
    parsed = urlsplit(url)
    if parsed.scheme not in {'http', 'https'} or not parsed.hostname:
        raise ValueError('Publication URL must be an absolute http(s) URL')
    target = parsed.path or '/'
    if parsed.query:
        target += '?' + parsed.query
    connection_type = http.client.HTTPSConnection if parsed.scheme == 'https' else http.client.HTTPConnection
    last = 'no attempt reached the server'
    for attempt in range(ATTEMPTS):
        if attempt:
            sleep(2 ** (attempt - 1))
        connection = connection_type(parsed.hostname, parsed.port, timeout=180)
        status = None
        try:
            connection.request(method, target, body=data, headers=headers)
            response = connection.getresponse()
            status = response.status
            body = response.read()
            if 300 <= status < 400:
                raise ValueError('Publication redirect rejected')
            if status == 404 and allow_missing:
                return None
            if 200 <= status < 300:
                return body
            if status < 500 and status != 429:
                raise ValueError('Publication HTTP ' + str(status))
            last = 'server answered HTTP ' + str(status)
        except (OSError, http.client.HTTPException) as error:
            last = transport_detail(error, token)
            print('Publication transport failure: ' + last, file=sys.stderr)
        finally:
            connection.close()
        print('Publication request failed (HTTP ' + str(status) + ', attempt ' + str(attempt + 1) + '/' + str(ATTEMPTS) + ')', file=sys.stderr)
    raise RuntimeError('Publication request failed after the initial attempt and ' + str(ATTEMPTS - 1) + ' retries (' + method + ' ' + parsed.path + '): ' + last)


def envelope(body):
    """Read the server's success envelope and return its payload."""
    try:
        result = json.loads(body or b'')
    except ValueError as error:
        raise ValueError('Publication API returned a non-JSON answer') from error
    if not isinstance(result, dict) or result.get('success') is not True or 'data' not in result:
        raise ValueError('Publication API returned an unconfirmed success envelope')
    return result['data']


def canonical_manifest(files):
    """Serialize the announced manifest exactly as the server canonicalizes it."""
    return json.dumps(files, separators=(',', ':'), ensure_ascii=False).encode('utf-8')


def announced_files(descriptor, release):
    """Announce the release manifest and every release file as digest-bound entries."""
    announced = [(MANIFEST, len(descriptor), hashlib.sha256(descriptor).hexdigest())]
    for item in release['files']:
        announced.append((item['path'], item['size'], item['sha256']))
    if len(announced) > MAX_FILE_COUNT:
        raise ValueError('Release inventory exceeds the artifact file count limit')
    total = 0
    for path, size, _ in announced:
        if size > MAX_FILE_BYTES:
            raise ValueError('Release file exceeds the per-file upload limit: ' + path)
        total += size
    if total > MAX_TOTAL_BYTES:
        raise ValueError('Release inventory exceeds the artifact size limit')
    return announced


def manifest_entries(announced):
    """Render the announced files in the wire shape the server deserializes."""
    return [dict(path=path, bytes=size, sha256=digest) for path, size, digest in announced]


def verify_live(base, descriptor, release):
    actual = request('GET', urljoin(base, MANIFEST), allow_missing=True)
    if actual is None:
        raise ValueError('Missing live release manifest')
    if actual != descriptor:
        raise ValueError('Live manifest differs from the uploaded artifact')
    receipts = []
    for item in release['files']:
        data = request('GET', urljoin(base, item['path']), allow_missing=True)
        if data is None:
            raise ValueError('Missing live release resource')
        verify_file(item, data)
        receipts.append(dict(path=item['path'], size=len(data), sha256=hashlib.sha256(data).hexdigest()))
    return receipts


def live_version(base):
    """Read the version of the release a public base already serves."""
    current = request('GET', urljoin(base, MANIFEST), allow_missing=True)
    if current is None:
        return None
    return version(inventory(current)['version'])


def upload_file(directory, endpoint, upload_id, index, entry, token):
    """Send one announced file as its own digest-bound request."""
    path, size, digest = entry
    source = directory / Path(*path.split('/'))
    if not source.is_file():
        raise ValueError('An announced release file disappeared before it was uploaded: ' + path)
    payload = source.read_bytes()
    if len(payload) != size or hashlib.sha256(payload).hexdigest() != digest:
        raise ValueError('An announced release file changed before it was uploaded: ' + path)
    request('PUT', endpoint + '/' + upload_id + '/files/' + str(index), payload, token,
            {'Content-Type': 'application/octet-stream', FILE_DIGEST_HEADER: digest})
    return len(payload)


def publish(directory, expected, token, org=DEFAULT_ORG, repo=DEFAULT_REPO,
            api='https://ltpp.vip/api/page-artifacts/', public=None, legacy_public=None):
    if not re.fullmatch(SLUG_PATTERN, org) or not re.fullmatch(SLUG_PATTERN, repo):
        raise ValueError('Invalid artifact organization or repository name')
    qualifier = org + '/' + repo
    api = api.rstrip('/') + '/'
    public = public or 'https://ltpp.vip/pages/' + qualifier + '/'
    legacy_public = legacy_public or 'https://ltpp.vip/pages/' + repo + '/'
    descriptor, release = validate_directory(directory, expected)
    live = live_version(legacy_public)
    if live is not None and live > version(release['version']):
        raise ValueError('Refusing to replace a newer live release')
    announced = announced_files(descriptor, release)
    manifest = canonical_manifest(manifest_entries(announced))
    artifact_sha256 = hashlib.sha256(manifest).hexdigest()
    total_bytes = sum(size for _, size, _ in announced)
    endpoint = urljoin(api, qualifier + '/uploads')
    session = envelope(request('POST', endpoint, json.dumps(dict(files=manifest_entries(announced)),
                                                            separators=(',', ':'), ensure_ascii=False).encode(),
                               token, {'Content-Type': 'application/json'}))
    if not isinstance(session, dict):
        raise ValueError('Invalid upload session')
    upload_id, file_count, announced_digest = session.get('uploadId'), session.get('fileCount'), session.get('manifestSha256')
    if not isinstance(upload_id, str) or not re.fullmatch('[A-Za-z0-9-]+', upload_id):
        raise ValueError('Invalid upload session identifier')
    if not isinstance(file_count, int) or file_count != len(announced):
        raise ValueError('Upload session file count does not match the announced manifest')
    if announced_digest != artifact_sha256:
        raise ValueError('Upload session manifest digest does not match the announced manifest')
    for index, entry in enumerate(announced):
        upload_file(directory, endpoint, upload_id, index, entry, token)
        print('Uploaded release file ' + str(index + 1) + '/' + str(len(announced)) + ': ' + entry[0], flush=True)
    receipt = envelope(request('POST', endpoint + '/' + upload_id + '/commit', b'', token))
    if not isinstance(receipt, dict):
        raise ValueError('Invalid artifact commit receipt')
    expected_receipt = dict(name=qualifier, org=org, repo=repo, fileCount=len(announced),
                            totalBytes=total_bytes, sha256=artifact_sha256)
    if any(receipt.get(field) != value for field, value in expected_receipt.items()):
        raise ValueError('Artifact commit receipt does not match the uploaded release')
    catalog = envelope(request('GET', api.rstrip('/')))
    if not isinstance(catalog, dict) or not isinstance(catalog.get('artifacts'), list):
        raise ValueError('Invalid public artifact catalog')
    matches = [item for item in catalog['artifacts']
               if isinstance(item, dict) and item.get('name') == qualifier
               and item.get('org') == org and item.get('repo') == repo]
    if len(matches) != 1:
        raise ValueError('Public artifact catalog does not contain exactly one matching artifact')
    published = matches[0]
    if (published.get('sha256') != artifact_sha256
            or not isinstance(published.get('fileCount'), int) or published['fileCount'] != len(announced)
            or not isinstance(published.get('totalBytes'), int) or published['totalBytes'] != total_bytes):
        raise ValueError('Public artifact catalog differs from the uploaded release')
    files_receipt = verify_live(public, descriptor, release)
    alias = request('GET', urljoin(legacy_public, MANIFEST), allow_missing=True)
    if alias is not None and alias != descriptor:
        raise ValueError('Legacy public alias differs from the uploaded artifact')
    return dict(version=release['version'], org=org, repo=repo, qualifier=qualifier,
                artifact_sha256=artifact_sha256, file_count=len(announced), total_bytes=total_bytes,
                release_manifest_sha256=hashlib.sha256(descriptor).hexdigest(),
                files=files_receipt, public_base=public, legacy_public_base=legacy_public,
                publication_route='page-artifacts API only; no GitHub Pages sync')


if __name__ == '__main__':
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('directory', type=Path)
    parser.add_argument('--version', required=True)
    parser.add_argument('--org', default=DEFAULT_ORG)
    parser.add_argument('--repo', default=DEFAULT_REPO)
    parser.add_argument('--base-url', default='https://ltpp.vip',
                        help='Origin that serves the page-artifact API and the public pages')
    parser.add_argument('--receipt', type=Path, required=True)
    args = parser.parse_args()
    token = os.environ.get('PAGE_ARTIFACT_API_KEY')
    if not token:
        raise SystemExit('PAGE_ARTIFACT_API_KEY is required')
    origin = args.base_url.rstrip('/')
    result = publish(args.directory, args.version, token, args.org, args.repo,
                     api=origin + '/api/page-artifacts/',
                     public=origin + '/pages/' + args.org + '/' + args.repo + '/',
                     legacy_public=origin + '/pages/' + args.repo + '/')
    args.receipt.write_text(json.dumps(result, indent=2))
    print('Published and verified', len(result['files']), 'release resources at', result['public_base'])
