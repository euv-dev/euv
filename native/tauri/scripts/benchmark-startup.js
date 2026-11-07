#!/usr/bin/env node
const assert = require('node:assert/strict');
const { execFileSync } = require('node:child_process');
const path = require('node:path');
const vm = require('node:vm');
const { getIndexHtml } = require('./apply-config');

async function measure(html, cacheReadyMs) {
  let now = 0;
  let bridgeCalls = 0;
  let completedAt = null;
  let ready;
  let sequence = 0;
  const timers = new Map();
  const window = {
    addEventListener() {},
    location: { replace() { completedAt = now; } },
    __TAURI_INTERNALS__: {
      invoke(command) {
        bridgeCalls++;
        if (command === 'wait_for_cached_resource') {
          return new Promise((resolve) => { ready = resolve; });
        }
        return Promise.resolve({ from_cache: now >= cacheReadyMs });
      },
    },
  };
  vm.runInNewContext(html.match(/<script>([\s\S]*?)<\/script>/)[1], {
    window,
    navigator: { userAgent: 'Android' },
    setTimeout(callback, delay) {
      timers.set(++sequence, { at: now + delay, callback });
      return sequence;
    },
    clearTimeout(id) { timers.delete(id); },
  });
  while (completedAt === null && now < cacheReadyMs + 5000) {
    await new Promise(setImmediate);
    if (ready && now <= cacheReadyMs) {
      now = cacheReadyMs;
      ready({ from_cache: true });
      ready = null;
      await new Promise(setImmediate);
    } else if (timers.size) {
      const [id, timer] = [...timers.entries()].sort((a, b) => a[1].at - b[1].at)[0];
      timers.delete(id);
      now = timer.at;
      timer.callback();
    } else if (completedAt === null) {
      throw new Error('Bootstrap stopped before navigating');
    }
  }
  assert.notEqual(completedAt, null);
  return { bridgeCalls, navigatedAtMs: completedAt };
}

async function main() {
  const root = path.resolve(__dirname, '..');
  const baselineHtml = execFileSync('git', ['show', 'HEAD:dist/index.html'], { cwd: root, encoding: 'utf8' });
  const optimizedHtml = getIndexHtml({ app: { name: 'Benchmark' }, ui: { backgroundColor: '#fff' } });
  const results = [];
  for (const cacheReadyMs of [0, 1500, 5000]) {
    const baseline = await measure(baselineHtml, cacheReadyMs);
    const optimized = await measure(optimizedHtml, cacheReadyMs);
    assert.equal(optimized.bridgeCalls, 1);
    results.push({ cacheReadyMs, baseline, optimized });
  }
  console.log(JSON.stringify({ fixture: 'deterministic bootstrap IPC simulation', results }, null, 2));
}

if (require.main === module) {
  main().catch((error) => { console.error(error); process.exitCode = 1; });
}

module.exports = { measure };
