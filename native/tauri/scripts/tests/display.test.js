'use strict';

const assert = require('node:assert/strict');
const fs = require('node:fs');
const path = require('node:path');
const { test } = require('node:test');
const os = require('node:os');
const { spawnSync } = require('node:child_process');
const { getTauriConfig } = require('../config/apply-config');

const root = path.resolve(__dirname, '../..');
const config = JSON.parse(fs.readFileSync(path.join(root, 'config/app.config.json'), 'utf8'));

test('Apple accepts the production hardware-derived display-link range', { skip: process.platform !== 'darwin' }, (t) => {
  const temporary = fs.mkdtempSync(path.join(os.tmpdir(), 'euv-native-display-'));
  t.after(() => fs.rmSync(temporary, { recursive: true, force: true }));
  const executable = path.join(temporary, 'display-probe');
  const compile = spawnSync('xcrun', ['clang', '-fobjc-arc', '-mmacosx-version-min=14.0',
    '-Wall', '-Wextra', '-Werror', '-framework', 'AppKit', '-framework', 'QuartzCore', '-framework', 'WebKit',
    path.join(root, 'src-tauri/apple/display.m'), path.join(__dirname, './display-probe.m'), '-o', executable],
    { encoding: 'utf8', timeout: 60000 });
  assert.equal(compile.status, 0, compile.error?.message || compile.stderr);
  const probe = spawnSync(executable, [], { encoding: 'utf8', timeout: 30000 });
  assert.equal(probe.status, 0, probe.error?.message || probe.stderr);
});

test('native defaults enable immersive presentation and maximum hardware refresh', () => {
  assert.equal(config.ui.immersiveMode, true);
  assert.equal(config.ui.maxFrameRateEnabled, true);
  const window = getTauriConfig(config).app.windows[0];
  assert.equal(window.decorations, false);
  assert.equal(window.fullscreen, false);
  assert.equal(window.width, config.ui.window.width);
  assert.equal(window.height, config.ui.window.height);
});

test('native configuration preserves explicit desktop fullscreen and decoration choices', () => {
  const ordinary = structuredClone(config);
  ordinary.ui.immersiveMode = false;
  ordinary.ui.window.fullscreen = true;
  const window = getTauriConfig(ordinary).app.windows[0];
  assert.equal(window.decorations, true);
  assert.equal(window.fullscreen, true);
  assert.equal(config.ui.immersiveMode, true);
  assert.equal(config.ui.window.fullscreen, false);
});

test('Apple native preferences opt into ProMotion and view-controller status-bar ownership', () => {
  const plist = fs.readFileSync(path.join(root, 'src-tauri/Info.ios.plist'), 'utf8');
  assert.match(plist, /<key>CADisableMinimumFrameDurationOnPhone<\/key>\s*<true\/>/);
  assert.match(plist, /<key>UIViewControllerBasedStatusBarAppearance<\/key>\s*<true\/>/);
});

test('Apple refresh preferences derive from the attached display without private WebKit APIs', () => {
  const source = fs.readFileSync(path.join(root, 'src-tauri/apple/display.m'), 'utf8');
  assert.match(source, /window\.screen\.maximumFramesPerSecond/);
  assert.match(source, /CAFrameRateRangeMake\(\(float\)rate, \(float\)rate, \(float\)rate\)/);
  assert.match(source, /NSWindowDidChangeScreenNotification/);
  assert.match(source, /UIApplicationDidEnterBackgroundNotification/);
  assert.match(source, /removeObserver:self/);
  assert.match(source, /weak\) EuvDisplayPolicy \*policy/);
  assert.match(source, /@"guaranteed": @NO/);
  assert.doesNotMatch(source, /_setEnabled|_updateVisibleContentRects|PreferPageRenderingUpdatesNear60FPS|setValue:.*forKey/);
  assert.doesNotMatch(source, /CAFrameRateRangeMake\([^\n]*(?:60|120|144)/);
});
