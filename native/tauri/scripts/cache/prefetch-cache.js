#!/usr/bin/env node
'use strict';
/**
 * prefetch-cache.js
 *
 * Downloads remote resources at build time and saves them into
 * src-tauri/bundled-cache/ so they are embedded into the binary.
 * This ensures the app can start offline on first launch.
 *
 * Usage: node scripts/cache/prefetch-cache.js
 *
 * The module is importable (requiring it has no side effects) so the behaviour
 * can be exercised against a real local HTTP server:
 *
 *   const { runPrefetch } = require('../scripts/prefetch-cache');
 *   await runPrefetch({ root, outDir, concurrency: 6 });
 */

const fs = require('fs');
const path = require('path');
const https = require('https');
const http = require('http');
const zlib = require('zlib');

const ROOT = path.resolve(__dirname, '../..');
const CONFIG_NAME = 'config/app.config.json';
const BUNDLED_CACHE_SUBDIR = path.join('src-tauri', 'bundled-cache');

/** Default number of parallel downloads. */
const DEFAULT_CONCURRENCY = 6;
/** Default per-request timeout (ms). */
const DEFAULT_TIMEOUT_MS = Number(process.env.PREFETCH_TIMEOUT_MS || 30000);
/** Maximum number of redirects followed for a single resource. */
const MAX_REDIRECTS = 10;

/**
 * Loads config/app.config.json from a project root.
 *
 * @param {string} [root] - Project root that contains config/app.config.json.
 * @returns {object} Parsed configuration.
 */
function loadConfig(root = ROOT) {
  const configPath = path.join(root, CONFIG_NAME);
  if (!fs.existsSync(configPath)) {
    throw new Error(`${CONFIG_NAME} not found at ${configPath}`);
  }
  return JSON.parse(fs.readFileSync(configPath, 'utf-8'));
}

/**
 * Splits a reference into its path part and its query part.
 * The fragment is always dropped (it is never sent to the server).
 *
 * @param {string} ref
 * @returns {{path: string, query: string}}
 */
function splitRef(ref) {
  let rest = ref;
  const hash = rest.indexOf('#');
  if (hash !== -1) rest = rest.slice(0, hash);
  const query = rest.includes('?') ? rest.slice(rest.indexOf('?')) : '';
  const queryless = query ? rest.slice(0, rest.indexOf('?')) : rest;
  return { path: queryless, query };
}

/**
 * Resolves a relative reference against a base directory, mirroring the Rust
 * runtime's `resolve_local_path`: `..` never escapes the cache root, it is
 * clamped at the root instead. This is what makes path traversal impossible.
 *
 * @param {string[]} baseSegments - Directory segments of the referring file.
 * @param {string} refPath - Reference path (no query, no fragment).
 * @returns {string} Clean, root-relative cache path.
 */
function resolveLocalPath(baseSegments, refPath) {
  const segments = baseSegments.slice();
  for (const part of refPath.split('/')) {
    if (part === '' || part === '.') continue;
    if (part === '..') {
      segments.pop();
      continue;
    }
    segments.push(part);
  }
  return segments.join('/');
}

/**
 * Returns true when the raw reference would escape the cache root before it
 * is clamped. Such references are still clamped (never written outside the
 * cache) but they are reported so the caller can warn about them.
 *
 * @param {string[]} baseSegments
 * @param {string} refPath
 * @returns {boolean}
 */
function escapesRoot(baseSegments, refPath) {
  let depth = baseSegments.length;
  for (const part of refPath.split('/')) {
    if (part === '' || part === '.') continue;
    if (part === '..') {
      depth -= 1;
      if (depth < 0) return true;
      continue;
    }
    depth += 1;
  }
  return false;
}

/**
 * Performs a single HTTP(S) GET, following redirects manually so that the
 * FINAL url of the resource is always known.
 *
 * Robustness rules enforced here:
 *  - the deadline covers the WHOLE request (connect + headers + body), not
 *    just socket inactivity, so a server that trickles bytes forever cannot
 *    keep the build hanging;
 *  - the agent is picked per URL, so an http -> https redirect never reuses
 *    the wrong protocol's agent;
 *  - every listener is detached and every stream destroyed on failure, so no
 *    socket or decompressor is leaked.
 *
 * @param {string} url - Absolute URL to fetch.
 * @param {object} [options]
 * @param {number} [options.maxRedirects]
 * @param {number} [options.timeoutMs]
 * @param {number} [options.deadline] - Absolute timestamp the request must
 *   finish before (shared across a redirect chain).
 * @param {(url: string) => object} [options.agentFor] - Picks the agent for
 *   a given URL (handles http/https switches on redirect).
 * @returns {Promise<{finalUrl: string, body: Buffer}>}
 */
