import contextlib
import hashlib
from http.server import BaseHTTPRequestHandler, ThreadingHTTPServer
import io
import json
from pathlib import Path
import tarfile
import tempfile
import threading
import unittest

import publish_release as publisher


class PublicationTests(unittest.TestCase):
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
        self.served, self.chunks, self.calls = {}, {}, []
        self.digest = None
        self.size = None
        self.corrupt = False
        self.token = 'local-test-credential'
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
            def do_GET(self):
                owner.calls.append(('GET',self.path))
                if self.path == '/api/page-artifacts':
                    self.reply(json.dumps(dict(success=True,data=dict(artifacts=[dict(name='euv',sha256=owner.digest)]))).encode())
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
                if self.path == '/api/page-artifacts/euv/uploads':
                    metadata = json.loads(data)
                    owner.digest, owner.size = metadata['archiveSha256'], metadata['archiveBytes']
                    self.reply(json.dumps(dict(success=True,data=dict(uploadId='local-session',chunkSizeBytes=100,chunkCount=(owner.size+99)//100))).encode())
                elif self.path == '/api/page-artifacts/euv/uploads/local-session/commit':
                    archive = b''.join(owner.chunks[index] for index in sorted(owner.chunks))
                    assert len(archive) == owner.size and hashlib.sha256(archive).hexdigest() == owner.digest
                    with tarfile.open(fileobj=io.BytesIO(archive)) as bundle:
                        owner.served = {member.name: bundle.extractfile(member).read() for member in bundle.getmembers()}
                    if owner.corrupt:
                        owner.served['pkg/app.wasm'] = b'tampered'
                    self.reply(b'{"success":true}')
                else:
                    self.reply(b'missing',404)
            def do_PUT(self):
                owner.calls.append(('PUT',self.path))
                data = self.body()
                assert self.headers.get('Authorization') == 'Bearer '+owner.token
                assert self.headers['X-Chunk-SHA256'] == hashlib.sha256(data).hexdigest()
                owner.chunks[int(self.path.rsplit('/',1)[1])] = data
                self.reply(b'{"success":true}')
        self.server = ThreadingHTTPServer(('127.0.0.1',0),Handler)
        self.thread = threading.Thread(target=self.server.serve_forever,daemon=True)
        self.thread.start()
        self.base = 'http://127.0.0.1:'+str(self.server.server_port)
    def tearDown(self):
        self.server.shutdown(); self.server.server_close(); self.thread.join(); self.temp.cleanup()
    def publish(self):
        return publisher.publish(self.directory,'0.36.4',self.token,self.base+'/api/page-artifacts/',self.base+'/pages/euv/')
    def test_chunk_upload_commits_one_complete_artifact_and_reads_every_live_resource(self):
        receipt = self.publish()
        self.assertEqual(len(receipt['files']),len(self.resources))
        self.assertEqual(self.served[publisher.MANIFEST],self.descriptor)
        self.assertEqual({item['path'] for item in receipt['files']},set(self.resources))
        self.assertEqual(sum(method=='POST' and path.endswith('/commit') for method,path in self.calls),1)
        self.assertFalse(any('github/pages/sync' in path for _,path in self.calls))
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
                publisher.request('PUT', 'http://127.0.0.1:1/uploads/local-session/chunks/0', b'chunk', 'token')
        message = str(caught.exception)
        self.assertIn('ConnectionRefusedError', message)
        self.assertIn('PUT', message)
        self.assertIn('/uploads/local-session/chunks/0', message)
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


if __name__ == '__main__':
    unittest.main()
