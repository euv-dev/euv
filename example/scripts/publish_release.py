#!/usr/bin/env python3
"""Publish one verified native/web release through the page-artifact API."""
import argparse
import hashlib
import io
import json
import os
from pathlib import Path, PurePosixPath
import re
import tarfile
import time
from urllib.error import HTTPError, URLError
from urllib.parse import urljoin
from urllib.request import HTTPRedirectHandler, Request, build_opener
from typing import Optional

MANIFEST = 'euv-release.json'

class NoRedirect(HTTPRedirectHandler):
    def redirect_request(self, req, fp, code, msg, headers, newurl):
        return None


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


def request(method, url, data=None, token=None, headers=None, allow_missing=False) -> Optional[bytes]:
    headers = dict(headers or {})
    if token:
        headers['Authorization'] = 'Bearer ' + token
    for attempt in range(9):
        if attempt:
            time.sleep(2 ** (attempt - 1))
        try:
            with build_opener(NoRedirect).open(Request(url, data=data, headers=headers, method=method), timeout=180) as response:
                if response.geturl() != url:
                    raise ValueError('Publication redirect rejected')
                return bytes(response.read())
        except HTTPError as error:
            if allow_missing and error.code == 404:
                return None
            if error.code < 500 and error.code != 429:
                raise ValueError('Publication HTTP ' + str(error.code)) from None
        except (URLError, TimeoutError):
            pass
    raise RuntimeError('Publication request failed after initial attempt plus eight retries')


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
