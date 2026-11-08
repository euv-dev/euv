use super::*;

/// Inherent implementation of [`NativePlatform`].
impl NativePlatform {
    /// Returns the platform name the native builders use on the command
    /// line and in their artifact directories.
    ///
    /// # Returns
    ///
    /// - `&'static str` - The platform name.
    pub fn platform_name(self) -> &'static str {
        match self {
            NativePlatform::Macos => PLATFORM_MACOS,
            NativePlatform::Linux => PLATFORM_LINUX,
            NativePlatform::Windows => PLATFORM_WINDOWS,
            NativePlatform::Ios => PLATFORM_IOS,
            NativePlatform::IosSimulator => PLATFORM_IOS_SIMULATOR,
            NativePlatform::Android => PLATFORM_ANDROID,
        }
    }

    /// Returns the host operating system this platform must be built on.
    ///
    /// The Android builder runs on macOS and Linux; the desktop and Apple
    /// builders each require their own host.
    ///
    /// # Returns
    ///
    /// - `&'static str` - The required host identifier.
    pub fn required_host(self) -> &'static str {
        match self {
            NativePlatform::Macos => HOST_MACOS,
            NativePlatform::Linux => HOST_LINUX,
            NativePlatform::Windows => HOST_WINDOWS,
            NativePlatform::Ios => HOST_MACOS,
            NativePlatform::IosSimulator => HOST_MACOS,
            NativePlatform::Android => "macos or linux",
        }
    }
}

/// Inherent implementation of [`HostOs`].
impl HostOs {
    /// Returns the identifier of the host operating system.
    ///
    /// # Returns
    ///
    /// - `&'static str` - The host identifier.
    pub fn host_name(self) -> &'static str {
        match self {
            HostOs::Macos => HOST_MACOS,
            HostOs::Linux => HOST_LINUX,
            HostOs::Windows => HOST_WINDOWS,
            HostOs::Unsupported => HOST_UNSUPPORTED,
        }
    }
}

/// Inherent implementation of [`NativeTask`].
impl NativeTask {
    /// Returns the name of the builder that produces this task's
    /// deliverable.
    ///
    /// # Returns
    ///
    /// - `&'static str` - The builder name.
    pub fn task_name(self) -> &'static str {
        match self {
            NativeTask::Android => "android builder",
            NativeTask::Client => "client builder",
        }
    }
}
