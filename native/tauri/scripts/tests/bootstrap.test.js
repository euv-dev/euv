const assert = require('node:assert/strict');
const fs = require('node:fs');
const os = require('node:os');
const path = require('node:path');
const vm = require('node:vm');
const { test } = require('node:test');
const { getIndexHtml, writeIfChanged } = require('../config/apply-config');

const config = { app: { name: 'Test' }, ui: { backgroundColor: '#fff' } };
const flushPromises = () => new Promise(setImmediate);

function bootstrap({ invoke, userAgent = 'Android' } = {}) {
  const timers = new Map();
  const listeners = new Map();
  const navigations = [];
  let nextTimer = 0;
  const window = {
    location: { replace: (url) => navigations.push(url) },
    addEventListener: (event, listener) => listeners.set(event, listener),
  };
  if (invoke) window.__TAURI_INTERNALS__ = { invoke };
  const context = {
    window,
    navigator: { userAgent },
    setTimeout: (callback, delay) => {
      timers.set(++nextTimer, { callback, delay });
      return nextTimer;
    },
    clearTimeout: (id) => timers.delete(id),
  };
  const script = getIndexHtml(config).match(/<script>([\s\S]*?)<\/script>/)[1];
  vm.runInNewContext(script, context);
  return {
    window, timers, listeners, navigations,
    tick() {
      assert.equal(timers.size, 1);
      const [id, timer] = timers.entries().next().value;
      timers.delete(id);
      timer.callback();
      return timer.delay;
    },
  };
}

test('cache initialization uses one pending IPC request and no polling timers', async () => {
  const commands = [];
  let ready;
  const page = bootstrap({
    invoke(command) {
      commands.push(command);
      return new Promise((resolve) => { ready = resolve; });
    },
  });
  await flushPromises();
  assert.deepEqual(commands, ['wait_for_cached_resource']);
  assert.equal(page.timers.size, 0);
  assert.deepEqual(page.navigations, []);
  ready({ from_cache: true });
  await flushPromises();
  assert.deepEqual(page.navigations, ['https://euv.localhost/index.html']);
  assert.equal(page.timers.size, 0);
});

test('bridge discovery waits locally without calling a missing bridge', async () => {
  const page = bootstrap();
  assert.equal(page.tick(), 16);
  assert.equal(page.tick(), 32);
  let calls = 0;
  page.window.__TAURI_INTERNALS__ = {
    invoke() {
      calls++;
      return Promise.resolve({ from_cache: true });
    },
  };
  page.tick();
  await flushPromises();
  assert.equal(calls, 1);
  assert.equal(page.navigations.length, 1);
});

test('initialization errors back off without overlapping requests', async () => {
  let calls = 0;
  const page = bootstrap({
    invoke() {
      calls++;
      return calls === 1 ? Promise.reject(new Error('not ready')) : Promise.resolve({ from_cache: true });
    },
  });
  await flushPromises();
  assert.equal(calls, 1);
  assert.equal(page.tick(), 250);
  await flushPromises();
  assert.equal(calls, 2);
  assert.equal(page.navigations.length, 1);
});

test('repeated initialization errors back off with a bounded retry delay', async () => {
  let calls = 0;
  const page = bootstrap({
    invoke() {
      calls++;
      return Promise.reject(new Error('cache unavailable'));
    },
  });
  const delays = [];
  for (let retry = 0; retry < 12; retry++) {
    await flushPromises();
    assert.equal(calls, retry + 1);
    assert.deepEqual(page.navigations, []);
    delays.push(page.tick());
  }
  assert.equal(delays[0], 250);
  assert.ok(delays.every((delay) => delay >= 250 && delay <= 5000));
  assert.ok(delays.some((delay) => delay > delays[0]));
  await flushPromises();
  page.listeners.get('pagehide')();
  assert.equal(page.timers.size, 0);
});

test('an unsuccessful readiness result is not treated as a usable snapshot', async () => {
  const page = bootstrap({ invoke: () => Promise.resolve({ from_cache: false }) });
  await flushPromises();
  assert.deepEqual(page.navigations, []);
  assert.equal(page.timers.size, 1);
});

test('page disposal cancels retry timers and prevents late navigation', async () => {
  const waiting = bootstrap();
  waiting.listeners.get('pagehide')();
  assert.equal(waiting.timers.size, 0);
  let ready;
  const pending = bootstrap({ invoke: () => new Promise((resolve) => { ready = resolve; }) });
  pending.listeners.get('pagehide')();
  ready({ from_cache: true });
  await flushPromises();
  assert.deepEqual(pending.navigations, []);
});

test('page disposal prevents a rejected request from scheduling another retry', async () => {
  let reject;
  const pending = bootstrap({
    invoke: () => new Promise((resolve, rejectPromise) => { reject = rejectPromise; }),
  });
  pending.listeners.get('pagehide')();
  reject(new Error('cache unavailable'));
  await flushPromises();
  assert.equal(pending.timers.size, 0);
  assert.deepEqual(pending.navigations, []);
});

test('desktop bootstrap retains the custom-protocol URL', async () => {
  const page = bootstrap({ userAgent: 'MacIntel', invoke: () => Promise.resolve({ from_cache: true }) });
  await flushPromises();
  assert.deepEqual(page.navigations, ['euv://localhost/index.html']);
});

for (const [platform, userAgent, expected] of [
  ['Windows', 'Mozilla/5.0 (Windows NT 10.0; Win64; x64)', 'https://euv.localhost/index.html'],
  ['Linux', 'Mozilla/5.0 (X11; Linux x86_64)', 'euv://localhost/index.html'],
  ['iOS', 'Mozilla/5.0 (iPhone; CPU iPhone OS 18_0 like Mac OS X)', 'euv://localhost/index.html'],
]) {
  test(`${platform} bootstrap uses the platform custom-protocol origin`, async () => {
    const page = bootstrap({ userAgent, invoke: () => Promise.resolve({ from_cache: true }) });
    await flushPromises();
    assert.deepEqual(page.navigations, [expected]);
  });
}

test('custom entry HTML is preserved verbatim', () => {
  assert.equal(getIndexHtml({ ui: { indexHtml: '<html>custom</html>' } }), '<html>custom</html>');
});

test('unchanged generated files retain their timestamps', () => {
  const directory = fs.mkdtempSync(path.join(os.tmpdir(), 'euv-config-'));
  try {
    const file = path.join(directory, 'nested', 'config');
    assert.equal(writeIfChanged(file, 'first'), true);
    const before = fs.statSync(file, { bigint: true }).mtimeNs;
    assert.equal(writeIfChanged(file, 'first'), false);
    assert.equal(fs.statSync(file, { bigint: true }).mtimeNs, before);
    assert.equal(writeIfChanged(file, 'second'), true);
    assert.equal(fs.readFileSync(file, 'utf8'), 'second');
  } finally {
    fs.rmSync(directory, { recursive: true, force: true });
  }
});
