'use strict';
const assert = require('node:assert/strict');
const fs = require('node:fs');
const path = require('node:path');
const vm = require('node:vm');
const { test } = require('node:test');
const { extract, fixture, burst } = require('../benchmark/benchmark-viewport');
const root = path.resolve(__dirname, '../..');
const script = extract(fs.readFileSync(path.join(root, 'src-tauri/src/cache/const.rs'), 'utf8'));

test('a resize burst performs one layout read and no redundant probe or CSS writes', () => {
  assert.deepEqual(burst(script), { layoutReads: 1, probeAllocations: 0, safeAreaWrites: 0 });
});

test('safe-area coalescing still applies changed orientation and native insets', () => {
  const page = fixture(script);
  page.reset();
  page.setTop('48px');
  page.dispatch('orientationchange');
  page.dispatch('resize');
  page.flush();
  assert.equal(page.document.documentElement.style.getPropertyValue('--euv-mobile-safe-top'), '48px');
  assert.equal(page.stats.safeAreaWrites, 1);
  page.reset();
  page.window.__EUV_NATIVE_SAFE_TOP__ = 42;
  page.dispatch('resize');
  page.flush();
  assert.equal(page.document.documentElement.style.getPropertyValue('--euv-mobile-safe-top'), '42px');
  assert.equal(page.stats.layoutReads, 0);
});

test('native keyboard insets remain authoritative after viewport coalescing', () => {
  const page = fixture(script);
  page.dispatch('euv:keyboard-height', { detail: { height: 200 } });
  assert.equal(page.window.__EUV_KEYBOARD_HEIGHT__, 200);
  assert.equal(page.document.documentElement.style.getPropertyValue('--euv-keyboard-height'), '200px');
});

test('Android GPU acceleration does not require a permanent full-window hardware texture', () => {
  const manifest = fs.readFileSync(path.join(root, 'src-tauri/gen/android/app/src/main/AndroidManifest.xml'), 'utf8');
  const activity = fs.readFileSync(path.join(root, 'src-tauri/gen/android/app/src/main/java/com/euv/MainActivity.kt'), 'utf8');
  assert.match(manifest, /<application[\s\S]*?android:hardwareAccelerated="true"/);
  assert.match(activity, /setLayerType\(View.LAYER_TYPE_NONE, null\)/);
  assert.doesNotMatch(activity, /LAYER_TYPE_SOFTWARE|LAYER_TYPE_HARDWARE|setRenderPriority/);
  assert.match(activity, /hardwareAccelerated=\$\{view.isHardwareAccelerated\}/);
});

test('native startup does not override engine safety policies to advertise GPU support', () => {
  const main = fs.readFileSync(path.join(root, 'src-tauri/src/main.rs'), 'utf8');
  assert.doesNotMatch(main, /WEBVIEW2_ADDITIONAL_BROWSER_ARGUMENTS|--ignore-gpu-blocklist|--enable-unsafe-webgpu/);
});

test('GPU diagnostics allocate no context during startup and report unavailable APIs honestly', async () => {
  let contexts = 0;
  const window = { isSecureContext: false };
  vm.runInNewContext(fs.readFileSync(path.join(root, 'resources/runtime/gpu-probe.js'), 'utf8'), {
    window, navigator: {}, document: { createElement() { contexts++; return { getContext() { return null; } }; } },
    setTimeout, clearTimeout,
  });
  assert.equal(contexts, 0);
  const result = await window.__EUV_GPU_PROBE__();
  assert.equal(contexts, 1);
  assert.equal(result.webgl.available, false);
  assert.equal(result.webgpu.available, false);
  assert.equal(result.secureContext, false);
});
