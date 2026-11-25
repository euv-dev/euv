/// Rust target triple for WebAssembly browser builds.
pub const TRIPLE_WEB: &str = "wasm32-unknown-unknown";
/// Rust target triple for Android ARM64 NDK builds.
pub const TRIPLE_ANDROID: &str = "aarch64-linux-android";
/// Rust target triple for iOS and iPadOS ARM64 device builds.
pub const TRIPLE_IOS: &str = "aarch64-apple-ios";
/// Rust target triple for macOS Apple Silicon native builds.
pub const TRIPLE_MACOS: &str = "aarch64-apple-darwin";
/// Rust target triple for Windows x86_64 MSVC native builds.
pub const TRIPLE_WINDOWS: &str = "x86_64-pc-windows-msvc";
/// Rust target triple for Linux x86_64 GNU native builds.
pub const TRIPLE_LINUX: &str = "x86_64-unknown-linux-gnu";

pub(crate) const NAME_WEB: &str = "web";
pub(crate) const NAME_ANDROID: &str = "android";
pub(crate) const NAME_IOS: &str = "ios";
pub(crate) const NAME_IPAD: &str = "ipad";
pub(crate) const NAME_MACOS: &str = "macos";
pub(crate) const NAME_WINDOWS: &str = "windows";
pub(crate) const NAME_LINUX: &str = "linux";

pub(crate) const API_WEB: &str = "DOM Canvas / WebGPU / WebGL2";
pub(crate) const API_ANDROID: &str = "NDK ANativeWindow / Vulkan / GLES";
pub(crate) const API_IOS: &str = "UIKit / CoreGraphics / Metal";
pub(crate) const API_IPAD: &str = "UIKit / Stage Manager / Pencil / Metal";
pub(crate) const API_MACOS: &str = "AppKit / CoreGraphics / Metal";
pub(crate) const API_WINDOWS: &str = "Win32 / GDI / Direct3D 12";
pub(crate) const API_LINUX: &str = "dlopen Wayland & X11 / Vulkan";

pub(crate) const ARTIFACT_WEB: &str = "dist/web/euv_app.wasm";
pub(crate) const ARTIFACT_ANDROID: &str = "dist/android/arm64-v8a/libeuv_app.so";
pub(crate) const ARTIFACT_IOS: &str = "dist/ios/EuvApp.app";
pub(crate) const ARTIFACT_IPAD: &str = "dist/ipad/EuvApp.app";
pub(crate) const ARTIFACT_MACOS: &str = "dist/macos/EuvApp.app";
pub(crate) const ARTIFACT_WINDOWS: &str = "dist/windows/euv_app.exe";
pub(crate) const ARTIFACT_LINUX: &str = "dist/linux/euv_app";

pub(crate) const BIN_CARGO: &str = "cargo";
pub(crate) const ARG_BUILD: &str = "build";
pub(crate) const ARG_TARGET: &str = "--target";
pub(crate) const ARG_RELEASE: &str = "--release";
pub(crate) const STEP_COMPILE_DESC: &str = "Compile native Rust binary via Cargo";
pub(crate) const STEP_PACKAGE_DESC: &str = "Package direct system bundle without WebView";
pub(crate) const BIN_WASM_PACK: &str = "wasm-pack";
pub(crate) const ARG_TARGET_WEB: &str = "web";
pub(crate) const ARG_OUT_DIR: &str = "--out-dir";
pub(crate) const OUT_DIR_WEB: &str = "dist/web";
