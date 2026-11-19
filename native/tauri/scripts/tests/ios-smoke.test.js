'use strict';

const assert = require('node:assert/strict');
const fs = require('node:fs');
const path = require('node:path');
const { test } = require('node:test');
const { execFileSync } = require('node:child_process');
const { pythonCommand } = require('../validation/verify-native');

const script = path.resolve(__dirname, '../validation/smoke-ios.py');
const source = fs.readFileSync(script, 'utf8');
const swift = source.split("SWIFT_TEST = r'''")[1].split("'''")[0];
const counter = swift.slice(swift.indexOf('func assertCounter('), swift.indexOf('func attach('));

// These are source-contract regressions, not a substitute for Simulator XCTest.
test('iOS counter resolves split text within the unique semantic paragraph', () => {
  assert.match(counter, /app\.otherElements\s*\.containing\(\.staticText, identifier: "The current count is"\)/);
  assert.match(counter, /children\(matching: \.staticText\)\s*\.matching\(identifier: "The current count is"\)\.count == 1/);
  assert.match(counter, /guard paragraphs\.count == 1 else \{ return false \}/);
  assert.match(counter, /paragraphs\[0\]\.children\(matching: \.staticText\)/);
  assert.match(counter, /NSPredicate\(format: "label MATCHES %@", "\^-\?\[0-9\]\+\$"\)/);
  assert.match(counter, /guard numbers\.count == 1 else \{ return false \}/);
  assert.match(counter, /numbers\.element\(boundBy: 0\)\.label == String\(expected\)/);
  assert.doesNotMatch(counter.slice(counter.indexOf('let paragraphs')), /firstMatch|label CONTAINS|app\.staticTexts/);
  assert.doesNotMatch(swift, /The current count is [01]/);
});

