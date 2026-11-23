import contextlib
import hashlib
from http.server import BaseHTTPRequestHandler, ThreadingHTTPServer
import io
import json
from pathlib import Path
import tempfile
import threading
import unittest
from typing import Optional

import publish_release as publisher


class PublicationTests(unittest.TestCase):
    """The per-file publisher must publish a complete, digest-bound artifact."""

    def setUp(self):
        self.temp = tempfile.TemporaryDirectory()
        self.directory = Path(self.temp.name)
        self.resources = {'index.html': b'<html>release</html>', 'pkg/app.wasm': b'\0asm\x01\0\0\0', 'themes/default/index.html': b'<html>default</html>', 'themes/default/pkg/app.wasm': b'\0asm\x01\0\0\0'}
        for name, data in self.resources.items():
            path = self.directory / name
            path.parent.mkdir(parents=True, exist_ok=True)
            path.write_bytes(data)
        self.descriptor = json.dumps(dict(format=1, version='0.36.4', files=[dict(path=name,size=len(data),sha256=hashlib.sha256(data).hexdigest()) for name,data in self.resources.items()])).encode()
        (self.directory/publisher.MANIFEST).write_bytes(self.descriptor)
        self.served, self.files, self.calls = {}, {}, []
        self.manifest_digest: Optional[str] = None
        self.corrupt_catalog = False
        self.receipt = dict(name='euv-dev/euv',org='euv-dev',repo='euv',fileCount=0,totalBytes=0,sha256='')
        self.corrupt = False
        self.fail_files = {}
        self.fail_first = {}
        self.token = '[REDACTED]'
        owner = self
        class Handler(BaseHTTPRequestHandler):
            def log_message(self, format, *args):
                pass
            def reply(self, data, status=200):
                self.send_response(status)
                self.send_header('Content-Length',str(len(data)))
                self.end_headers()
                self.wfile.write(data)
            def body(self):
                return self.rfile.read(int(self.headers.get('Content-Length',0)))
            def success(self, data):
                self.reply(json.dumps(dict(success=True,data=data)).encode())
            def do_GET(self):
                owner.calls.append(('GET',self.path))
                if self.path == '/api/page-artifacts':
                    digest = 'f'*64 if owner.corrupt_catalog else owner.manifest_digest
                    self.success(dict(artifacts=[dict(name='euv-dev/euv',org='euv-dev',repo='euv',fileCount=owner.file_count(),totalBytes=owner.total_bytes(),sha256=digest)]))
                elif self.path.startswith('/pages/euv-dev/euv/'):
                    data = owner.served.get(self.path[len('/pages/euv-dev/euv/'):])
                    if data is None:
                        self.reply(b'missing',404)
                    else:
                        self.reply(data)
                elif self.path.startswith('/pages/euv/'):
                    data = owner.served.get(self.path[len('/pages/euv/'):])
                    if data is None:
                        self.reply(b'missing',404)
                    else:
                        self.reply(data)
                else:
                    self.reply(b'missing',404)
            def do_POST(self):
                owner.calls.append(('POST',self.path))
                if self.headers.get('Authorization') != 'Bearer '+owner.token:
                    self.reply(b'unauthorized',401)
                    return
                data = self.body()
                if self.path == '/api/page-artifacts/euv-dev/euv/uploads':
                    payload = json.loads(data)
                    owner.files = {}
                    owner.manifest_digest = hashlib.sha256(publisher.canonical_manifest(payload['files'])).hexdigest()
                    self.success(dict(uploadId='local-session',fileCount=len(payload['files']),manifestSha256=owner.manifest_digest))
                elif self.path == '/api/page-artifacts/euv-dev/euv/uploads/local-session/commit':
                    owner.commit()
                    self.success(owner.receipt)
                else:
                    self.reply(b'missing',404)
            def do_PUT(self):
                owner.calls.append(('PUT',self.path))
                data = self.body()
                assert self.headers.get('Authorization') == 'Bearer '+owner.token
                index = int(self.path.rsplit('/',1)[1])
                digest = hashlib.sha256(data).hexdigest()
                if self.headers.get('X-File-SHA256') != digest:
                    self.reply(b'digest mismatch',400)
                    return
                pending = owner.fail_first.get(index, 0)
                if pending:
                    owner.fail_first[index] = pending - 1
                    self.reply(b'temporarily unavailable',503)
                    return
                if index in owner.fail_files:
                    self.reply(b'rejected',400)
                    return
                owner.files[index] = data
                self.success(dict(stored=True))
        self.handler = Handler
        self.server = ThreadingHTTPServer(('127.0.0.1',0),Handler)
        self.thread = threading.Thread(target=self.server.serve_forever,daemon=True)
        self.thread.start()
        self.base = 'http://127.0.0.1:'+str(self.server.server_port)
    def file_count(self):
        return len(self.resources) + 1
    def total_bytes(self):
        return sum(len(data) for data in self.resources.values()) + len(self.descriptor)
    def commit(self):
        published = dict(self.files)
        published[publisher.MANIFEST] = self.descriptor
        for name, data in self.resources.items():
            published[name] = data
        if self.corrupt:
            published['pkg/app.wasm'] = b'tampered'
        self.served = published
        self.receipt = dict(name='euv-dev/euv',org='euv-dev',repo='euv',fileCount=self.file_count(),
                            totalBytes=self.total_bytes(),sha256=self.manifest_digest)
    def tearDown(self):
        self.server.shutdown(); self.server.server_close(); self.thread.join(); self.temp.cleanup()
    def publish(self):
        return publisher.publish(self.directory,'0.36.4',self.token,api=self.base+'/api/page-artifacts/',
                                 public=self.base+'/pages/euv-dev/euv/',legacy_public=self.base+'/pages/euv/')
    def test_per_file_upload_commits_one_complete_artifact_and_reads_every_live_resource(self):
        receipt = self.publish()
        self.assertEqual(len(receipt['files']),len(self.resources))
        self.assertEqual(self.served[publisher.MANIFEST],self.descriptor)
        self.assertEqual({item['path'] for item in receipt['files']},set(self.resources))
        self.assertEqual(sum(method=='POST' and path.endswith('/commit') for method,path in self.calls),1)
        self.assertFalse(any('github/pages/sync' in path for _,path in self.calls))
    def test_every_release_file_is_sent_as_its_own_digest_bound_request(self):
        self.publish()
        puts = [path for method,path in self.calls if method=='PUT']
        self.assertEqual(len(puts),len(self.resources)+1)
        self.assertTrue(all('/files/' in path for path in puts))
        self.assertTrue(any(path.endswith('/files/0') for path in puts))
        self.assertTrue(all('/api/page-artifacts/euv-dev/euv/uploads/' in path for path in puts))
        self.assertFalse(any('/chunks/' in path for path in puts))
    def test_the_published_artifact_carries_the_organization_and_repository(self):
        receipt = self.publish()
        self.assertEqual(receipt['qualifier'],'euv-dev/euv')
        self.assertEqual(receipt['org'],'euv-dev')
        self.assertEqual(receipt['repo'],'euv')
        self.assertEqual(receipt['public_base'],self.base+'/pages/euv-dev/euv/')
        self.assertEqual(receipt['legacy_public_base'],self.base+'/pages/euv/')
    def test_a_missing_announced_file_is_refused_before_it_is_uploaded(self):
        (self.directory/'pkg/app.wasm').unlink()
        with self.assertRaises(ValueError):
            self.publish()
        self.assertFalse(any(method=='PUT' for method,_ in self.calls))
    def test_a_digest_mismatch_is_refused_instead_of_being_published(self):
        manifest = json.loads(self.descriptor)
        manifest['files'][0]['sha256'] = '0'*64
        (self.directory/publisher.MANIFEST).write_bytes(json.dumps(manifest).encode())
        with self.assertRaisesRegex(ValueError,'integrity mismatch'):
            self.publish()
    def test_tampered_live_bytes_fail_instead_of_claiming_publication_success(self):
        self.corrupt = True
        with self.assertRaisesRegex(ValueError,'integrity mismatch'):
            self.publish()
    def test_dev_reload_is_rejected_before_any_upload(self):
        (self.directory/'index.html').write_bytes(b'__euv_reload')
        with self.assertRaises(ValueError):
            self.publish()
        self.assertFalse(self.calls)
    def test_missing_manifest_is_rejected_before_any_upload(self):
        (self.directory/publisher.MANIFEST).unlink()
        with self.assertRaises(FileNotFoundError):
            self.publish()
        self.assertFalse(self.calls)
    def test_newer_live_version_is_never_replaced(self):
        manifest = json.loads(self.descriptor)
        manifest['version'] = '0.36.9'
        self.served[publisher.MANIFEST] = json.dumps(manifest).encode()
        with self.assertRaisesRegex(ValueError,'newer live release'):
            self.publish()
        self.assertFalse(any(method!='GET' for method,_ in self.calls))
    def test_unlisted_files_and_unsafe_paths_are_rejected(self):
        (self.directory/'stale.txt').write_bytes(b'stale')
        with self.assertRaises(ValueError):
            self.publish()
        self.assertFalse(self.calls)
        manifest = json.loads(self.descriptor)
        manifest['files'][0]['path'] = '../outside'
        with self.assertRaises(ValueError):
            publisher.inventory(json.dumps(manifest))
    def test_a_catalog_that_does_not_match_the_upload_fails_closed(self):
        self.corrupt_catalog = True
        with self.assertRaisesRegex(ValueError,'catalog differs'):
            self.publish()
    def test_a_receipt_that_does_not_match_the_upload_fails_closed(self):
        original = self.commit
        def commit():
            original()
            self.receipt['fileCount'] = 999
        self.commit = commit
        with self.assertRaisesRegex(ValueError,'commit receipt'):
            self.publish()


