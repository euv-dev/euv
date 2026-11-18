'use strict';

const assert = require('node:assert/strict');
const { spawnSync } = require('node:child_process');
const fs = require('node:fs');
const path = require('node:path');
const { test } = require('node:test');

const script = path.resolve(__dirname, '../validation/smoke-ios.py');
// Import the actual Python selector without invoking main or requiring a Simulator.
const select = String.raw`
import importlib.util
import json
import sys

spec = importlib.util.spec_from_file_location('euv_ios_smoke', sys.argv[1])
module = importlib.util.module_from_spec(spec)
spec.loader.exec_module(module)
request = json.load(sys.stdin)
try:
    runtime = module.select_ios_runtime(request['runtimes'], request['sdk'])
except ValueError as error:
    print(json.dumps({'error': str(error)}))
else:
    print(json.dumps({'runtime': runtime}))
`;

function choose(runtimes, sdk = '18.5') {
  const result = spawnSync('python3', ['-B', '-c', select, script], {
    input: JSON.stringify({ runtimes, sdk }), encoding: 'utf8',
  });
  assert.ifError(result.error);
  assert.equal(result.status, 0, result.stderr);
  return JSON.parse(result.stdout);
}

function runtime(version, overrides = {}) {
  return {
    version, isAvailable: true, platform: 'iOS',
    identifier: `com.apple.CoreSimulator.SimRuntime.iOS-${version.replaceAll('.', '-')}`,
    ...overrides,
  };
}

function assertNoCompatible(runtimes) {
  const result = choose(runtimes);
  assert.equal(result.runtime, undefined);
  assert.match(result.error, /No available installed iOS Simulator runtime compatible with active iphonesimulator SDK 18\.5/);
  assert.match(result.error, /runtime <= SDK/);
}

test('iOS runtime uses SDK 18.5 instead of a newer installed Xcode runtime 26.1', () => {
  const expected = runtime('18.5');
  assert.deepEqual(choose([expected, runtime('26.1')]).runtime, expected);
  assert.deepEqual(choose([runtime('26.1'), expected]).runtime, expected);
});

test('iOS runtime normalizes SDK and runtime trailing zero patch versions', () => {
  for (const [sdk, version] of [['18.5', '18.5.0'], ['18.5.0', '18.5'], ['18.5.0.0', '18.5.0']]) {
    const expected = runtime(version);
    assert.deepEqual(choose([runtime('18.4'), expected, runtime('26.1')], sdk).runtime, expected);
  }
});

test('iOS runtime compares patch versions without selecting a newer patch than SDK', () => {
  const expected = runtime('18.5.1');
  const runtimes = [runtime('18.5.2'), expected, runtime('18.5.0')];
  assert.deepEqual(choose(runtimes, '18.5.1').runtime, expected);
  assert.equal(choose(runtimes, '18.5').runtime.version, '18.5.0');
});

test('iOS runtime falls back to the newest older installed version numerically', () => {
  const expected = runtime('18.4');
  assert.deepEqual(choose([runtime('17.10'), runtime('17.9'), runtime('26.1'), expected]).runtime, expected);
  assert.equal(choose([runtime('17.9'), runtime('17.10')]).runtime.version, '17.10');
});

test('iOS runtime excludes unavailable and non-iOS candidates before ranking', () => {
  const expected = runtime('18.4');
  assert.deepEqual(choose([
    runtime('18.5', { isAvailable: false }),
    runtime('18.5', { isAvailable: undefined }),
    runtime('18.5', { platform: 'tvOS', identifier: 'com.apple.CoreSimulator.SimRuntime.tvOS-18-5' }),
    expected,
  ]).runtime, expected);
});

test('iOS runtime retains platform and identifier based iOS detection', () => {
  const identified = runtime('18.5');
  delete identified.platform;
  assert.deepEqual(choose([identified]).runtime, identified);
  const platform = runtime('18.5', { identifier: 'platform-only-runtime' });
  assert.deepEqual(choose([platform]).runtime, platform);
});

test('iOS runtime clearly fails when only newer runtimes are installed', () => {
  assertNoCompatible([runtime('18.5.1'), runtime('26.1')]);
});

test('iOS runtime clearly fails when all installed iOS runtimes are unavailable', () => {
  assertNoCompatible([runtime('18.5', { isAvailable: false })]);
});

test('iOS runtime clearly fails when no installed runtime is iOS', () => {
  assertNoCompatible([
    runtime('18.5', { platform: 'tvOS', identifier: 'com.apple.CoreSimulator.SimRuntime.tvOS-18-5' }),
  ]);
});

test('iOS runtime clearly fails when no runtimes are installed', () => {
  assertNoCompatible([]);
});

test('iOS smoke queries the active SDK and uses the tested selector with diagnostics', () => {
  const source = fs.readFileSync(script, 'utf8');
  assert.match(source, /sdk_version = capture\(\['xcrun', '--sdk', 'iphonesimulator', '--show-sdk-version'\]\)/);
  assert.match(source, /runtime = select_ios_runtime\(runtimes, sdk_version\)/);
  assert.match(source, /'simulator_sdk_version': sdk_version/);
  assert.match(source, /'runtime_version': runtime\['version'\]/);
  assert.match(source, /'runtime_identifier': runtime\['identifier'\]/);
  assert.match(source, /report\.update\(runtime_report\)/);
});
