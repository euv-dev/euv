'use strict';
/**
 * Shared local HTTP fixture server for prefetch-cache tests and benchmarks.
 *
 * Serves a deterministic set of resources under /base/ with an artificial
 * per-response delay so that sequential vs. concurrent downloads show a
 * measurable elapsed-time difference.
 *
 * Scenarios:
 *   'default'   - the bulk resource set used by the benchmark (unchanged).
 *   'recursive' - a JS dependency graph that is 3 levels deep.
 *   'query'     - resources referenced with a query string / fragment.
 *   'traversal' - resources referenced with `../` segments.
 *   'stall'     - a resource whose response never arrives (timeout tests).
 *   'trickle'   - a resource that trickles bytes forever (deadline tests).
 *   'fanout'    - one JS file that imports many others (worker-pool tests).
 *   'abort'     - a gzip encoded resource and a connection aborted mid-body.
 *   'proto-redirect' - a resource redirecting from http to https.
 *   'empty'     - a resource answering 200 with an empty body.
 *   'root-relative' - JS with '/pkg/x.js' specifiers and a fragment-only
 *                     wasm reference.
 */
const http = require('http');
const zlib = require('zlib');

const DELAY_MS = Number(process.env.FIXTURE_DELAY_MS || 120);

/** Resources of the default scenario (also used by the benchmark). */
function makeResources() {
  const res = new Map();

  const bulkTags = [];
  for (let i = 0; i < 12; i++) {
    bulkTags.push(`<script src="chunk${i}.js"></script>`);
    bulkTags.push(`<link rel="stylesheet" href="asset${i}.css" />`);
  }
  const html = `<!DOCTYPE html>
<html>
<head>
<meta name="viewport" content="width=device-width, initial-scale=1.0" />
<link rel="stylesheet" href="styles.css" />
<script src="app.js"></script>
<script src="app.js"></script>
<script src="lib/util.js"></script>
${bulkTags.join('\n')}
</head>
<body>
<img src="img/logo.png" />
</body>
</html>`;
  res.set('/base/index.html', {
    type: 'text/html; charset=utf-8',
    body: Buffer.from(html),
  });

  // A JS bundle served through a redirect: its relative dependencies must be
  // resolved against the FINAL url.
  res.set('/base/app.js', {
    type: 'text/javascript',
    status: 302,
    location: '/base/redirected/app.js',
  });
  res.set('/base/redirected/app.js', {
    type: 'text/javascript',
    body: Buffer.from(
      "import mod from './chunk.js';\nconst wasm = new URL('app_bg.wasm', import.meta.url);\nexport default mod;\n",
    ),
  });
  res.set('/base/redirected/chunk.js', {
    type: 'text/javascript',
    body: Buffer.from('export const chunk = 1;\n'),
  });
  res.set('/base/redirected/app_bg.wasm', {
    type: 'application/wasm',
    body: Buffer.from('WASM-FIXTURE-0'),
  });

  res.set('/base/lib/util.js', {
    type: 'text/javascript',
    body: Buffer.from('export const util = 2;\n'),
  });
  res.set('/base/styles.css', {
    type: 'text/css',
    body: Buffer.from('body{margin:0}\n'),
  });
  res.set('/base/img/logo.png', {
    type: 'image/png',
    body: Buffer.from('PNGFIXTURE'),
  });

  // Extra bulk resources to make the download loop non-trivial.
  for (let i = 0; i < 12; i++) {
    res.set(`/base/chunk${i}.js`, {
      type: 'text/javascript',
      body: Buffer.from(`export const chunk${i} = ${i};\n`),
    });
    res.set(`/base/asset${i}.css`, {
      type: 'text/css',
      body: Buffer.from(`.a${i}{color:red}\n`),
    });
  }
  return res;
}

