'use strict';

const assert = require('node:assert/strict');
const { spawnSync } = require('node:child_process');
const fs = require('node:fs');
const path = require('node:path');
const { test } = require('node:test');

const script = path.resolve(__dirname, '../validation/smoke-ios.py');

test('frame evidence window includes long acceptance sessions and rounds upward', () => {
  const probe = String.raw`
import importlib.util
import sys
spec = importlib.util.spec_from_file_location('euv_ios_smoke', sys.argv[1])
module = importlib.util.module_from_spec(spec)
spec.loader.exec_module(module)
assert module.display_log_window_seconds(100, 100) == 1
assert module.display_log_window_seconds(100, 100.01) == 1
assert module.display_log_window_seconds(100, 1000) == 900
assert module.display_log_window_seconds(100, 1000.01) == 901
assert module.display_log_window_seconds(100, 3700) == 3600
`;
  const result = spawnSync('python3', ['-B', '-c', probe, script], { encoding: 'utf8' });
  assert.ifError(result.error);
  assert.equal(result.status, 0, result.stderr);
});

test('frame query follows the acceptance start and still requires real samples', () => {
  const source = fs.readFileSync(script, 'utf8');
  const started = source.indexOf('test_started = time.monotonic()');
  assert.ok(started >= 0 && started < source.indexOf('run(command, stdout=log'));
  assert.match(source, /display_log_window_seconds\(test_started, time\.monotonic\(\)\)/);
  assert.ok(source.includes("'--last', f'{display_log_seconds}s'"));
  assert.ok(source.includes("raise AssertionError('No real requestAnimationFrame sample from the production WebView')"));
});
