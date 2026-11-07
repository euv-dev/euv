#!/usr/bin/env bash
#
# EUV SDK Setup Script
# Downloads JDK and Android SDK into the project-local sdk/ directory.
# Usage:
#   ./scripts/setup-sdk.sh           Install both JDK and Android SDK
#   ./scripts/setup-sdk.sh jdk       Install JDK only
#   ./scripts/setup-sdk.sh android   Install Android SDK only

set -euo pipefail

RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

info()  { echo -e "${GREEN}[INFO]${NC} $*"; }
warn()  { echo -e "${YELLOW}[WARN]${NC} $*"; }
error() { echo -e "${RED}[ERROR]${NC} $*"; exit 1; }

PROJECT_ROOT="$(cd "$(dirname "$0")/.." && pwd)"
SDK_DIR="$PROJECT_ROOT/sdk"
JDK_DIR="$SDK_DIR/jdk"
ANDROID_DIR="$SDK_DIR/android"

# JDK version and download URL
JDK_VERSION="21.0.8"
JDK_VENDOR="temurin"

detect_os_arch() {
    local os arch
    case "$(uname -s)" in
        Darwin*)  os="mac" ;;
        Linux*)   os="linux" ;;
        MINGW*|MSYS*|CYGWIN*) os="win" ;;
        *)        os="unknown" ;;
    esac
    case "$(uname -m)" in
        x86_64|amd64) arch="x64" ;;
        aarch64|arm64) arch="aarch64" ;;
        *) arch="unknown" ;;
    esac
    echo "$os $arch"
}

