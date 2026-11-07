#!/usr/bin/env bash

set -euo pipefail

PLATFORM="${1:-android}"
MODE="${2:-release}"

RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

info()  { echo -e "${GREEN}[INFO]${NC} $*"; }
warn()  { echo -e "${YELLOW}[WARN]${NC} $*"; }
error() { echo -e "${RED}[ERROR]${NC} $*"; exit 1; }
step()  { echo -e "${BLUE}[STEP]${NC} $*"; }

cd "$(dirname "$0")"
PROJECT_ROOT="$(pwd)"

CONFIG_FILE="app.config.json"
if [ ! -f "$CONFIG_FILE" ]; then
    error "Config file not found: $CONFIG_FILE"
fi

APP_NAME=$(node -e "console.log(require('./$CONFIG_FILE').app.name)")
APP_VERSION=$(node -e "console.log(require('./$CONFIG_FILE').app.version)")
APP_ID=$(node -e "console.log(require('./$CONFIG_FILE').app.identifier)")

info "App: $APP_NAME v$APP_VERSION ($APP_ID)"

case "$PLATFORM" in
    android)
        ;;
    release|debug)
        MODE="$PLATFORM"
        PLATFORM="android"
        ;;
    -h|--help|help)
        echo "Usage: $0 [platform] [mode]"
        echo ""
        echo "Platform:"
        echo "  android     Build Android APK (default)"
        echo ""
        echo "Mode:"
        echo "  release     Signed release package (default)"
        echo "  debug       Debug package"
        exit 0
        ;;
    *)
        error "Unknown platform: $PLATFORM (only android)"
        ;;
esac

case "$MODE" in
    release|debug)
        ;;
    *)
        error "Unknown mode: $MODE (only release or debug)"
        ;;
esac

command -v cargo >/dev/null 2>&1 || error "cargo not found, please install Rust toolchain"

# Fresh-clone bootstrap: `tauri android build` compiles libeuv_lib.so per
# ABI, but rustup only installs the host target by default. Add the Android
# targets here — `rustup target add` is idempotent and skips installed ones.
if command -v rustup >/dev/null 2>&1; then
    ANDROID_RUST_TARGETS=(aarch64-linux-android armv7-linux-androideabi i686-linux-android x86_64-linux-android)
    INSTALLED_TARGETS=$(rustup target list --installed)
    MISSING_TARGETS=()
    for TARGET in "${ANDROID_RUST_TARGETS[@]}"; do
        if ! grep -qx "$TARGET" <<< "$INSTALLED_TARGETS"; then
            MISSING_TARGETS+=("$TARGET")
        fi
    done
    if [ "${#MISSING_TARGETS[@]}" -gt 0 ]; then
        step "Installing missing Rust Android targets: ${MISSING_TARGETS[*]}"
        rustup target add "${MISSING_TARGETS[@]}"
    fi
fi

# Best-effort nvm: images like GitHub-hosted runners ship nvm with NO
# installed node versions, where `nvm use` exits 3 and (with --silent)
# kills the script with no message. Never let nvm abort the build — the
# real gate is the npx check below (setup-node/PATH-provided node is fine).
if [ -f "$HOME/.nvm/nvm.sh" ]; then
    source "$HOME/.nvm/nvm.sh"
    nvm use 20 --silent 2>/dev/null || nvm use node --silent 2>/dev/null || true
fi
command -v npx >/dev/null 2>&1 || error "npx not found, please install Node.js"

# Fresh-clone bootstrap: install npm dependencies once. Without
# node_modules, `npx @tauri-apps/cli` re-downloads the CLI on every build
# and may silently resolve a different version.
if [ ! -d "$PROJECT_ROOT/node_modules" ]; then
    step "node_modules not found, installing npm dependencies..."
    if [ -f "$PROJECT_ROOT/package-lock.json" ]; then
        # `npm ci` is deterministic, but older lockfiles miss the CLI's
        # platform-specific optional deps and abort with EUSAGE — fall
        # back to a regular install in that case.
        npm ci || npm install
    else
        npm install
    fi