/** index.html + a 3-level deep JS dependency graph. */
function makeRecursiveResources() {
  const res = new Map();
  res.set('/base/index.html', {
    type: 'text/html; charset=utf-8',
    body: Buffer.from(
      `<!DOCTYPE html><html><head>
<script src="graph/a.js"></script>
</head><body></body></html>`,
    ),
  });
  res.set('/base/graph/a.js', {
    type: 'text/javascript',
    body: Buffer.from(
      "import b from './b.js';\nconst w = new URL('a_bg.wasm', import.meta.url);\nexport default b;\n",
    ),
  });
  res.set('/base/graph/b.js', {
    type: 'text/javascript',
    body: Buffer.from("import c from './c.js';\nexport default c;\n"),
  });
  res.set('/base/graph/c.js', {
    type: 'text/javascript',
    body: Buffer.from("export const c = 3;\n"),
  });
  res.set('/base/graph/a_bg.wasm', {
    type: 'application/wasm',
    body: Buffer.from('WASM-GRAPH-A'),
  });
  return res;
}

/** index.html referencing the same resource with different query/fragment. */
function makeQueryResources() {
  const res = new Map();
  res.set('/base/index.html', {
    type: 'text/html; charset=utf-8',
    body: Buffer.from(
      `<!DOCTYPE html><html><head>
<script src="app.js?v=2"></script>
<script src="app.js?v=2#frag"></script>
<script src="app.js"></script>
<link rel="stylesheet" href="styles.css?v=1" />
</head><body></body></html>`,
    ),
  });
  res.set('/base/app.js', {
    type: 'text/javascript',
    body: Buffer.from('export const app = 1;\n'),
  });
  res.set('/base/styles.css', {
    type: 'text/css',
    body: Buffer.from('body{margin:0}\n'),
  });
  return res;
}

/** index.html referencing resources with `../` traversal segments. */
function makeTraversalResources() {
  const res = new Map();
  res.set('/base/index.html', {
    type: 'text/html; charset=utf-8',
    body: Buffer.from(
      `<!DOCTYPE html><html><head>
<script src="../escape.js"></script>
<script src="sub/../../escape2.js"></script>
</head><body></body></html>`,
    ),
  });
  res.set('/base/escape.js', {
    type: 'text/javascript',
    body: Buffer.from('export const escape = 1;\n'),
  });
  res.set('/base/escape2.js', {
    type: 'text/javascript',
    body: Buffer.from('export const escape2 = 2;\n'),
  });
  res.set('/base/sub/deep.js', {
    type: 'text/javascript',
    body: Buffer.from("import x from '../../outside.js';\nexport default x;\n"),
  });
  return res;
}

/** index.html referencing a resource that never answers. */
function makeStallResources() {
  const res = new Map();
  res.set('/base/index.html', {
    type: 'text/html; charset=utf-8',
    body: Buffer.from(
      `<!DOCTYPE html><html><head>
<script src="stall.js"></script>
</head><body></body></html>`,
    ),
  });
  res.set('/base/stall.js', { type: 'text/javascript', stall: true });
  return res;
}

/**
 * index.html referencing a resource that trickles a byte every 50ms forever.
 * A socket-inactivity timeout would never fire here, only a real whole-request
 * deadline does.
 */
function makeTrickleResources() {
  const res = new Map();
  res.set('/base/index.html', {
    type: 'text/html; charset=utf-8',
    body: Buffer.from(
      `<!DOCTYPE html><html><head>
<script src="trickle.js"></script>
</head><body></body></html>`,
    ),
  });
  res.set('/base/trickle.js', { type: 'text/javascript', trickle: true });
  return res;
}

/**
 * index.html with a single script that fans out to many dependencies.
 * Used to verify that the worker pool keeps all workers busy even when the
 * queue starts with one item.
 */
function makeFanoutResources() {
  const res = new Map();
  res.set('/base/index.html', {
    type: 'text/html; charset=utf-8',
    body: Buffer.from(
      `<!DOCTYPE html><html><head>
<script src="root.js"></script>
</head><body></body></html>`,
    ),
  });
  const imports = [];
  for (let i = 0; i < 8; i++) {
    imports.push(`import d${i} from './dep${i}.js';`);
    res.set(`/base/dep${i}.js`, {
      type: 'text/javascript',
      body: Buffer.from(`export const d${i} = ${i};\n`),
    });
  }
  res.set('/base/root.js', {
    type: 'text/javascript',
    body: Buffer.from(`${imports.join('\n')}\nexport default 0;\n`),
  });
  return res;
}

