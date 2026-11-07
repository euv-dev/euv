'use strict';

const assert = require('node:assert/strict');
const fs = require('node:fs');
const os = require('node:os');
const path = require('node:path');
const { test } = require('node:test');
const { spawnSync } = require('node:child_process');
const crypto = require('node:crypto');
const { bundleExample, readExample } = require('../bundle-example');
const { buildPlan, packageIpa } = require('../build-client');
const { verifyNativeBytes, verifyIpa } = require('../verify-native');
const { verifyOutput } = require('../verify-output');

function fixture(t) {
  const root = fs.mkdtempSync(path.join(os.tmpdir(), 'euv-platform-test-'));
  t.after(() => fs.rmSync(root, { recursive: true, force: true }));
  const source = path.join(root, 'www');
  const destination = path.join(root, 'cache');
  fs.mkdirSync(path.join(source, 'pkg'), { recursive: true });
  fs.writeFileSync(path.join(source, 'index.html'), '<html>local platform fixture</html>');
  fs.writeFileSync(path.join(source, 'pkg/app.wasm'), Buffer.from([0, 97, 115, 109, 1, 0, 0, 0]));
  bundleExample({ source, destination });
  return { root, source, destination };
}

function nativeFixture(platform, snapshot) {
  let header;
  if (platform === 'linux') header = Buffer.from([127, 69, 76, 70]);
  else if (platform === 'windows') {
    header = Buffer.alloc(80);
    header.write('MZ');
    header.writeUInt32LE(64, 60);
    header.write('PE\0\0', 64);
  } else header = Buffer.from([207, 250, 237, 254]);
  return Buffer.concat([header, snapshot.manifestBytes, ...snapshot.files.map((file) => file.bytes)]);
}

test('iOS and Android enable the same native cache bridge permissions', () => {
  const capability = JSON.parse(fs.readFileSync(path.resolve(__dirname, '../../src-tauri/capabilities/default.json'), 'utf8'));
  assert.ok(capability.platforms.includes('iOS'));
  assert.ok(capability.platforms.includes('android'));
  for (const permission of ['allow-load-cached-resource', 'allow-wait-for-cached-resource', 'allow-update-cache', 'allow-resolve-bridge-group-permissions']) {
    assert.ok(capability.permissions.includes(permission), permission);
  }
});

test('client plans select real host-compatible targets and installer formats', () => {
  assert.deepEqual(buildPlan('windows', 'win32', 'x64').bundles, ['nsis', 'msi']);
  assert.equal(buildPlan('windows', 'win32', 'x64').target, 'x86_64-pc-windows-msvc');
  assert.equal(buildPlan('macos', 'darwin', 'arm64').target, 'aarch64-apple-darwin');
  assert.equal(buildPlan('macos', 'darwin', 'x64').target, 'x86_64-apple-darwin');
  assert.deepEqual(buildPlan('linux', 'linux', 'x64').bundles, ['deb', 'appimage']);
  assert.equal(buildPlan('linux', 'linux', 'arm64').target, 'aarch64-unknown-linux-gnu');
  assert.equal(buildPlan('ios', 'darwin', 'arm64').target, 'aarch64-apple-ios');
  assert.equal(buildPlan('ios-simulator', 'darwin', 'arm64').target, 'aarch64-apple-ios-sim');
  assert.equal(buildPlan('ios-simulator', 'darwin', 'x64').target, 'x86_64-apple-ios');
  assert.throws(() => buildPlan('windows', 'darwin', 'arm64'), /must be built/);
  assert.throws(() => buildPlan('windows', 'win32', 'arm64'), /x64 runner/);
  assert.throws(() => buildPlan('unknown'), /Unsupported platform/);
});

test('all four platforms verify the same complete local page snapshot', (t) => {
  const options = fixture(t);
  const snapshot = readExample(options.source);
  for (const platform of ['windows', 'macos', 'linux', 'ios', 'ios-simulator']) {
    const result = verifyNativeBytes(nativeFixture(platform, snapshot), platform, options);
    assert.equal(result.files.length, 2);
    assert.deepEqual(result.files, snapshot.manifest.files);
  }
});

