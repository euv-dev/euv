#!/usr/bin/env node
'use strict';

const fs = require('node:fs');
const path = require('node:path');
const crypto = require('node:crypto');
const { execFileSync } = require('node:child_process');

const ROOT = path.resolve(__dirname, '../..');
const MANIFEST_NAME = '__euv_local_bundle__.json';
const DEFAULT_DESTINATION = path.join(ROOT, 'src-tauri', 'bundled-cache');
const WASM_HEADER = Buffer.from([0, 97, 115, 109, 1, 0, 0, 0]);

function sourceDirectory() {
  return path.resolve(process.env.EUV_WEB_DIST || path.join(ROOT, '../../example/www'));
}

function prepareExampleThemes({ source = sourceDirectory(), run = execFileSync } = {}) {
  if (path.resolve(source) !== path.resolve(ROOT, '../../example/www')) return false;
  const script = path.resolve(ROOT, '../../example/scripts/build_theme_variants.sh');
  if (!fs.existsSync(script)) throw new Error('The example theme build script is required for native packaging');
  run('bash', [script], { cwd: path.resolve(ROOT, '../..'), stdio: 'inherit', env: process.env });
  return true;
}

function listFiles(directory, prefix = '') {
  if (!fs.lstatSync(directory).isDirectory()) throw new Error(`Not a regular directory: ${directory}`);
  const files = [];
  for (const entry of fs.readdirSync(directory, { withFileTypes: true }).sort((a, b) => a.name.localeCompare(b.name, 'en'))) {
    const relative = prefix ? `${prefix}/${entry.name}` : entry.name;
    if (entry.isDirectory()) files.push(...listFiles(path.join(directory, entry.name), relative));
    else if (entry.isFile()) files.push(relative);
    else throw new Error(`Unsupported local example entry: ${relative}`);
  }
  return files.sort();
}

function readExample(source = sourceDirectory()) {
  const names = listFiles(source);
  if (names.some((name) => [MANIFEST_NAME, '_manifest.json', '.empty'].includes(name))) {
    throw new Error('Local example output contains a reserved bundle filename');
  }
  const files = names.map((name) => {
    const bytes = fs.readFileSync(path.join(source, name));
    return { path: name, bytes, sha256: crypto.createHash('sha256').update(bytes).digest('hex'), size: bytes.length };
  });
  const index = files.find((file) => file.path === 'index.html');
  if (!index || !index.bytes.toString('utf8').trim()) throw new Error('Local example index.html is missing or empty');
  const wasm = files.filter((file) => file.path.endsWith('.wasm'));
  if (!wasm.length || wasm.some((file) => !file.bytes.subarray(0, 8).equals(WASM_HEADER) || !WebAssembly.validate(file.bytes))) {
    throw new Error('Local example output must contain valid WebAssembly files');
  }
  const manifest = {
    format: 1,
    source: 'euv-example',
    files: files.map(({ path: name, sha256, size }) => ({ path: name, sha256, size })),
  };
  return { files, manifest, manifestBytes: Buffer.from(`${JSON.stringify(manifest, null, 2)}\n`) };
}

function matchesBundle(destination, snapshot) {
  try {
    const expected = [...snapshot.files.map((file) => file.path), MANIFEST_NAME].sort();
    const actual = listFiles(destination).filter((name) => name !== '.empty');
    if (JSON.stringify(actual) !== JSON.stringify(expected)) return false;
    if (!fs.readFileSync(path.join(destination, MANIFEST_NAME)).equals(snapshot.manifestBytes)) return false;
    return snapshot.files.every((file) => fs.readFileSync(path.join(destination, file.path)).equals(file.bytes));
  } catch (error) {
    if (error.code === 'ENOENT') return false;
    throw error;
  }
}

function verifyBundle({ source = sourceDirectory(), destination = DEFAULT_DESTINATION } = {}) {
  const snapshot = readExample(source);
  if (!matchesBundle(destination, snapshot)) throw new Error('Bundled pages do not match the local example output');
  return snapshot.manifest;
}

function bundleExample({ source = sourceDirectory(), destination = DEFAULT_DESTINATION } = {}) {
  source = path.resolve(source);
  destination = path.resolve(destination);
  const relative = path.relative(source, destination);
  const reverse = path.relative(destination, source);
  if (!relative || (!relative.startsWith(`..${path.sep}`) && relative !== '..') ||
      (!reverse.startsWith(`..${path.sep}`) && reverse !== '..')) {
    throw new Error('Example source and bundle destination must not overlap');
  }
  const snapshot = readExample(source);
  if (matchesBundle(destination, snapshot)) return { ...snapshot.manifest, changed: false };
  fs.mkdirSync(path.dirname(destination), { recursive: true });
  const staging = fs.mkdtempSync(path.join(path.dirname(destination), '.bundled-cache-stage-'));
  const previous = `${staging}-previous`;
  let movedPrevious = false;
  let published = false;
  try {
    for (const file of snapshot.files) {
      const target = path.join(staging, file.path);
      fs.mkdirSync(path.dirname(target), { recursive: true });
      fs.writeFileSync(target, file.bytes);
    }
    fs.writeFileSync(path.join(staging, MANIFEST_NAME), snapshot.manifestBytes);
    if (fs.existsSync(destination)) {
      fs.renameSync(destination, previous);
      movedPrevious = true;
    }
    try {
      fs.renameSync(staging, destination);
      published = true;
    } catch (error) {
      if (movedPrevious) fs.renameSync(previous, destination);
      movedPrevious = false;
      throw error;
    }
    return { ...snapshot.manifest, changed: true };
  } finally {
    fs.rmSync(staging, { recursive: true, force: true });
    if (published && movedPrevious) fs.rmSync(previous, { recursive: true, force: true });
  }
}

if (require.main === module) {
  try {
    prepareExampleThemes();
    const result = bundleExample();
    console.log(`[OK] Bundled ${result.files.length} local example files (${result.changed ? 'updated' : 'unchanged'})`);
  } catch (error) {
    console.error(`[ERROR] Cannot bundle local example: ${error.message}`);
    process.exitCode = 1;
  }
}

module.exports = { MANIFEST_NAME, DEFAULT_DESTINATION, sourceDirectory, prepareExampleThemes, readExample, bundleExample, verifyBundle };