/** index.html referencing a gzip-encoded resource and an aborted response. */
function makeAbortResources() {
  const res = new Map();
  res.set('/base/index.html', {
    type: 'text/html; charset=utf-8',
    body: Buffer.from(
      `<!DOCTYPE html><html><head>
<script src="gz.js"></script>
<script src="aborted.js"></script>
</head><body></body></html>`,
    ),
  });
  res.set('/base/gz.js', {
    type: 'text/javascript',
    body: Buffer.from('export const gz = 1;\n'),
    gzip: true,
  });
  res.set('/base/aborted.js', {
    type: 'text/javascript',
    body: Buffer.from('export const aborted = 1;\n'),
    abort: true,
  });
  return res;
}

/**
 * index.html referencing a resource that redirects to an https URL on the very
 * same server. The follow-up request must be issued with an https agent.
 */
function makeProtoRedirectResources() {
  const res = new Map();
  res.set('/base/index.html', {
    type: 'text/html; charset=utf-8',
    body: Buffer.from(
      `<!DOCTYPE html><html><head>
<script src="redirect.js"></script>
</head><body></body></html>`,
    ),
  });
  res.set('/base/redirect.js', {
    type: 'text/javascript',
    status: 302,
    location: '{{SELF}}/base/app.js',
  });
  res.set('/base/app.js', {
    type: 'text/javascript',
    body: Buffer.from('export const app = 1;\n'),
  });
  return res;
}

/**
 * index.html referencing a resource that answers 200 with an EMPTY body.
 * An empty resource is broken and must be treated as a download failure so
 * that the previously published snapshot survives.
 */
function makeEmptyResources() {
  const res = new Map();
  res.set('/base/index.html', {
    type: 'text/html; charset=utf-8',
    body: Buffer.from(
      `<!DOCTYPE html><html><head>
<script src="good.js"></script>
<script src="empty.js"></script>
</head><body></body></html>`,
    ),
  });
  res.set('/base/good.js', {
    type: 'text/javascript',
    body: Buffer.from('export const good = 1;\n'),
  });
  res.set('/base/empty.js', { type: 'text/javascript', empty: true });
  return res;
}

/**
 * index.html referencing a JS file that uses root-relative module specifiers
 * ('/pkg/root.js') and a wasm reference carrying only a fragment.
 */
function makeRootRelativeResources() {
  const res = new Map();
  res.set('/base/index.html', {
    type: 'text/html; charset=utf-8',
    body: Buffer.from(
      `<!DOCTYPE html><html><head>
<script src="root-rel.js"></script>
</head><body></body></html>`,
    ),
  });
  res.set('/base/root-rel.js', {
    type: 'text/javascript',
    body: Buffer.from(
      "import a from '/pkg/root.js';\n" +
        "const b = await import('/pkg/dyn.js');\n" +
        "const w = new URL('frag_bg.wasm#v1', import.meta.url);\n" +
        'export default a;\n',
    ),
  });
  res.set('/pkg/root.js', {
    type: 'text/javascript',
    body: Buffer.from('export const root = 1;\n'),
  });
  res.set('/pkg/dyn.js', {
    type: 'text/javascript',
    body: Buffer.from('export const dyn = 2;\n'),
  });
  res.set('/base/frag_bg.wasm', {
    type: 'application/wasm',
    body: Buffer.from('WASM-FRAG'),
  });
  return res;
}

function resourcesForScenario(scenario) {
  switch (scenario) {
    case 'recursive':
      return makeRecursiveResources();
    case 'query':
      return makeQueryResources();
    case 'traversal':
      return makeTraversalResources();
    case 'stall':
      return makeStallResources();
    case 'trickle':
      return makeTrickleResources();
    case 'fanout':
      return makeFanoutResources();
    case 'abort':
      return makeAbortResources();
    case 'proto-redirect':
      return makeProtoRedirectResources();
    case 'empty':
      return makeEmptyResources();
    case 'root-relative':
      return makeRootRelativeResources();
    default:
      return makeResources();
  }
}

