'use strict';
/**
 * Tests for scripts/prefetch-cache.js using a real local HTTP fixture server.
 * Run: node --test scripts/tests/prefetch.test.js
 */
const test = require('node:test');
const assert = require('node:assert');
const fs = require('fs');
const os = require('os');
const path = require('path');
const http = require('http');

const {
  startFixtureServer,
  writeConfig,
} = require('./fixture-server');
const {
  runPrefetch,
  mapWithConcurrency,
  deriveBaseUrl,
  extractJsDependencies,
  downloadUrl,
} = require('../prefetch-cache');

function tmpRoot(name) {
  return fs.mkdtempSync(path.join(os.tmpdir(), `prefetch-${name}-`));
}

test('mapWithConcurrency respects the concurrency limit', async () => {
  let active = 0;
  let peak = 0;
  const items = Array.from({ length: 30 }, (_, i) => i);
  const out = await mapWithConcurrency(items, 6, async (n) => {
    active++;
    peak = Math.max(peak, active);
    await new Promise((r) => setTimeout(r, 5));
    active--;
    return n * 2;
  });
  assert.strictEqual(out.length, 30);
  assert.deepStrictEqual(
    out,
    items.map((n) => n * 2),
  );
  assert.ok(peak <= 6, `peak concurrency ${peak} must be <= 6`);
  assert.ok(peak > 1, 'work should actually run in parallel');
});

test('deriveBaseUrl keeps directory base and strips file segment', () => {
  assert.strictEqual(
    deriveBaseUrl('https://x.test/github/pages/euv-dev/euv/'),
    'https://x.test/github/pages/euv-dev/euv',
  );
  assert.strictEqual(deriveBaseUrl('https://x.test/euv'), 'https://x.test');
});

test('extractJsDependencies resolves against the final (redirected) url', () => {
  const deps = extractJsDependencies(
    "import mod from './chunk.js';\nnew URL('a_bg.wasm', import.meta.url);\n",
    'https://x.test/base/redirected/app.js',
    'app.js',
  );
  const byPath = Object.fromEntries(deps.map((d) => [d.cleanPath, d.url]));
  assert.strictEqual(
    byPath['chunk.js'],
    'https://x.test/base/redirected/chunk.js',
  );
  assert.strictEqual(
    byPath['a_bg.wasm'],
    'https://x.test/base/redirected/a_bg.wasm',
  );
});

test('prefetch downloads all resources with bounded concurrency and dedupes paths', async () => {
  const fixture = await startFixtureServer();
  const root = tmpRoot('main');
  try {
    writeConfig(root, `${fixture.url}/base/index.html`);
    const result = await runPrefetch({
      root,
      outDir: path.join(root, 'bundled-cache'),
      logger: () => {},
      warner: () => {},
    });

    // index.html fetched once even though referenced twice.
    const indexHits = fixture.stats.paths.filter((p) => p === '/base/index.html');
    assert.strictEqual(indexHits.length, 1);
    // app.js is referenced twice -> downloaded once (plus its redirect target).
    const appHits = fixture.stats.paths.filter((p) => p === '/base/app.js');
    assert.strictEqual(appHits.length, 1);

    assert.ok(
      fixture.stats.maxConcurrent > 1,
      `expected parallel downloads, got max ${fixture.stats.maxConcurrent}`,
    );
    assert.ok(
      fixture.stats.maxConcurrent <= 6,
      `max concurrency ${fixture.stats.maxConcurrent} exceeded limit 6`,
    );

    const cacheDir = path.join(root, 'bundled-cache');
    assert.ok(fs.existsSync(path.join(cacheDir, 'index.html')));
    assert.ok(fs.existsSync(path.join(cacheDir, 'app.js')));
    assert.ok(fs.existsSync(path.join(cacheDir, 'styles.css')));
    assert.ok(fs.existsSync(path.join(cacheDir, 'img', 'logo.png')));
    assert.ok(fs.existsSync(path.join(cacheDir, 'chunk.js')));
    assert.ok(fs.existsSync(path.join(cacheDir, 'app_bg.wasm')));

    // Redirected JS dependencies are resolved against the redirected base.
    assert.strictEqual(
      fs.readFileSync(path.join(cacheDir, 'chunk.js'), 'utf-8'),
      'export const chunk = 1;\n',
    );
    assert.strictEqual(
      fs.readFileSync(path.join(cacheDir, 'app_bg.wasm'), 'utf-8'),
      'WASM-FIXTURE-0',
    );

    // Manifest lists every bundled file.
    const manifest = JSON.parse(
      fs.readFileSync(path.join(cacheDir, '_manifest.json'), 'utf-8'),
    );
    assert.ok(manifest.files.includes('index.html'));
    assert.ok(manifest.files.includes('chunk.js'));
    assert.ok(manifest.files.includes('img/logo.png'));
    assert.ok(manifest.files.length > 20);

    // Non-JS resource bodies are not retained in memory.
    const css = result.downloadedFiles.find((f) => f.cleanPath === 'styles.css');
    assert.ok(css, 'css entry present');
    assert.strictEqual(css.jsContent, null);
    assert.strictEqual(css.body, undefined);
    const js = result.downloadedFiles.find(
      (f) => f.cleanPath === 'redirected/app.js' || f.cleanPath === 'app.js',
    );
    assert.ok(js);
    assert.ok(typeof js.jsContent === 'string');
  } finally {
    await fixture.close();
    fs.rmSync(root, { recursive: true, force: true });
  }
});

