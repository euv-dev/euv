'use strict';
const assert = require('node:assert/strict');
const fs = require('node:fs');
const path = require('node:path');
const os = require('node:os');
const { spawnSync } = require('node:child_process');
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

test('Android backup initialization creates missing temporary parent directories', (t) => {
  const fixture = fs.mkdtempSync(path.join(os.tmpdir(), 'euv-backup-root-'));
  t.after(() => fs.rmSync(fixture, { recursive: true, force: true }));
  const source = fs.readFileSync(path.join(root, 'scripts/build/build.sh'), 'utf8');
  const start = source.indexOf('ANDROID_DIR=');
  const end = source.indexOf('restore_webviews()');
  assert.ok(start >= 0 && end > start);
  const initialization = source.slice(start, end);
  for (const customTemporaryRoot of [false, true]) {
    const project = path.join(fixture, customTemporaryRoot ? 'custom' : 'default');
    fs.mkdirSync(project);
    const environment = { ...process.env, PROJECT_ROOT: project };
    delete environment.TMPDIR;
    const expectedParent = customTemporaryRoot ? path.join(project, 'missing-temp') : path.join(project, '.cache/sdk');
    if (customTemporaryRoot) environment.TMPDIR = expectedParent;
    assert.equal(fs.existsSync(expectedParent), false);
    const result = spawnSync('bash', ['-c', `set -eu\n${initialization}\nprintf '%s' "$BACKUP_DIR"`],
      { env: environment, encoding: 'utf8', timeout: 10000 });
    assert.equal(result.status, 0, result.error?.message || result.stderr);
    assert.equal(path.dirname(result.stdout), expectedParent);
    assert.ok(fs.statSync(result.stdout).isDirectory());
  }
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
