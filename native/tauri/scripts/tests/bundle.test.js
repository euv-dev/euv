'use strict';

const assert = require('node:assert/strict');
const fs = require('node:fs');
const os = require('node:os');
const path = require('node:path');
const { execFileSync, spawnSync } = require('node:child_process');
const { test } = require('node:test');
const { MANIFEST_NAME, bundleExample, readExample, verifyBundle } = require('../build/bundle-example');
const { verifyApk } = require('../validation/verify-apk');

function fixture(t) {
  const root = fs.mkdtempSync(path.join(os.tmpdir(), 'euv-local-bundle-'));
  t.after(() => fs.rmSync(root, { recursive: true, force: true }));
  const source = path.join(root, 'example', 'www');
  const destination = path.join(root, 'bundled-cache');
  fs.mkdirSync(path.join(source, 'pkg'), { recursive: true });
  fs.writeFileSync(path.join(source, 'index.html'), '<html><script src="pkg/euv.js"></script></html>');
  fs.writeFileSync(path.join(source, 'pkg', 'euv.js'), 'export const wasm = "./euv_bg.wasm";');
  fs.writeFileSync(path.join(source, 'pkg', 'euv_bg.wasm'), Buffer.from([0, 97, 115, 109, 1, 0, 0, 0]));
  return { root, source, destination };
}

function apkFixture(file, bytes, library = 'lib/arm64-v8a/libeuv_lib.so') {
  execFileSync('python3', ['-c',
    'import base64,sys,zipfile; z=zipfile.ZipFile(sys.argv[1],"w",zipfile.ZIP_DEFLATED); z.writestr(sys.argv[2],base64.b64decode(sys.argv[3])); z.close()',
    file, library, bytes.toString('base64')]);
}

test('local bundling copies the complete output without fetching remote pages', (t) => {
  const options = fixture(t);
  fs.mkdirSync(path.join(options.source, 'images'));
  fs.writeFileSync(path.join(options.source, 'images', 'logo.svg'), '<svg/>');
  const result = bundleExample(options);
  assert.equal(result.changed, true);
  assert.equal(result.source, 'euv-example');
  assert.equal(result.files.length, 4);
  for (const file of result.files) {
    assert.deepEqual(fs.readFileSync(path.join(options.source, file.path)), fs.readFileSync(path.join(options.destination, file.path)));
    assert.match(file.sha256, /^[a-f0-9]{64}$/);
  }
  assert.deepEqual(verifyBundle(options).files, result.files);
});

test('identical local bundles preserve timestamps, including after Cargo creates its marker', (t) => {
  const options = fixture(t);
  bundleExample(options);
  const index = path.join(options.destination, 'index.html');
  const before = fs.statSync(index, { bigint: true }).mtimeNs;
  fs.writeFileSync(path.join(options.destination, '.empty'), '');
  assert.equal(bundleExample(options).changed, false);
  assert.equal(fs.statSync(index, { bigint: true }).mtimeNs, before);
});

test('publishing a new local snapshot removes stale files from older bundles', (t) => {
  const options = fixture(t);
  bundleExample(options);
  fs.writeFileSync(path.join(options.destination, 'old-remote.js'), 'stale remote bytes');
  fs.writeFileSync(path.join(options.source, 'index.html'), '<html>new local output</html>');
  assert.equal(bundleExample(options).changed, true);
  assert.equal(fs.existsSync(path.join(options.destination, 'old-remote.js')), false);
  verifyBundle(options);
});

test('missing or invalid local output fails without replacing the last valid bundle', (t) => {
  const options = fixture(t);
  bundleExample(options);
  const manifest = fs.readFileSync(path.join(options.destination, MANIFEST_NAME));
  fs.unlinkSync(path.join(options.source, 'pkg', 'euv_bg.wasm'));
  assert.throws(() => bundleExample(options), /WebAssembly/);
  assert.deepEqual(fs.readFileSync(path.join(options.destination, MANIFEST_NAME)), manifest);
  fs.writeFileSync(path.join(options.source, 'pkg', 'euv_bg.wasm'), 'not wasm');
  assert.throws(() => bundleExample(options), /WebAssembly/);
  fs.writeFileSync(path.join(options.source, 'index.html'), '');
  assert.throws(() => bundleExample(options), /index.html/);
});