test('a missing resource causes runPrefetch to reject', async () => {
  const http = require('http');
  const server = http.createServer((req, res) => {
    if (req.url.endsWith('index.html')) {
      res.setHeader('content-type', 'text/html');
      res.end(
        '<html><head><script src="missing.js"></script></head><body></body></html>',
      );
    } else {
      res.statusCode = 500;
      res.end('boom');
    }
  });
  await new Promise((r) => server.listen(0, '127.0.0.1', r));
  const port = server.address().port;
  const root = tmpRoot('missing');
  try {
    writeConfig(root, `http://127.0.0.1:${port}/index.html`);
    await assert.rejects(
      () =>
        runPrefetch({
          root,
          outDir: path.join(root, 'bundled-cache'),
          logger: () => {},
          warner: () => {},
        }),
      (err) => {
        assert.match(err.message, /failed to download/);
        assert.ok(Array.isArray(err.failures));
        assert.strictEqual(err.failures[0].cleanPath, 'missing.js');
        return true;
      },
    );
  } finally {
    await new Promise((r) => server.close(r));
    fs.rmSync(root, { recursive: true, force: true });
  }
});

test('PREFETCH_INDEX_ONLY still bundles only index.html', async () => {
  const fixture = await startFixtureServer();
  const root = tmpRoot('indexonly');
  const prev = process.env.PREFETCH_INDEX_ONLY;
  process.env.PREFETCH_INDEX_ONLY = '1';
  try {
    writeConfig(root, `${fixture.url}/base/index.html`);
    const result = await runPrefetch({
      root,
      outDir: path.join(root, 'bundled-cache'),
      logger: () => {},
      warner: () => {},
    });
    assert.deepStrictEqual(result.files, ['index.html']);
    assert.ok(fs.existsSync(path.join(root, 'bundled-cache', 'index.html')));
  } finally {
    if (prev === undefined) delete process.env.PREFETCH_INDEX_ONLY;
    else process.env.PREFETCH_INDEX_ONLY = prev;
    await fixture.close();
    fs.rmSync(root, { recursive: true, force: true });
  }
});

test('a failing run preserves the previously published cache snapshot', async () => {
  const fixture = await startFixtureServer();
  const root = tmpRoot('preserve');
  const outDir = path.join(root, 'bundled-cache');
  try {
    writeConfig(root, `${fixture.url}/base/index.html`);
    // 1st run: succeeds and publishes a snapshot.
    await runPrefetch({
      root,
      outDir,
      logger: () => {},
      warner: () => {},
    });
    const goodIndex = fs.readFileSync(path.join(outDir, 'index.html'), 'utf-8');
    const goodManifest = fs.readFileSync(
      path.join(outDir, '_manifest.json'),
      'utf-8',
    );
    assert.ok(goodIndex.includes('viewport-fit=cover'));

    // 2nd run: the server starts failing -> the run must reject and must NOT
    // touch the published snapshot.
    const broken = await startFixtureServer();
    await broken.close();
    const failing = http.createServer((req, res) => {
      res.statusCode = 500;
      res.end('boom');
    });
    await new Promise((r) => failing.listen(0, '127.0.0.1', r));
    const failingUrl = `http://127.0.0.1:${failing.address().port}/index.html`;
    try {
      writeConfig(root, failingUrl);
      await assert.rejects(
        () => runPrefetch({ root, outDir, logger: () => {}, warner: () => {} }),
        (err) => {
          assert.ok(Array.isArray(err.failures));
          assert.ok(err.failures.length > 0);
          return true;
        },
      );
    } finally {
      await new Promise((r) => failing.close(r));
    }

    // The old snapshot is intact.
    assert.strictEqual(
      fs.readFileSync(path.join(outDir, 'index.html'), 'utf-8'),
      goodIndex,
    );
    assert.strictEqual(
      fs.readFileSync(path.join(outDir, '_manifest.json'), 'utf-8'),
      goodManifest,
    );
    assert.ok(fs.existsSync(path.join(outDir, 'app.js')));
    // No staging leftovers.
    assert.ok(!fs.existsSync(`${outDir}.staging`));
  } finally {
    await fixture.close();
    fs.rmSync(root, { recursive: true, force: true });
  }
});

