#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
# shellcheck source=common.sh
source "${SCRIPT_DIR}/common.sh"

ANDROID_API_LEVEL="${EUV_ANDROID_API_LEVEL:-35}"
ANDROID_BUILD_TOOLS="${EUV_ANDROID_BUILD_TOOLS:-35.0.0}"
ANDROID_NDK_VERSION="${EUV_ANDROID_NDK_VERSION:-27.2.12479018}"
ANDROID_AVD_NAME="${EUV_ANDROID_AVD_NAME:-euv_android_${ANDROID_API_LEVEL}}"
INSTALL_EMULATOR="${EUV_ANDROID_INSTALL_EMULATOR:-1}"

HOST_OS="$(detect_os)"
HOST_ARCH="$(detect_arch)"
ANDROID_HOME_DIR="$(resolve_default_android_home)"

log_info "Provisioning Android environment (${HOST_OS}/${HOST_ARCH}) -> ${ANDROID_HOME_DIR}"

if JAVA_FOUND="$(resolve_java_home)"; then
  export JAVA_HOME="${JAVA_FOUND}"
  export PATH="${JAVA_HOME}/bin:${PATH}"
  log_info "Using JAVA_HOME=${JAVA_HOME}"
else
  if [ "${HOST_OS}" = "macos" ] && command -v brew >/dev/null 2>&1; then
    log_info "Installing OpenJDK 17 via Homebrew..."
    brew install openjdk@17
    JAVA_FOUND="$(resolve_java_home)"
    export JAVA_HOME="${JAVA_FOUND}"
    export PATH="${JAVA_HOME}/bin:${PATH}"
  else
    log_error "Java 17+ runtime not found. Install Android Studio JBR or OpenJDK 17."
    exit 1
  fi
fi

mkdir -p "${ANDROID_HOME_DIR}/cmdline-tools"

SDKMANAGER="${ANDROID_HOME_DIR}/cmdline-tools/latest/bin/sdkmanager"
if [ ! -x "${SDKMANAGER}" ]; then
  case "${HOST_OS}" in
    macos) CMDLINE_ZIP="commandlinetools-mac-11076708_latest.zip" ;;
    linux) CMDLINE_ZIP="commandlinetools-linux-11076708_latest.zip" ;;
    *)
      log_error "Unsupported OS for automated cmdline-tools download: ${HOST_OS}"
      exit 1
      ;;
  esac
  TMP_DIR="$(mktemp -d)"
  trap 'rm -rf "${TMP_DIR}"' EXIT
  URL="https://dl.google.com/android/repository/${CMDLINE_ZIP}"
  log_info "Downloading Android command-line tools from ${URL}..."
  curl -fsSL "${URL}" -o "${TMP_DIR}/cmdline-tools.zip"
  unzip -q "${TMP_DIR}/cmdline-tools.zip" -d "${TMP_DIR}/unzipped"
  rm -rf "${ANDROID_HOME_DIR}/cmdline-tools/latest"
  mv "${TMP_DIR}/unzipped/cmdline-tools" "${ANDROID_HOME_DIR}/cmdline-tools/latest"
  chmod +x "${ANDROID_HOME_DIR}/cmdline-tools/latest/bin/"*
fi

export ANDROID_HOME="${ANDROID_HOME_DIR}"
export ANDROID_SDK_ROOT="${ANDROID_HOME_DIR}"
export PATH="${ANDROID_HOME}/cmdline-tools/latest/bin:${ANDROID_HOME}/platform-tools:${ANDROID_HOME}/emulator:${PATH}"

log_info "Accepting Android SDK licenses..."
set +o pipefail
yes | "${SDKMANAGER}" --sdk_root="${ANDROID_HOME}" --licenses >/dev/null 2>&1 || true
set -o pipefail

PACKAGES=(
  "platform-tools"
  "platforms;android-${ANDROID_API_LEVEL}"
  "build-tools;${ANDROID_BUILD_TOOLS}"
  "ndk;${ANDROID_NDK_VERSION}"
)