test('a failed snapshot publication restores the previous local bundle', (t) => {
  const options = fixture(t);
  bundleExample(options);
  const previous = fs.readFileSync(path.join(options.destination, MANIFEST_NAME));
  fs.writeFileSync(path.join(options.source, 'index.html'), '<html>next build</html>');
  const rename = fs.renameSync;
  t.mock.method(fs, 'renameSync', (from, to) => {
    if (from.includes('.bundled-cache-stage-') && !from.endsWith('-previous') && to === options.destination) {
      throw new Error('Injected publication failure');
    }
    return rename(from, to);
  });
  assert.throws(() => bundleExample(options), /publication failure/);
  assert.deepEqual(fs.readFileSync(path.join(options.destination, MANIFEST_NAME)), previous);
  assert.equal(fs.readdirSync(options.root).some((name) => name.startsWith('.bundled-cache-stage-')), false);
});

test('bundling rejects symlinks, reserved filenames, and overlapping directories', (t) => {
  const options = fixture(t);
  fs.symlinkSync(path.join(options.source, 'index.html'), path.join(options.source, 'linked.html'));
  assert.throws(() => bundleExample(options), /Unsupported/);
  fs.unlinkSync(path.join(options.source, 'linked.html'));
  fs.writeFileSync(path.join(options.source, MANIFEST_NAME), '{}');
  assert.throws(() => bundleExample(options), /reserved/);
  fs.unlinkSync(path.join(options.source, MANIFEST_NAME));
  assert.throws(() => bundleExample({ ...options, destination: path.join(options.source, 'nested') }), /overlap/);
  assert.throws(() => bundleExample({ ...options, destination: path.dirname(options.source) }), /overlap/);
});

test('bundle CLI exits nonzero when local example output is absent', (t) => {
  const options = fixture(t);
  const result = spawnSync(process.execPath, [path.resolve(__dirname, '../build/bundle-example.js')], {
    env: { ...process.env, EUV_WEB_DIST: path.join(options.root, 'missing') }, encoding: 'utf8',
  });
  assert.equal(result.status, 1);
  assert.match(result.stderr, /Cannot bundle local example/);
});

test('APK verification checks actual ZIP native payload bytes against local output', (t) => {
  const options = fixture(t);
  bundleExample(options);
  const snapshot = readExample(options.source);
  const prefix = Buffer.concat([Buffer.from([127, 69, 76, 70]), snapshot.manifestBytes]);
  const apk = path.join(options.root, 'fixture.apk');
  apkFixture(apk, Buffer.concat([prefix, ...snapshot.files.map((file) => file.bytes)]));
  assert.equal(verifyApk(apk, options).files, 3);
  apkFixture(apk, Buffer.concat([prefix, ...snapshot.files.filter((file) => !file.path.endsWith('.wasm')).map((file) => file.bytes)]));
  assert.throws(() => verifyApk(apk, options), /Local example file missing/);
  apkFixture(apk, Buffer.concat([Buffer.from([127, 69, 76, 70]), ...snapshot.files.map((file) => file.bytes)]));
  assert.throws(() => verifyApk(apk, options), /provenance/);
  apkFixture(apk, Buffer.from('not an APK native library'), 'assets/index.html');
  assert.throws(() => verifyApk(apk, options), /does not contain/);
});

test('APK verification rejects a bundle from a different example build', (t) => {
  const options = fixture(t);
  bundleExample(options);
  fs.writeFileSync(path.join(options.source, 'index.html'), '<html>different build</html>');
  assert.throws(() => verifyApk(path.join(options.root, 'missing.apk'), options), /do not match/);
});

test('generated Tauri config uses local bundling instead of remote prefetch', () => {
  const generator = fs.readFileSync(path.resolve(__dirname, '../config/apply-config.js'), 'utf8');
  const config = JSON.parse(fs.readFileSync(path.resolve(__dirname, '../../src-tauri/tauri.conf.json'), 'utf8'));
  assert.match(generator, /node scripts\/build\/bundle-example.js && node scripts\/config\/apply-config.js/);
  assert.doesNotMatch(generator, /node scripts\/prefetch-cache.js/);
  assert.equal(config.build.beforeBuildCommand, 'node scripts/build/bundle-example.js && node scripts/config/apply-config.js');
});
