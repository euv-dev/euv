'use strict';

const assert = require('node:assert/strict');
const fs = require('node:fs');
const path = require('node:path');
const { test } = require('node:test');

const source = fs.readFileSync(path.resolve(__dirname, '../validation/smoke-ios.py'), 'utf8');
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

test('iOS theme smoke resolves exact readiness across nested WebViews after navigation', () => {
  const readiness = swift.slice(swift.indexOf('func assertThemeReady('), swift.indexOf('func testThemeVariantsAndRestart('));
  const themes = swift.slice(swift.indexOf('func testThemeVariantsAndRestart('), swift.indexOf('func testLocalPagesCounterAndRestart('));
  assert.match(readiness, /guard app\.webViews\.firstMatch\.exists else \{ return false \}/);
  assert.match(readiness, /let selected = app\.descendants\(matching: \.any\)\.matching\(NSPredicate\(\s*format: "label == %@ AND value == %@", "Theme feature", theme/);
  assert.match(readiness, /guard selected\.firstMatch\.exists else \{ return false \}/);
  assert.match(readiness, /return app\.buttons\.matching\(identifier: "☰"\)\.firstMatch\.exists/);
  assert.doesNotMatch(readiness, /let web =|web\.staticTexts|web\.descendants|web\.buttons|label CONTAINS|Thread\.sleep/);
  assert.match(readiness, /XCTWaiter\.wait\(for: \[expectation\], timeout: 60\)/);
  assert.match(readiness, /var result = waitForSnapshot\(\)\s*if result == \.timedOut \{[\s\S]*result = waitForSnapshot\(\)\s*\}\s*guard result == \.completed else \{/);
  assert.equal((readiness.match(/result = waitForSnapshot\(\)/g) || []).length, 2);
  assert.match(readiness, /XCTFail\("Theme \\\(theme\) was not ready/);
  assert.doesNotMatch(readiness, /\.tap\(/);
  assert.match(readiness, /XCTAssertFalse\(app\.staticTexts\.matching\(identifier: "Page Not Found"\)\.firstMatch\.exists/);
  assert.match(themes, /for theme in __THEME_NAMES__[\s\S]*option\.tap\(\)[\s\S]*assertThemeReady\(app, theme: theme\)\s*attach\(app, "theme-" \+ theme\)/);
  assert.doesNotMatch(themes, /Thread\.sleep/);
  assert.match(source, /'-test-timeouts-enabled', 'YES',[\s\S]*'-maximum-test-execution-time-allowance', '\d+'/);
});

test('iOS version checks tolerate duplicate exact badges without accepting another version', () => {
  const readiness = swift.slice(swift.indexOf('func assertThemeReady('), swift.indexOf('func testThemeVariantsAndRestart('));
  const exactVersion = /let version = app\.staticTexts\.matching\(identifier: "__EXPECTED_VERSION__"\)\.firstMatch/g;
  assert.equal([...swift.matchAll(exactVersion)].length, 2, 'theme readiness and counter startup both use a non-ambiguous exact match');
  assert.match(readiness, /pending = "version badge"[\s\S]*let version = app\.staticTexts\.matching\(identifier: "__EXPECTED_VERSION__"\)\.firstMatch\s*guard version\.exists else \{ return false \}/);
  assert.doesNotMatch(swift, /staticTexts\["__EXPECTED_VERSION__"\]/);
  assert.match(source, /SWIFT_TEST\.replace\('__EXPECTED_VERSION__', 'v' \+ version\)/);
});

test('iOS theme restart rechecks the rendered exact version and final selected theme', () => {
  const themes = swift.slice(swift.indexOf('func testThemeVariantsAndRestart('), swift.indexOf('func testLocalPagesCounterAndRestart('));
  assert.match(source, /theme_names = sorted\([\s\S]*\+ \['default'\]/);
  assert.match(themes, /app\.terminate\(\)\s*app\.launch\(\)\s*XCTAssertTrue\(selector\.waitForExistence\(timeout: 60\), app\.debugDescription\)\s*assertThemeReady\(app, theme: "default"\)\s*XCTAssertFalse\(app\.staticTexts\["Page Not Found"\]\.exists\)\s*attach\(app, "theme-restart"\)/);
});
