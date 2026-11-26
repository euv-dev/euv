use super::*;

/// Detects the compile-time target `PlatformKind` for the current binary.
///
/// # Returns
///
/// - `PlatformKind` - Active platform kind (`iPadOS` is refined dynamically on iOS via `UIDevice`).
#[must_use]
pub fn detect_current_platform() -> PlatformKind {
    #[cfg(target_arch = "wasm32")]
    {
        PlatformKind::Web
    }
    #[cfg(target_os = "android")]
    {
        PlatformKind::Android
    }
    #[cfg(target_os = "ios")]
    {
        ios_detect_platform_kind()
    }
    #[cfg(target_os = "macos")]
    {
        PlatformKind::MacOs
    }
    #[cfg(target_os = "windows")]
    {
        PlatformKind::Windows
    }
    #[cfg(target_os = "linux")]
    {
        PlatformKind::Linux
    }
    #[cfg(not(any(target_arch = "wasm32", unix, windows)))]
    {
        PlatformKind::Headless
    }
}

/// Instantiates the native `PlatformBackend` for the active compile target.
///
/// # Returns
///
/// - `Box<dyn PlatformBackend>` - Ready-to-use native OS or web platform backend.
#[must_use]
pub fn create_native_platform() -> Box<dyn PlatformBackend> {
    #[cfg(target_arch = "wasm32")]
    {
        Box::new(WebPlatform::default())
    }
    #[cfg(target_os = "android")]
    {
        Box::new(AndroidPlatform::default())
    }
    #[cfg(target_os = "ios")]
    {
        Box::new(IosPlatform::default())
    }
    #[cfg(target_os = "macos")]
    {
        Box::new(MacOsPlatform::default())
    }
    #[cfg(target_os = "windows")]
    {
        Box::new(WindowsPlatform::default())
    }
    #[cfg(target_os = "linux")]
    {
        Box::new(LinuxPlatform::default())
    }
    #[cfg(not(any(target_arch = "wasm32", unix, windows)))]
    {
        Box::new(HeadlessPlatform::default())
    }
}
