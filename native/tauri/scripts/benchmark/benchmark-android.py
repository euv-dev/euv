#!/usr/bin/env python3
"""Measure real emulator activity startup, visible content and app-process memory."""

import argparse
import hashlib
import json
from pathlib import Path
import re
import statistics
import subprocess
import time
import xml.etree.ElementTree as ET

ROOT = Path(__file__).resolve().parents[2]


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--serial', required=True)
    parser.add_argument('--apk', required=True)
    parser.add_argument('--version', required=True)
    parser.add_argument('--iterations', type=int, default=3)
    parser.add_argument('--output', required=True)
    args = parser.parse_args()
    if not re.fullmatch(r'emulator-\d+', args.serial):
        raise SystemExit('Only task-owned disposable emulators are supported')
    if not 1 <= args.iterations <= 10:
        raise SystemExit('Iterations must be between 1 and 10')
    output = Path(args.output).resolve()
    output.mkdir(parents=True, exist_ok=True)
    adb = str(ROOT / '.cache/sdk/android/platform-tools/adb')

    def run(*command):
        return subprocess.check_output([adb, '-s', args.serial, *command], timeout=90, text=True).strip()

    def hierarchy():
        run('shell', 'uiautomator', 'dump', '/sdcard/euv-benchmark.xml')
        return ET.fromstring(run('exec-out', 'cat', '/sdcard/euv-benchmark.xml'))

    def ready():
        deadline = time.monotonic() + 60
        while time.monotonic() < deadline:
            try:
                tree = hierarchy()
            except (subprocess.CalledProcessError, subprocess.TimeoutExpired) as error:
                print('Accessibility sampler transient failure:', error, flush=True)
                time.sleep(0.5)
                continue
            if any(n.get('resource-id') == 'com.android.systemui:id/immersive_cling_title' for n in tree.iter('node')):
                button = next(n for n in tree.iter('node') if n.get('resource-id') == 'com.android.systemui:id/ok')
                left, top, right, bottom = map(int, re.findall(r'\d+', button.get('bounds', '')))
                run('shell', 'input', 'tap', str((left + right) // 2), str((top + bottom) // 2))
                continue
            if any(n.get('text') == 'v' + args.version for n in tree.iter('node')):
                return
            time.sleep(0.25)
        raise AssertionError('The expected local page did not render')

    settings = [(namespace, key, run('shell', 'settings', 'get', namespace, key)) for namespace, key in [
        ('global', 'wifi_on'), ('global', 'mobile_data'), ('system', 'screen_off_timeout'),
        ('global', 'stay_on_while_plugged_in')]]
    rows = []
    try:
        run('shell', 'settings', 'put', 'system', 'screen_off_timeout', '2147483647')
        run('shell', 'svc', 'power', 'stayon', 'true')
        run('shell', 'input', 'keyevent', 'KEYCODE_WAKEUP')
        run('shell', 'wm', 'dismiss-keyguard')
        run('install', '-r', str(Path(args.apk).resolve()))
        run('shell', 'svc', 'wifi', 'disable')
        run('shell', 'svc', 'data', 'disable')
        for iteration in range(args.iterations):
            for mode in ['cold', 'cached']:
                if mode == 'cold':
                    run('shell', 'pm', 'clear', 'com.euv')
                    run('shell', 'pm', 'grant', 'com.euv', 'android.permission.POST_NOTIFICATIONS')
                run('shell', 'am', 'force-stop', 'com.euv')
                started = time.monotonic()
                activity = run('shell', 'am', 'start', '-W', '-n', 'com.euv/.MainActivity')
                ready()
                observed_ms = (time.monotonic() - started) * 1000
                time.sleep(1)
                memory = run('shell', 'dumpsys', 'meminfo', 'com.euv')
                graphics = run('shell', 'dumpsys', 'gfxinfo', 'com.euv')
                prefix = f'{iteration + 1}-{mode}'
                (output / (prefix + '-activity.txt')).write_text(activity)
                (output / (prefix + '-memory.txt')).write_text(memory)
                (output / (prefix + '-graphics.txt')).write_text(graphics)
                total = re.search(r'TOTAL\s+PSS:\s*(\d+)', memory)
                if total is None:
                    total = re.search(r'^\s*TOTAL\s+(\d+)', memory, re.MULTILINE)
                timing = re.search(r'TotalTime:\s*(\d+)', activity)
                if total is None or timing is None:
                    raise AssertionError('Required real activity/memory counters are missing')
                rows.append({'iteration': iteration + 1, 'mode': mode,
                             'activity_total_ms': int(timing.group(1)),
                             'observed_ui_ready_ms': round(observed_ms, 2),
                             'app_pss_kib': int(total.group(1))})
        display = run('shell', 'dumpsys', 'display')
        renderer = run('logcat', '-d', '-v', 'brief', '-s', 'EUV_RENDERER')
        (output / 'display.txt').write_text(display)
        (output / 'renderer.txt').write_text(renderer)
        summary = {mode: {metric: statistics.median(r[metric] for r in rows if r['mode'] == mode)
                          for metric in ['activity_total_ms', 'observed_ui_ready_ms', 'app_pss_kib']}
                   for mode in ['cold', 'cached']}
        report = {'serial': args.serial, 'apk_sha256': hashlib.sha256(Path(args.apk).read_bytes()).hexdigest(),
                  'web_version': args.version, 'iterations': args.iterations, 'summary': summary, 'samples': rows,
                  'limitations': ['Emulator GPU is host-emulated, not proof of physical GPU throughput',
                                  'Activity TotalTime is not WebView first paint',
                                  'Observed UI readiness includes ADB/accessibility sampling latency',
                                  'PSS is the app process, not the sum of all isolated WebView/GPU processes']}
        (output / 'results.json').write_text(json.dumps(report, indent=2))
        print(json.dumps(report, indent=2))
    finally:
        run('shell', 'am', 'force-stop', 'com.euv')
        for namespace, key, value in settings:
            if value == 'null':
                run('shell', 'settings', 'delete', namespace, key)
            else:
                run('shell', 'settings', 'put', namespace, key, value)
        for service, key in [('wifi', 'wifi_on'), ('data', 'mobile_data')]:
            value = next(v for n, k, v in settings if k == key)
            run('shell', 'svc', service, 'enable' if value == '1' else 'disable')


if __name__ == '__main__':
    main()
