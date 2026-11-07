#!/usr/bin/env node
'use strict';

const path = require('node:path');
const { execFileSync } = require('node:child_process');
const { MANIFEST_NAME, DEFAULT_DESTINATION, sourceDirectory, verifyBundle } = require('./bundle-example');

// Read ZIP and ELF payloads directly instead of trusting an earlier build step.
const CHECK_APK = String.raw`
import hashlib, json, pathlib, sys, zipfile
apk, cache, manifest_name = pathlib.Path(sys.argv[1]), pathlib.Path(sys.argv[2]), sys.argv[3]
manifest_bytes = (cache / manifest_name).read_bytes()
manifest = json.loads(manifest_bytes)
with zipfile.ZipFile(apk) as archive:
    libraries = sorted(name for name in archive.namelist()
                       if name.startswith('lib/') and name.endswith('/libeuv_lib.so'))
    if not libraries:
        raise ValueError('APK does not contain the native euv library')
    for name in libraries:
        library = archive.read(name)
        if not library.startswith(b'\x7fELF'):
            raise ValueError('APK native library is not ELF: ' + name)
        if manifest_bytes not in library:
            raise ValueError('Local example provenance is not embedded in ' + name)
        for entry in manifest['files']:
            content = (cache / entry['path']).read_bytes()
            if len(content) != entry['size'] or hashlib.sha256(content).hexdigest() != entry['sha256']:
                raise ValueError('Bundle changed during APK verification: ' + entry['path'])
            if content and content not in library:
                raise ValueError('Local example file missing from ' + name + ': ' + entry['path'])
print(json.dumps({'apk': str(apk), 'libraries': libraries, 'files': len(manifest['files'])}))
`;

function verifyApk(apk, { source = sourceDirectory(), destination = DEFAULT_DESTINATION } = {}) {
  if (!apk) throw new Error('Usage: node scripts/verify-apk.js <apk>');
  verifyBundle({ source, destination });
  const output = execFileSync('python3', ['-c', CHECK_APK, path.resolve(apk), destination, MANIFEST_NAME], {
    encoding: 'utf8',
    stdio: ['ignore', 'pipe', 'pipe'],
  });
  return JSON.parse(output);
}

if (require.main === module) {
  try {
    const result = verifyApk(process.argv[2]);
    console.log(`[OK] APK embeds all ${result.files} local example files in ${result.libraries.length} native libraries`);
  } catch (error) {
    console.error(`[ERROR] APK verification failed: ${error.stderr?.toString().trim() || error.message}`);
    process.exitCode = 1;
  }
}

module.exports = { verifyApk };
