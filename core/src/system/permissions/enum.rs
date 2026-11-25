/// Unified cross-platform operating system privacy and capability permission categories.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum SystemPermission {
    /// Camera / video capture access.
    Camera,
    /// Microphone / audio recording access.
    Microphone,
    /// Foreground precise or approximate location.
    LocationWhenInUse,
    /// Background continuous location access.
    LocationAlways,
    /// Local and push user notifications.
    Notifications,
    /// Photo library and media gallery access.
    Photos,
    /// Address book / contacts read-write access.
    Contacts,
    /// System calendar read-write access.
    Calendar,
    /// Bluetooth Low Energy / peripheral scanning and connection.
    Bluetooth,
    /// Face ID, Touch ID, Android BiometricPrompt, Windows Hello, or WebAuthn passkeys.
    Biometrics,
    /// User-selected or scoped local file system storage.
    FileStorage,
    /// System clipboard read access.
    ClipboardRead,
    /// Accelerometer, gyroscope, and motion/fitness sensors.
    MotionSensors,
    /// Local LAN / Bonjour / mDNS peer discovery.
    LocalNetwork,
}

/// Normalized authorization state returned across iOS, iPadOS, macOS, Android, Windows, Linux, and Web.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum PermissionStatus {
    /// User has not yet been prompted for this permission.
    #[default]
    NotDetermined,
    /// Full authorization granted by the user or OS policy.
    Granted,
    /// Explicitly denied by the user.
    Denied,
    /// Blocked by parental controls, MDM, or OS sandbox policy.
    Restricted,
    /// Partial access granted (e.g. iOS Selected Photos or approximate location).
    Limited,
    /// Hardware or permission API does not exist on the active platform.
    Unsupported,
}
