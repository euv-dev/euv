#!/usr/bin/env python3
"""Verify a real Android emulator install, offline startup, UI and cache restart."""

import argparse
import hashlib
import json

from pathlib import Path
import re
import shutil
import subprocess
import time
import xml.etree.ElementTree as ET

ROOT = Path(__file__).resolve().parents[2]


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--serial', required=True)
    parser.add_argument('--apk', default=str(ROOT / 'artifacts/android/euv.apk'))
    parser.add_argument('--fresh', action='store_true', help='Clear only this app data on the selected test emulator')
    parser.add_argument('--output', default=str(ROOT / 'artifacts/validation/android'))
    args = parser.parse_args()
    if not re.fullmatch(r'emulator-\d+', args.serial):
        raise SystemExit('This offline smoke test only accepts an emulator, never a physical device')
    adb_path = shutil.which('adb') or str(ROOT / '.cache/sdk/android/platform-tools/adb')
    output = Path(args.output)
    output.mkdir(parents=True, exist_ok=True)
    manifest = json.loads((ROOT / 'src-tauri/bundled-cache/__euv_local_bundle__.json').read_text())
    version_match = re.search(r'\[workspace\.package\]\s*\nversion\s*=\s*"([^"]+)"', (ROOT / '../../Cargo.toml').resolve().read_text())
    if version_match is None:
        raise SystemExit('Cannot read the expected example workspace version')
    version = version_match.group(1)

    def adb_bytes(*command, check=True) -> bytes:
        result = subprocess.run([adb_path, '-s', args.serial, *command], capture_output=True, timeout=60, check=check)
        return result.stdout

    def adb(*command, check=True) -> str:
        return adb_bytes(*command, check=check).decode(errors='replace').strip()

    def ui():
        adb('shell', 'uiautomator', 'dump', '/sdcard/euv-smoke.xml')
        data = adb('exec-out', 'cat', '/sdcard/euv-smoke.xml')
        (output / 'latest-ui.xml').write_text(data)
        return ET.fromstring(data)

    def text(tree):
        return ' '.join(n.get('text', '') + ' ' + n.get('content-desc', '') for n in tree.iter('node'))

    def await_text(pattern, seconds=60):
        deadline = time.monotonic() + seconds
        while time.monotonic() < deadline:
            tree = ui()
            if any(n.get('resource-id') == 'com.android.systemui:id/immersive_cling_title' for n in tree.iter('node')):
                # This is Android's first-use fullscreen tutorial, not a permission dialog.
                tap(tree, 'Got it')
                report['steps'].append('dismissed the system first-use immersive tutorial')
                continue
            if re.search(pattern, text(tree)):
                return tree
            time.sleep(1)
        raise AssertionError('UI did not display expected content: ' + pattern)

    def tap(tree, label):
        def nodes(node, ancestors):
            yield node, ancestors
            for child in node:
                yield from nodes(child, ancestors + [node])

        def bounds(node):
            match = re.fullmatch(r'\[(\d+),(\d+)\]\[(\d+),(\d+)\]', node.get('bounds', ''))
            return list(map(int, match.groups())) if match else None

        for _ in range(8):
            scroll = None
            for node, ancestors in nodes(tree, []):
                if node.get('text') != label and not node.get('content-desc', '').endswith(label):
                    continue
                rect = bounds(node)
                if rect is None:
                    continue
                left, top, right, bottom = rect
                for ancestor in ancestors:
                    clip = bounds(ancestor)
                    if clip:
                        left, top = max(left, clip[0]), max(top, clip[1])
                        right, bottom = min(right, clip[2]), min(bottom, clip[3])
                if right > left and bottom > top:
                    adb('shell', 'input', 'tap', str((left + right) // 2), str((top + bottom) // 2))
                    return
                scroll = next((bounds(a) for a in reversed(ancestors) if a.get('scrollable') == 'true' and bounds(a)), None)
            if scroll is None:
                break
            left, top, right, bottom = scroll
            center = (left + right) // 2
            margin = max(20, (bottom - top) // 6)
            adb('shell', 'input', 'swipe', str(center), str(bottom - margin), str(center), str(top + margin), '350')
            tree = ui()
        raise AssertionError('No visible control after scrolling: ' + label)

    def screenshot(name):
        content = adb_bytes('exec-out', 'screencap', '-p')
        if not content.startswith(b'\x89PNG\r\n\x1a\n'):
            raise AssertionError('Emulator did not return a PNG screenshot')
        (output / name).write_bytes(content)

    wifi = adb('shell', 'settings', 'get', 'global', 'wifi_on')
    mobile = adb('shell', 'settings', 'get', 'global', 'mobile_data')
    screen_timeout = adb('shell', 'settings', 'get', 'system', 'screen_off_timeout')
    stay_awake = adb('shell', 'settings', 'get', 'global', 'stay_on_while_plugged_in')
    report = {'serial': args.serial, 'apk': str(Path(args.apk).resolve()), 'expected_version': version, 'fresh_install': args.fresh, 'passed': False, 'steps': []}
    try:
        adb('shell', 'settings', 'put', 'system', 'screen_off_timeout', '2147483647')
        adb('shell', 'svc', 'power', 'stayon', 'true')
        adb('shell', 'input', 'keyevent', 'KEYCODE_WAKEUP')
        adb('shell', 'wm', 'dismiss-keyguard')
        adb('install', '-r', str(Path(args.apk).resolve()))
        if args.fresh:
            adb('shell', 'pm', 'clear', 'com.euv')
        adb('shell', 'pm', 'grant', 'com.euv', 'android.permission.POST_NOTIFICATIONS')
        adb('shell', 'svc', 'wifi', 'disable')
        adb('shell', 'svc', 'data', 'disable')
        assert adb('shell', 'settings', 'get', 'global', 'wifi_on') == '0'
        assert adb('shell', 'settings', 'get', 'global', 'mobile_data') == '0'
        adb('shell', 'am', 'force-stop', 'com.euv')
        adb('shell', 'am', 'start', '-W', '-n', 'com.euv/.MainActivity')
        tree = await_text(re.escape('v' + version))
        screenshot('01-offline-startup.png')
        report['steps'].append('offline startup displays the locally built example version')
        if any(n.get('text') == '☰' for n in tree.iter('node')):
            tap(tree, '☰')
            tree = await_text('Counter')
        tap(tree, 'Counter')
        tree = await_text(r'The current count is\s*0\D')
        tap(tree, 'Add')
        tree = await_text(r'The current count is\s*1\D')
        screenshot('02-counter-add.png')
        tap(tree, 'Reset')
        await_text(r'The current count is\s*0\D')
        screenshot('03-counter-reset.png')
        report['steps'].append('native WebView Counter interaction changes 0 -> 1 -> 0')
        # This image is a task-owned Google APIs emulator, not a personal device.
        adb('root')
        adb('wait-for-device')
        cache = '/data/user/0/com.euv/cache/euv_web_cache'
        active = adb('exec-out', 'cat', cache + '/active')
        if not re.fullmatch(r'v_\d+', active):
            raise AssertionError('Invalid active cache version: ' + active)
        for entry in manifest['files']:
            content = adb_bytes('exec-out', 'cat', cache + '/' + active + '/' + entry['path'])
            assert len(content) == entry['size'] and hashlib.sha256(content).hexdigest() == entry['sha256'], entry['path']
        report['cache_version'] = active
        report['steps'].append('all deployed cache files match the local example SHA-256 values')
        adb('shell', 'am', 'force-stop', 'com.euv')
        adb('shell', 'am', 'start', '-W', '-n', 'com.euv/.MainActivity')
        await_text(re.escape('v' + version))
        assert adb('exec-out', 'cat', cache + '/active') == active
        screenshot('04-offline-cache-restart.png')
        report['steps'].append('offline restart reuses the same verified cache snapshot')
        report['passed'] = True
        print(json.dumps(report, indent=2))
    finally:
        for namespace, key, value in [('system', 'screen_off_timeout', screen_timeout), ('global', 'stay_on_while_plugged_in', stay_awake)]:
            if value == 'null':
                adb('shell', 'settings', 'delete', namespace, key, check=False)
            else:
                adb('shell', 'settings', 'put', namespace, key, value, check=False)
        if wifi == '1':
            adb('shell', 'svc', 'wifi', 'enable', check=False)
        if mobile == '1':
            adb('shell', 'svc', 'data', 'enable', check=False)
        adb('shell', 'am', 'force-stop', 'com.euv', check=False)
        (output / 'verification.json').write_text(json.dumps(report, indent=2))


if __name__ == '__main__':
    main()