function requestOnce(url, options = {}) {
  const maxRedirects =
    options.maxRedirects === undefined ? MAX_REDIRECTS : options.maxRedirects;
  const timeoutMs =
    options.timeoutMs === undefined ? DEFAULT_TIMEOUT_MS : options.timeoutMs;
  const deadline =
    options.deadline === undefined ? Date.now() + timeoutMs : options.deadline;
  const agentFor = options.agentFor;

  return new Promise((resolve, reject) => {
    let client;
    try {
      client = new URL(url).protocol === 'https:' ? https : http;
    } catch (err) {
      reject(new Error(`Invalid URL: ${url}`));
      return;
    }

    let settled = false;
    let request = null;
    let timer = null;
    let activeStream = null;
    const cleanup = () => {
      if (timer) {
        clearTimeout(timer);
        timer = null;
      }
    };
    const fail = (err) => {
      if (settled) return;
      settled = true;
      cleanup();
      // Destroy the decompression chain (if any) and the request so no
      // socket, zlib handle or timer survives a failed request.
      if (activeStream && typeof activeStream.destroy === 'function') {
        activeStream.destroy();
        activeStream = null;
      }
      if (request) request.destroy();
      reject(err);
    };
    const succeed = (value) => {
      if (settled) return;
      settled = true;
      cleanup();
      resolve(value);
    };

    const remaining = deadline - Date.now();
    if (remaining <= 0) {
      fail(new Error(`Timeout after ${timeoutMs}ms for ${url}`));
      return;
    }
    // Whole-request deadline: covers connect, headers AND body streaming.
    timer = setTimeout(() => {
      fail(new Error(`Timeout after ${timeoutMs}ms for ${url}`));
    }, remaining);

    const requestOptions = {
      headers: { 'Accept-Encoding': 'gzip, deflate, identity' },
    };
    const agent = agentFor ? agentFor(url) : undefined;
    if (agent) requestOptions.agent = agent;

    request = client.get(url, requestOptions, (res) => {
      const status = res.statusCode || 0;
      if (status >= 300 && status < 400 && res.headers.location) {
        // Detach every listener and drain the redirect body so the socket is
        // released before the next hop is requested. The outer request must
        // not fail the chain any more: the redirect target is fetched by the
        // nested request below.
        res.removeAllListeners('data');
        res.removeAllListeners('end');
        res.removeAllListeners('error');
        res.on('error', () => {});
        res.resume();
        if (request) {
          // Keep a listener attached (an 'error' event without a listener
          // would crash the process) but ignore it: the redirect target is
          // fetched by the nested request below.
          request.removeAllListeners('error');
          request.on('error', () => {});
        }
        if (maxRedirects <= 0) {
          fail(new Error(`Too many redirects for ${url}`));
          return;
        }
        let redirectUrl;
        try {
          redirectUrl = new URL(res.headers.location, url).toString();
        } catch (err) {
          fail(new Error(`Invalid redirect location for ${url}`));
          return;
        }
        requestOnce(redirectUrl, {
          maxRedirects: maxRedirects - 1,
          timeoutMs,
          deadline,
          agentFor,
        }).then(succeed, fail);
        return;
      }
      if (status !== 200) {
        res.removeAllListeners('data');
        res.removeAllListeners('end');
        res.removeAllListeners('error');
        res.on('error', () => {});
        res.resume();
        fail(new Error(`HTTP ${status} for ${url}`));
        return;
      }

      // Handle content-encoding (decompress if needed).
      let stream = res;
      const encoding = (res.headers['content-encoding'] || '').toLowerCase();
      if (encoding === 'gzip') {
        stream = res.pipe(zlib.createGunzip());
      } else if (encoding === 'deflate') {
        stream = res.pipe(zlib.createInflate());
      } else if (encoding === 'br') {
        stream = res.pipe(zlib.createBrotliDecompress());
      }
      const chunks = [];
      activeStream = stream;
      const onStreamError = (err) => {
        // Destroy the decompressor chain so nothing keeps the socket alive.
        if (stream !== res && typeof stream.destroy === 'function') {
          stream.destroy();
        }
        if (typeof res.destroy === 'function') res.destroy();
        fail(err);
      };
      stream.on('data', (chunk) => chunks.push(chunk));
      stream.on('error', onStreamError);
      res.on('error', onStreamError);
      stream.on('end', () => {
        activeStream = null;
        succeed({ finalUrl: url, body: Buffer.concat(chunks) });
      });
    });

    request.on('error', (err) => fail(err));
  });
}

