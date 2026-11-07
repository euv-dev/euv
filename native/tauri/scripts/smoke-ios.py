#!/usr/bin/env python3
"""Install a real iOS Simulator app and verify its UI, cache and restart with XCTest."""

import argparse
import hashlib
import json
import os
from pathlib import Path
import re
import shutil
import subprocess
import tarfile

ROOT = Path(__file__).resolve().parents[1]

SWIFT_TEST = r'''
import XCTest

final class EuvSmokeTests: XCTestCase {
    func attach(_ app: XCUIApplication, _ name: String) {
        let image = XCTAttachment(screenshot: app.screenshot())
        image.name = name
        image.lifetime = .keepAlways
        add(image)
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
        let zero = app.descendants(matching: .any).matching(NSPredicate(format: "label CONTAINS %@", "The current count is 0")).firstMatch
        XCTAssertTrue(zero.waitForExistence(timeout: 10), app.debugDescription)
        add.tap()
        let one = app.descendants(matching: .any).matching(NSPredicate(format: "label CONTAINS %@", "The current count is 1")).firstMatch
        XCTAssertTrue(one.waitForExistence(timeout: 10), app.debugDescription)
        attach(app, "02-counter-add")
        app.buttons["Reset"].tap()
        XCTAssertTrue(zero.waitForExistence(timeout: 10), app.debugDescription)
        attach(app, "03-counter-reset")
        app.terminate()
        app.launch()
        XCTAssertTrue(version.waitForExistence(timeout: 45), app.debugDescription)
        attach(app, "04-cache-restart")
        app.terminate()
    }
}
'''


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

    run(['node', str(ROOT / 'scripts/verify-output.js'), 'ios-simulator'], cwd=ROOT)
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
    if owned:
        runtimes = json.loads(capture(['xcrun', 'simctl', 'list', 'runtimes', '-j']))['runtimes']
        runtimes = [r for r in runtimes if r.get('isAvailable') and (r.get('platform') == 'iOS' or '.iOS-' in r.get('identifier', ''))]
        runtimes.sort(key=lambda r: tuple(map(int, r['version'].split('.'))), reverse=True)
        if not runtimes:
            raise SystemExit('No installed iOS Simulator runtime')
        runtime = runtimes[0]
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
        (project / 'SmokeTests.swift').write_text(SWIFT_TEST.replace('__EXPECTED_VERSION__', 'v' + version))
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
                   '-resultBundlePath', str(result), 'CODE_SIGN_IDENTITY=-', 'CODE_SIGNING_REQUIRED=NO']
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
        display_logs = json.loads(capture(['xcrun', 'simctl', 'spawn', udid, 'log', 'show', '--last', '10m',
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
