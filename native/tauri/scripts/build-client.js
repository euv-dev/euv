#!/usr/bin/env node
'use strict';

const fs = require('node:fs');
const path = require('node:path');
const crypto = require('node:crypto');
const { execFileSync } = require('node:child_process');
const { bundleExample, verifyBundle, MANIFEST_NAME, DEFAULT_DESTINATION } = require('./bundle-example');
const { pythonCommand, verifyNative, verifyIpa } = require('./verify-native');
const { verifyOutput } = require('./verify-output');

const ROOT = path.resolve(__dirname, '..');
const ARTIFACT_ROOT = path.join(ROOT, 'artifacts');

function buildPlan(platform, host = process.platform, arch = process.arch) {
  const expected = { windows: 'win32', macos: 'darwin', linux: 'linux', ios: 'darwin', 'ios-simulator': 'darwin' }[platform];
  if (!expected) throw new Error(`Unsupported platform: ${platform}`);
  if (host !== expected) throw new Error(`${platform} packages must be built on ${expected}, not ${host}`);
  if (!['arm64', 'x64'].includes(arch)) throw new Error(`Unsupported host architecture: ${arch}`);
  if (platform === 'ios') return { platform, target: 'aarch64-apple-ios', arch: 'arm64', bundles: [] };
  if (platform === 'ios-simulator') return { platform, target: arch === 'arm64' ? 'aarch64-apple-ios-sim' : 'x86_64-apple-ios', arch, bundles: [] };
  if (platform === 'windows') {
    if (arch !== 'x64') throw new Error('Windows installer builds currently require an x64 runner');
    return { platform, target: 'x86_64-pc-windows-msvc', arch: 'x64', bundles: ['nsis', 'msi'] };
  }
  if (platform === 'macos') return { platform, target: `${arch === 'arm64' ? 'aarch64' : 'x86_64'}-apple-darwin`, arch, bundles: ['app', 'dmg'] };
  return { platform, target: `${arch === 'arm64' ? 'aarch64' : 'x86_64'}-unknown-linux-gnu`, arch, bundles: ['deb', 'appimage'] };
}

function run(command, args, options = {}) {
  return execFileSync(command, args, { cwd: ROOT, stdio: 'inherit', ...options });
}

function tauri(args, env) {
  const cli = path.join(ROOT, 'node_modules', '@tauri-apps', 'cli', 'tauri.js');
  if (!fs.existsSync(cli)) throw new Error('Tauri CLI is missing; run npm ci first');
  if (args[0] === 'ios') {
    const npm = fs.realpathSync(path.join(path.dirname(process.execPath), 'npm'));
    return run(process.execPath, [npm, 'run', 'tauri', '--', ...args], { env });
  }
  return run(process.execPath, [cli, ...args], { env });
}

function findDirectories(root, suffix) {
  if (!fs.existsSync(root)) return [];
  const result = [];
  for (const entry of fs.readdirSync(root, { withFileTypes: true })) {
    if (!entry.isDirectory()) continue;
    const file = path.join(root, entry.name);
    if (entry.name.endsWith(suffix)) result.push(file);
    else result.push(...findDirectories(file, suffix));
  }
  return result.sort();
}

function findFiles(root, extension) {
  if (!fs.existsSync(root)) return [];
  return fs.readdirSync(root, { withFileTypes: true }).flatMap((entry) => {
    const file = path.join(root, entry.name);
    return entry.isDirectory() ? findFiles(file, extension) : entry.isFile() && entry.name.toLowerCase().endsWith(extension) ? [file] : [];
  }).sort();
}

function appExecutable(app, desktop) {
  const plist = path.join(app, desktop ? 'Contents/Info.plist' : 'Info.plist');
  const name = run(pythonCommand(), ['-c', 'import plistlib,sys; print(plistlib.load(open(sys.argv[1],"rb"))["CFBundleExecutable"])', plist], { encoding: 'utf8', stdio: ['ignore', 'pipe', 'pipe'] }).trim();
  if (path.basename(name) !== name) throw new Error('Invalid app executable path');
  return path.join(app, desktop ? 'Contents/MacOS' : '', name);
}

