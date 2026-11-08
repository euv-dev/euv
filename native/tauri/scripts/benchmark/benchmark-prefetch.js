#!/usr/bin/env node
'use strict';
/**
 * benchmark-prefetch.js
 *
 * Compares the HEAD (baseline) version of scripts/cache/prefetch-cache.js against the
 * working-tree (modified) version, using the exact same local HTTP fixture
 * server and the exact same resource set.
 *
 * For each variant we report:
 *   - elapsed wall-clock time
 *   - peak RSS (via /usr/bin/time -l)
 *   - the sha256 of every produced cache file (hashes must match between the
 *     two variants, proving the optimization did not change the output)
 *
 * Usage: node scripts/benchmark/benchmark-prefetch.js
 */
const fs = require('fs');
const os = require('os');
const path = require('path');
const crypto = require('crypto');
const { spawn, execFileSync } = require('child_process');

const { startFixtureServer, writeConfig } = require('../tests/fixture-server');

const REPO_ROOT = path.resolve(__dirname, '../..');
const SCRIPT_REL = 'scripts/cache/prefetch-cache.js';

function gitShowHead(relPath) {
  const prefix = execFileSync('git', ['-C', REPO_ROOT, 'rev-parse', '--show-prefix'], { encoding: 'utf8' }).trim();
  let sourcePath = prefix + relPath;
  try {
    execFileSync('git', ['-C', REPO_ROOT, 'cat-file', '-e', `HEAD:${sourcePath}`], { stdio: 'ignore' });
  } catch (_) {
    sourcePath = prefix + 'scripts/prefetch-cache.js';
  }
  return execFileSync('git', ['-C', REPO_ROOT, 'show', `HEAD:${sourcePath}`], {
    encoding: 'utf-8',
    maxBuffer: 32 * 1024 * 1024,
  }).replace("path.resolve(__dirname, '..')", "path.resolve(__dirname, '../..')")
    .replace("const CONFIG_NAME = 'app.config.json'", "const CONFIG_NAME = 'config/app.config.json'");
}

function sha256File(file) {
  return crypto
    .createHash('sha256')
    .update(fs.readFileSync(file))
    .digest('hex');
}

function hashTree(dir) {
  const out = {};
  const walk = (d, prefix = '') => {
    for (const entry of fs.readdirSync(d, { withFileTypes: true })) {
      const full = path.join(d, entry.name);
      const rel = prefix ? `${prefix}/${entry.name}` : entry.name;
      if (entry.isDirectory()) walk(full, rel);
      else out[rel] = sha256File(full);
    }
  };
  walk(dir);
  return out;
}

/**
 * Run one variant in an isolated root:
 *   <root>/scripts/cache/prefetch-cache.js
 *   <root>/config/app.config.json
 */
function runChild(cmd, args) {
  return new Promise((resolve) => {
    const child = spawn(cmd, args, { env: { ...process.env } });
    let stdout = '';
    let stderr = '';
    child.stdout.on('data', (d) => (stdout += d));
    child.stderr.on('data', (d) => (stderr += d));
    child.on('close', (status) =>
      resolve({ status, stdout, stderr }),
    );
  });
}

async function runVariant({ label, source, url, delayMs }) {
  const root = fs.mkdtempSync(path.join(os.tmpdir(), `prefetch-bench-${label}-`));
  fs.mkdirSync(path.join(root, 'scripts', 'cache'), { recursive: true });
  fs.writeFileSync(path.join(root, 'scripts', 'cache', 'prefetch-cache.js'), source);
  writeConfig(root, url);

  const script = path.join(root, 'scripts', 'cache', 'prefetch-cache.js');
  const timeBin = '/usr/bin/time';
  const hasTime = fs.existsSync(timeBin);

  // NOTE: the fixture server lives in this process, so the child must be
  // spawned asynchronously - a synchronous spawn would block the event loop
  // and deadlock the server.
  const start = process.hrtime.bigint();
  const res = await runChild(
    hasTime ? timeBin : process.execPath,
    hasTime ? ['-l', process.execPath, script] : [script],
  );
  const elapsedMs = Number(process.hrtime.bigint() - start) / 1e6;

  let maxRssBytes = null;
  if (hasTime) {
    const m = /(\d+)\s+maximum resident set size/.exec(res.stderr);
    // macOS reports bytes.
    if (m) maxRssBytes = Number(m[1]);
  }

  const cacheDir = path.join(root, 'src-tauri', 'bundled-cache');
  const hashes = fs.existsSync(cacheDir) ? hashTree(cacheDir) : {};
  // The manifest embeds a timestamp, so it is compared separately.
  const manifest = fs.existsSync(path.join(cacheDir, '_manifest.json'))
    ? JSON.parse(fs.readFileSync(path.join(cacheDir, '_manifest.json'), 'utf-8'))
    : null;
  const fileCount = Object.keys(hashes).length;

  if (res.status !== 0) {
    console.error(`[bench] ${label} failed (status ${res.status}):`);
    console.error(res.stderr);
  }

  return {
    label,
    root,
    elapsedMs,
    maxRssBytes,
    hashes,
    fileCount,
    manifestFileCount: manifest ? manifest.files.length : 0,
    manifestFiles: manifest ? manifest.files.slice().sort() : [],
    status: res.status,
  };
}