/**
 * Follows redirects and downloads a URL, returning { finalUrl, body } where
 * finalUrl is the URL after all redirects and body is a Buffer.
 *
 * @param {string} url - Absolute URL to download.
 * @param {object} [options] - See requestOnce.
 * @returns {Promise<{finalUrl: string, body: Buffer}>}
 */
function downloadUrl(url, options = {}) {
  return requestOnce(url, options);
}

/**
 * Extract relative resource paths from HTML (script src, link href, img src).
 *
 * @param {string} html
 * @returns {string[]} Unique relative resource paths.
 */
function extractResourcePaths(html) {
  const paths = [];
  const patterns = [
    /< *script[^>]+src=["']([^"']+)["']/gi,
    /< *link[^>]+href=["']([^"']+)["']/gi,
    /< *img[^>]+src=["']([^"']+)["']/gi,
  ];
  for (const pattern of patterns) {
    let match;
    while ((match = pattern.exec(html)) !== null) {
      const val = match[1];
      // Skip absolute URLs and data URIs
      if (
        val.startsWith('http://') ||
        val.startsWith('https://') ||
        val.startsWith('//') ||
        val.startsWith('data:')
      ) {
        continue;
      }
      paths.push(val);
    }
  }
  // Also extract ES module imports: import ... from './path'
  const importPattern = /import\s+.*?from\s+['"](\.[^'"]+)['"]/g;
  let m;
  while ((m = importPattern.exec(html)) !== null) {
    paths.push(m[1]);
  }
  // IIFE inline-bridge format (euv PR #250): no external script/link tags,
  // but the inline JS sets `var __euv_wasm_url = "pkg/<name>_bg.wasm"`.
  const iifeWasmPattern = /__euv_wasm_url\s*=\s*["']([^"']+\.wasm)["']/g;
  while ((m = iifeWasmPattern.exec(html)) !== null) {
    const val = m[1];
    if (!val.includes('://')) {
      paths.push(val);
    }
  }
  return [...new Set(paths)];
}

/**
 * Derive the base URL for resolving relative resource paths.
 *
 * The base URL is the DIRECTORY that contains the final page, so relative
 * references resolve against it. The directory is computed with
 * `new URL('.', finalUrl)`, which correctly ignores the query string and the
 * fragment (a query may itself contain a `/`, which a naive string scan would
 * mistake for a path separator).
 *
 * e.g. "https://ltpp.vip/github/pages/euv-dev/euv/" -> "https://ltpp.vip/github/pages/euv-dev/euv"
 *      "https://ltpp.vip/euv"                        -> "https://ltpp.vip"
 *      "https://ltpp.vip/euv/index.html?a=/b"        -> "https://ltpp.vip/euv"
 *
 * @param {string} finalUrl
 * @returns {string}
 */
function deriveBaseUrl(finalUrl) {
  const raw = String(finalUrl);
  try {
    const parsed = new URL(raw);
    // `new URL('.', ...)` resolves to the containing directory; drop the
    // trailing slash so callers can append `/${path}`.
    const dir = new URL('.', parsed).toString();
    return dir.endsWith('/') ? dir.slice(0, -1) : dir;
  } catch (err) {
    // Fallback for non-absolute urls: strip the query/fragment, then the last
    // path segment.
    const withoutHash = raw.split('#')[0];
    const withoutQuery = withoutHash.split('?')[0];
    const trimmed = withoutQuery.replace(/\/$/, '');
    if (withoutQuery.endsWith('/')) {
      return trimmed;
    }
    const lastSlash = trimmed.lastIndexOf('/');
    const schemeEnd = trimmed.indexOf('://');
    if (lastSlash > schemeEnd + 2) {
      return trimmed.slice(0, lastSlash);
    }
    return trimmed;
  }
}

/**
 * Extract dependency references (.wasm and relative ES imports) from JS content.
 *
 * Each discovered dependency is resolved against the JS file's FINAL URL (after
 * redirects) so that the resulting download URL is correct even when the JS file
 * itself was served from a redirected location. The local cache path is derived
 * from the JS file's clean cache path (its directory), so `pkg/euv.js` referencing
 * `euv_bg.wasm` lands in `pkg/euv_bg.wasm` — exactly how it is served later.
 *
 * Query strings and fragments are stripped from the cache path: the runtime
 * cache is keyed by URL path only, so `a.js?v=2` and `a.js` are one entry.
 *
 * @param {string} jsContent - The JS source code.
 * @param {string} jsFinalUrl - The final URL of the JS file (after redirects).
 * @param {string} jsCleanPath - The JS file's path relative to the cache root.
 * @returns {Array<{url: string, cleanPath: string, ref: string, unsafe: boolean}>}
 */