function packageIpa(app, destination) {
  const script = String.raw`
import os, pathlib, stat, sys, zipfile
app, target = pathlib.Path(sys.argv[1]), pathlib.Path(sys.argv[2])
with zipfile.ZipFile(target, 'w', zipfile.ZIP_DEFLATED) as archive:
    for item in sorted(app.rglob('*')):
        name = 'Payload/' + app.name + '/' + item.relative_to(app).as_posix()
        if item.is_symlink():
            info = zipfile.ZipInfo(name)
            info.create_system = 3
            info.external_attr = item.lstat().st_mode << 16
            archive.writestr(info, os.readlink(item))
        elif item.is_file():
            archive.write(item, name)
`;
  run(pythonCommand(), ['-c', script, app, destination]);
}

function copyPackages(files, output, prefix) {
  if (!files.length) throw new Error(`Missing ${prefix} package output`);
  for (const file of files) {
    if (!fs.statSync(file).size) throw new Error(`Empty package output: ${file}`);
    fs.copyFileSync(file, path.join(output, `${prefix}-${path.basename(file)}`));
  }
}

function prepareAppleProject(apple, env) {
  const specFile = path.join(apple, 'project.yml');
  const script = 'puts JSON.generate(YAML.load_file(ARGV[0]))';
  const spec = JSON.parse(run('/usr/bin/ruby', ['-ryaml', '-rjson', '-e', script, specFile], { encoding: 'utf8', stdio: ['ignore', 'pipe', 'pipe'] }));
  const minimum = JSON.parse(fs.readFileSync(path.join(ROOT, 'app.config.json'), 'utf8')).ios?.minimumSystemVersion || '15.0';
  spec.options.deploymentTarget.iOS = minimum;
  const targets = Object.entries(spec.targets).filter(([, entry]) => entry.platform === 'iOS' && entry.type === 'application');
  if (targets.length !== 1) throw new Error('Expected one generated iOS application target');
  const extraInfo = JSON.parse(run('plutil', ['-convert', 'json', '-o', '-', path.join(ROOT, 'src-tauri/Info.ios.plist')], { encoding: 'utf8', stdio: ['ignore', 'pipe', 'pipe'] }));
  targets[0][1].info.properties = { ...targets[0][1].info.properties, ...extraInfo };
  const phases = targets[0][1].preBuildScripts.filter((entry) => entry.name === 'Build Rust Code');
  if (phases.length !== 1) throw new Error('Expected one real Rust build phase');
  if (!phases[0].script.includes('EUV_IOS_NATIVE_BUILD')) {
    const original = phases[0].script;
    phases[0].script = 'if [ "${EUV_IOS_NATIVE_BUILD:-}" = "1" ]; then\n  ' + JSON.stringify(process.execPath) + ' ' + JSON.stringify(path.join(__dirname, 'build-ios-native.js')) + '\nelse\n' + original + '\nfi\n';
  }
  // JSON is valid YAML; xcodegen writes the canonical OpenStep pbxproj format.
  fs.writeFileSync(specFile, `${JSON.stringify(spec, null, 2)}\n`);
  run('xcodegen', ['generate', '--spec', specFile, '--project', apple], { env });
  return targets[0][0];
}

function buildSimulator(apple, plan, env, scheme) {
  env.EUV_IOS_NATIVE_BUILD = '1';
  env.EUV_CARGO = run('which', ['cargo'], { encoding: 'utf8', stdio: ['ignore', 'pipe', 'pipe'] }).trim();
  const minimum = JSON.parse(fs.readFileSync(path.join(ROOT, 'app.config.json'), 'utf8')).ios?.minimumSystemVersion || '15.0';
  run('xcodebuild', ['build', '-project', path.join(apple, 'euv-app.xcodeproj'), '-scheme', scheme,
    '-sdk', 'iphonesimulator', '-configuration', 'release',
    '-destination', env.EUV_IOS_SIMULATOR_UDID ? `id=${env.EUV_IOS_SIMULATOR_UDID}` : 'generic/platform=iOS Simulator',
    '-derivedDataPath', path.join(apple, 'build', 'simulator'),
    `ARCHS=${plan.arch === 'arm64' ? 'arm64' : 'x86_64'}`, 'ONLY_ACTIVE_ARCH=YES',
    `IPHONEOS_DEPLOYMENT_TARGET=${minimum}`, 'SUPPORTED_PLATFORMS=iphonesimulator', 'CODE_SIGNING_ALLOWED=NO'], { env });
}