test('an aborted response fails the resource and a gzip response is decoded', async () => {
  const fixture = await startFixtureServer({ scenario: 'abort' });
  const root = tmpRoot('abort');
  const outDir = path.join(root, 'bundled-cache');
  try {
    // 1) A gzip encoded response is transparently decoded.
    const gz = await downloadUrl(`${fixture.url}/base/gz.js`, {
      timeoutMs: 5000,
    });
    assert.strictEqual(gz.body.toString('utf-8'), 'export const gz = 1;\n');

    // 2) An aborted response must surface as an error, not as a truncated
    //    success.
    await assert.rejects(
      () => downloadUrl(`${fixture.url}/base/aborted.js`, { timeoutMs: 5000 }),
      /socket hang up|ECONNRESET|aborted/i,
    );

    // 3) A full run with the aborted resource fails and publishes nothing.
    writeConfig(root, `${fixture.url}/base/index.html`);
    await assert.rejects(
      () => runPrefetch({ root, outDir, logger: () => {}, warner: () => {} }),
      (err) => {
        assert.ok(Array.isArray(err.failures));
        assert.strictEqual(err.failures[0].cleanPath, 'aborted.js');
        return true;
      },
    );
    assert.ok(!fs.existsSync(path.join(outDir, '_manifest.json')));
    assert.ok(!fs.existsSync(`${outDir}.staging`));
  } finally {
    await fixture.close();
    fs.rmSync(root, { recursive: true, force: true });
  }
});

test('a single JS file fanning out to many deps uses all workers', async () => {
  const fixture = await startFixtureServer({ scenario: 'fanout' });
  const root = tmpRoot('fanout');
  try {
    writeConfig(root, `${fixture.url}/base/index.html`);
    await runPrefetch({
      root,
      outDir: path.join(root, 'bundled-cache'),
      concurrency: 6,
      logger: () => {},
      warner: () => {},
    });
    const cacheDir = path.join(root, 'bundled-cache');
    for (let i = 0; i < 8; i++) {
      assert.ok(
        fs.existsSync(path.join(cacheDir, `dep${i}.js`)),
        `dep${i}.js downloaded`,
      );
    }
    // The queue started with a single item (root.js) but the deps were still
    // fetched in parallel.
    assert.ok(
      fixture.stats.maxConcurrent > 1,
      `expected parallel dependency downloads, got max ${fixture.stats.maxConcurrent}`,
    );
    assert.ok(
      fixture.stats.maxConcurrent <= 6,
      `max concurrency ${fixture.stats.maxConcurrent} exceeded limit 6`,
    );
  } finally {
    await fixture.close();
    fs.rmSync(root, { recursive: true, force: true });
  }
});

test('a stalled response times out and is reported as a failure', async () => {
  const fixture = await startFixtureServer({ scenario: 'stall' });
  const root = tmpRoot('stall');
  try {
    writeConfig(root, `${fixture.url}/base/index.html`);
    await assert.rejects(
      () =>
        runPrefetch({
          root,
          outDir: path.join(root, 'bundled-cache'),
          timeoutMs: 300,
          logger: () => {},
          warner: () => {},
        }),
      (err) => {
        assert.ok(Array.isArray(err.failures));
        assert.strictEqual(err.failures[0].cleanPath, 'stall.js');
        assert.match(err.failures[0].error, /Timeout/);
        return true;
      },
    );
  } finally {
    await fixture.close();
    fs.rmSync(root, { recursive: true, force: true });
  }
});

