#!/usr/bin/env python3
"""Publish one verified native/web release through the page-artifact API."""
import argparse
import hashlib
import http.client
import io
import json
import os
from pathlib import Path, PurePosixPath
import re
import sys
import tarfile
import time
from urllib.parse import urljoin, urlsplit
from typing import Optional

MANIFEST = 'euv-release.json'
ATTEMPTS = 9


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


def archive_bytes(directory, release):
    output = io.BytesIO()
    with tarfile.open(fileobj=output, mode='w:gz') as archive:
        for name in [MANIFEST] + [item['path'] for item in release['files']]:
            archive.add(directory / name, arcname=name, recursive=False)
    return output.getvalue()


def verify_live(base, descriptor, release):
    actual = request('GET', urljoin(base, MANIFEST))
    if actual != descriptor:
        raise ValueError('Live manifest differs from the uploaded artifact')
    receipts = []
    for item in release['files']:
        data = request('GET', urljoin(base, item['path']))
        if data is None:
            raise ValueError('Missing live release resource')
        verify_file(item, data)
        receipts.append(dict(path=item['path'], size=len(data), sha256=hashlib.sha256(data).hexdigest()))
    return receipts


def publish(directory, expected, token, api='https://ltpp.vip/api/page-artifacts/', public='https://ltpp.vip/pages/euv/'):
    descriptor, release = validate_directory(directory, expected)
    current = request('GET', urljoin(public, MANIFEST), allow_missing=True)
    if current is not None and version(inventory(current)['version']) > version(release['version']):
        raise ValueError('Refusing to replace a newer live release')
    archive = archive_bytes(directory, release)
    digest = hashlib.sha256(archive).hexdigest()
    endpoint = urljoin(api, 'euv/uploads')
    initialized = json.loads(request('POST', endpoint, json.dumps(dict(archiveBytes=len(archive), archiveSha256=digest)).encode(), token, {'Content-Type': 'application/json'}) or b'')
    parameters = initialized['data']
    upload_id, size, count = parameters['uploadId'], parameters['chunkSizeBytes'], parameters['chunkCount']
    if (initialized.get('success') is not True or not re.fullmatch('[a-zA-Z0-9-]+', upload_id)
            or not isinstance(size, int) or not 1 <= size <= 8388608
            or count != (len(archive)+size-1)//size):
        raise ValueError('Invalid upload session')
    for index in range(count):
        chunk = archive[index*size:(index+1)*size]
        request('PUT', endpoint+'/'+upload_id+'/chunks/'+str(index), chunk, token, {'Content-Type': 'application/octet-stream', 'X-Chunk-SHA256': hashlib.sha256(chunk).hexdigest()})
    committed = json.loads(request('POST', endpoint+'/'+upload_id+'/commit', b'', token) or b'')
    if committed.get('success') is not True:
        raise ValueError('Artifact commit rejected')
    catalog = json.loads(request('GET', api.rstrip('/')) or b'')
    if catalog.get('success') is not True or not any(item['name'] == 'euv' and item['sha256'] == digest for item in catalog['data']['artifacts']):
        raise ValueError('Public artifact catalog differs from upload')
    files = verify_live(public, descriptor, release)
    return dict(version=release['version'], archive_sha256=digest, archive_bytes=len(archive), manifest_sha256=hashlib.sha256(descriptor).hexdigest(), files=files, public_base=public, publication_route='page-artifacts API only; no GitHub Pages sync')


if __name__ == '__main__':
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('directory', type=Path)
    parser.add_argument('--version', required=True)
    parser.add_argument('--receipt', type=Path, required=True)
    args = parser.parse_args()
    token = os.environ.get('PAGE_ARTIFACT_API_KEY')
    if not token:
        raise SystemExit('PAGE_ARTIFACT_API_KEY is required')
    result = publish(args.directory, args.version, token)
    args.receipt.write_text(json.dumps(result, indent=2))
    print('Published and verified', len(result['files']), 'release resources at', result['public_base'])