if [ "${INSTALL_EMULATOR}" = "1" ]; then
  if [ "${HOST_ARCH}" = "arm64" ]; then
    SYS_IMG="system-images;android-${ANDROID_API_LEVEL};google_apis;arm64-v8a"
  else
    SYS_IMG="system-images;android-${ANDROID_API_LEVEL};google_apis;x86_64"
  fi
  PACKAGES+=("emulator" "${SYS_IMG}")
fi

log_info "Installing Android SDK packages: ${PACKAGES[*]}"
"${SDKMANAGER}" --sdk_root="${ANDROID_HOME}" "${PACKAGES[@]}"

NDK_DIR="${ANDROID_HOME}/ndk/${ANDROID_NDK_VERSION}"
case "${HOST_OS}" in
  macos) NDK_HOST_TAG="darwin-x86_64" ;;
  linux) NDK_HOST_TAG="linux-x86_64" ;;
  *) NDK_HOST_TAG="darwin-x86_64" ;;
esac
NDK_LLVM_BIN="${NDK_DIR}/toolchains/llvm/prebuilt/${NDK_HOST_TAG}/bin"
ANDROID_LINKER="${NDK_LLVM_BIN}/aarch64-linux-android${ANDROID_API_LEVEL}-clang"

if [ ! -x "${ANDROID_LINKER}" ]; then
  ANDROID_LINKER="$(find "${NDK_LLVM_BIN}" -maxdepth 1 -name 'aarch64-linux-android*-clang' | sort -V | tail -n 1)"
fi

if command -v rustup >/dev/null 2>&1; then
  log_info "Ensuring Rust target aarch64-linux-android is installed..."
  rustup target add aarch64-linux-android
fi

if [ "${INSTALL_EMULATOR}" = "1" ] && [ -x "${ANDROID_HOME}/cmdline-tools/latest/bin/avdmanager" ]; then
  if ! "${ANDROID_HOME}/cmdline-tools/latest/bin/avdmanager" list avd -c 2>/dev/null | grep -qx "${ANDROID_AVD_NAME}"; then
    log_info "Creating Android Virtual Device '${ANDROID_AVD_NAME}' (${SYS_IMG})..."
    echo "no" | "${ANDROID_HOME}/cmdline-tools/latest/bin/avdmanager" create avd \
      --force \
      --name "${ANDROID_AVD_NAME}" \
      --package "${SYS_IMG}" \
      --device "pixel_6"
  else
    log_info "AVD '${ANDROID_AVD_NAME}' already exists."
  fi
fi

cat > "${ENV_LOCAL_FILE}" <<EOF
#!/usr/bin/env bash
export JAVA_HOME="${JAVA_HOME}"
export ANDROID_HOME="${ANDROID_HOME}"
export ANDROID_SDK_ROOT="${ANDROID_HOME}"
export ANDROID_NDK_HOME="${NDK_DIR}"
export ANDROID_API_LEVEL="${ANDROID_API_LEVEL}"
export ANDROID_BUILD_TOOLS="${ANDROID_BUILD_TOOLS}"
export ANDROID_AVD_NAME="${ANDROID_AVD_NAME}"
export CARGO_TARGET_AARCH64_LINUX_ANDROID_LINKER="${ANDROID_LINKER}"
export CC_aarch64_linux_android="${ANDROID_LINKER}"
export AR_aarch64_linux_android="${NDK_LLVM_BIN}/llvm-ar"
export PATH="${JAVA_HOME}/bin:${ANDROID_HOME}/cmdline-tools/latest/bin:${ANDROID_HOME}/platform-tools:${ANDROID_HOME}/build-tools/${ANDROID_BUILD_TOOLS}:${ANDROID_HOME}/emulator:${NDK_LLVM_BIN}:\${PATH}"
EOF
chmod +x "${ENV_LOCAL_FILE}"

log_info "Wrote local environment profile to ${ENV_LOCAL_FILE}"
log_info "adb version: $("${ANDROID_HOME}/platform-tools/adb" version | head -n 1)"
log_info "Android environment setup complete."
