#!/usr/bin/env bash
set -euo pipefail

PLATFORM="${1:-android}"
MODE="${2:-release}"
case "$PLATFORM" in
    release|debug) MODE="$PLATFORM"; PLATFORM=android ;;
    -h|--help|help) printf 'Usage: %s [android] [release|debug]\nBuild example/www first. EUV_WEB_DIST can override its location.\n' "$0"; exit 0 ;;
    android) ;;
    *) printf '[ERROR] Unsupported platform: %s\n' "$PLATFORM" >&2; exit 1 ;;
esac
case "$MODE" in
    release|debug) ;;
    *) printf '[ERROR] Unsupported mode: %s\n' "$MODE" >&2; exit 1 ;;
esac

cd "$(dirname "$0")/../.."
PROJECT_ROOT="$(pwd)"
for TOOL in node npm cargo rustup python3; do
    command -v "$TOOL" >/dev/null 2>&1 || { printf '[ERROR] Required tool missing: %s\n' "$TOOL" >&2; exit 1; }
done

# Fail before downloading toolchains if the local web build is absent.
node scripts/build/bundle-example.js
if [ ! -x node_modules/.bin/tauri ]; then
    npm ci
fi
node scripts/config/apply-config.js
rustup target add aarch64-linux-android

export JAVA_HOME="${JAVA_HOME:-$PROJECT_ROOT/.cache/sdk/jdk}"
if [ ! -x "$JAVA_HOME/bin/java" ]; then
    [ "$JAVA_HOME" = "$PROJECT_ROOT/.cache/sdk/jdk" ] || { printf '[ERROR] Invalid JAVA_HOME: %s\n' "$JAVA_HOME" >&2; exit 1; }
    bash scripts/toolchain/setup-sdk.sh jdk
fi
[ -x "$JAVA_HOME/bin/java" ] || { printf '[ERROR] JDK installation is incomplete\n' >&2; exit 1; }
export PATH="$JAVA_HOME/bin:$PATH"
export ANDROID_HOME="${ANDROID_HOME:-${ANDROID_SDK_ROOT:-$PROJECT_ROOT/.cache/sdk/android}}"
if [ ! -d "$ANDROID_HOME/platforms/android-36" ]; then
    [ "$ANDROID_HOME" = "$PROJECT_ROOT/.cache/sdk/android" ] || { printf '[ERROR] Android SDK platform 36 is missing from %s\n' "$ANDROID_HOME" >&2; exit 1; }
    bash scripts/toolchain/setup-sdk.sh android
fi
export ANDROID_SDK_ROOT="$ANDROID_HOME"
export NDK_HOME="${NDK_HOME:-$ANDROID_HOME/ndk/28.0.13004108}"
[ -d "$NDK_HOME/toolchains/llvm/prebuilt" ] || { printf '[ERROR] Android NDK is missing: %s\n' "$NDK_HOME" >&2; exit 1; }
case "$(uname -s)" in
    Darwin*) NDK_HOST=darwin-x86_64 ;;
    Linux*) NDK_HOST=linux-x86_64 ;;
    *) printf '[ERROR] This build script supports macOS and Linux\n' >&2; exit 1 ;;
esac
NDK_BIN="$NDK_HOME/toolchains/llvm/prebuilt/$NDK_HOST/bin"
export CARGO_TARGET_AARCH64_LINUX_ANDROID_LINKER="$NDK_BIN/aarch64-linux-android24-clang"
export CC_aarch64_linux_android="$CARGO_TARGET_AARCH64_LINUX_ANDROID_LINKER"
export AR_aarch64_linux_android="$NDK_BIN/llvm-ar"
export CARGO_TARGET_DIR="${CARGO_TARGET_DIR:-$PROJECT_ROOT/src-tauri/target}"
CARGO_TARGET_DIR="$(python3 -c 'import os; print(os.path.abspath(os.environ["CARGO_TARGET_DIR"]))')"
export CARGO_TARGET_DIR
# Tauri invokes Gradle itself; disable persistent daemons for both passes.
export GRADLE_OPTS="${GRADLE_OPTS:-} -Dorg.gradle.daemon=false"