test('native verification rejects wrong formats, missing provenance and incomplete payloads', (t) => {
  const options = fixture(t);
  const snapshot = readExample(options.source);
  assert.throws(() => verifyNativeBytes(nativeFixture('linux', snapshot), 'windows', options), /Wrong native/);
  assert.throws(() => verifyNativeBytes(Buffer.concat([Buffer.from([127, 69, 76, 70]), ...snapshot.files.map((file) => file.bytes)]), 'linux', options), /provenance/);
  assert.throws(() => verifyNativeBytes(Buffer.concat([Buffer.from([127, 69, 76, 70]), snapshot.manifestBytes, snapshot.files[0].bytes]), 'linux', options), /missing local example file/);
});

test('unsigned IPA packaging preserves the actual app executable and local payload', (t) => {
  const options = fixture(t);
  const app = path.join(options.root, 'Euv.app');
  fs.mkdirSync(app);
  fs.writeFileSync(path.join(app, 'Info.plist'), '<?xml version="1.0"?><plist version="1.0"><dict><key>CFBundleExecutable</key><string>Euv</string><key>CFBundleSupportedPlatforms</key><array><string>iPhoneOS</string></array><key>UIApplicationSceneManifest</key><dict><key>UIApplicationSupportsMultipleScenes</key><true/></dict></dict></plist>');
  fs.writeFileSync(path.join(app, 'Euv'), nativeFixture('ios', readExample(options.source)), { mode: 0o755 });
  const ipa = path.join(options.root, 'fixture-unsigned.ipa');
  packageIpa(app, ipa);
  assert.equal(verifyIpa(ipa, options).files.length, 2);
  const plist = path.join(app, 'Info.plist');
  const devicePlist = fs.readFileSync(plist, 'utf8');
  fs.writeFileSync(plist, devicePlist.replace('iPhoneOS', 'iPhoneSimulator'));
  packageIpa(app, ipa);
  assert.throws(() => verifyIpa(ipa, options), /device IPA/);
  fs.writeFileSync(plist, devicePlist.replace('<true/>', '<false/>'));
  packageIpa(app, ipa);
  assert.throws(() => verifyIpa(ipa, options), /scene lifecycle/);
  fs.writeFileSync(plist, devicePlist);
  fs.writeFileSync(path.join(app, 'Euv'), Buffer.from([207, 250, 237, 254]));
  packageIpa(app, ipa);
  assert.throws(() => verifyIpa(ipa, options), /provenance/);
});

test('client CLI fails before native build when local pages are absent', (t) => {
  const options = fixture(t);
  const platform = { darwin: 'macos', linux: 'linux', win32: 'windows' }[process.platform];
  const result = spawnSync(process.execPath, [path.resolve(__dirname, '../build-client.js'), platform], {
    env: { ...process.env, EUV_WEB_DIST: path.join(options.root, 'absent') }, encoding: 'utf8',
  });
  assert.equal(result.status, 1);
  assert.match(result.stderr, /Client build failed/);
});

test('output verification rejects tampered files and incomplete package sets', (t) => {
  const options = fixture(t);
  const output = path.join(options.root, 'windows');
  fs.mkdirSync(output);
  const snapshot = readExample(options.source);
  const fixtures = {
    'euv-windows-x64.exe': nativeFixture('windows', snapshot),
    'euv-windows-x64-installer-fixture.exe': Buffer.from('installer test fixture'),
    'euv-windows-x64-fixture.msi': Buffer.from('MSI test fixture'),
  };
  for (const [name, bytes] of Object.entries(fixtures)) fs.writeFileSync(path.join(output, name), bytes);
  const report = {
    format: 1, platform: 'windows', arch: 'x64', pages: snapshot.manifest.files,
    files: Object.entries(fixtures).map(([name, bytes]) => ({ path: name, size: bytes.length, sha256: crypto.createHash('sha256').update(bytes).digest('hex') })),
  };
  const reportFile = path.join(output, 'euv-windows-x64-build.json');
  fs.writeFileSync(reportFile, JSON.stringify(report));
  assert.equal(verifyOutput('windows', output, options).files.length, 3);
  fs.appendFileSync(path.join(output, 'euv-windows-x64-fixture.msi'), 'tampering');
  assert.throws(() => verifyOutput('windows', output, options), /integrity failure/);
  fs.unlinkSync(path.join(output, 'euv-windows-x64-fixture.msi'));
  report.files = report.files.filter((file) => !file.path.endsWith('.msi'));
  fs.writeFileSync(reportFile, JSON.stringify(report));
  assert.throws(() => verifyOutput('windows', output, options), /Missing required/);
});