test('a trickling response is aborted by the whole-request deadline', async () => {
  const fixture = await startFixtureServer({ scenario: 'trickle' });
  const root = tmpRoot('trickle');
  try {
    writeConfig(root, `${fixture.url}/base/index.html`);
    const started = Date.now();
    await assert.rejects(
      () =>
        runPrefetch({
          root,
          outDir: path.join(root, 'bundled-cache'),
          timeoutMs: 400,
          logger: () => {},
          warner: () => {},
        }),
      (err) => {
        assert.ok(Array.isArray(err.failures));
        assert.strictEqual(err.failures[0].cleanPath, 'trickle.js');
        assert.match(err.failures[0].error, /Timeout/);
        return true;
      },
    );
    // The deadline fired: the request did not hang until the socket idled out.
    assert.ok(
      Date.now() - started < 3000,
      'the run aborted on the deadline, not by hanging',
    );
  } finally {
    await fixture.close();
    fs.rmSync(root, { recursive: true, force: true });
  }
});

test('a cross-protocol redirect switches to the https agent', async () => {
  const fixture = await startFixtureServer({ scenario: 'proto-redirect' });
  try {
    // The redirect target is an https URL on the plain-http fixture server, so
    // the request fails at the TLS layer. What matters is that the follow-up
    // request was issued through the *https* agent and produced a TLS error
    // instead of a protocol/agent mismatch.
    const agentsFor = [];
    await assert.rejects(
      () =>
        downloadUrl(`${fixture.url}/base/redirect.js`, {
          agentFor: (url) => {
            agentsFor.push(url);
            return url.startsWith('https://')
              ? new (require('https').Agent)()
              : new http.Agent();
          },
        }),
      (err) => {
        assert.ok(err instanceof Error);
        // A plain http agent on an https URL would fail with a protocol
        // error ("Protocol ... not supported"); the https agent fails on the
        // TLS handshake against the plain-http fixture server.
        assert.ok(
          !/Protocol .* not supported/.test(err.message),
          `unexpected protocol mismatch: ${err.message}`,
        );
        return true;
      },
    );
    // The agent was re-selected for the redirected (https) URL.
    assert.strictEqual(agentsFor.length, 2);
    assert.ok(agentsFor[0].startsWith('http://'));
    assert.ok(agentsFor[1].startsWith('https://'));
  } finally {
    await fixture.close();
  }
});

test('a recursive JS dependency graph is downloaded completely', async () => {
  const fixture = await startFixtureServer({ scenario: 'recursive' });
  const root = tmpRoot('recursive');
  try {
    writeConfig(root, `${fixture.url}/base/index.html`);
    await runPrefetch({
      root,
      outDir: path.join(root, 'bundled-cache'),
      logger: () => {},
      warner: () => {},
    });
    const cacheDir = path.join(root, 'bundled-cache');
    for (const file of [
      'index.html',
      'graph/a.js',
      'graph/b.js',
      'graph/c.js',
      'graph/a_bg.wasm',
    ]) {
      assert.ok(fs.existsSync(path.join(cacheDir, file)), `${file} downloaded`);
    }
  } finally {
    await fixture.close();
    fs.rmSync(root, { recursive: true, force: true });
  }
});

test('query strings and fragments are stripped from cache paths', async () => {
  const fixture = await startFixtureServer({ scenario: 'query' });
  const root = tmpRoot('query');
  try {
    writeConfig(root, `${fixture.url}/base/index.html`);
    await runPrefetch({
      root,
      outDir: path.join(root, 'bundled-cache'),
      logger: () => {},
      warner: () => {},
    });
    const cacheDir = path.join(root, 'bundled-cache');
    assert.ok(fs.existsSync(path.join(cacheDir, 'app.js')));
    assert.ok(fs.existsSync(path.join(cacheDir, 'styles.css')));
    // Only one download per canonical path.
    const appHits = fixture.stats.paths.filter((p) => p === '/base/app.js');
    assert.strictEqual(appHits.length, 1);
  } finally {
    await fixture.close();
    fs.rmSync(root, { recursive: true, force: true });
  }
});

