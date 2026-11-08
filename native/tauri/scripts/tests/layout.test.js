'use strict';
const assert = require('node:assert/strict');
const fs = require('node:fs');
const path = require('node:path');
const { test } = require('node:test');
const root = path.resolve(__dirname, '../..');

test('native scripts are grouped by responsibility rather than placed at the root', () => {
  const groups = ['benchmark', 'build', 'cache', 'config', 'tests', 'toolchain', 'validation'];
  const entries = fs.readdirSync(path.join(root, 'scripts'), { withFileTypes: true });
  assert.deepEqual(entries.filter((entry) => entry.isDirectory()).map((entry) => entry.name).sort(), groups);
  assert.deepEqual(entries.filter((entry) => entry.isFile() && /\.(js|py|sh|m)$/.test(entry.name)).map((entry) => entry.name), []);
  for (const obsolete of ['build.sh', 'app.config.json', 'keystore.jks', 'sdk']) {
    assert.equal(fs.existsSync(path.join(root, obsolete)), false, obsolete);
  }
});

test('native configuration, signing input, runtime resources and cache have explicit locations', () => {
  const config = JSON.parse(fs.readFileSync(path.join(root, 'config/app.config.json'), 'utf8'));
  assert.equal(config.build.signing.android.keystorePath, 'resources/signing/keystore.jks');
  assert.ok(fs.existsSync(path.join(root, config.build.signing.android.keystorePath)));
  assert.ok(fs.existsSync(path.join(root, 'resources/runtime/gpu-probe.js')));
  assert.match(fs.readFileSync(path.join(root, '.gitignore'), 'utf8'), /\/\.cache\//);
  assert.match(fs.readFileSync(path.join(root, 'scripts/build/build.sh'), 'utf8'), /artifacts\/android/);
});

test('every package command resolves a grouped script instead of an obsolete entry point', () => {
  const pkg = JSON.parse(fs.readFileSync(path.join(root, 'package.json'), 'utf8'));
  for (const [name, command] of Object.entries(pkg.scripts)) {
    for (const match of command.matchAll(/scripts\/[A-Za-z0-9_.\/-]+/g)) {
      const file = match[0].replace(/\/$/, '');
      assert.ok(fs.existsSync(path.join(root, file)), `${name}: ${file}`);
    }
  }
});
