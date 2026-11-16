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

test('iOS display policy gates the scene display-link API by SDK availability', { skip: process.platform !== 'darwin' }, (t) => {
  const temporary = fs.mkdtempSync(path.join(os.tmpdir(), 'euv-ios-display-'));
  t.after(() => fs.rmSync(temporary, { recursive: true, force: true }));
  const legacyHeader = path.join(temporary, 'legacy-sdk.h');
  fs.writeFileSync(legacyHeader, [
    '#import <UIKit/UIKit.h>',
    '#undef __IPHONE_OS_VERSION_MAX_ALLOWED',
    '#define __IPHONE_OS_VERSION_MAX_ALLOWED __IPHONE_26_0',

    '',
  ].join('\n'));
  for (const [sdk, target] of [
    ['iphoneos', 'arm64-apple-ios15.0'],
    ['iphonesimulator', 'arm64-apple-ios15.0-simulator'],
  ]) {
    const sdkPath = spawnSync('xcrun', ['--sdk', sdk, '--show-sdk-path'], { encoding: 'utf8', timeout: 30000 });
    assert.equal(sdkPath.status, 0, sdkPath.error?.message || sdkPath.stderr);
    for (const legacy of [false, true]) {
      const compileArgs = ['clang', '-fsyntax-only', '-fobjc-arc', '-Wall', '-Wextra', '-Werror',
        '-target', target, '-isysroot', sdkPath.stdout.trim()];
      if (legacy) compileArgs.push('-include', legacyHeader);
      compileArgs.push(path.join(root, 'src-tauri/apple/display.m'));
      const compile = spawnSync('xcrun', compileArgs, { encoding: 'utf8', timeout: 60000 });
      assert.equal(compile.status, 0, `${sdk} legacy=${legacy}: ${compile.error?.message || compile.stderr}`);
      if (legacy) {
        const preprocess = spawnSync('xcrun', compileArgs.map((arg) => arg === '-fsyntax-only' ? '-E' : arg),
          { encoding: 'utf8', timeout: 60000, maxBuffer: 64 * 1024 * 1024 });
        assert.equal(preprocess.status, 0, preprocess.error?.message || preprocess.stderr);
        assert.ok(!/\[web\.window\.windowScene\s+displayLinkWithTarget:/.test(preprocess.stdout),
          `${sdk}: legacy SDK must not compile the iOS 27 scene selector`);
      }
    }
  }
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
