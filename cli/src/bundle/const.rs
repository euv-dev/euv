/// The host operating system identifier used by the native builders.
///
/// The values match the `process.platform` strings Node reports, so the
/// host a plan is resolved against is named the same way on both sides
/// of the boundary.
pub(crate) const HOST_MACOS: &str = "macos";
pub(crate) const HOST_LINUX: &str = "linux";
pub(crate) const HOST_WINDOWS: &str = "windows";

/// The host operating system this binary was compiled for.
pub(crate) const HOST_OS: &str = if cfg!(target_os = "macos") {
    HOST_MACOS
} else if cfg!(target_os = "linux") {
    HOST_LINUX
} else if cfg!(target_os = "windows") {
    HOST_WINDOWS
} else {
    HOST_UNSUPPORTED
};

/// The host identifier used when the operating system has no native
/// builder in this repository.
pub(crate) const HOST_UNSUPPORTED: &str = "unsupported";

/// The host architecture this binary was compiled for.
pub(crate) const HOST_ARCH: &str = if cfg!(target_arch = "aarch64") {
    ARCH_ARM64
} else if cfg!(target_arch = "x86_64") {
    ARCH_X64
} else {
    ARCH_UNSUPPORTED
};

/// The host architecture identifier used when the processor is not one
/// the native builders support.
pub(crate) const ARCH_UNSUPPORTED: &str = "unsupported";

/// The 64-bit ARM architecture identifier.
pub(crate) const ARCH_ARM64: &str = "arm64";

/// The 64-bit Intel architecture identifier.
pub(crate) const ARCH_X64: &str = "x64";

/// The environment variable carrying the web build output directory into
/// the native builders.
pub(crate) const ENV_WEB_DIST: &str = "EUV_WEB_DIST";

/// The environment variable selecting the native client build mode.
pub(crate) const ENV_NATIVE_MODE: &str = "EUV_NATIVE_MODE";

/// The native client build mode that produces release artifacts.
pub(crate) const NATIVE_MODE_RELEASE: &str = "release";

/// The relative path of the native client project inside the repository.
pub(crate) const NATIVE_PROJECT_RELATIVE_PATH: &str = "native/tauri";

/// The command that runs the per-platform native client builder.
pub(crate) const NODE_COMMAND: &str = "node";

/// The command that runs the Android build script.
pub(crate) const BASH_COMMAND: &str = "bash";

/// The project-relative path of the Android build script.
pub(crate) const ANDROID_BUILD_SCRIPT_PATH: &str = "scripts/build/build.sh";

/// The scripts-relative path of the per-platform native client builder.
pub(crate) const BUILD_CLIENT_SCRIPT_PATH: &str = "build/build-client.js";

/// The Android platform name used by the native builders.
pub(crate) const PLATFORM_ANDROID: &str = "android";

/// The macOS platform name used by the native builders.
pub(crate) const PLATFORM_MACOS: &str = "macos";

/// The Linux platform name used by the native builders.
pub(crate) const PLATFORM_LINUX: &str = "linux";

/// The Windows platform name used by the native builders.
pub(crate) const PLATFORM_WINDOWS: &str = "windows";

/// The iOS device platform name used by the native builders.
pub(crate) const PLATFORM_IOS: &str = "ios";

/// The iOS simulator platform name used by the native builders.
pub(crate) const PLATFORM_IOS_SIMULATOR: &str = "ios-simulator";

/// The argument that selects the native platforms to build.
pub(crate) const PLATFORMS_ARG: &str = "--platforms";

/// The argument that selects every host-compatible native deliverable.
pub(crate) const ALL_ARG: &str = "--all";

/// The argument that selects the native client project directory.
pub(crate) const NATIVE_PATH_ARG: &str = "--native-path";

/// The argument that prints the plan without building anything.
pub(crate) const PLAN_ARG: &str = "--plan";