// The manifest embeds a wall-clock timestamp, so it is compared by its file
// list (see manifestSame) rather than by raw hash.
const TIMESTAMPED = new Set(['_manifest.json']);

function compareHashes(a, b) {
  const keys = new Set([...Object.keys(a), ...Object.keys(b)]);
  const diffs = [];
  for (const k of keys) {
    if (TIMESTAMPED.has(k)) continue;
    if (a[k] !== b[k]) diffs.push({ file: k, baseline: a[k], modified: b[k] });
  }
  return diffs;
}

async function main() {
  const fixture = await startFixtureServer();
  const url = `${fixture.url}/base/index.html`;
  console.log(
    `[bench] fixture server: ${fixture.url} (delay ${process.env.FIXTURE_DELAY_MS || 120}ms/response)`,
  );

  const baselineSource = gitShowHead(SCRIPT_REL);
  const modifiedSource = fs.readFileSync(path.join(REPO_ROOT, SCRIPT_REL), 'utf-8');
  console.log(`[bench] baseline source: HEAD:${SCRIPT_REL}`);
  console.log(`[bench] modified source: ${SCRIPT_REL} (working tree)`);

  const baseline = await runVariant({
    label: 'baseline',
    source: baselineSource,
    url,
  });
  console.log(
    `[bench] baseline done: ${baseline.elapsedMs.toFixed(0)} ms, ${baseline.fileCount} files`,
  );

  const modified = await runVariant({
    label: 'modified',
    source: modifiedSource,
    url,
  });
  console.log(
    `[bench] modified done: ${modified.elapsedMs.toFixed(0)} ms, ${modified.fileCount} files`,
  );

  const diffs = compareHashes(baseline.hashes, modified.hashes);

  const speedup = baseline.elapsedMs / modified.elapsedMs;
  console.log('');
  console.log('=== prefetch-cache benchmark (same fixture, same resources) ===');
  console.log(`baseline (HEAD)   elapsed: ${baseline.elapsedMs.toFixed(0)} ms`);
  console.log(`modified          elapsed: ${modified.elapsedMs.toFixed(0)} ms`);
  console.log(`speedup: ${speedup.toFixed(2)}x`);
  console.log(
    `baseline files: ${baseline.fileCount} (manifest lists ${baseline.manifestFileCount})`,
  );
  console.log(
    `modified files: ${modified.fileCount} (manifest lists ${modified.manifestFileCount})`,
  );
  if (baseline.maxRssBytes !== null && modified.maxRssBytes !== null) {
    console.log(
      `peak RSS: baseline ${(baseline.maxRssBytes / 1048576).toFixed(1)} MB, modified ${(modified.maxRssBytes / 1048576).toFixed(1)} MB`,
    );
  }
  console.log(`hash differences: ${diffs.length}`);
  for (const d of diffs) {
    console.log(`  ${d.file}: baseline=${d.baseline} modified=${d.modified}`);
  }
  const manifestSame =
    JSON.stringify(baseline.manifestFiles) ===
    JSON.stringify(modified.manifestFiles);
  console.log(`manifest file lists identical: ${manifestSame}`);

  // Exact per-file hash table for the report.
  console.log('');
  console.log('--- resource file hashes (baseline | modified) ---');
  for (const k of Object.keys(baseline.hashes).sort()) {
    if (TIMESTAMPED.has(k)) continue;
    console.log(`${k} ${baseline.hashes[k]} ${modified.hashes[k]}`);
  }
  console.log('--- timestamped files (hash expected to differ) ---');
  for (const k of Object.keys(baseline.hashes).sort()) {
    if (!TIMESTAMPED.has(k)) continue;
    console.log(`${k} ${baseline.hashes[k]} ${modified.hashes[k]}`);
  }

  await fixture.close();

  const ok = diffs.length === 0 && manifestSame && baseline.status === 0 && modified.status === 0;
  console.log('');
  console.log(ok ? 'RESULT: PASS' : 'RESULT: FAIL');
  // Use exitCode instead of process.exit() so buffered stdout is flushed.
  process.exitCode = ok ? 0 : 1;
}

main().catch((err) => {
  console.error('[bench] fatal:', err);
  process.exitCode = 1;
});