fi

step "Applying config to platform files..."
node scripts/apply-config.js

info "Starting Android $MODE build..."

case "$(uname -s)" in
    Darwin*)  HOST_OS="mac" ;;
    Linux*)   HOST_OS="linux" ;;
    MINGW*|MSYS*|CYGWIN*) HOST_OS="win" ;;
    *)        HOST_OS="unknown" ;;
esac

export JAVA_HOME="$PROJECT_ROOT/sdk/jdk"
if [ ! -x "$JAVA_HOME/bin/java" ] && [ ! -x "$JAVA_HOME/bin/java.exe" ]; then
    info "JDK not found in sdk/jdk/, auto-downloading..."
    bash "$PROJECT_ROOT/scripts/setup-sdk.sh" jdk
fi
if [ ! -x "$JAVA_HOME/bin/java" ] && [ ! -x "$JAVA_HOME/bin/java.exe" ]; then
    error "JDK auto-download failed. Run ./scripts/setup-sdk.sh jdk manually."
fi
info "Using JDK: sdk/jdk/"
export PATH="$JAVA_HOME/bin:$PATH"
JAVA_HOME_WIN=$(cygpath -w "$JAVA_HOME" 2>/dev/null || echo "$JAVA_HOME")

export ANDROID_HOME="$PROJECT_ROOT/sdk/android"
if [ ! -d "$ANDROID_HOME/platforms" ]; then
    info "Android SDK not found in sdk/android/, auto-downloading..."
    bash "$PROJECT_ROOT/scripts/setup-sdk.sh" android
fi
if [ ! -d "$ANDROID_HOME/platforms" ]; then
    error "Android SDK auto-download failed. Run ./scripts/setup-sdk.sh android manually."
fi
info "Using Android SDK: sdk/android/"

if [ -z "${NDK_HOME:-}" ] && [ -d "$ANDROID_HOME/ndk" ]; then
    NDK_VER=$(ls "$ANDROID_HOME/ndk" 2>/dev/null | sort -V | tail -1)
    if [ -n "$NDK_VER" ]; then
        export NDK_HOME="$ANDROID_HOME/ndk/$NDK_VER"
    fi
fi
[ -n "${NDK_HOME:-}" ] || error "Android NDK not found, install via sdkmanager"

ANDROID_HOME_WIN=$(cygpath -w "$ANDROID_HOME" 2>/dev/null || echo "$ANDROID_HOME")
echo "sdk.dir=${ANDROID_HOME_WIN//\\/\\\\}" > src-tauri/gen/android/local.properties
info "JAVA_HOME: $JAVA_HOME"
info "ANDROID_HOME: $ANDROID_HOME"
info "NDK_HOME: $NDK_HOME"

info "Node: $(node --version), Java: $(java -version 2>&1 | head -1)"

BUILD_START=$(date +%s)

GENERATED_DIR="src-tauri/gen/android/app/src/main/java/com/euv/generated"
BACKUP_FILE="/tmp/euv_RustWebViewClient_backup.kt"
BACKUP_FILE_WV="/tmp/euv_RustWebView_backup.kt"
# These two files carry local customizations and are committed to git, but
# Tauri regenerates `generated/` on every build — back them up so we can
# restore them afterwards. Guards keep a fresh clone (where the files exist
# in git) and any legacy checkout (where they may not) both working.
if [ -f "$GENERATED_DIR/RustWebViewClient.kt" ]; then
    cp "$GENERATED_DIR/RustWebViewClient.kt" "$BACKUP_FILE"
fi
if [ -f "$GENERATED_DIR/RustWebView.kt" ]; then
    cp "$GENERATED_DIR/RustWebView.kt" "$BACKUP_FILE_WV"
fi

