#!/usr/bin/env python3
"""Install a real iOS Simulator app and verify its UI, cache and restart with XCTest."""

import argparse
import hashlib
import json
import math
import os
from pathlib import Path
import re
import shutil
import subprocess
import tarfile
import time

ROOT = Path(__file__).resolve().parents[2]

SWIFT_TEST = r'''
import XCTest

final class EuvSmokeTests: XCTestCase {
    func assertCounter(_ app: XCUIApplication, equals expected: Int, file: StaticString = #filePath, line: UInt = #line) {
        let predicate = NSPredicate { _, _ in
            // Newer WebKit exposes the entire paragraph as one text element.
            let combined = app.webViews.firstMatch.staticTexts.matching(NSPredicate(
                format: "label MATCHES %@", "^The current count is -?[0-9]+\\. Each click updates the Signal, .+$"
            ))
            let combinedCount = combined.count
            if combinedCount > 0 {
                return combinedCount == 1 && combined.element(boundBy: 0).label
                    .hasPrefix("The current count is \(expected). Each click updates the Signal, ")
            }
            // Older WebKit splits the paragraph into text nodes. Require the
            // number beside the exact prefix, not an unrelated value on the page.
            let paragraphs = app.otherElements
                .containing(.staticText, identifier: "The current count is")
                .allElementsBoundByIndex.filter {
                    $0.children(matching: .staticText)
                        .matching(identifier: "The current count is").count == 1
                }
            guard paragraphs.count == 1 else { return false }
            let numbers = paragraphs[0].children(matching: .staticText)
                .matching(NSPredicate(format: "label MATCHES %@", "^-?[0-9]+$"))
            guard numbers.count == 1 else { return false }
            return numbers.element(boundBy: 0).label == String(expected)
        }
        let expectation = XCTNSPredicateExpectation(predicate: predicate, object: app)
        XCTAssertEqual(XCTWaiter.wait(for: [expectation], timeout: 10), .completed,
                       "Expected counter \(expected) in its paragraph\n\(app.debugDescription)",
                       file: file, line: line)
    }

    func attach(_ app: XCUIApplication, _ name: String) {
        let image = XCTAttachment(screenshot: app.screenshot())
        image.name = name
        image.lifetime = .keepAlways
        add(image)
    }

    func assertThemeReady(_ app: XCUIApplication, theme: String, file: StaticString = #filePath, line: UInt = #line) {
        let web = app.webViews.firstMatch
        var pending = "version badge"
        let ready = NSPredicate { _, _ in
            // Resolve the selector only after WebKit exposes the rendered page.
            pending = "version badge"
            guard web.staticTexts["__EXPECTED_VERSION__"].exists else { return false }
            pending = "theme selector value"
            let selected = web.descendants(matching: .any).matching(NSPredicate(
                format: "label == %@ AND value == %@", "Theme feature", theme
            ))
            guard selected.firstMatch.exists else { return false }
            pending = "mobile menu button"
            return web.buttons["☰"].exists
        }
        func waitForSnapshot() -> XCTWaiter.Result {
            let expectation = XCTNSPredicateExpectation(predicate: ready, object: app)
            return XCTWaiter.wait(for: [expectation], timeout: 60)
        }
        var result = waitForSnapshot()
        if result == .timedOut {
            // One WebKit accessibility request can consume the entire wait on
            // the Intel simulator. Retry the observation, not the selection.
            print("Retrying theme \(theme) accessibility snapshot after waiting for \(pending)")
            result = waitForSnapshot()
        }
        guard result == .completed else {
            XCTFail("Theme \(theme) was not ready (\(pending), \(result))\n\(app.debugDescription)", file: file, line: line)
            return
        }
        XCTAssertFalse(web.staticTexts["Page Not Found"].exists, theme, file: file, line: line)
    }

    func testThemeVariantsAndRestart() throws {
        continueAfterFailure = false
        XCUIDevice.shared.orientation = .portrait
        let app = XCUIApplication(bundleIdentifier: "com.euv")
        app.launch()
        let selector = app.descendants(matching: .any).matching(identifier: "Theme feature").firstMatch
        XCTAssertTrue(selector.waitForExistence(timeout: 60), app.debugDescription)
        for theme in __THEME_NAMES__ {
            let optionLabel = theme
            selector.tap()
            let wheel = app.pickerWheels.firstMatch
            if wheel.waitForExistence(timeout: 2) {
                wheel.adjust(toPickerWheelValue: optionLabel)
                let done = app.buttons["Done"]
                if done.exists { done.tap() }
            } else {
                let option = app.descendants(matching: .any).matching(NSPredicate(format: "label == %@", optionLabel)).firstMatch
                for _ in 0..<4 {
                    if option.exists { break }
                    let menu = app.collectionViews.firstMatch
                    if !menu.exists { break }
                    let start = menu.coordinate(withNormalizedOffset: CGVector(dx: 0.5, dy: 0.15))
                    let end = menu.coordinate(withNormalizedOffset: CGVector(dx: 0.5, dy: 0.45))
                    start.press(forDuration: 0.05, thenDragTo: end)
                }
                XCTAssertTrue(option.waitForExistence(timeout: 5), app.debugDescription)
                option.tap()
            }
            assertThemeReady(app, theme: theme)
            attach(app, "theme-" + theme)
        }
        app.terminate()
        app.launch()
        XCTAssertTrue(selector.waitForExistence(timeout: 60), app.debugDescription)
        XCTAssertFalse(app.staticTexts["Page Not Found"].exists)
        attach(app, "theme-restart")
        app.terminate()
    }

    func testLocalPagesCounterAndRestart() throws {
        continueAfterFailure = false
        XCUIDevice.shared.orientation = .portrait
        let app = XCUIApplication(bundleIdentifier: "com.euv")
        app.launchEnvironment["EUV_CLIENT_DIAGNOSTICS"] = "1"
        app.launch()
        XCTAssertTrue(app.webViews.firstMatch.waitForExistence(timeout: 45), app.debugDescription)
        let version = app.staticTexts["__EXPECTED_VERSION__"]
        XCTAssertTrue(version.waitForExistence(timeout: 45), app.debugDescription)
        XCTAssertEqual(app.statusBars.count, 0, "Immersive mode must hide the status bar")
        Thread.sleep(forTimeInterval: 3)
        attach(app, "01-local-startup")
        let menu = app.buttons["☰"]
        XCTAssertTrue(menu.waitForExistence(timeout: 10), app.debugDescription)
        menu.tap()
        attach(app, "menu-after-tap")
        let drawers = app.otherElements.containing(.button, identifier: "✕").allElementsBoundByIndex
        guard let drawer = drawers.first(where: { $0.frame.width > 0 && $0.frame.width < app.frame.width && $0.frame.height > app.frame.height * 0.7 }) else {
            XCTFail("No open mobile navigation drawer", file: #filePath, line: #line)
            return
        }
        let counter = drawer.links.matching(NSPredicate(format: "label CONTAINS %@", "Counter")).firstMatch
        XCTAssertTrue(counter.waitForExistence(timeout: 10), app.debugDescription)
        // WebKit exposes unclipped link frames; scroll the visible drawer itself.
        let window = app.windows.firstMatch
        let start = window.coordinate(withNormalizedOffset: CGVector(dx: 0.25, dy: 0.45))
        let end = window.coordinate(withNormalizedOffset: CGVector(dx: 0.25, dy: 0.20))
        start.press(forDuration: 0.01, thenDragTo: end)
        attach(app, "navigation-after-scroll")
        counter.tap()
        let add = app.buttons["Add"]
        XCTAssertTrue(add.waitForExistence(timeout: 15), app.debugDescription)
        assertCounter(app, equals: 0)
        add.tap()
        assertCounter(app, equals: 1)
        attach(app, "02-counter-add")
        app.buttons["Reset"].tap()
        assertCounter(app, equals: 0)
        attach(app, "03-counter-reset")
        app.terminate()
        app.launch()
        XCTAssertTrue(version.waitForExistence(timeout: 45), app.debugDescription)
        attach(app, "04-cache-restart")
        app.terminate()
    }
}
'''


