#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "${SCRIPT_DIR}/../.." && pwd)"
ENV_LOCAL_FILE="${SCRIPT_DIR}/env.sh"

log_info() {
  printf '[euv-env] %s\n' "$*"
}

log_warn() {
  printf '[euv-env][warn] %s\n' "$*" >&2
}

log_error() {
  printf '[euv-env][error] %s\n' "$*" >&2
}

detect_os() {
  case "$(uname -s)" in
    Darwin) echo "macos" ;;
    Linux) echo "linux" ;;
    MINGW*|MSYS*|CYGWIN*) echo "windows" ;;
    *) echo "unknown" ;;
  esac
}

detect_arch() {
  case "$(uname -m)" in
    arm64|aarch64) echo "arm64" ;;
    x86_64|amd64) echo "x86_64" ;;
    *) uname -m ;;
  esac
}

resolve_java_home() {
  if [ -n "${JAVA_HOME:-}" ] && [ -x "${JAVA_HOME}/bin/java" ]; then
    echo "${JAVA_HOME}"
    return 0
  fi
  local candidates=(
    "/Applications/Android Studio.app/Contents/jbr/Contents/Home"
    "/opt/homebrew/opt/openjdk@17/libexec/openjdk.jdk/Contents/Home"
    "/opt/homebrew/opt/openjdk/libexec/openjdk.jdk/Contents/Home"
    "/usr/local/opt/openjdk@17/libexec/openjdk.jdk/Contents/Home"
    "/usr/lib/jvm/java-17-openjdk-amd64"
    "/usr/lib/jvm/java-17-openjdk-arm64"
  )
  local c
  for c in "${candidates[@]}"; do
    if [ -x "${c}/bin/java" ]; then
      echo "${c}"
      return 0
    fi
  done
  if [ "$(detect_os)" = "macos" ] && /usr/libexec/java_home -v 17 >/dev/null 2>&1; then
    /usr/libexec/java_home -v 17
    return 0
  fi
  return 1
}

resolve_default_android_home() {
  if [ -n "${ANDROID_HOME:-}" ]; then
    echo "${ANDROID_HOME}"
  elif [ -n "${ANDROID_SDK_ROOT:-}" ]; then
    echo "${ANDROID_SDK_ROOT}"
  elif [ "$(detect_os)" = "macos" ]; then
    echo "${HOME}/Library/Android/sdk"
  else
    echo "${HOME}/Android/Sdk"
  fi
}

load_local_env() {
  if [ -f "${ENV_LOCAL_FILE}" ]; then
    # shellcheck source=/dev/null
    source "${ENV_LOCAL_FILE}"
  fi
}