test('path traversal references are skipped with a warning, not a failure', async () => {
  const fixture = await startFixtureServer({ scenario: 'traversal' });
  const root = tmpRoot('traversal');
  const warnings = [];
  try {
    writeConfig(root, `${fixture.url}/base/index.html`);
    const result = await runPrefetch({
      root,
      outDir: path.join(root, 'bundled-cache'),
      logger: () => {},
      warner: (msg) => warnings.push(msg),
    });
    // The run succeeds: unsafe paths are skipped, they never fail the build.
    assert.deepStrictEqual(result.failures, []);
    assert.ok(
      warnings.some((w) => w.includes('Skipped unsafe resource path')),
      'a warning about the skipped path was emitted',
    );
    const cacheDir = path.join(root, 'bundled-cache');
    // Nothing was written outside the cache root.
    assert.ok(!fs.existsSync(path.join(root, 'escape.js')));
    assert.ok(!fs.existsSync(path.join(cacheDir, '..', 'escape.js')));
    // The safe sibling resources are still bundled.
    assert.ok(fs.existsSync(path.join(cacheDir, 'index.html')));
  } finally {
    await fixture.close();
    fs.rmSync(root, { recursive: true, force: true });
  }
});

test('extractJsDependencies picks up side-effect imports, re-exports and wasm queries', () => {
  const deps = extractJsDependencies(
    [
      "import './side-effect.js';",
      "import x from './dep.js';",
      "export { y } from './reexport.js';",
      "export * from './star.js';",
      "const w = new URL('app_bg.wasm?v=3', import.meta.url);",
    ].join('\n'),
    'https://x.test/base/app.js',
    'app.js',
  );
  const paths = deps.map((d) => d.cleanPath);
  for (const expected of [
    'side-effect.js',
    'dep.js',
    'reexport.js',
    'star.js',
    'app_bg.wasm',
  ]) {
    assert.ok(paths.includes(expected), `${expected} discovered`);
  }
  const wasm = deps.find((d) => d.cleanPath === 'app_bg.wasm');
  assert.strictEqual(wasm.url, 'https://x.test/base/app_bg.wasm?v=3');
});

test('deriveBaseUrl ignores a query string that contains a slash', () => {
  assert.strictEqual(
    deriveBaseUrl('https://x.test/euv/index.html?next=/a/b'),
    'https://x.test/euv',
  );
  assert.strictEqual(
    deriveBaseUrl('https://x.test/euv/index.html?next=/a/b#frag'),
    'https://x.test/euv',
  );
});

test('extractJsDependencies finds fragment-only wasm refs and root-relative specifiers', () => {
  const deps = extractJsDependencies(
    [
      "const w = new URL('frag_bg.wasm#v1', import.meta.url);",
      "import a from '/pkg/root.js';",
      "const b = await import('/pkg/dyn.js');",
    ].join('\n'),
    'https://x.test/base/app.js',
    'app.js',
  );
  const paths = deps.map((d) => d.cleanPath);
  // The fragment is dropped from the cache path, the file name is kept.
  assert.ok(paths.includes('frag_bg.wasm'), 'fragment-only wasm discovered');
  // Root-relative specifiers are anchored at the cache root.
  assert.ok(paths.includes('pkg/root.js'), 'root-relative import discovered');
  assert.ok(paths.includes('pkg/dyn.js'), 'root-relative dynamic import found');

  const wasm = deps.find((d) => d.cleanPath === 'frag_bg.wasm');
  assert.strictEqual(wasm.url, 'https://x.test/base/frag_bg.wasm#v1');

  const root = deps.find((d) => d.cleanPath === 'pkg/root.js');
  assert.strictEqual(root.url, 'https://x.test/pkg/root.js');
});

test('root-relative JS specifiers and fragment-only wasm are downloaded', async () => {
  const fixture = await startFixtureServer({ scenario: 'root-relative' });
  const root = tmpRoot('root-relative');
  const outDir = path.join(root, 'bundled-cache');
  try {
    writeConfig(root, `${fixture.url}/base/index.html`);
    const result = await runPrefetch({
      root,
      outDir,
      logger: () => {},
      warner: () => {},
    });
    assert.deepStrictEqual(result.failures, []);
    // The root-relative specifier lands at the cache root.
    assert.ok(fs.existsSync(path.join(outDir, 'pkg', 'root.js')));
    assert.ok(fs.existsSync(path.join(outDir, 'pkg', 'dyn.js')));
    // The fragment-only wasm reference is stored without the fragment.
    assert.ok(fs.existsSync(path.join(outDir, 'frag_bg.wasm')));
    assert.strictEqual(
      fs.readFileSync(path.join(outDir, 'frag_bg.wasm'), 'utf-8'),
      'WASM-FRAG',
    );
  } finally {
    await fixture.close();
    fs.rmSync(root, { recursive: true, force: true });
  }
});

