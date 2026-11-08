#!/usr/bin/env node
'use strict';

// A real Cargo build phase for Xcode simulator archives. This avoids
// cargo-mobile2's SDK/runtime-version equality gate, not native compilation.
const fs = require('node:fs');
const path = require('node:path');
const { execFileSync } = require('node:child_process');

const ROOT = path.resolve(__dirname, '../..');

function buildNative(env = process.env) {
  const simulator = env.PLATFORM_NAME === 'iphonesimulator';
  if (!simulator) throw new Error('The direct native phase is restricted to iOS Simulator builds');
  const arches = (env.ARCHS || '').trim().split(/\s+/);
  if (arches.length !== 1 || !['arm64', 'x86_64'].includes(arches[0])) throw new Error('Expected one supported simulator architecture');
  const arch = arches[0];
  const target = arch === 'arm64' ? 'aarch64-apple-ios-sim' : 'x86_64-apple-ios';
  const profile = env.CONFIGURATION?.toLowerCase() === 'release' ? 'release' : 'debug';
  if (!env.SDKROOT || !fs.existsSync(path.join(env.SDKROOT, 'usr/include'))) throw new Error('Xcode did not provide a valid simulator SDK');
  const buildEnv = { ...env };
  const key = target.replaceAll('-', '_');
  buildEnv[`CFLAGS_${key}`] = `-isysroot ${env.SDKROOT}`;
  buildEnv[`CXXFLAGS_${key}`] = `-isysroot ${env.SDKROOT}`;
  buildEnv[`OBJC_INCLUDE_PATH_${key}`] = path.join(env.SDKROOT, 'usr/include');
  buildEnv.TAURI_IOS_PROJECT_PATH = path.join(ROOT, 'src-tauri/gen/apple');
  if (env.EUV_IOS_SWIFT_TOOLS) buildEnv.PATH = `${env.EUV_IOS_SWIFT_TOOLS}${path.delimiter}${env.PATH}`;
  const args = ['build', '--manifest-path', path.join(ROOT, 'src-tauri/Cargo.toml'), '--lib', '--target', target, '--features', 'tauri/custom-protocol', '--locked'];
  if (profile === 'release') args.push('--release');
  execFileSync(env.EUV_CARGO || 'cargo', args, { cwd: ROOT, env: buildEnv, stdio: 'inherit' });
  const targetDir = env.CARGO_TARGET_DIR || path.join(ROOT, 'src-tauri/target');
  const library = path.join(targetDir, target, profile, 'libeuv_lib.a');
  if (!fs.existsSync(library) || !fs.statSync(library).size) throw new Error('The real Cargo build did not produce the iOS static library');
  const destination = path.join(ROOT, 'src-tauri/gen/apple/Externals', arch, profile, 'libapp.a');
  fs.mkdirSync(path.dirname(destination), { recursive: true });
  fs.copyFileSync(library, destination);
  console.log(`[OK] Built and staged real simulator Rust library: ${destination}`);
  return destination;
}

if (require.main === module) {
  try {
    buildNative();
  } catch (error) {
    console.error(`[ERROR] Native iOS build failed: ${error.message}`);
    process.exitCode = 1;
  }
}

module.exports = { buildNative };