class PerFileRetryTests(unittest.TestCase):
    """Every file request spends its own initial attempt plus eight retries."""

    def setUp(self):
        self.temp = tempfile.TemporaryDirectory()
        self.directory = Path(self.temp.name)
        self.resources = {'index.html': b'<html>release</html>', 'pkg/app.wasm': b'\0asm\x01\0\0\0', 'themes/default/index.html': b'<html>default</html>', 'themes/default/pkg/app.wasm': b'\0asm\x01\0\0\0'}
        for name, data in self.resources.items():
            path = self.directory / name
            path.parent.mkdir(parents=True, exist_ok=True)
            path.write_bytes(data)
        self.descriptor = json.dumps(dict(format=1, version='0.36.4', files=[dict(path=name,size=len(data),sha256=hashlib.sha256(data).hexdigest()) for name,data in self.resources.items()])).encode()
        (self.directory/publisher.MANIFEST).write_bytes(self.descriptor)
        self.served, self.files = {}, {}
        self.digest: Optional[str] = None
        self.attempts = {}
        self.reject_index = None
        self.flaky_index = None
        self.flaky_remaining = 0
        self.token = '[REDACTED]'
        owner = self
        class Handler(BaseHTTPRequestHandler):
            def log_message(self, format, *args):
                pass
            def reply(self, data, status=200):
                self.send_response(status)
                self.send_header('Content-Length',str(len(data)))
                self.end_headers()
                self.wfile.write(data)
            def success(self, data):
                self.reply(json.dumps(dict(success=True,data=data)).encode())
            def do_GET(self):
                if self.path == '/api/page-artifacts':
                    self.success(dict(artifacts=[dict(name='euv-dev/euv',org='euv-dev',repo='euv',fileCount=len(owner.resources)+1,
                                                       totalBytes=sum(len(d) for d in owner.resources.values())+len(owner.descriptor),
                                                       sha256=owner.digest)]))
                elif self.path.startswith('/pages/euv-dev/euv/'):
                    data = owner.served.get(self.path[len('/pages/euv-dev/euv/'):])
                    self.reply(data if data is not None else b'missing', 200 if data is not None else 404)
                elif self.path.startswith('/pages/euv/'):
                    data = owner.served.get(self.path[len('/pages/euv/'):])
                    self.reply(data if data is not None else b'missing', 200 if data is not None else 404)
                else:
                    self.reply(b'missing',404)
            def do_POST(self):
                if self.path == '/api/page-artifacts/euv-dev/euv/uploads':
                    payload = json.loads(self.rfile.read(int(self.headers.get('Content-Length',0))))
                    owner.files = {}
                    owner.digest = hashlib.sha256(publisher.canonical_manifest(payload['files'])).hexdigest()
                    self.success(dict(uploadId='local-session',fileCount=len(payload['files']),manifestSha256=owner.digest))
                elif self.path == '/api/page-artifacts/euv-dev/euv/uploads/local-session/commit':
                    published = dict(owner.files)
                    published[publisher.MANIFEST] = owner.descriptor
                    published.update(owner.resources)
                    owner.served = published
                    self.success(dict(name='euv-dev/euv',org='euv-dev',repo='euv',fileCount=len(owner.resources)+1,
                                      totalBytes=sum(len(d) for d in owner.resources.values())+len(owner.descriptor),
                                      sha256=owner.digest))
                else:
                    self.reply(b'missing',404)
            def do_PUT(self):
                index = int(self.path.rsplit('/',1)[1])
                owner.attempts[index] = owner.attempts.get(index,0) + 1
                data = self.rfile.read(int(self.headers.get('Content-Length',0)))
                if hashlib.sha256(data).hexdigest() != self.headers.get('X-File-SHA256'):
                    self.reply(b'digest mismatch',400)
                    return
                if index == owner.flaky_index and owner.flaky_remaining > 0:
                    owner.flaky_remaining -= 1
                    self.reply(b'temporarily unavailable',503)
                    return
                if index == owner.reject_index:
                    self.reply(b'rejected',400)
                    return
                owner.files[index] = data
                self.success(dict(stored=True))
        self.server = ThreadingHTTPServer(('127.0.0.1',0),Handler)
        self.thread = threading.Thread(target=self.server.serve_forever,daemon=True)
        self.thread.start()
        self.base = 'http://127.0.0.1:'+str(self.server.server_port)
        self.saved = publisher.sleep
        publisher.sleep = lambda seconds: None
        self.stderr = io.StringIO()
        self.addCleanup(self.restore)
    def restore(self):
        publisher.sleep = self.saved
    def tearDown(self):
        self.server.shutdown(); self.server.server_close(); self.thread.join(); self.temp.cleanup()
    def publish(self):
        with contextlib.redirect_stderr(self.stderr):
            return publisher.publish(self.directory,'0.36.4',self.token,api=self.base+'/api/page-artifacts/',
                                     public=self.base+'/pages/euv-dev/euv/',legacy_public=self.base+'/pages/euv/')
    def test_a_retry_that_eventually_succeeds_keeps_the_upload_within_budget(self):
        self.flaky_index = 2
        self.flaky_remaining = 3
        receipt = self.publish()
        self.assertEqual(len(receipt['files']),len(self.resources))
        self.assertEqual(self.attempts[2],4)
        self.assertEqual(self.attempts[0],1)
    def test_a_rejected_file_exhausts_the_per_file_retry_budget(self):
        self.reject_index = 1
        with self.assertRaises(ValueError) as caught:
            self.publish()
        self.assertIn('HTTP 400', str(caught.exception))
        self.assertEqual(self.attempts[1],1)
    def test_a_server_error_exhausts_the_per_file_retry_budget(self):
        self.reject_index = 1
        self.flaky_index = 1
        self.flaky_remaining = publisher.ATTEMPTS
        with self.assertRaises(RuntimeError) as caught:
            self.publish()
        self.assertIn('HTTP 503', str(caught.exception))
        self.assertEqual(self.attempts[1],publisher.ATTEMPTS)
        self.assertIn('initial attempt and 8 retries', str(caught.exception))


