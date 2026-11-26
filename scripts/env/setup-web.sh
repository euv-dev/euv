#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
# shellcheck source=common.sh
source "${SCRIPT_DIR}/common.sh"

"${SCRIPT_DIR}/setup-rust.sh"
if ! command -v wasm-pack >/dev/null 2>&1; then
  log_info "Installing wasm-pack via cargo..."
  cargo install wasm-pack --locked
fi
log_info "Web toolchain ready: $(wasm-pack --version)"
