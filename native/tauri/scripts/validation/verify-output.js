#!/usr/bin/env node
'use strict';

const fs = require('node:fs');
const path = require('node:path');
const crypto = require('node:crypto');
const { execFileSync } = require('node:child_process');
const { verifyBundle } = require('../build/bundle-example');
const { pythonCommand, verifyNative, verifyNativeBytes, verifyIpa } = require('./verify-native');

function verifyOutput(platform, directory = path.resolve(__dirname, '../../artifacts', platform), options) {
  const names = fs.readdirSync(directory).sort();
  const reports = names.filter((name) => name.endsWith('-build.json'));
  if (reports.length !== 1) throw new Error('Expected one client build report');
  const report = JSON.parse(fs.readFileSync(path.join(directory, reports[0]), 'utf8'));
  if (report.format !== 1 || report.platform !== platform) throw new Error('Client build report platform mismatch');
  if (JSON.stringify(report.pages) !== JSON.stringify(verifyBundle(options).files)) throw new Error('Client pages do not match the local example build');
  const listed = new Set();
  for (const entry of report.files) {
    if (path.basename(entry.path) !== entry.path || listed.has(entry.path)) throw new Error('Invalid or duplicate artifact path');
    listed.add(entry.path);
    const bytes = fs.readFileSync(path.join(directory, entry.path));
    if (!bytes.length || bytes.length !== entry.size || crypto.createHash('sha256').update(bytes).digest('hex') !== entry.sha256) {
      throw new Error(`Artifact integrity failure: ${entry.path}`);
    }
  }
  if (names.some((name) => !listed.has(name) && name !== reports[0])) throw new Error('Unexpected files in client artifact directory');
  const prefix = `euv-${platform}-${report.arch}`;
  const patterns = {
    windows: [/\.msi$/i, /-installer-.*\.exe$/i, new RegExp(`^${prefix}\\.exe$`)],
    macos: [/\.app\.tar\.gz$/, /\.dmg$/i],
    linux: [/\.deb$/i, /\.appimage$/i, new RegExp(`^${prefix}$`)],
    ios: [/\.ipa$/i, /\.xcarchive\.tar\.gz$/],
    'ios-simulator': [/\.app\.tar\.gz$/],
  }[platform];
  if (!patterns || patterns.some((pattern) => ![...listed].some((name) => pattern.test(name)))) throw new Error(`Missing required ${platform} package type`);
  if (platform === 'windows' || platform === 'linux') verifyNative(path.join(directory, `${prefix}${platform === 'windows' ? '.exe' : ''}`), platform, options);
  if (platform === 'ios') for (const name of names.filter((name) => name.endsWith('.ipa'))) verifyIpa(path.join(directory, name), options);
  if (platform === 'macos' || platform === 'ios-simulator') {
    const script = String.raw`
import plistlib, sys, tarfile
with tarfile.open(sys.argv[1], 'r:gz') as archive:
    desktop = sys.argv[2] == 'macos'
    suffix = '.app/Contents/Info.plist' if desktop else '.app/Info.plist'
    infos = [m for m in archive.getmembers() if m.name.endswith(suffix)]
    if len(infos) != 1: raise ValueError('Expected exactly one app in archive')
    info = plistlib.load(archive.extractfile(infos[0]))
    prefix = infos[0].name.rsplit('/Info.plist', 1)[0]
    if not desktop and 'iPhoneSimulator' not in info.get('CFBundleSupportedPlatforms', []):
        raise ValueError('Expected an iOS Simulator app, not a device build')
    if not desktop and not info.get('UIApplicationSceneManifest', {}).get('UIApplicationSupportsMultipleScenes'):
        raise ValueError('Simulator app is missing the required scene lifecycle configuration')
    sys.stdout.buffer.write(archive.extractfile(prefix + ('/MacOS/' if desktop else '/') + info['CFBundleExecutable']).read())
`;
    const archive = path.join(directory, names.find((name) => name.endsWith('.app.tar.gz')));
    const bytes = execFileSync(pythonCommand(), ['-c', script, archive, platform], { maxBuffer: 512 * 1024 * 1024 });
    verifyNativeBytes(bytes, platform, options);
  }
  return report;
}

if (require.main === module) {
  try {
    const report = verifyOutput(process.argv[2], process.argv[3]);
    console.log(`[OK] Verified ${report.platform} package types, all file checksums and embedded local example pages`);
  } catch (error) {
    console.error(`[ERROR] ${error.stderr?.toString().trim() || error.message}`);
    process.exitCode = 1;
  }
}

module.exports = { verifyOutput };