/**
 * Start the fixture server.
 * @param {object} [options]
 * @param {string} [options.scenario] - See the module docblock.
 * @returns {Promise<{url:string, port:number, stats:object, close:()=>Promise<void>}>}
 */
function startFixtureServer(options = {}) {
  const scenario = options.scenario || 'default';
  const resources = resourcesForScenario(scenario);
  const stats = { requests: 0, maxConcurrent: 0, concurrent: 0, paths: [] };
  const sockets = new Set();

  const server = http.createServer((req, res) => {
    const pathname = new URL(req.url, 'http://127.0.0.1').pathname;
    stats.requests++;
    stats.paths.push(pathname);
    stats.concurrent++;
    if (stats.concurrent > stats.maxConcurrent) {
      stats.maxConcurrent = stats.concurrent;
    }
    const entry = resources.get(pathname);
    const finish = () => {
      stats.concurrent--;
    };
    if (!entry) {
      setTimeout(() => {
        res.statusCode = 404;
        res.end('not found');
        finish();
      }, DELAY_MS);
      return;
    }
    if (entry.stall) {
      // Never answer: the client must time out and clean the socket up.
      req.socket.on('close', finish);
      return;
    }
    if (entry.trickle) {
      // Send a byte every 50ms, forever: the socket is never idle, so only a
      // whole-request deadline can abort this.
      res.statusCode = 200;
      res.setHeader('content-type', entry.type);
      res.setHeader('transfer-encoding', 'chunked');
      const timer = setInterval(() => {
        if (res.writableEnded || res.destroyed) {
          clearInterval(timer);
          return;
        }
        res.write('x');
      }, 50);
      req.socket.on('close', () => {
        clearInterval(timer);
        finish();
      });
      return;
    }
    if (entry.abort) {
      // Send headers, then destroy the socket mid-body: the client must see an
      // error (not a silent success with a truncated body).
      setTimeout(() => {
        res.statusCode = 200;
        res.setHeader('content-type', entry.type);
        res.setHeader('content-length', String(entry.body.length + 64));
        res.write(entry.body.subarray(0, Math.max(1, entry.body.length - 4)));
        res.socket.destroy();
        finish();
      }, DELAY_MS);
      return;
    }
    setTimeout(() => {
      if (entry.status && entry.location) {
        res.statusCode = entry.status;
        // `{{SELF}}` resolves to the server's own https origin, which is used
        // to build cross-protocol (http -> https) redirects.
        const location = entry.location.replace(
          '{{SELF}}',
          `https://127.0.0.1:${server.address().port}`,
        );
        res.setHeader('location', location);
        res.end();
      } else if (entry.gzip) {
        res.statusCode = 200;
        res.setHeader('content-type', entry.type);
        res.setHeader('content-encoding', 'gzip');
        res.end(zlib.gzipSync(entry.body));
      } else if (entry.empty) {
        // A broken resource: 200 OK but with an empty body.
        res.statusCode = 200;
        res.setHeader('content-type', entry.type);
        res.setHeader('content-length', '0');
        res.end();
      } else {
        res.statusCode = 200;
        res.setHeader('content-type', entry.type);
        res.end(entry.body);
      }
      finish();
    }, DELAY_MS);
  });

  server.on('connection', (socket) => {
    sockets.add(socket);
    socket.on('close', () => sockets.delete(socket));
  });

  return new Promise((resolve, reject) => {
    server.on('error', reject);
    server.listen(0, '127.0.0.1', () => {
      const { port } = server.address();
      resolve({
        url: `http://127.0.0.1:${port}`,
        port,
        stats,
        close: () =>
          new Promise((r) => {
            for (const socket of sockets) socket.destroy();
            server.close(() => r());
          }),
      });
    });
  });
}

/** Write an app.config.json pointing at the fixture server. */
function writeConfig(root, url) {
  const fs = require('fs');
  const path = require('path');
  fs.mkdirSync(root, { recursive: true });
  fs.writeFileSync(
    path.join(root, 'app.config.json'),
    JSON.stringify(
      { app: { version: '0.0.0-fixture' }, remote: { url } },
      null,
      2,
    ),
  );
}

module.exports = { startFixtureServer, writeConfig, DELAY_MS };