class CanonicalManifestTests(unittest.TestCase):
    """The announced manifest must serialize exactly as the server canonicalizes it.

    The server digests its own re-serialization of the announced manifest, so a
    client that encodes the same manifest differently computes a different
    digest and is refused at the session boundary. These digests were produced
    by serializing the server's own PageArtifactFileEntry type with the
    server's serde_json version, so they pin the wire form.
    """

    def test_the_announced_manifest_matches_the_server_canonical_form(self):
        cases = [
            ([dict(path='euv-release.json',bytes=1234,sha256='a'*64)],
             '0aa2212e06203d226f009176ebdd324bf841009d15f165652ddc15cafd798f60'),
            ([dict(path='euv-release.json',bytes=0,sha256='0'*64),
              dict(path='index.html',bytes=4096,sha256='f'*64),
              dict(path='pkg/app.wasm',bytes=8388608,sha256='b'*64)],
             '7d5fb87255e45deefea8737e0eb7ab1ef9d0c216f18343817a16654029d8fb57'),
            ([dict(path='docs/日本語/index.html',bytes=42,sha256='c'*64)],
             'f03ef95aab98c0490d722f3e26b37f9e7003fab199b56768daf36fcfa94e60f2'),
            ([dict(path='assets/quote"and\\slash/page.html',bytes=7,sha256='d'*64)],
             '85e5ff9093d6df136ad3f8abd7f7207acdfd45e7871ba54da4764e56b2c6000e'),
        ]
        for entries, expected in cases:
            digest = hashlib.sha256(publisher.canonical_manifest(entries)).hexdigest()
            self.assertEqual(digest, expected, entries)