download_jdk() {
    info "Setting up JDK $JDK_VERSION..."

    if [ -d "$JDK_DIR" ] && [ -x "$JDK_DIR/bin/java" ] || [ -x "$JDK_DIR/bin/java.exe" ]; then
        local ver
        ver=$("$JDK_DIR/bin/java" -version 2>&1 | head -1 || "$JDK_DIR/bin/java.exe" -version 2>&1 | head -1 || true)
        info "JDK already installed at sdk/jdk/: $ver"
        return 0
    fi

    read -r host_os host_arch <<< "$(detect_os_arch)"

    local url suffix
    case "$host_os" in
        mac)
            if [ "$host_arch" = "aarch64" ]; then
                suffix="aarch64"
            else
                suffix="x64"
            fi
            url="https://api.adoptium.net/v3/binary/latest/${JDK_VERSION%%.*}/ga/${host_os}/${suffix}/jdk/hotspot/normal/eclipse"
            ;;
        linux)
            url="https://api.adoptium.net/v3/binary/latest/${JDK_VERSION%%.*}/ga/linux/${host_arch}/jdk/hotspot/normal/eclipse"
            ;;
        win)
            url="https://api.adoptium.net/v3/binary/latest/${JDK_VERSION%%.*}/ga/windows/${host_arch}/jdk/hotspot/normal/eclipse"
            ;;
        *)
            error "Unsupported OS: $host_os"
            ;;
    esac

    info "Downloading JDK from Adoptium (Temurin)..."
    local tmp_dir
    tmp_dir=$(mktemp -d)

    CURL_OPTS=(-fSL)
    if [ "$host_os" = "win" ]; then
        CURL_OPTS+=(--ssl-no-revoke)
    fi
    if command -v curl >/dev/null 2>&1; then
        curl "${CURL_OPTS[@]}" -o "$tmp_dir/jdk.zip" "$url"
    elif command -v wget >/dev/null 2>&1; then
        wget -O "$tmp_dir/jdk.zip" "$url"
    else
        error "Neither curl nor wget found. Please install one."
    fi

    info "Extracting JDK..."
    mkdir -p "$JDK_DIR"
    if [ "$host_os" = "win" ]; then
        unzip -q "$tmp_dir/jdk.zip" -d "$tmp_dir/jdk-extract"
        local jdk_inner
        jdk_inner=$(ls -d "$tmp_dir/jdk-extract"/jdk-* 2>/dev/null | head -1)
        if [ -n "$jdk_inner" ]; then
            cp -r "$jdk_inner"/. "$JDK_DIR/"
        else
            cp -r "$tmp_dir/jdk-extract"/. "$JDK_DIR/"
        fi
    elif [ "$host_os" = "mac" ]; then
        unzip -q "$tmp_dir/jdk.zip" -d "$tmp_dir/jdk-extract"
        local jdk_inner
        jdk_inner=$(ls -d "$tmp_dir/jdk-extract"/*.jdk/Contents/Home 2>/dev/null | head -1)
        if [ -n "$jdk_inner" ]; then
            cp -r "$jdk_inner"/. "$JDK_DIR/"
        else
            jdk_inner=$(ls -d "$tmp_dir/jdk-extract"/jdk-* 2>/dev/null | head -1)
            if [ -n "$jdk_inner" ]; then
                cp -r "$jdk_inner"/. "$JDK_DIR/"
            else
                cp -r "$tmp_dir/jdk-extract"/. "$JDK_DIR/"
            fi
        fi
    else
        tar xzf "$tmp_dir/jdk.zip" -C "$tmp_dir"
        local jdk_inner
        jdk_inner=$(ls -d "$tmp_dir"/jdk-* 2>/dev/null | head -1)
        if [ -n "$jdk_inner" ]; then
            cp -r "$jdk_inner"/. "$JDK_DIR/"
        else
            cp -r "$tmp_dir"/. "$JDK_DIR/"
        fi
    fi

    rm -rf "$tmp_dir"

    if [ -x "$JDK_DIR/bin/java" ] || [ -x "$JDK_DIR/bin/java.exe" ]; then
        info "JDK installed successfully at sdk/jdk/"
        local ver
        ver=$("$JDK_DIR/bin/java" -version 2>&1 | head -1 || "$JDK_DIR/bin/java.exe" -version 2>&1 | head -1 || true)
        info "Version: $ver"
    else
        error "JDK installation failed. Check sdk/jdk/ directory."
    fi
}

download_android_sdk() {
    info "Setting up Android SDK..."

    if [ -d "$ANDROID_DIR" ] && [ -d "$ANDROID_DIR/platforms" ]; then
        info "Android SDK already installed at sdk/android/"
        return 0
    fi

    read -r host_os host_arch <<< "$(detect_os_arch)"

    local cmdtools_url
    case "$host_os" in
        mac)  cmdtools_url="https://dl.google.com/android/repository/commandlinetools-mac-13114758_latest.zip" ;;
        linux) cmdtools_url="https://dl.google.com/android/repository/commandlinetools-linux-13114758_latest.zip" ;;
        win)  cmdtools_url="https://dl.google.com/android/repository/commandlinetools-win-13114758_latest.zip" ;;
        *) error "Unsupported OS: $host_os" ;;
    esac

    info "Downloading Android command-line tools..."
    local tmp_dir
    tmp_dir=$(mktemp -d)

    CURL_OPTS=(-fSL)
    if [ "$host_os" = "win" ]; then
        CURL_OPTS+=(--ssl-no-revoke)
    fi
    if command -v curl >/dev/null 2>&1; then
        curl "${CURL_OPTS[@]}" -o "$tmp_dir/cmdtools.zip" "$cmdtools_url"
    elif command -v wget >/dev/null 2>&1; then
        wget -O "$tmp_dir/cmdtools.zip" "$cmdtools_url"
    else
        error "Neither curl nor wget found. Please install one."
    fi

    info "Extracting command-line tools..."
    mkdir -p "$ANDROID_DIR/cmdline-tools"
    unzip -q "$tmp_dir/cmdtools.zip" -d "$tmp_dir/cmdtools-extract"
    mv "$tmp_dir/cmdtools-extract/cmdline-tools" "$ANDROID_DIR/cmdline-tools/latest"
    rm -rf "$tmp_dir"

    local sdkmanager
    if [ "$host_os" = "win" ]; then
        sdkmanager="$ANDROID_DIR/cmdline-tools/latest/bin/sdkmanager.bat"
    else
        sdkmanager="$ANDROID_DIR/cmdline-tools/latest/bin/sdkmanager"
        chmod +x "$sdkmanager"
    fi

    if [ ! -f "$sdkmanager" ]; then
        error "sdkmanager not found at $sdkmanager"
    fi

    info "Accepting Android SDK licenses..."
    yes | "$sdkmanager" --licenses >/dev/null 2>&1 || true

    info "Installing required SDK components (this may take a while)..."
    "$sdkmanager" \
        "platforms;android-36" \
        "build-tools;36.0.0" \
        "ndk;28.0.13004108" \
        "platform-tools"

    info "Android SDK installed successfully at sdk/android/"
}

WHAT="${1:-all}"

case "$WHAT" in
    jdk)
        download_jdk
        ;;
    android)
        download_android_sdk
        ;;
    all)
        download_jdk
        download_android_sdk
        ;;
    *)
        echo "Usage: $0 [all|jdk|android]"
        exit 1
        ;;
esac

echo ""
info "Setup complete!"
info "  JDK:       sdk/jdk/"
info "  Android:   sdk/android/"
info "Run ./build.sh to start building."