function buildClient(platform) {
  const plan = buildPlan(platform);
  bundleExample();
  run(process.execPath, ['scripts/apply-config.js']);
  run('rustup', ['target', 'add', plan.target]);
  run('cargo', ['metadata', '--manifest-path', 'src-tauri/Cargo.toml', '--format-version', '1', '--filter-platform', plan.target], { stdio: ['ignore', 'ignore', 'inherit'] });
  const env = { ...process.env, CARGO_TARGET_DIR: path.resolve(process.env.CARGO_TARGET_DIR || path.join(ROOT, 'src-tauri/target')) };
  env.CI = 'true';
  if (platform === 'macos') env.SDKROOT = run('xcrun', ['--sdk', 'macosx', '--show-sdk-path'], { encoding: 'utf8', stdio: ['ignore', 'pipe', 'pipe'] }).trim();
  if (platform === 'linux') env.APPIMAGE_EXTRACT_AND_RUN = '1';
  const prefix = `euv-${platform}-${plan.arch}`;
  fs.mkdirSync(ARTIFACT_ROOT, { recursive: true });
  const output = fs.mkdtempSync(path.join(ARTIFACT_ROOT, `.${platform}-stage-`));
  let swiftTools;
  let published = false;
  try {
    if (platform === 'ios' || platform === 'ios-simulator') {
      const simulator = platform === 'ios-simulator';
      env.SDKROOT = run('xcrun', ['--sdk', simulator ? 'iphonesimulator' : 'iphoneos', '--show-sdk-path'], { encoding: 'utf8', stdio: ['ignore', 'pipe', 'pipe'] }).trim();
      env.EUV_REAL_SWIFT = run('xcrun', ['--find', 'swift'], { encoding: 'utf8', stdio: ['ignore', 'pipe', 'pipe'] }).trim();
      swiftTools = fs.mkdtempSync(path.join(ARTIFACT_ROOT, '.ios-swift-'));
      env.EUV_IOS_SWIFT_TOOLS = swiftTools;
      fs.copyFileSync(path.join(__dirname, 'swift-ios.sh'), path.join(swiftTools, 'swift'));
      fs.chmodSync(path.join(swiftTools, 'swift'), 0o755);
      env.PATH = `${swiftTools}${path.delimiter}${env.PATH}`;
      const apple = path.join(ROOT, 'src-tauri/gen/apple');
      const projects = findDirectories(apple, '.xcodeproj');
      if (!projects.some((project) => fs.existsSync(path.join(project, 'project.pbxproj')))) {
        for (const command of ['xcodegen', 'pod', 'idevicesyslog']) {
          run(command, ['--version'], { stdio: 'ignore' });
        }
        tauri(['ios', 'init', '--ci', '--skip-targets-install'], env);
      }
      const scheme = prepareAppleProject(apple, env);
      const build = path.join(apple, 'build');
      for (const archive of findDirectories(build, '.xcarchive')) fs.rmSync(archive, { recursive: true, force: true });
      const signed = !simulator && process.env.EUV_IOS_SIGN === '1';
      if (signed && !process.env.APPLE_DEVELOPMENT_TEAM) throw new Error('Signed iOS exports require APPLE_DEVELOPMENT_TEAM and installed Apple signing credentials');
      const args = ['ios', 'build', '--ci', '--target', simulator ? (plan.arch === 'arm64' ? 'aarch64-sim' : 'x86_64') : 'aarch64'];
      if (signed) args.push('--export-method', 'debugging');
      else args.push('--no-sign', '--archive-only');
      if (simulator) buildSimulator(apple, plan, env, scheme);
      else tauri(args, env);
      const archives = findDirectories(build, '.xcarchive');
      if (!simulator && archives.length !== 1) throw new Error(`Expected one new iOS archive, found ${archives.length}`);
      const applications = findDirectories(simulator ? path.join(build, 'simulator/Build/Products/release-iphonesimulator') : path.join(archives[0], 'Products/Applications'), '.app');
      if (applications.length !== 1) throw new Error('iOS archive does not contain exactly one application');
      verifyNative(appExecutable(applications[0], false), platform);
      if (simulator) {
        run('tar', ['-czf', path.join(output, `${prefix}.app.tar.gz`), '-C', path.dirname(applications[0]), path.basename(applications[0])]);
      } else {
        const ipa = path.join(output, `${prefix}${signed ? '' : '-unsigned'}.ipa`);
        if (signed) {
          const exports = findFiles(build, '.ipa');
          if (exports.length !== 1) throw new Error('Expected one signed IPA export');
          fs.copyFileSync(exports[0], ipa);
        } else packageIpa(applications[0], ipa);
        verifyIpa(ipa);
        run('tar', ['-czf', path.join(output, `${prefix}${signed ? '' : '-unsigned'}.xcarchive.tar.gz`), '-C', path.dirname(archives[0]), path.basename(archives[0])]);
      }
    } else {
      const release = path.join(env.CARGO_TARGET_DIR, plan.target, 'release');
      const bundle = path.join(release, 'bundle');
      fs.rmSync(bundle, { recursive: true, force: true });
      tauri(['build', '--ci', '--target', plan.target, '--bundles', plan.bundles.join(','), '--no-sign', '--', '--locked'], env);
      const binary = path.join(release, platform === 'windows' ? 'euv-app.exe' : 'euv-app');
      verifyNative(binary, platform);
      if (platform === 'macos') {
        const apps = findDirectories(path.join(bundle, 'macos'), '.app');
        if (apps.length !== 1) throw new Error('Expected one macOS app bundle');
        verifyNative(appExecutable(apps[0], true), platform);
        run('tar', ['-czf', path.join(output, `${prefix}.app.tar.gz`), '-C', path.dirname(apps[0]), path.basename(apps[0])]);
        copyPackages(findFiles(path.join(bundle, 'dmg'), '.dmg'), output, prefix);
      } else if (platform === 'windows') {
        fs.copyFileSync(binary, path.join(output, `${prefix}.exe`));
        copyPackages(findFiles(path.join(bundle, 'nsis'), '.exe'), output, `${prefix}-installer`);
        copyPackages(findFiles(path.join(bundle, 'msi'), '.msi'), output, prefix);
      } else {
        fs.copyFileSync(binary, path.join(output, prefix));
        copyPackages(findFiles(path.join(bundle, 'deb'), '.deb'), output, prefix);
        copyPackages(findFiles(path.join(bundle, 'appimage'), '.appimage'), output, prefix);
      }
    }
    const manifest = verifyBundle();
    fs.copyFileSync(path.join(DEFAULT_DESTINATION, MANIFEST_NAME), path.join(output, `${prefix}-pages.json`));
    const files = fs.readdirSync(output).sort().map((name) => {
      const bytes = fs.readFileSync(path.join(output, name));
      return { path: name, size: bytes.length, sha256: crypto.createHash('sha256').update(bytes).digest('hex') };
    });
    fs.writeFileSync(path.join(output, `${prefix}-build.json`), `${JSON.stringify({ format: 1, ...plan, signing: platform === 'ios' && process.env.EUV_IOS_SIGN === '1' ? 'apple-development' : 'unsigned', pages: manifest.files, files }, null, 2)}\n`);
    verifyOutput(platform, output);
    const destination = path.join(ARTIFACT_ROOT, platform);
    fs.rmSync(destination, { recursive: true, force: true });
    fs.renameSync(output, destination);
    published = true;
    console.log(`[OK] Verified ${platform} client artifacts: ${destination}`);
    return destination;
  } finally {
    if (swiftTools) fs.rmSync(swiftTools, { recursive: true, force: true });
    if (!published) fs.rmSync(output, { recursive: true, force: true });
  }
}

if (require.main === module) {
  try {
    buildClient(process.argv[2]);
  } catch (error) {
    console.error(`[ERROR] Client build failed: ${error.stderr?.toString().trim() || error.message}`);
    process.exitCode = 1;
  }
}

module.exports = { buildPlan, buildClient, appExecutable, packageIpa };