class TransportFailureTests(unittest.TestCase):
    """The retry loop must name the transport failure instead of collapsing it."""

    def setUp(self):
        self.saved = publisher.sleep
        publisher.sleep = lambda seconds: None
        self.stderr = io.StringIO()
        self.addCleanup(self.restore)

    def restore(self):
        publisher.sleep = self.saved

    def test_transport_failure_is_reported_with_its_cause(self):
        """A dead peer must surface the underlying transport error, not a generic one."""
        with self.assertRaises(RuntimeError) as caught:
            with contextlib.redirect_stderr(self.stderr):
                publisher.request('PUT', 'http://127.0.0.1:1/uploads/local-session/files/0', b'file', 'token')
        message = str(caught.exception)
        self.assertIn('ConnectionRefusedError', message)
        self.assertIn('PUT', message)
        self.assertIn('/uploads/local-session/files/0', message)
        self.assertNotIn('token', message)
        self.assertIn('ConnectionRefusedError', self.stderr.getvalue())

    def test_transport_failure_never_prints_the_bearer_token(self):
        """The rendered transport detail must not carry the credential."""
        with self.assertRaises(RuntimeError) as caught:
            with contextlib.redirect_stderr(self.stderr):
                publisher.request('GET', 'http://127.0.0.1:1/api/page-artifacts', None, 'super-secret-token')
        self.assertNotIn('super-secret-token', str(caught.exception))
        self.assertNotIn('super-secret-token', self.stderr.getvalue())

    def test_client_errors_are_not_retried(self):
        """A 4xx answer is a defect in the request, so it fails immediately."""
        class Handler(BaseHTTPRequestHandler):
            def log_message(self, format, *args):
                pass
            def do_GET(self):
                self.send_response(403)
                self.send_header('Content-Length','0')
                self.end_headers()
        server = ThreadingHTTPServer(('127.0.0.1',0),Handler)
        thread = threading.Thread(target=server.serve_forever,daemon=True)
        thread.start()
        self.addCleanup(lambda: (server.shutdown(), server.server_close(), thread.join()))
        base = 'http://127.0.0.1:'+str(server.server_port)
        with self.assertRaisesRegex(ValueError,'Publication HTTP 403'):
            publisher.request('GET', base+'/api/page-artifacts')

    def test_redirects_are_rejected(self):
        """A redirecting publication endpoint must never be followed."""
        class Handler(BaseHTTPRequestHandler):
            def log_message(self, format, *args):
                pass
            def do_GET(self):
                self.send_response(302)
                self.send_header('Location','http://example.com/other')
                self.send_header('Content-Length','0')
                self.end_headers()
        server = ThreadingHTTPServer(('127.0.0.1',0),Handler)
        thread = threading.Thread(target=server.serve_forever,daemon=True)
        thread.start()
        self.addCleanup(lambda: (server.shutdown(), server.server_close(), thread.join()))
        base = 'http://127.0.0.1:'+str(server.server_port)
        with self.assertRaisesRegex(ValueError,'redirect rejected'):
            publisher.request('GET', base+'/euv-release.json')

    def test_missing_resource_is_only_allowed_when_asked(self):
        """A 404 is an answer, so it is returned as missing, not raised."""
        class Handler(BaseHTTPRequestHandler):
            def log_message(self, format, *args):
                pass
            def do_GET(self):
                self.send_response(404)
                self.send_header('Content-Length','0')
                self.end_headers()
        server = ThreadingHTTPServer(('127.0.0.1',0),Handler)
        thread = threading.Thread(target=server.serve_forever,daemon=True)
        thread.start()
        self.addCleanup(lambda: (server.shutdown(), server.server_close(), thread.join()))
        base = 'http://127.0.0.1:'+str(server.server_port)
        self.assertIsNone(publisher.request('GET', base+'/euv-release.json', allow_missing=True))
        with self.assertRaisesRegex(ValueError,'Publication HTTP 404'):
            publisher.request('GET', base+'/euv-release.json')

    def test_server_errors_are_retried_and_then_reported(self):
        """A 5xx answer is retried, and the final failure names the status."""
        attempts = []
        class Handler(BaseHTTPRequestHandler):
            def log_message(self, format, *args):
                pass
            def do_GET(self):
                attempts.append(1)
                self.send_response(503)
                self.send_header('Content-Length','0')
                self.end_headers()
        server = ThreadingHTTPServer(('127.0.0.1',0),Handler)
        thread = threading.Thread(target=server.serve_forever,daemon=True)
        thread.start()
        self.addCleanup(lambda: (server.shutdown(), server.server_close(), thread.join()))
        base = 'http://127.0.0.1:'+str(server.server_port)
        with contextlib.redirect_stderr(io.StringIO()):
            with self.assertRaises(RuntimeError) as caught:
                publisher.request('GET', base+'/api/page-artifacts')
        self.assertEqual(len(attempts), publisher.ATTEMPTS)
        self.assertIn('HTTP 503', str(caught.exception))

    def test_an_unconfirmed_success_envelope_is_refused(self):
        """A 2xx answer without the server's success envelope is not a publication."""
        class Handler(BaseHTTPRequestHandler):
            def log_message(self, format, *args):
                pass
            def do_GET(self):
                body = b'{"data":{"uploadId":"x"}}'
                self.send_response(200)
                self.send_header('Content-Length',str(len(body)))
                self.end_headers()
                self.wfile.write(body)
        server = ThreadingHTTPServer(('127.0.0.1',0),Handler)
        thread = threading.Thread(target=server.serve_forever,daemon=True)
        thread.start()
        self.addCleanup(lambda: (server.shutdown(), server.server_close(), thread.join()))
        base = 'http://127.0.0.1:'+str(server.server_port)
        with self.assertRaisesRegex(ValueError,'unconfirmed success envelope'):
            publisher.envelope(publisher.request('GET', base+'/api/page-artifacts'))


if __name__ == '__main__':
    unittest.main()
