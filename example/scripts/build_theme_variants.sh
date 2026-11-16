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

build_count=0
while IFS= read -r feature; do
    [[ -n "$feature" ]] || continue
    output_dir="www/themes/${feature}"
    artifact_dir="example/${output_dir}"
    log_file="${BUILD_LOG_DIR}/${feature}.log"
    feature_args=(--no-default-features)
    if [[ "$feature" != "default" ]]; then
        feature_args+=(--features "$feature")
    fi

    printf 'Building %s...\n' "$feature"
    if ! cargo run -p euv-cli -- build \
        --crate-path ./example \
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

    build_count=$((build_count + 1))
    printf 'Built %s -> %s/index.html\n' "$feature" "$artifact_dir"
done <<< "$THEME_FEATURES"

printf 'Built %s independent example variants.\n' "$build_count"