function extractJsDependencies(jsContent, jsFinalUrl, jsCleanPath) {
  const refs = [];
  // 1) .wasm string literals, e.g. new URL('euv_bg.wasm', import.meta.url).
  //    A query string and/or a fragment may follow the file name.
  const wasmPattern =
    /['"]([^'"]*?\.wasm(?:\?[^'"]*)?(?:#[^'"]*)?)['"]/g;
  let match;
  while ((match = wasmPattern.exec(jsContent)) !== null) {
    refs.push(match[1]);
  }
  // 2) static ES module imports and re-exports:
  //      import './side-effect.js';
  //      import x from './dep.js';
  //      export { x } from './dep.js';
  //      export * from './dep.js';
  //    Root-relative specifiers ('/pkg/x.js') are matched as well; protocol
  //    relative ('//host/x.js') and absolute URLs are not.
  const importPattern =
    /(?:import|export)\s+(?:[^'"]*?\s+from\s+)?['"]((?:\.\.?\/|\/(?!\/))[^'"]+)['"]/g;
  while ((match = importPattern.exec(jsContent)) !== null) {
    refs.push(match[1]);
  }
  // 3) dynamic import('./dep.js') / import('/pkg/dep.js')
  const dynamicPattern =
    /import\s*\(\s*['"]((?:\.\.?\/|\/(?!\/))[^'"]+)['"]\s*\)/g;
  while ((match = dynamicPattern.exec(jsContent)) !== null) {
    refs.push(match[1]);
  }

  const results = [];
  const seen = new Set();
  const jsCacheDir = path.posix.dirname(jsCleanPath); // dir within cache root
  const baseSegments =
    jsCacheDir === '.' || jsCacheDir === '' ? [] : jsCacheDir.split('/');
  for (const ref of refs) {
    if (ref.includes('://') || ref.startsWith('//') || ref.includes(' ')) {
      continue;
    }
    // Resolve the absolute download URL against the JS file's FINAL url.
    let absoluteUrl;
    try {
      absoluteUrl = new URL(ref, jsFinalUrl).toString();
    } catch (err) {
      continue;
    }
    const { path: refPath } = splitRef(ref);
    // Root-relative references are anchored at the cache root, everything else
    // at the directory of the referring file.
    const segments = refPath.startsWith('/') ? [] : baseSegments;
    const cleanPath = resolveLocalPath(segments, refPath);
    const unsafe = escapesRoot(segments, refPath);
    if (!seen.has(cleanPath)) {
      seen.add(cleanPath);
      results.push({ url: absoluteUrl, cleanPath, ref, unsafe });
    }
  }
  return results;
}

/**
 * Maps `items` through `fn` with at most `limit` concurrent invocations.
 *
 * The result array keeps the input order. Items appended to `items` while the
 * mapping is running are picked up as well, which is how the prefetcher
 * schedules transitive dependencies discovered on the fly.
 *
 * All `limit` workers are started upfront — even when the queue holds a single
 * item — so a dependency graph that fans out from one file is still downloaded
 * with the full parallelism.
 *
 * @template T, R
 * @param {T[]} items - Work list (may grow while running).
 * @param {number} limit - Maximum number of concurrent workers.
 * @param {(item: T, index: number) => Promise<R>|R} fn - Worker.
 * @returns {Promise<R[]>} Results in input order.
 */
async function mapWithConcurrency(items, limit, fn) {
  const bounded = Math.max(1, Math.floor(limit) || 1);
  const results = new Array(items.length);
  let cursor = 0;
  let active = 0;
  const waiters = [];
  const wake = () => {
    while (waiters.length > 0) {
      const resolve = waiters.shift();
      resolve();
    }
  };

  const worker = async () => {
    for (;;) {
      if (cursor < items.length) {
        const index = cursor++;
        active++;
        try {
          results[index] = await fn(items[index], index);
        } finally {
          active--;
          wake();
        }
        continue;
      }
      // No work right now. Wait while another worker is still running (it may
      // discover more work), otherwise we are done.
      if (active === 0) break;
      await new Promise((resolve) => {
        waiters.push(resolve);
        // Re-check: the last active worker may have finished (and may have
        // appended new items) between the check above and this registration.
        if (cursor < items.length || active === 0) {
          const pos = waiters.indexOf(resolve);
          if (pos !== -1) waiters.splice(pos, 1);
          resolve();
        }
      });
    }
  };

  await Promise.all(Array.from({ length: bounded }, worker));
  return results;
}

/**
 * Patches the downloaded HTML:
 *  - adds `viewport-fit=cover` so `env(safe-area-inset-*)` is honoured;
 *  - forces the initial route to `#/form` so the IME bridge is visible.
 *
 * @param {string} html
 * @returns {string}
 */
function patchHtml(html) {
  let out = html;
  // euv-app immersive patch. The downloaded euv Pages build does NOT set
  // `viewport-fit=cover` on its <meta name="viewport"> tag (euv master
  // template PR #59 removed it for the regular web target), which means
  // CSS `env(safe-area-inset-top)` returns 0px on Android. Without that
  // signal, `IMMERSIVE_SAFE_AREA_SCRIPT.applyInset()` writes
  // `--euv-mobile-safe-top: 0px` and `c_mobile_header`'s `padding-top`
  // collapses onto the status bar. We add `viewport-fit=cover` here at
  // prefetch time so the page is parsed with the cover-fit flag from the
  // very first byte — runtime meta rewrites via JS are too late because
  // the browser locks the viewport-fit value at parse time.
  out = out.replace(
    /<meta\s+name="viewport"\s+content="([^"]*)"\s*\/>/,
    (match, content) => {
      if (!/viewport-fit\s*=\s*cover/.test(content)) {
        return `<meta name="viewport" content="${content}, viewport-fit=cover" />`;
      }
      return match;
    },
  );
  // Redirect to the form demo on launch so the IME bridge and the
  // framework's focus-scroll fix are immediately visible. The downloaded
  // euv Pages build opens the static About screen, which has no inputs.
  if (!/window\.location\.hash\s*=\s*['"]#\/form['"]/.test(out)) {
    const redirectScript =
      '<script>(function(){if(!/^#\/form$/.test(window.location.hash))' +
      '{window.location.hash="#/form";}})();</script>';
    if (/<\/head>/i.test(out)) {
      out = out.replace(/<\/head>/i, redirectScript + '</head>');
    } else if (/<body[^>]*>/i.test(out)) {
      out = out.replace(/(<body[^>]*>)/i, '$1' + redirectScript);
    } else {
      out = redirectScript + out;
    }
  }
  return out;
}

