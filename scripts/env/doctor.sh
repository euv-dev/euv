#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
# shellcheck source=common.sh
source "${SCRIPT_DIR}/common.sh"
load_local_env

log_info "=== euv Cross-Platform Toolchain Doctor ==="
printf '  os/arch       : %s/%s\n' "$(detect_os)" "$(detect_arch)"
printf '  rustc         : %s\n' "$(rustc --version 2>/dev/null || echo missing)"
printf '  cargo         : %s\n' "$(cargo --version 2>/dev/null || echo missing)"
printf '  wasm-pack     : %s\n' "$(wasm-pack --version 2>/dev/null || echo missing)"
printf '  java          : %s\n' "$(java -version 2>&1 | head -n 1 || echo missing)"
printf '  ANDROID_HOME  : %s\n' "${ANDROID_HOME:-missing}"
printf '  ANDROID_NDK   : %s\n' "${ANDROID_NDK_HOME:-missing}"
printf '  adb           : %s\n' "$(adb version 2>/dev/null | head -n 1 || echo missing)"
if command -v adb >/dev/null 2>&1; then
  log_info "Connected ADB devices:"
  adb devices -l || true
fi
if [ "$(detect_os)" = "macos" ] && command -v xcrun >/dev/null 2>&1; then
  printf '  iOS Sim SDK   : %s\n' "$(xcrun --sdk iphonesimulator --show-sdk-path 2>/dev/null || echo missing)"
fi
