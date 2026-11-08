#!/usr/bin/env node
'use strict';

const fs = require('node:fs');
const { spawnSync, execFileSync } = require('node:child_process');
const { readExample, verifyBundle, DEFAULT_DESTINATION, sourceDirectory } = require('../build/bundle-example');

function pythonCommand() {
  for (const command of ['python3', 'python']) {
    const result = spawnSync(command, ['--version'], { encoding: 'utf8' });
    if (result.status === 0 && /^Python 3\./.test(result.stdout || result.stderr)) return command;
  }
  throw new Error('Python 3 is required for native artifact verification');
}

function binaryFormat(bytes) {
  if (bytes.subarray(0, 4).equals(Buffer.from([127, 69, 76, 70]))) return 'elf';
  if (bytes.length >= 64 && bytes.subarray(0, 2).toString() === 'MZ') {
    const offset = bytes.readUInt32LE(60);
    if (offset <= bytes.length - 4 && bytes.subarray(offset, offset + 4).equals(Buffer.from('PE\0\0'))) return 'pe';
  }
  if (['feedface', 'feedfacf', 'cefaedfe', 'cffaedfe', 'cafebabe', 'bebafeca'].includes(bytes.subarray(0, 4).toString('hex'))) return 'macho';
  throw new Error('Artifact does not contain a recognized native executable');
}

function verifyNativeBytes(bytes, platform, { source = sourceDirectory(), destination = DEFAULT_DESTINATION } = {}) {
  verifyBundle({ source, destination });
  const expected = { windows: 'pe', macos: 'macho', linux: 'elf', ios: 'macho', 'ios-simulator': 'macho' }[platform];
  if (!expected) throw new Error(`Unsupported client platform: ${platform}`);
  if (binaryFormat(bytes) !== expected) throw new Error(`Wrong native executable format for ${platform}`);
  const snapshot = readExample(source);
  if (!bytes.includes(snapshot.manifestBytes)) throw new Error('Native artifact is missing the local example provenance');
  for (const file of snapshot.files) {
    if (file.size && !bytes.includes(file.bytes)) throw new Error(`Native artifact is missing local example file: ${file.path}`);
  }
  return { platform, format: expected, files: snapshot.manifest.files };
}

function verifyNative(file, platform, options) {
  return verifyNativeBytes(fs.readFileSync(file), platform, options);
}

function verifyIpa(file, options) {
  const script = String.raw`
import pathlib, plistlib, sys, zipfile
with zipfile.ZipFile(sys.argv[1]) as archive:
    infos = [n for n in archive.namelist() if n.startswith('Payload/') and n.count('/') == 2 and n.endswith('.app/Info.plist')]
    if len(infos) != 1:
        raise ValueError('IPA must contain exactly one Payload app')
    info = plistlib.loads(archive.read(infos[0]))
    if 'iPhoneOS' not in info.get('CFBundleSupportedPlatforms', []):
        raise ValueError('Expected an iOS device IPA, not a simulator app')
    if not info.get('UIApplicationSceneManifest', {}).get('UIApplicationSupportsMultipleScenes'):
        raise ValueError('iOS app is missing the required scene lifecycle configuration')
    executable = info['CFBundleExecutable']
    if pathlib.PurePosixPath(executable).name != executable:
        raise ValueError('Invalid IPA executable path')
    binary = infos[0].rsplit('/', 1)[0] + '/' + executable
    sys.stdout.buffer.write(archive.read(binary))
`;
  const bytes = execFileSync(pythonCommand(), ['-c', script, file], { maxBuffer: 512 * 1024 * 1024, stdio: ['ignore', 'pipe', 'pipe'] });
  return verifyNativeBytes(bytes, 'ios', options);
}

if (require.main === module) {
  try {
    const [, , platform, file] = process.argv;
    const result = platform === 'ios' && file?.endsWith('.ipa') ? verifyIpa(file) : verifyNative(file, platform);
    console.log(`[OK] ${platform} executable contains all ${result.files.length} local example files and provenance`);
  } catch (error) {
    console.error(`[ERROR] ${error.stderr?.toString().trim() || error.message}`);
    process.exitCode = 1;
  }
}

module.exports = { pythonCommand, binaryFormat, verifyNativeBytes, verifyNative, verifyIpa };
