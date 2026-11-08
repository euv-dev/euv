#!/usr/bin/env node
'use strict';
const fs = require('node:fs');
const path = require('node:path');
const vm = require('node:vm');
const { execFileSync } = require('node:child_process');

function extract(source) {
  const start = source.indexOf('const IMMERSIVE_SAFE_AREA_SCRIPT: &str = r#"');
  if (start < 0) throw new Error('Missing immersive script');
  const body = source.indexOf('r#"', start) + 3;
  return source.slice(body, source.indexOf('"#;', body));
}

function fixture(script) {
  const stats = { layoutReads: 0, probeAllocations: 0, safeAreaWrites: 0 };
  const listeners = new Map();
  const ids = new Map();
  const metas = [];
  const frames = [];
  let top = '24px';
  function element(tag) {
    if (tag === 'div') stats.probeAllocations++;
    const values = new Map();
    return { tag, style: { getPropertyValue: (key) => values.get(key) || '',
      setProperty(key, value) { values.set(key, value); if (key === '--euv-mobile-safe-top') stats.safeAreaWrites++; } },
      setAttribute() {},
      appendChild(child) { child.parentNode = this; child.isConnected = true; if (child.id) ids.set(child.id, child); if (child.tag === 'meta') metas.push(child); },
      removeChild(child) { child.parentNode = null; child.isConnected = false; } };
  }
  const listen = (name, callback) => { if (!listeners.has(name)) listeners.set(name, []); listeners.get(name).push(callback); };
  const document = { head: element('head'), body: element('body'), documentElement: element('html'),
    createElement: element, querySelectorAll: () => metas, getElementById: (id) => ids.get(id), addEventListener: listen };
  const window = { innerHeight: 800, addEventListener: listen, visualViewport: { height: 800, offsetTop: 0, addEventListener: listen } };
  vm.runInNewContext(script, { window, document, getComputedStyle() { stats.layoutReads++; return { paddingTop: top }; },
    requestAnimationFrame(callback) { frames.push(callback); }, setTimeout(callback) { callback(); } });
  const flush = () => { const pending = frames.splice(0); pending.forEach((callback) => callback()); };
  flush();
  return { stats, window, document, flush, setTop(value) { top = value; },
    dispatch(name, value = {}) { for (const callback of listeners.get(name) || []) callback(value); },
    reset() { for (const name of Object.keys(stats)) stats[name] = 0; } };
}

function burst(script, count = 100) {
  const page = fixture(script);
  page.reset();
  for (let index = 0; index < count; index++) page.dispatch('resize');
  page.flush();
  return { ...page.stats };
}

if (require.main === module) {
  const ref = process.argv[2];
  if (!ref) throw new Error('Pass an explicit baseline Git ref');
  const root = path.resolve(__dirname, '../../../..');
  const before = execFileSync('git', ['show', `${ref}:native/tauri/src-tauri/src/cache/fn.rs`], { cwd: root, encoding: 'utf8' });
  const after = fs.readFileSync(path.join(__dirname, '../../src-tauri/src/cache/const.rs'), 'utf8');
  console.log(JSON.stringify({ fixture: 'instrumented DOM/event simulation; not measured client FPS',
    resizeEvents: 100, before: burst(extract(before)), after: burst(extract(after)) }, null, 2));
}
module.exports = { extract, fixture, burst };
