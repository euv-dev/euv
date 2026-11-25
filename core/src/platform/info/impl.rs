use super::*;

/// Metadata queries for `PlatformKind`.
impl PlatformKind {
    /// Returns the canonical lowercase name of this platform.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Platform kind.
    ///
    /// # Returns
    ///
    /// - `&'static str` - Canonical platform name string.
    #[must_use]
    pub const fn as_str(&self) -> &'static str {
        match *self {
            Self::Web => PLATFORM_NAME_WEB,
            Self::Android => PLATFORM_NAME_ANDROID,
            Self::Ios => PLATFORM_NAME_IOS,
            Self::IpadOs => PLATFORM_NAME_IPADOS,
            Self::MacOs => PLATFORM_NAME_MACOS,
            Self::Windows => PLATFORM_NAME_WINDOWS,
            Self::Linux => PLATFORM_NAME_LINUX,
            Self::Headless => PLATFORM_NAME_HEADLESS,
        }
    }

    /// Returns the default `DeviceFormFactor` associated with this platform.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Platform kind.
    ///
    /// # Returns
    ///
    /// - `DeviceFormFactor` - Device form factor.
    #[must_use]
    pub const fn form_factor(&self) -> DeviceFormFactor {
        match *self {
            Self::MacOs | Self::Windows | Self::Linux => DeviceFormFactor::Desktop,
            Self::IpadOs => DeviceFormFactor::Tablet,
            Self::Ios | Self::Android => DeviceFormFactor::Phone,
            Self::Web => DeviceFormFactor::Browser,
            Self::Headless => DeviceFormFactor::Headless,
        }
    }

    /// Returns true for mobile or tablet touch-first platforms (`Android`, `Ios`, `IpadOs`).
    ///
    /// # Arguments
    ///
    /// - `&Self` - Platform kind.
    ///
    /// # Returns
    ///
    /// - `bool` - True for mobile/tablet targets.
    #[must_use]
    pub const fn is_mobile_or_tablet(&self) -> bool {
        matches!(*self, Self::Android | Self::Ios | Self::IpadOs)
    }

    /// Returns the list of all 7 production platforms supported by euv.
    ///
    /// # Returns
    ///
    /// - `[Self; 7]` - Array containing Web, Android, iOS, iPadOS, macOS, Windows, and Linux.
    #[must_use]
    pub const fn all_targets() -> [Self; 7] {
        [
            Self::Web,
            Self::Android,
            Self::Ios,
            Self::IpadOs,
            Self::MacOs,
            Self::Windows,
            Self::Linux,
        ]
    }
}

/// Capability matrix factory for `PlatformCapabilities`.
impl PlatformCapabilities {
    /// Returns the canonical capability matrix for `platform`.
    ///
    /// # Arguments
    ///
    /// - `PlatformKind` - Target platform.
    ///
    /// # Returns
    ///
    /// - `Self` - Normalized platform capabilities.
    #[must_use]
    pub fn for_platform(platform: PlatformKind) -> Self {
        match platform {
            PlatformKind::MacOs | PlatformKind::Windows | PlatformKind::Linux => Self {
                multi_window: true,
                hover_pointer: true,
                touch: false,
                stylus: false,
                hardware_keyboard: true,
                safe_area_insets: false,
                hardware_gpu: true,
                software_blit: true,
                clipboard: true,
                ime: true,
            },
            PlatformKind::IpadOs => Self {
                multi_window: true,
                hover_pointer: true,
                touch: true,
                stylus: true,
                hardware_keyboard: true,
                safe_area_insets: true,
                hardware_gpu: true,
                software_blit: true,
                clipboard: true,
                ime: true,
            },
            PlatformKind::Ios | PlatformKind::Android => Self {
                multi_window: false,
                hover_pointer: false,
                touch: true,
                stylus: true,
                hardware_keyboard: false,
                safe_area_insets: true,
                hardware_gpu: true,
                software_blit: true,
                clipboard: true,
                ime: true,
            },
            PlatformKind::Web => Self {
                multi_window: false,
                hover_pointer: true,
                touch: true,
                stylus: true,
                hardware_keyboard: true,
                safe_area_insets: true,
                hardware_gpu: true,
                software_blit: true,
                clipboard: true,
                ime: true,
            },
            PlatformKind::Headless => Self {
                multi_window: true,
                hover_pointer: true,
                touch: true,
                stylus: false,
                hardware_keyboard: true,
                safe_area_insets: true,
                hardware_gpu: false,
                software_blit: true,
                clipboard: true,
                ime: true,
            },
        }
    }
}