test('iOS counter also resolves one combined paragraph without accepting unrelated numbers', () => {
  assert.match(counter, /app\.webViews\.firstMatch\.staticTexts\.matching\(NSPredicate\(/);
  const pattern = counter.match(/"label MATCHES %@", "(\^The current count is [^"]+)"/)[1].replace('\\\\.', '\\.');
  const combined = new RegExp(pattern);
  assert.ok(combined.test('The current count is 0. Each click updates the Signal, and the Virtual DOM patches the text.'));
  assert.ok(combined.test('The current count is -1. Each click updates the Signal, and the Virtual DOM patches the text.'));
  assert.ok(!combined.test('The current count is 10 elsewhere.'));
  assert.ok(!combined.test('The current count is 0x Each click updates the Signal, and the Virtual DOM patches the text.'));
  assert.match(counter, /combinedCount == 1 && combined\.element\(boundBy: 0\)\.label\s*\.hasPrefix\("The current count is \\\(expected\)\. Each click updates the Signal, "\)/);
});

test('iOS counter waits for strict values after real Add and Reset taps', () => {
  assert.match(counter, /XCTNSPredicateExpectation\(predicate: predicate, object: app\)/);
  assert.match(counter, /XCTAssertEqual\(XCTWaiter\.wait\(for: \[expectation\], timeout: 10\), \.completed/);
  assert.match(counter, /file: file, line: line/);
  assert.match(swift, /assertCounter\(app, equals: 0\)\s*add\.tap\(\)\s*assertCounter\(app, equals: 1\)\s*attach\(app, "02-counter-add"\)\s*app\.buttons\["Reset"\]\.tap\(\)\s*assertCounter\(app, equals: 0\)/);
});

test('iOS smoke keeps real launch, navigation, restart, screenshots and cleanup', () => {
  assert.match(swift, /continueAfterFailure = false/);
  assert.match(swift, /XCUIApplication\(bundleIdentifier: "com\.euv"\)/);
  assert.match(swift, /app\.launch\(\)[\s\S]*version\.waitForExistence\(timeout: 45\)/);
  assert.match(swift, /menu\.tap\(\)[\s\S]*counter\.tap\(\)/);
  assert.match(swift, /attach\(app, "03-counter-reset"\)\s*app\.terminate\(\)\s*app\.launch\(\)\s*XCTAssertTrue\(version\.waitForExistence\(timeout: 45\), app\.debugDescription\)\s*attach\(app, "04-cache-restart"\)/);
  assert.match(source, /run\(command, stdout=log, stderr=subprocess\.STDOUT\)/);
  assert.match(source, /finally:\s*cleanup\(\['xcrun', 'simctl', 'terminate', udid, 'com\.euv'\]\)/);
  assert.match(source, /if owned:\s*cleanup\(\['xcrun', 'simctl', 'shutdown', udid\]\)\s*cleanup\(\['xcrun', 'simctl', 'delete', udid\]\)/);
  assert.doesNotMatch(swift, /XCTSkip|try\?/);
});

test('iOS theme readiness caches the innermost WebView hosting Theme feature', () => {
  const resolver = swift.slice(swift.indexOf('func themeContentWebView('), swift.indexOf('func assertThemeReady('));
  const readiness = swift.slice(swift.indexOf('func assertThemeReady('), swift.indexOf('func assertThemeVariant('));
  assert.match(resolver, /app\.webViews\.containing\(\.any, identifier: "Theme feature"\)\.allElementsBoundByIndex/);
  assert.match(resolver, /candidates\.first\(where: \{\s*!\$0\.webViews\.containing\(\.any, identifier: "Theme feature"\)\.firstMatch\.exists/);
  assert.match(readiness, /if content == nil \{ content = self\.themeContentWebView\(app\) \}/);
  assert.match(readiness, /guard let web = content, web\.exists else \{ return false \}/);
  assert.match(readiness, /let selected = web\.descendants\(matching: \.any\)\.matching\(NSPredicate\(\s*format: "label == %@ AND value == %@", "Theme feature", theme/);
  assert.match(readiness, /guard selected\.firstMatch\.exists else \{ return false \}/);
  assert.match(readiness, /guard web\.buttons\.matching\(identifier: "☰"\)\.firstMatch\.exists else \{ return false \}/);
  assert.match(readiness, /return !web\.staticTexts\.matching\(NSPredicate\(format: "label == %@", "Page Not Found"\)\)\.firstMatch\.exists/);
  assert.doesNotMatch(readiness, /app\.(staticTexts|descendants|buttons|webViews)|label CONTAINS|Thread\.sleep|\.tap\(/);
});

test('iOS theme readiness has one bounded observation and no same-tree retry', () => {
  const readiness = swift.slice(swift.indexOf('func assertThemeReady('), swift.indexOf('func assertThemeVariant('));
  assert.match(readiness, /let result = XCTWaiter\.wait\(for: \[expectation\], timeout: 60\)/);
  assert.equal((readiness.match(/XCTWaiter\.wait\(/g) || []).length, 1);
  assert.match(readiness, /guard result == \.completed else \{\s*XCTFail\("Theme \\\(theme\) was not ready/);
  assert.doesNotMatch(readiness, /retry|waitForSnapshot|\.launch\(|\.terminate\(/i);
  assert.match(source, /'-test-timeouts-enabled', 'YES',[\s\S]*'-maximum-test-execution-time-allowance', '1800'/);
});

test('iOS version checks require exact rendered labels while tolerating duplicate exact badges', () => {
  const readiness = swift.slice(swift.indexOf('func assertThemeReady('), swift.indexOf('func assertThemeVariant('));
  assert.match(readiness, /pending = "version badge"[\s\S]*let version = web\.staticTexts\.matching\(NSPredicate\(\s*format: "label == %@", "__EXPECTED_VERSION__"\s*\)\)\.firstMatch\s*guard version\.exists else \{ return false \}/);
  assert.match(swift, /let version = app\.staticTexts\.matching\(identifier: "__EXPECTED_VERSION__"\)\.firstMatch/);
  assert.doesNotMatch(readiness, /version\.count == 1|version.*CONTAINS|staticTexts\["__EXPECTED_VERSION__"\]/);
  assert.match(source, /SWIFT_TEST\.replace\('__EXPECTED_VERSION__', 'v' \+ version\)/);
});

test('iOS metadata generates one XCTest per theme with a fresh launch and guaranteed termination', () => {
  const themes = swift.slice(swift.indexOf('func assertThemeVariant('), swift.indexOf('__THEME_TEST_METHODS__'));
  assert.match(themes, /let app = XCUIApplication\(bundleIdentifier: "com\.euv"\)\s*app\.launch\(\)\s*defer \{ app\.terminate\(\) \}/);
  assert.match(themes, /let selector = web\.descendants\(matching: \.any\)\.matching\(identifier: "Theme feature"\)\.firstMatch\s*selector\.tap\(\)/);
  assert.match(themes, /option\.tap\(\)[\s\S]*assertThemeReady\(app, theme: theme\)\s*attach\(app, "theme-" \+ theme\)/);
  assert.doesNotMatch(themes, /for theme in|Thread\.sleep/);
  assert.match(source, /theme_names = sorted\([\s\S]*\+ \['default'\]/);
  assert.match(source, /swift_test = render_swift_tests\(version, theme_names\)/);
  const manifest = fs.readFileSync(path.resolve(__dirname, '../../../../example/Cargo.toml'), 'utf8');
  const names = [...manifest.matchAll(/^theme-([a-z0-9]+)\s*=/gm)].map((match) => match[1]).sort().concat('default');
  const generated = execFileSync(pythonCommand(), ['-B', '-c', String.raw`
import importlib.util, json, sys
spec = importlib.util.spec_from_file_location('euv_ios_smoke', sys.argv[1])
module = importlib.util.module_from_spec(spec)
spec.loader.exec_module(module)
print(module.render_swift_tests('0.36.2', json.load(sys.stdin)))
`, script], { input: JSON.stringify(names), encoding: 'utf8' });
  const methods = [...generated.matchAll(/func testTheme_([a-z0-9]+)\(\) throws \{\s*assertThemeVariant\("([a-z0-9]+)"\)\s*\}/g)];
  assert.deepEqual(methods.map((match) => match[1]), names);
  assert.deepEqual(methods.map((match) => match[2]), names);
  assert.equal([...generated.matchAll(/func test\w+\(\)/g)].length, names.length + 2);
  assert.doesNotMatch(generated, /__THEME_|__EXPECTED_VERSION__|testThemeVariantsAndRestart|XCTSkip/);
});

test('iOS dedicated default-theme restart repeats every rendered readiness check', () => {
  const restart = swift.slice(swift.indexOf('func testDefaultThemeRestart('), swift.indexOf('func testLocalPagesCounterAndRestart('));
  assert.match(restart, /app\.launch\(\)\s*defer \{ app\.terminate\(\) \}\s*assertThemeReady\(app, theme: "default"\)\s*app\.terminate\(\)\s*app\.launch\(\)\s*assertThemeReady\(app, theme: "default"\)\s*attach\(app, "theme-restart"\)/);
});

test('iOS cache verification reports every manifest-backed SHA-256 check', () => {
  assert.match(source, /for entry in manifest\['files'\]:[\s\S]*hashlib\.sha256\(content\)\.hexdigest\(\) == entry\['sha256'\]/);
  assert.match(source, /report\['cache_files_verified'\] = len\(manifest\['files'\]\)/);
});