ANDROID_DIR="$PROJECT_ROOT/src-tauri/gen/android"
GENERATED_DIR="$ANDROID_DIR/app/src/main/java/com/euv/generated"
BACKUP_DIR="$(mktemp -d "${TMPDIR:-$PROJECT_ROOT/.cache/sdk}/euv-webview.XXXXXX")"
restore_webviews() {
    for FILE in RustWebView.kt RustWebViewClient.kt; do
        if [ -f "$BACKUP_DIR/$FILE" ]; then
            mkdir -p "$GENERATED_DIR"
            cp "$BACKUP_DIR/$FILE" "$GENERATED_DIR/$FILE"
        fi
    done
}
cleanup() {
    RESULT=$?
    trap - EXIT
    restore_webviews
    rm -rf "$BACKUP_DIR"
    exit "$RESULT"
}
trap cleanup EXIT
for FILE in RustWebView.kt RustWebViewClient.kt; do
    [ ! -f "$GENERATED_DIR/$FILE" ] || cp "$GENERATED_DIR/$FILE" "$BACKUP_DIR/$FILE"
done
python3 - "$ANDROID_DIR/local.properties" <<'PY'
import os, pathlib, sys
sdk = os.environ['ANDROID_HOME'].replace('\\', '\\\\').replace(':', '\\:')
pathlib.Path(sys.argv[1]).write_text('sdk.dir=' + sdk + '\n')
PY
APP_NAME="$(node -e 'console.log(require("./config/app.config.json").app.name.toLowerCase())')"
if [ "$MODE" = release ]; then
    OUTPUT_NAME="$PROJECT_ROOT/artifacts/android/$APP_NAME.apk"
    APK_PATH="$ANDROID_DIR/app/build/outputs/apk/universal/release/app-universal-release.apk"
    TASK=assembleUniversalRelease
    CARGO_PROFILE=release
else
    OUTPUT_NAME="$PROJECT_ROOT/artifacts/android/$APP_NAME-debug.apk"
    APK_PATH="$ANDROID_DIR/app/build/outputs/apk/universal/debug/app-universal-debug.apk"
    TASK=assembleUniversalDebug
    CARGO_PROFILE=debug
fi
mkdir -p "$PROJECT_ROOT/artifacts/android"
rm -f "$OUTPUT_NAME" "$APK_PATH"
# Tauri must successfully compile the native library and generate Android glue.
# Never continue after a failed native or initial Gradle build.
if [ "$MODE" = release ]; then
    ./node_modules/.bin/tauri android build --apk --target aarch64 --ci
else
    ./node_modules/.bin/tauri android build --apk --target aarch64 --ci --debug
fi
restore_webviews
NATIVE_LIBRARY="$CARGO_TARGET_DIR/aarch64-linux-android/$CARGO_PROFILE/libeuv_lib.so"
[ -s "$NATIVE_LIBRARY" ] || { printf '[ERROR] Native library is missing: %s\n' "$NATIVE_LIBRARY" >&2; exit 1; }
mkdir -p "$ANDROID_DIR/app/src/main/jniLibs/arm64-v8a"
rm -f "$ANDROID_DIR/app/src/main/jniLibs/arm64-v8a/libeuv_lib.so"
cp "$NATIVE_LIBRARY" "$ANDROID_DIR/app/src/main/jniLibs/arm64-v8a/libeuv_lib.so"
# Package the restored WebView customizations, not Tauri's generated defaults.
"$ANDROID_DIR/gradlew" --no-daemon --project-dir "$ANDROID_DIR" \
    -Dorg.gradle.java.home="$JAVA_HOME" \
    -PabiList=arm64-v8a -ParchList=arm64 -PtargetList=aarch64 "$TASK"
[ -s "$APK_PATH" ] || { printf '[ERROR] Expected APK is missing: %s\n' "$APK_PATH" >&2; exit 1; }
cp "$APK_PATH" "$OUTPUT_NAME"
node scripts/validation/verify-apk.js "$OUTPUT_NAME"
"$ANDROID_HOME/build-tools/36.0.0/apksigner" verify "$OUTPUT_NAME"
printf '[OK] Android %s APK with local example pages: %s\n' "$MODE" "$OUTPUT_NAME"
