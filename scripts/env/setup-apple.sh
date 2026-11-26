#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
# shellcheck source=common.sh
source "${SCRIPT_DIR}/common.sh"

if [ "$(detect_os)" != "macos" ]; then
  log_warn "Apple SDK checks require macOS; skipping on $(detect_os)."
  exit 0
fi

log_info "macOS SDK: $(xcrun --sdk macosx --show-sdk-path)"
log_info "iOS Simulator SDK: $(xcrun --sdk iphonesimulator --show-sdk-path)"
log_info "Available iOS Simulators:"
xcrun simctl list devices available | grep -E "iPhone|iPad" | head -n 10 || true
