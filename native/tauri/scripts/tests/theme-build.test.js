'use strict';

const assert = require('node:assert/strict');
const fs = require('node:fs');
const os = require('node:os');
const path = require('node:path');
const { execFileSync, spawnSync } = require('node:child_process');
const { test } = require('node:test');

// Unit test only: mock Cargo's metadata/build boundary, not a real WASM build.
// The production shell script, source fingerprint, and manifest writer all run.
test('theme build delegates to the CLI without global WASM tools and still requires cargo', (t) => {
  const root = fs.mkdtempSync(path.join(process.env.TMPDIR || os.tmpdir(), 'euv-theme-unit-'));
  t.after(() => fs.rmSync(root, { recursive: true, force: true }));
  const bin = path.join(root, 'bin');
  const scripts = path.join(root, 'example/scripts');
  fs.mkdirSync(bin);
  fs.mkdirSync(scripts, { recursive: true });
  for (const name of ['build_theme_variants.sh', 'write_release_manifest.py']) {
    fs.copyFileSync(path.resolve(__dirname, '../../../../example/scripts', name), path.join(scripts, name));
  }
  fs.writeFileSync(path.join(root, 'Cargo.toml'), '[workspace]\n');
  fs.writeFileSync(path.join(root, 'Cargo.lock'), '# unit fixture\n');
  fs.writeFileSync(path.join(root, 'example/Cargo.toml'), '[package]\nname = "euv-example"\nversion = "0.0.36"\n');
  fs.writeFileSync(path.join(root, 'example/index.html'), '<html>unit source</html>');
  fs.mkdirSync(path.join(root, 'example/www'), { recursive: true });
  fs.writeFileSync(path.join(root, 'example/www/index.html'), '<html>unit distribution</html>');
  const python = execFileSync('python3', ['-c', 'import sys; print(sys.executable)'], { encoding: 'utf8' }).trim();
  fs.symlinkSync(python, path.join(bin, 'python3'));
  for (const name of ['bash', 'dirname', 'mkdir']) {
    const executable = execFileSync('/bin/sh', ['-c', `command -v ${name}`], { encoding: 'utf8' }).trim();
    fs.symlinkSync(executable, path.join(bin, name));
  }
  fs.writeFileSync(path.join(bin, 'cargo'), `#!${python}
import json
import os
from pathlib import Path
import sys
root = Path.cwd()
args = sys.argv[1:]
if args[0] == 'metadata':
    print(json.dumps({'packages': [{'name': 'euv-example', 'version': '0.0.36', 'features': {}, 'manifest_path': str(root / 'example/Cargo.toml')}]}))
elif args == ['run', '-p', 'euv-cli', '--', 'build', '--crate-path', './example', '--release', '--www-dir', 'www/themes/default', '--', '--no-default-features']:
    if os.environ.get('EUV_UNIT_FAIL_BUILD') == '1':
        raise SystemExit('unit feature compiler diagnostic')
    (root / 'cli-invoked.json').write_text(json.dumps(args))
    output = root / 'example/www/themes/default'
    (output / 'pkg').mkdir(parents=True)
    (output / 'index.html').write_text('<html>mock Cargo output</html>')
    (output / 'pkg/euv_example_bg.wasm').write_bytes(b'unit fixture, not compiled WASM')
else:
    raise SystemExit('Unexpected mock Cargo arguments: ' + repr(args))
`, { mode: 0o755 });
  const env = { ...process.env, PATH: bin, TMPDIR: root };
  delete env.WASM_BINDGEN_BIN_DIR;
  delete env.BASH_ENV;
  const absent = spawnSync(path.join(bin, 'bash'), ['-c', 'command -v wasm-pack; command -v wasm-bindgen'], { env, encoding: 'utf8', timeout: 10000 });
  assert.equal(absent.status, 1);
  assert.equal(absent.stdout, '');
  const script = path.join(scripts, 'build_theme_variants.sh');
  const result = spawnSync(path.join(bin, 'bash'), [script], { env, encoding: 'utf8', timeout: 10000 });
  assert.equal(result.status, 0, result.stderr + result.stdout);
  assert.match(result.stdout, /Built 1 independent example variants\./);
  assert.equal(JSON.parse(fs.readFileSync(path.join(root, 'cli-invoked.json'), 'utf8'))[2], 'euv-cli');
  const manifest = JSON.parse(fs.readFileSync(path.join(root, 'example/www/euv-release.json'), 'utf8'));
  assert.equal(manifest.version, '0.0.36');
  assert.ok(manifest.files.some((file) => file.path === 'themes/default/pkg/euv_example_bg.wasm'));

  fs.unlinkSync(path.join(root, 'native/tauri/.cache/themes/default.sha256'));
  const failedBuild = spawnSync(path.join(bin, 'bash'), [script], {
    env: { ...env, EUV_UNIT_FAIL_BUILD: '1' }, encoding: 'utf8', timeout: 10000,
  });
  assert.equal(failedBuild.status, 1);
  assert.match(failedBuild.stderr, /unit feature compiler diagnostic/);

  fs.unlinkSync(path.join(bin, 'cargo'));
  fs.unlinkSync(path.join(root, 'cli-invoked.json'));
  const missingCargo = spawnSync(path.join(bin, 'bash'), [script], { env, encoding: 'utf8', timeout: 10000 });
  assert.equal(missingCargo.status, 1, missingCargo.stderr);
  assert.match(missingCargo.stderr, /Required command not found: cargo/);
  assert.equal(fs.existsSync(path.join(root, 'cli-invoked.json')), false);
});
