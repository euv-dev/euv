#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
# shellcheck source=common.sh
source "${SCRIPT_DIR}/common.sh"

TARGETS=(
  "wasm32-unknown-unknown"
  "aarch64-apple-darwin"
  "aarch64-apple-ios"
  "aarch64-apple-ios-sim"
  "aarch64-linux-android"
  "x86_64-pc-windows-msvc"
  "x86_64-unknown-linux-gnu"
)

if ! command -v rustup >/dev/null 2>&1; then
  log_error "rustup is not installed. Visit https://rustup.rs"
  exit 1
fi

log_info "Installing all 7 cross-platform Rust targets..."
rustup target add "${TARGETS[@]}"
log_info "Rust targets ready."
