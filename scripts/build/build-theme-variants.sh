#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT_DIR"

if [[ -n "${WASM_BINDGEN_BIN_DIR:-}" ]]; then
    PATH="${WASM_BINDGEN_BIN_DIR}:${PATH}"
    export PATH
fi

for command_name in cargo python3 wasm-pack wasm-bindgen; do
    if ! command -v "$command_name" >/dev/null 2>&1; then
        printf 'Required command not found: %s\n' "$command_name" >&2
        exit 1
    fi
done

THEME_FEATURES="$(cargo metadata --no-deps --format-version 1 | python3 -c 'import json, sys; metadata = json.load(sys.stdin); package = next(item for item in metadata["packages"] if item["name"] == "euv-example"); print("default"); print("\n".join(sorted(name for name in package["features"] if name.startswith("theme-"))))')"
BUILD_LOG_DIR="${TMPDIR:-/tmp}/euv-theme-feature-build"
mkdir -p "$BUILD_LOG_DIR"
THEME_CACHE_DIR="$ROOT_DIR/native/tauri/.cache/themes"
for directory in example/www example/www/themes native/tauri/.cache "$THEME_CACHE_DIR"; do
    if [[ -L "$directory" ]]; then
        printf 'Cannot manage example theme outputs through symlink: %s\n' "$directory" >&2
        exit 1
    fi
done
mkdir -p "$THEME_CACHE_DIR"
SOURCE_FINGERPRINT="$(python3 - "$ROOT_DIR" <<'PY'
import hashlib
from pathlib import Path
import sys
root = Path(sys.argv[1])
paths = [
    root / 'Cargo.toml',
    root / 'Cargo.lock',
    root / 'example/index.html',
    root / 'scripts/build/build-theme-variants.sh',
    root / 'scripts/build/write-release-manifest.py',
]
paths.extend((root / 'src').rglob('*'))
for name in ['core', 'macros', 'ui', 'cli', 'example']:
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

# Drop only the retired example variant before inventorying the distribution.
rm -rf -- example/www/themes/theme-custom
rm -f -- "$THEME_CACHE_DIR/theme-custom.sha256"

printf 'Built %s independent example variants.\n' "$build_count"
python3 "$ROOT_DIR/scripts/build/write-release-manifest.py" --dist "$ROOT_DIR/example/www"
