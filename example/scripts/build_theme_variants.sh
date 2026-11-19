#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT_DIR"

if [[ -n "${WASM_BINDGEN_BIN_DIR:-}" ]]; then
    PATH="${WASM_BINDGEN_BIN_DIR}:${PATH}"
    export PATH
fi

for command_name in cargo python3; do
    if ! command -v "$command_name" >/dev/null 2>&1; then
        printf 'Required command not found: %s\n' "$command_name" >&2
        exit 1
    fi
done

THEME_FEATURES="$(cargo metadata --no-deps --format-version 1 | python3 -c 'import json, sys; metadata = json.load(sys.stdin); package = next(item for item in metadata["packages"] if item["name"] == "euv-example"); print("default"); print("\n".join(sorted(name for name in package["features"] if name.startswith("theme-"))))')"
BUILD_LOG_DIR="${TMPDIR:-/tmp}/euv-theme-feature-build"
mkdir -p "$BUILD_LOG_DIR"
THEME_CACHE_DIR="$ROOT_DIR/native/tauri/.cache/themes"
mkdir -p "$THEME_CACHE_DIR"
python3 - "$ROOT_DIR" "$THEME_FEATURES" <<'PY'
from pathlib import Path
import shutil
import sys

root = Path(sys.argv[1])
features = set(sys.argv[2].splitlines())
output = root / 'example/www/themes'
cache = root / 'native/tauri/.cache/themes'
for directory in [output, cache]:
    if directory.is_symlink():
        raise ValueError(f'Theme artifact directory must not be a symlink: {directory}')
if output.exists():
    for path in output.iterdir():
        if path.name in features:
            if path.is_symlink():
                raise ValueError(f'Theme output must not be a symlink: {path}')
            continue
        if path.is_dir() and not path.is_symlink():
            shutil.rmtree(path)
        else:
            path.unlink()
        print(f'Removed retired theme output: {path.name}')
for path in cache.glob('*.sha256'):
    if path.stem not in features:
        path.unlink()
        print(f'Removed retired theme stamp: {path.name}')
    elif path.is_symlink():
        raise ValueError(f'Theme stamp must not be a symlink: {path}')
PY
SOURCE_FINGERPRINT="$(python3 - "$ROOT_DIR" <<'PY'
import hashlib
from pathlib import Path
import sys
root = Path(sys.argv[1])
paths = [root / 'Cargo.toml', root / 'Cargo.lock', root / 'example/index.html', root / 'example/scripts/build_theme_variants.sh']
paths.extend((root / 'src').rglob('*'))
for name in ['core', 'macros', 'ui', 'engine', 'cli', 'example']:
    paths.append(root / name / 'Cargo.toml')
    paths.extend((root / name / 'src').rglob('*'))
digest = hashlib.sha256()
for path in sorted(path for path in paths if path.is_file()):
    digest.update(str(path.relative_to(root)).encode())
    digest.update(path.read_bytes())
print(digest.hexdigest())
PY
)"

build_count=0
while IFS= read -r feature; do
    [[ -n "$feature" ]] || continue
    output_dir="www/themes/${feature}"
    artifact_dir="example/${output_dir}"
    log_file="${BUILD_LOG_DIR}/${feature}.log"
    stamp_file="${THEME_CACHE_DIR}/${feature}.sha256"
    cached_source=""
    if [[ -f "$stamp_file" ]]; then
        read -r cached_source < "$stamp_file" || true
    fi
    if [[ "$cached_source" == "$SOURCE_FINGERPRINT" && -s "${artifact_dir}/index.html" && -s "${artifact_dir}/pkg/euv_example_bg.wasm" ]]; then
        build_count=$((build_count + 1))
        printf 'Unchanged %s -> %s/index.html\n' "$feature" "$artifact_dir"
        continue
    fi
    feature_args=(--no-default-features)
    if [[ "$feature" != "default" ]]; then
        feature_args+=(--features "$feature")
    fi

    printf 'Building %s...\n' "$feature"
    if ! cargo run -p euv-cli -- build \
        --crate-path ./example \
        --release \
        --www-dir "$output_dir" \
        -- "${feature_args[@]}" >"$log_file" 2>&1; then
        printf 'Build failed for %s; see %s\n' "$feature" "$log_file" >&2
        python3 - "$log_file" >&2 <<'PY'
from pathlib import Path
import sys
print(Path(sys.argv[1]).read_text(), end="")
PY
        exit 1
    fi

    for artifact in "${artifact_dir}/index.html" "${artifact_dir}/pkg/euv_example_bg.wasm"; do
        if [[ ! -s "$artifact" ]]; then
            printf 'Build for %s did not produce %s\n' "$feature" "$artifact" >&2
            printf 'Build log: %s\n' "$log_file" >&2
            exit 1
        fi
    done

    printf '%s\n' "$SOURCE_FINGERPRINT" > "$stamp_file"
    build_count=$((build_count + 1))
    printf 'Built %s -> %s/index.html\n' "$feature" "$artifact_dir"
done <<< "$THEME_FEATURES"

printf 'Built %s independent example variants.\n' "$build_count"
python3 - "$ROOT_DIR" "$THEME_FEATURES" <<'PY'
from pathlib import Path
import sys

root = Path(sys.argv[1])
features = set(sys.argv[2].splitlines())
output = root / 'example/www/themes'
assert {path.name for path in output.iterdir()} == features, 'Theme output differs from current features'
for path in [root / 'example/www/index.html', *output.glob('*/index.html')]:
    assert b'__euv_reload' not in path.read_bytes(), f'Development bootstrap in release output: {path}'
print(f'Verified exactly {len(features)} current release themes without a development bootstrap')
PY
python3 "$ROOT_DIR/example/scripts/write_release_manifest.py" --dist "$ROOT_DIR/example/www"