/**
 * Builds the absolute download URL for a reference found in the HTML.
 *
 * The cache path and the download URL use the SAME relative path (relative to
 * the page's base directory), so what is stored on disk always matches what is
 * served later. `..` segments are clamped at the base directory, so a
 * reference can never escape it.
 *
 * @param {string} baseUrl - Base URL derived from the index.html final URL.
 * @param {string} ref - Reference as written in the document.
 * @returns {string}
 */
function resolveHtmlRefUrl(baseUrl, ref) {
  const { query } = splitRef(ref);
  const { cleanPath } = htmlRefCleanPath(ref);
  return `${baseUrl}/${cleanPath}${query}`;
}

/**
 * Computes the canonical cache path for a reference found in the HTML.
 *
 * Follows the runtime cache convention (the `euv://` scheme is keyed by the
 * request path): the query string and fragment are dropped, and leading `/`
 * or `./` are stripped. `..` segments are clamped at the cache root, so a
 * reference can never escape the cache directory.
 *
 * @param {string} ref
 * @returns {{cleanPath: string, unsafe: boolean}}
 */
function htmlRefCleanPath(ref) {
  const { path: refPath } = splitRef(ref);
  const trimmed = refPath.replace(/^\.\//, '').replace(/^\/+/, '');
  return {
    cleanPath: resolveLocalPath([], trimmed),
    unsafe: escapesRoot([], trimmed),
  };
}

/**
 * Writes a file inside `dir`, creating parent directories as needed.
 *
 * @param {string} dir
 * @param {string} cleanPath - Path relative to `dir`.
 * @param {Buffer|string} data
 */
function writeInto(dir, cleanPath, data) {
  const target = path.join(dir, cleanPath);
  fs.mkdirSync(path.dirname(target), { recursive: true });
  fs.writeFileSync(target, data);
}

/** Compare a pair of resource files without retaining either complete body. */
function sameFileContents(left, right) {
  const size = fs.statSync(left).size;
  if (size !== fs.statSync(right).size) return false;
  const leftBuffer = Buffer.allocUnsafe(64 * 1024);
  const rightBuffer = Buffer.allocUnsafe(leftBuffer.length);
  let leftFd;
  let rightFd;
  try {
    leftFd = fs.openSync(left, 'r');
    rightFd = fs.openSync(right, 'r');
    for (let offset = 0; offset < size;) {
      const length = Math.min(leftBuffer.length, size - offset);
      const leftRead = fs.readSync(leftFd, leftBuffer, 0, length, offset);
      const rightRead = fs.readSync(rightFd, rightBuffer, 0, length, offset);
      if (leftRead === 0 || leftRead !== rightRead ||
          !leftBuffer.subarray(0, leftRead).equals(rightBuffer.subarray(0, rightRead))) {
        return false;
      }
      offset += leftRead;
    }
    return true;
  } finally {
    if (leftFd !== undefined) fs.closeSync(leftFd);
    if (rightFd !== undefined) fs.closeSync(rightFd);
  }
}

/** Ignore only fetch timestamps when checking whether a snapshot changed. */
function sameSnapshot(stagingDir, outDir) {
  if (!fs.existsSync(outDir)) return false;
  try {
    const next = JSON.parse(fs.readFileSync(path.join(stagingDir, '_manifest.json'), 'utf8'));
    const previous = JSON.parse(fs.readFileSync(path.join(outDir, '_manifest.json'), 'utf8'));
    if (next.version !== previous.version || !Array.isArray(previous.files)) return false;
    const expected = next.files.slice().sort();
    if (JSON.stringify(expected) !== JSON.stringify(previous.files.slice().sort())) return false;
    const actual = [];
    let regular = true;
    const visit = (directory, prefix = '') => {
      for (const entry of fs.readdirSync(directory, { withFileTypes: true })) {
        const relative = prefix ? prefix + '/' + entry.name : entry.name;
        if (entry.isDirectory()) visit(path.join(directory, entry.name), relative);
        else if (!entry.isFile()) regular = false;
        else if (relative !== '_manifest.json') actual.push(relative);
      }
    };
    visit(outDir);
    if (!regular || JSON.stringify(actual.sort()) !== JSON.stringify(expected)) return false;
    return expected.every((relative) => sameFileContents(
      path.join(stagingDir, relative), path.join(outDir, relative),
    ));
  } catch (error) {
    // A missing or unreadable old cache is repaired by publishing the complete one.
    return false;
  }
}

/**
 * Publishes a complete snapshot, preserving all timestamps when bytes match.
 * The previous snapshot is only removed once the new one is in place.
 *
 * @param {string} stagingDir
 * @param {string} outDir
 */
function publish(stagingDir, outDir) {
  if (sameSnapshot(stagingDir, outDir)) {
    fs.rmSync(stagingDir, { recursive: true, force: true });
    return;
  }
  fs.mkdirSync(path.dirname(outDir), { recursive: true });
  const backupDir = `${outDir}.backup`;
  fs.rmSync(backupDir, { recursive: true, force: true });
  if (fs.existsSync(outDir)) {
    fs.renameSync(outDir, backupDir);
  }
  try {
    fs.renameSync(stagingDir, outDir);
  } catch (err) {
    if (fs.existsSync(backupDir)) {
      fs.renameSync(backupDir, outDir);
    }
    throw err;
  }
  fs.rmSync(backupDir, { recursive: true, force: true });
}

/**
 * Runs the whole prefetch pipeline.
 *
 * The snapshot is built inside a staging directory and only published into
 * `outDir` once every resource has been downloaded successfully. A failing run
 * therefore leaves the previously published cache untouched.
 *
 * @param {object} [options]
 * @param {string} [options.root] - Project root (used to find config/app.config.json).
 * @param {string} [options.outDir] - Destination cache directory.
 * @param {object|string} [options.config] - Config object or path to config/app.config.json.
 * @param {number} [options.concurrency] - Parallel downloads (default 6).
 * @param {number} [options.timeoutMs] - Per-request timeout.
 * @param {(msg: string) => void} [options.logger]
 * @param {(msg: string) => void} [options.warner]
 * @returns {Promise<{files: string[], downloadedFiles: object[], failures: object[], skipped: object[]}>}
 */
async function runPrefetch(options = {}) {
  const root = options.root || ROOT;
  const outDir = options.outDir || path.join(root, BUNDLED_CACHE_SUBDIR);
  const concurrency = options.concurrency || DEFAULT_CONCURRENCY;
  const timeoutMs =
    options.timeoutMs === undefined ? DEFAULT_TIMEOUT_MS : options.timeoutMs;
  const logger = options.logger || (() => {});
  const warner = options.warner || (() => {});

  let config;
  if (options.config && typeof options.config === 'object') {
    config = options.config;
  } else if (typeof options.config === 'string') {
    config = JSON.parse(fs.readFileSync(options.config, 'utf-8'));
  } else {
    config = loadConfig(root);
  }
  const remoteUrl = config.remote.url;
  const version = (config.app && config.app.version) || '0.0.0';

  const indexOnly =
    process.env.PREFETCH_INDEX_ONLY === '1' ||
    process.env.PREFETCH_INDEX_ONLY === 'true';

  // Staging directory: nothing is published until the snapshot is complete.
  const stagingDir = `${outDir}.staging`;
  fs.rmSync(stagingDir, { recursive: true, force: true });
  fs.mkdirSync(stagingDir, { recursive: true });

  // Keep-alive is enabled: sockets are pooled and reused across the many
  // requests of one prefetch run, which removes a TCP handshake per file.
  // Both agents are destroyed in the `finally` block below, so every pooled
  // socket is closed and the event loop still drains on its own.
  const httpAgent = new http.Agent({
    keepAlive: true,
    keepAliveMsecs: 1000,
    maxSockets: concurrency,
  });
  const httpsAgent = new https.Agent({
    keepAlive: true,
    keepAliveMsecs: 1000,
    maxSockets: concurrency,
  });
  const agentFor = (url) => (url.startsWith('https') ? httpsAgent : httpAgent);

  const failures = [];
  const downloadedFiles = [];
  const seenCleanPaths = new Set();
  const skipped = [];

  try {
    // ---- Step 1: download index.html -------------------------------------
    logger(`[prefetch-cache] Downloading HTML: ${remoteUrl}`);
    let indexFinalUrl;
    let html;
    try {
      const result = await downloadUrl(remoteUrl, {
        timeoutMs,
        agentFor,
      });
      indexFinalUrl = result.finalUrl;
      html = result.body.toString('utf-8');
    } catch (err) {
      fs.rmSync(stagingDir, { recursive: true, force: true });
      const error = new Error(
        `failed to download index.html (${remoteUrl}): ${err.message}`,
      );
      error.failures = [
        { cleanPath: 'index.html', url: remoteUrl, error: err.message },
      ];
      throw error;
    }
    logger(`[prefetch-cache] Final URL after redirects: ${indexFinalUrl}`);
    if (!html || html.length === 0) {
      fs.rmSync(stagingDir, { recursive: true, force: true });
      const error = new Error(
        `failed to download index.html (${remoteUrl}): empty response`,
      );
      error.failures = [
        { cleanPath: 'index.html', url: remoteUrl, error: 'empty response' },
      ];
      throw error;
    }
    const baseUrl = deriveBaseUrl(indexFinalUrl);
    logger(`[prefetch-cache] Base URL for resources: ${baseUrl}`);

    html = patchHtml(html);
    writeInto(stagingDir, 'index.html', html);

    const writeManifest = () => {
      const manifest = {
        version,
        timestamp: new Date().toISOString(),
        files: [],
      };
      const walkDir = (dir, prefix = '') => {
        const entries = fs.readdirSync(dir, { withFileTypes: true });
        for (const entry of entries) {
          const fullPath = path.join(dir, entry.name);
          const relName = prefix ? `${prefix}/${entry.name}` : entry.name;
          if (entry.isDirectory()) {
            walkDir(fullPath, relName);
          } else {
            manifest.files.push(relName);
          }
        }
      };
      walkDir(stagingDir);
      fs.writeFileSync(
        path.join(stagingDir, '_manifest.json'),
        JSON.stringify(manifest, null, 2),
      );
      return manifest;
    };

    if (indexOnly) {
      logger(
        '[prefetch-cache] PREFETCH_INDEX_ONLY enabled: bundling index.html only',
      );
      const manifest = writeManifest();
      publish(stagingDir, outDir);
      logger(
        '[prefetch-cache] Done! 1 file bundled (index.html only) into src-tauri/bundled-cache/',
      );
      return { files: manifest.files, downloadedFiles, failures, skipped };
    }

    // ---- Step 2: queue all linked resources ------------------------------
    const resourcePaths = extractResourcePaths(html);
    logger(
      `[prefetch-cache] Found ${resourcePaths.length} resources to download`,
    );

    const queue = [];
    for (const ref of resourcePaths) {
      const { cleanPath, unsafe } = htmlRefCleanPath(ref);
      if (!cleanPath || cleanPath === '.') continue;
      if (unsafe) {
        // A reference that escapes the cache root cannot be represented in the
        // runtime's flat cache layout. It is skipped (and reported) instead of
        // being written to an unexpected location or failing the whole run.
        warner(
          `[prefetch-cache]   WARN: Skipped unsafe resource path: ${ref}`,
        );
        skipped.push({ ref, reason: 'path traversal' });
        continue;
      }
      if (seenCleanPaths.has(cleanPath)) continue;
      seenCleanPaths.add(cleanPath);
      queue.push({ url: resolveHtmlRefUrl(baseUrl, ref), cleanPath });
    }

    // ---- Step 3: download with bounded concurrency, discovering deps ------
    await mapWithConcurrency(queue, concurrency, async (item) => {
      const { url, cleanPath } = item;
      try {
        const result = await downloadUrl(url, {
          timeoutMs,
          agentFor,
        });
        if (result.body.length === 0) {
          // An empty body means the resource is broken; treating it as a
          // failure keeps the previously published snapshot intact instead of
          // shipping a cache with a missing/empty file.
          failures.push({
            cleanPath,
            url,
            error: 'empty response',
          });
          warner(
            `[prefetch-cache]   WARN: Empty response for ${cleanPath}, treating as failure`,
          );
          return;
        }
        writeInto(stagingDir, cleanPath, result.body);
        logger(
          `[prefetch-cache]   Saved: ${cleanPath} (${result.body.length} bytes)`,
        );
        if (result.finalUrl !== url) {
          logger(`[prefetch-cache]     (redirected to: ${result.finalUrl})`);
        }
        const isJs = cleanPath.endsWith('.js') || cleanPath.endsWith('.mjs');
        const jsContent = isJs ? result.body.toString('utf-8') : null;
        downloadedFiles.push({
          cleanPath,
          finalUrl: result.finalUrl,
          jsContent,
        });

        if (!isJs) return;

        // Recursively discover the dependencies of this JS file. The
        // dependency URL is resolved against the file's FINAL url (so
        // redirects work) while the cache path stays relative to the
        // referring file's directory (matching how it is served).
        const deps = extractJsDependencies(jsContent, result.finalUrl, cleanPath);
        for (const dep of deps) {
          if (dep.unsafe) {
            // Same policy as for HTML references: unsafe paths are skipped and
            // reported, they never fail the whole run.
            warner(
              `[prefetch-cache]   WARN: Skipped unsafe dependency path: ${dep.ref}`,
            );
            skipped.push({ ref: dep.ref, reason: 'path traversal' });
            continue;
          }
          if (!dep.cleanPath) continue;
          if (seenCleanPaths.has(dep.cleanPath)) continue;
          seenCleanPaths.add(dep.cleanPath);
          queue.push({ url: dep.url, cleanPath: dep.cleanPath });
        }
      } catch (err) {
        failures.push({ cleanPath, url, error: err.message });
        warner(
          `[prefetch-cache]   WARN: Failed to fetch ${cleanPath}: ${err.message}`,
        );
      }
    });

    if (failures.length > 0) {
      // Incomplete snapshot: keep the previously published cache intact.
      fs.rmSync(stagingDir, { recursive: true, force: true });
      const error = new Error(
        `failed to download ${failures.length} resource(s): ${failures
          .map((f) => f.cleanPath)
          .join(', ')}`,
      );
      error.failures = failures;
      throw error;
    }

    // ---- Step 4: manifest -------------------------------------------------
    const manifest = writeManifest();

    // euv-app override: force soft-keyboard demo route on launch.
    // The downloaded index.html opens the static About screen (no inputs),
    // which makes the IME/keyboard fix invisible. We replace it with a copy
    // from src-tauri/bundled-cache/_euv_app_index.html that injects a hash
    // redirect to /form before the WASM bundle loads.
    const localIndex = path.join(outDir, '_euv_app_index.html');
    if (fs.existsSync(localIndex)) {
      fs.copyFileSync(localIndex, path.join(stagingDir, 'index.html'));
      logger('[prefetch-cache] Replaced index.html with euv-app override.');
    }

    publish(stagingDir, outDir);
    logger(
      `[prefetch-cache] Done! ${manifest.files.length} files bundled into src-tauri/bundled-cache/`,
    );
    return { files: manifest.files, downloadedFiles, failures, skipped };
  } finally {
    httpAgent.destroy();
    httpsAgent.destroy();
  }
}

async function main() {
  console.log('[prefetch-cache] Starting resource prefetch...');
  try {
    await runPrefetch({
      root: ROOT,
      outDir: path.join(ROOT, BUNDLED_CACHE_SUBDIR),
      logger: (msg) => console.log(msg),
      warner: (msg) => console.warn(msg),
    });
  } catch (err) {
    console.error('[prefetch-cache] Fatal error:', err);
    // Set the exit code instead of forcing the process down, so buffered
    // stdout/stderr is flushed before the process ends.
    process.exitCode = 1;
  }
  // No forced exit: the agents are destroyed when runPrefetch settles, which
  // closes every pooled (keep-alive) socket, so the event loop drains on its
  // own and buffered stdout/stderr is flushed.
}

module.exports = {
  runPrefetch,
  mapWithConcurrency,
  deriveBaseUrl,
  extractJsDependencies,
  extractResourcePaths,
  downloadUrl,
  loadConfig,
  patchHtml,
  DEFAULT_CONCURRENCY,
  DEFAULT_TIMEOUT_MS,
};

if (require.main === module) {
  main().catch((err) => {
    console.error('[prefetch-cache] Fatal error:', err);
    process.exitCode = 1;
  });
}