test('an empty resource fails the run and preserves the previous snapshot', async () => {
  const fixture = await startFixtureServer();
  const root = tmpRoot('empty-preserve');
  const outDir = path.join(root, 'bundled-cache');
  try {
    writeConfig(root, `${fixture.url}/base/index.html`);
    // 1st run: succeeds and publishes a snapshot.
    await runPrefetch({ root, outDir, logger: () => {}, warner: () => {} });
    const goodIndex = fs.readFileSync(path.join(outDir, 'index.html'), 'utf-8');
    const goodManifest = fs.readFileSync(
      path.join(outDir, '_manifest.json'),
      'utf-8',
    );

    // 2nd run: one resource answers 200 with an empty body. The run must
    // reject, must report the empty resource as a failure, and must not
    // touch the published snapshot.
    const emptyFixture = await startFixtureServer({ scenario: 'empty' });
    try {
      writeConfig(root, `${emptyFixture.url}/base/index.html`);
      await assert.rejects(
        () => runPrefetch({ root, outDir, logger: () => {}, warner: () => {} }),
        (err) => {
          assert.ok(Array.isArray(err.failures));
          const emptyFailure = err.failures.find(
            (f) => f.cleanPath === 'empty.js',
          );
          assert.ok(emptyFailure, 'the empty resource is reported');
          assert.strictEqual(emptyFailure.error, 'empty response');
          return true;
        },
      );
    } finally {
      await emptyFixture.close();
    }

    // The old snapshot is intact.
    assert.strictEqual(
      fs.readFileSync(path.join(outDir, 'index.html'), 'utf-8'),
      goodIndex,
    );
    assert.strictEqual(
      fs.readFileSync(path.join(outDir, '_manifest.json'), 'utf-8'),
      goodManifest,
    );
    assert.ok(fs.existsSync(path.join(outDir, 'app.js')));
    // Nothing from the failing run leaked into the snapshot.
    assert.ok(!fs.existsSync(path.join(outDir, 'good.js')));
    assert.ok(!fs.existsSync(path.join(outDir, 'empty.js')));
    // No staging leftovers.
    assert.ok(!fs.existsSync(`${outDir}.staging`));
  } finally {
    await fixture.close();
    fs.rmSync(root, { recursive: true, force: true });
  }
});

test('identical snapshots retain timestamps while changed resources are published', async () => {
  const http = require('node:http');
  let source = 'export const version = 1;';
  const server = http.createServer((request, response) => {
    response.end(request.url === '/index.html'
      ? '<html><head><script src="app.js"></script></head><body></body></html>'
      : source);
  });
  await new Promise((resolve) => server.listen(0, '127.0.0.1', resolve));
  const root = tmpRoot('unchanged');
  const outDir = path.join(root, 'bundled-cache');
  const files = ['index.html', 'app.js', '_manifest.json'];
  try {
    writeConfig(root, `http://127.0.0.1:${server.address().port}/index.html`);
    const run = () => runPrefetch({ root, outDir });
    await run();
    const before = files.map((file) => ({
      file,
      mtime: fs.statSync(path.join(outDir, file), { bigint: true }).mtimeNs,
      contents: fs.readFileSync(path.join(outDir, file)),
    }));
    const directoryTime = fs.statSync(outDir, { bigint: true }).mtimeNs;
    await run();
    assert.deepStrictEqual(files.map((file) => ({
      file,
      mtime: fs.statSync(path.join(outDir, file), { bigint: true }).mtimeNs,
      contents: fs.readFileSync(path.join(outDir, file)),
    })), before);
    assert.strictEqual(fs.statSync(outDir, { bigint: true }).mtimeNs, directoryTime);
    source = 'export const version = 2;';
    await run();
    assert.strictEqual(fs.readFileSync(path.join(outDir, 'app.js'), 'utf8'), source);
    fs.writeFileSync(path.join(outDir, 'stale.js'), 'obsolete');
    await run();
    assert.ok(!fs.existsSync(path.join(outDir, 'stale.js')));
    assert.ok(!fs.existsSync(`${outDir}.staging`));
    assert.ok(!fs.existsSync(`${outDir}.backup`));
  } finally {
    await new Promise((resolve) => server.close(resolve));
    fs.rmSync(root, { recursive: true, force: true });
  }
});