def select_ios_runtime(runtimes, sdk_version):
    """Choose the newest available iOS runtime no newer than the active SDK."""
    def normalized_version(version):
        if not re.fullmatch(r'\d+(?:\.\d+)*', version):
            raise ValueError(f'Invalid iOS version: {version!r}')
        parts = list(map(int, version.split('.')))
        while len(parts) > 1 and parts[-1] == 0:
            parts.pop()
        return tuple(parts)

    sdk = normalized_version(sdk_version)
    candidates = [
        runtime for runtime in runtimes
        if runtime.get('isAvailable')
        and (runtime.get('platform') == 'iOS' or '.iOS-' in runtime.get('identifier', ''))
        and normalized_version(runtime['version']) <= sdk
    ]
    if not candidates:
        raise ValueError(
            f'No available installed iOS Simulator runtime compatible with active '
            f'iphonesimulator SDK {sdk_version} (requires runtime <= SDK)'
        )
    return max(candidates, key=lambda runtime: normalized_version(runtime['version']))


def display_log_window_seconds(started, now):
    """Include the whole acceptance session without assuming its duration."""
    return max(1, math.ceil(now - started))


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--udid', help='Use an existing simulator; only automatically created devices are deleted')
    parser.add_argument('--archive', help='Verified ios-simulator app archive')
    parser.add_argument('--output', default=str(ROOT / 'artifacts/validation/ios'))
    args = parser.parse_args()
    output = Path(args.output).resolve()
    output.mkdir(parents=True, exist_ok=True)
    simulator_env = dict(os.environ)
    # Host SDK helpers must not inject their libraries into Simulator migration.
    for name in ['DYLD_LIBRARY_PATH', 'LD_LIBRARY_PATH', 'GEM_HOME', 'GEM_PATH', 'SDKROOT']:
        simulator_env.pop(name, None)

    def run(command, **kwargs):
        return subprocess.run(command, check=True, env=simulator_env, **kwargs)

    def cleanup(command):
        try:
            result = subprocess.run(command, capture_output=True, timeout=30, env=simulator_env)
            if result.returncode:
                print('Cleanup command returned nonzero:', command, result.stderr.decode(errors='replace').strip(), flush=True)
        except subprocess.TimeoutExpired:
            print('Cleanup command timed out:', command, flush=True)

    def capture(command):
        return subprocess.check_output(command, text=True, env=simulator_env).strip()

    run(['node', str(ROOT / 'scripts/validation/verify-output.js'), 'ios-simulator'], cwd=ROOT)
    archives = list((ROOT / 'artifacts/ios-simulator').glob('*.app.tar.gz'))
    archive = Path(args.archive) if args.archive else archives[0] if len(archives) == 1 else None
    if archive is None:
        raise SystemExit('Expected exactly one verified iOS Simulator app archive')
    extracted = output / 'installed-source'
    extracted.mkdir(exist_ok=True)
    with tarfile.open(archive, 'r:gz') as package:
        for entry in package.getmembers():
            if entry.name.startswith('/') or '..' in Path(entry.name).parts:
                raise SystemExit('Unsafe path in app archive')
        package.extractall(extracted)
    applications = [p for p in extracted.glob('*.app') if p.is_dir()]
    if len(applications) != 1:
        raise SystemExit('Expected one simulator app')
    version_match = re.search(r'\[workspace\.package\]\s*\nversion\s*=\s*"([^"]+)"', (ROOT / '../../Cargo.toml').resolve().read_text())
    if version_match is None:
        raise SystemExit('Cannot read expected example version')
    version = version_match.group(1)
    udid = args.udid
    owned = not udid
    runtime_report = {}
    if owned:
        sdk_version = capture(['xcrun', '--sdk', 'iphonesimulator', '--show-sdk-version'])
        runtimes = json.loads(capture(['xcrun', 'simctl', 'list', 'runtimes', '-j']))['runtimes']
        try:
            runtime = select_ios_runtime(runtimes, sdk_version)
        except ValueError as error:
            raise SystemExit(str(error)) from error
        runtime_report = {'simulator_sdk_version': sdk_version,
                          'runtime_version': runtime['version'], 'runtime_identifier': runtime['identifier']}
        print(f'Active iphonesimulator SDK {sdk_version}; selected iOS runtime '
              f'{runtime["version"]} ({runtime["identifier"]})', flush=True)
        devices = [d for d in runtime.get('supportedDeviceTypes', []) if d.get('productFamily') == 'iPhone']
        if not devices:
            inventory = json.loads(capture(['xcrun', 'simctl', 'list', 'devices', 'available', '-j']))
            devices = [{'identifier': d['deviceTypeIdentifier']} for d in inventory['devices'].get(runtime['identifier'], []) if 'iPhone' in d['name']]
        if not devices:
            raise SystemExit('No compatible iPhone simulator device type')
        udid = capture(['xcrun', 'simctl', 'create', 'EuvValidation', devices[0]['identifier'], runtime['identifier']])
    if udid is None:
        raise SystemExit('No simulator selected')
    report = {'udid': udid, 'archive': str(archive), 'expected_version': version, 'passed': False, 'steps': []}
    report.update(runtime_report)
    try:
        inventory = json.loads(capture(['xcrun', 'simctl', 'list', 'devices', 'available', '-j']))
        selected = next((d for group in inventory['devices'].values() for d in group if d['udid'] == udid), None)
        if selected is None:
            raise SystemExit('Selected simulator is not available')
        if selected['state'] != 'Booted':
            run(['xcrun', 'simctl', 'boot', udid])
        run(['xcrun', 'simctl', 'bootstatus', udid, '-b'], timeout=300)
        run(['xcrun', 'simctl', 'install', udid, str(applications[0])])
        project = output / 'test-project'
        project.mkdir(exist_ok=True)
        example_manifest = (ROOT / '../../example/Cargo.toml').resolve().read_text()
        theme_names = sorted(re.findall(r'^theme-([a-z0-9]+)\s*=', example_manifest, re.MULTILINE)) + ['default']
        swift_test = SWIFT_TEST.replace('__EXPECTED_VERSION__', 'v' + version).replace('__THEME_NAMES__', json.dumps(theme_names))
        (project / 'SmokeTests.swift').write_text(swift_test)
        spec = {
            'name': 'EuvSmoke',
            'targets': {'EuvSmokeUITests': {
                'type': 'bundle.ui-testing', 'platform': 'iOS', 'deploymentTarget': '15.0',
                'sources': ['SmokeTests.swift'],
                'settings': {'base': {'PRODUCT_BUNDLE_IDENTIFIER': 'com.euv.validation',
                                      'GENERATE_INFOPLIST_FILE': 'YES', 'SWIFT_VERSION': '5.0',
                                      'CODE_SIGN_IDENTITY': '-', 'CODE_SIGNING_REQUIRED': 'NO'}},
            }},
            'schemes': {'EuvSmoke': {'build': {'targets': {'EuvSmokeUITests': ['test']}},
                                    'test': {'targets': ['EuvSmokeUITests']}}},
        }
        # JSON is also a valid YAML document for xcodegen's project specification.
        (project / 'project.yml').write_text(json.dumps(spec, indent=2))
        run(['xcodegen', 'generate', '--spec', str(project / 'project.yml'), '--project', str(project)])
        result = output / 'Smoke.xcresult'
        if result.exists():
            shutil.rmtree(result)
        command = ['xcodebuild', 'test', '-project', str(project / 'EuvSmoke.xcodeproj'), '-scheme', 'EuvSmoke',
                   '-sdk', 'iphonesimulator',
                   '-destination', 'id=' + udid, '-derivedDataPath', str(output / 'derived-data'),
                   '-resultBundlePath', str(result), 'CODE_SIGN_IDENTITY=-', 'CODE_SIGNING_REQUIRED=NO',
                   '-test-timeouts-enabled', 'YES', '-default-test-execution-time-allowance', '1800',
                   '-maximum-test-execution-time-allowance', '1800']
        test_started = time.monotonic()
        with (output / 'xcodebuild.log').open('w') as log:
            run(command, stdout=log, stderr=subprocess.STDOUT)
        report['steps'].append('XCTest verifies rendered local version, Counter 0 -> 1 -> 0 and restart')
        data = Path(capture(['xcrun', 'simctl', 'get_app_container', udid, 'com.euv', 'data']))
        caches = [p for p in data.rglob('euv_web_cache') if p.is_dir() and (p / 'active').exists()]
        if len(caches) != 1:
            raise AssertionError('Expected one deployed local page cache')
        active = (caches[0] / 'active').read_text().strip()
        if not re.fullmatch(r'v_\d+', active):
            raise AssertionError('Invalid active cache version')
        manifest = json.loads((ROOT / 'src-tauri/bundled-cache/__euv_local_bundle__.json').read_text())
        for entry in manifest['files']:
            content = (caches[0] / active / entry['path']).read_bytes()
            assert len(content) == entry['size'] and hashlib.sha256(content).hexdigest() == entry['sha256'], entry['path']
        display_log_seconds = display_log_window_seconds(test_started, time.monotonic())
        report['display_log_window_seconds'] = display_log_seconds
        display_logs = json.loads(capture(['xcrun', 'simctl', 'spawn', udid, 'log', 'show', '--last', f'{display_log_seconds}s',
                                           '--style', 'json', '--predicate',
                                           'eventMessage BEGINSWITH "[EUV] Frame-rate sample:"']))
        samples = []
        for entry in display_logs:
            message = entry.get('eventMessage', '')
            if message.startswith('[EUV] Frame-rate sample: '):
                samples.append(json.loads(message.split('[EUV] Frame-rate sample: ', 1)[1]))
        if not samples:
            raise AssertionError('No real requestAnimationFrame sample from the production WebView')
        for sample in samples:
            policy = sample['native']
            assert sample['frames'] == 120 and sample['framesPerSecond'] > 0
            assert policy['immersive'] is True and policy['policy'] == 'maximum'
            assert policy['maximumFramesPerSecond'] > 0
            assert policy['requestedFramesPerSecond'] == policy['maximumFramesPerSecond']
            assert policy['guaranteed'] is False
        report['display_samples'] = samples
        report['steps'].append('status bar hidden; native hardware maximum requested; real page FPS sampled separately')
        report['cache_version'] = active
        report['steps'].append('deployed simulator cache matches all local example SHA-256 values')
        report['passed'] = True
        print(json.dumps(report, indent=2))
    except BaseException:
        cleanup(['xcrun', 'simctl', 'io', udid, 'screenshot', str(output / 'failure.png')])
        raise
    finally:
        cleanup(['xcrun', 'simctl', 'terminate', udid, 'com.euv'])
        if owned:
            cleanup(['xcrun', 'simctl', 'shutdown', udid])
            cleanup(['xcrun', 'simctl', 'delete', udid])
        (output / 'verification.json').write_text(json.dumps(report, indent=2))


if __name__ == '__main__':
    main()