if [ "$MODE" = "release" ]; then
    npx @tauri-apps/cli android build --apk 2>&1 || true
else
    npx @tauri-apps/cli android build --apk --debug 2>&1 || true
fi

if [ -f "$BACKUP_FILE" ]; then
    cp "$BACKUP_FILE" "$GENERATED_DIR/RustWebViewClient.kt"
fi
if [ -f "$BACKUP_FILE_WV" ]; then
    cp "$BACKUP_FILE_WV" "$GENERATED_DIR/RustWebView.kt"
fi
info "Restored custom RustWebViewClient.kt and RustWebView.kt"

LIB_NAME="libeuv_lib.so"
JNI_BASE_DIR="src-tauri/gen/android/app/src/main/jniLibs"
declare -A ARCH_MAP=(
    ["arm64-v8a"]="aarch64-linux-android"
    ["armeabi-v7a"]="armv7-linux-androideabi"
    ["x86"]="i686-linux-android"
    ["x86_64"]="x86_64-linux-android"
)
for ARCH in "${!ARCH_MAP[@]}"; do
    RUST_TARGET="${ARCH_MAP[$ARCH]}"
    JNI_DIR="$JNI_BASE_DIR/$ARCH"
    TARGET_SO="$JNI_DIR/$LIB_NAME"
    RUST_SO="src-tauri/target/$RUST_TARGET/$MODE/libeuv_lib.so"
    rm -f "$TARGET_SO"
    if [ -f "$RUST_SO" ]; then
        cp "$RUST_SO" "$TARGET_SO"
        info "Copied $ARCH $LIB_NAME from Rust target"
    else
        warn "$ARCH .so file not found at: $RUST_SO — APK will be missing native lib for $ARCH"
    fi
done

info "Running Gradle build..."
ANDROID_DIR="src-tauri/gen/android"
if [ "$MODE" = "release" ]; then
    "$ANDROID_DIR/gradlew" --project-dir "$ANDROID_DIR" -Dorg.gradle.java.home="$JAVA_HOME_WIN" assembleUniversalRelease
else
    "$ANDROID_DIR/gradlew" --project-dir "$ANDROID_DIR" -Dorg.gradle.java.home="$JAVA_HOME_WIN" assembleUniversalDebug
fi

BUILD_END=$(date +%s)
BUILD_DURATION=$((BUILD_END - BUILD_START))

APK_DIR="src-tauri/gen/android/app/build/outputs/apk"
app_lower=$(printf '%s' "$APP_NAME" | tr '[:upper:]' '[:lower:]')
if [ "$MODE" = "release" ]; then
    APK_PATH="$APK_DIR/universal/release/app-universal-release.apk"
    OUTPUT_NAME="${app_lower}.apk"
else
    APK_PATH="$APK_DIR/universal/debug/app-universal-debug.apk"
    OUTPUT_NAME="${app_lower}-debug.apk"
fi

if [ -f "$APK_PATH" ]; then
    cp "$APK_PATH" "$OUTPUT_NAME"
    APK_SIZE=$(du -h "$OUTPUT_NAME" | cut -f1)
    info "Android build complete! Duration: ${BUILD_DURATION}s"
    info "Output: ./$OUTPUT_NAME"
    info "Size: $APK_SIZE"

    if command -v adb >/dev/null 2>&1; then
        DEVICE_COUNT=$(adb devices | grep -c -w 'device' || true)
        if [ "$DEVICE_COUNT" -gt 0 ]; then
            info "Device detected, installing..."
            adb install -r "$OUTPUT_NAME"
            info "Install complete!"
        else
            warn "No connected device detected, skipping install"
        fi
    else
        warn "adb not found, skipping auto-install"
    fi
else
    warn "Build may have completed, but APK not found at expected path"
    find "$APK_DIR" -name "*.apk" 2>/dev/null | while read -r apk; do
        info "  Found: $apk"
    done
fi

echo ""
info "Build complete!"
