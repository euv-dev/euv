use super::*;

/// Native OS manifest key and support queries for `SystemPermission`.
impl SystemPermission {
    /// Returns the canonical OS permission identifier for `platform`.
    ///
    /// # Arguments
    ///
    /// - `&Self` - System permission category.
    /// - `PlatformKind` - Target execution platform.
    ///
    /// # Returns
    ///
    /// - `&'static str` - Native Info.plist key, Android manifest permission, WinRT capability, XDG portal, or Web API name.
    #[must_use]
    pub const fn native_key(&self, platform: PlatformKind) -> &'static str {
        match platform {
            PlatformKind::MacOs | PlatformKind::Ios | PlatformKind::IpadOs => match *self {
                Self::Camera => "NSCameraUsageDescription",
                Self::Microphone => "NSMicrophoneUsageDescription",
                Self::LocationWhenInUse => "NSLocationWhenInUseUsageDescription",
                Self::LocationAlways => "NSLocationAlwaysAndWhenInUseUsageDescription",
                Self::Notifications => "UNUserNotificationCenter.requestAuthorization",
                Self::Photos => "NSPhotoLibraryUsageDescription",
                Self::Contacts => "NSContactsUsageDescription",
                Self::Calendar => "NSCalendarsUsageDescription",
                Self::Bluetooth => "NSBluetoothAlwaysUsageDescription",
                Self::Biometrics => "NSFaceIDUsageDescription",
                Self::FileStorage => "NSDocumentsFolderUsageDescription",
                Self::ClipboardRead => "UIPasteboard.general",
                Self::MotionSensors => "NSMotionUsageDescription",
                Self::LocalNetwork => "NSLocalNetworkUsageDescription",
            },
            PlatformKind::Android => match *self {
                Self::Camera => "android.permission.CAMERA",
                Self::Microphone => "android.permission.RECORD_AUDIO",
                Self::LocationWhenInUse => "android.permission.ACCESS_FINE_LOCATION",
                Self::LocationAlways => "android.permission.ACCESS_BACKGROUND_LOCATION",
                Self::Notifications => "android.permission.POST_NOTIFICATIONS",
                Self::Photos => "android.permission.READ_MEDIA_IMAGES",
                Self::Contacts => "android.permission.READ_CONTACTS",
                Self::Calendar => "android.permission.READ_CALENDAR",
                Self::Bluetooth => "android.permission.BLUETOOTH_CONNECT",
                Self::Biometrics => "android.permission.USE_BIOMETRIC",
                Self::FileStorage => "android.permission.READ_EXTERNAL_STORAGE",
                Self::ClipboardRead => "android.content.ClipboardManager",
                Self::MotionSensors => "android.permission.BODY_SENSORS",
                Self::LocalNetwork => "android.permission.NEARBY_WIFI_DEVICES",
            },
            PlatformKind::Windows => match *self {
                Self::Camera => "winrt.capability.webcam",
                Self::Microphone => "winrt.capability.microphone",
                Self::LocationWhenInUse | Self::LocationAlways => "winrt.capability.location",
                Self::Notifications => "winrt.capability.userNotificationListener",
                Self::Photos => "winrt.capability.picturesLibrary",
                Self::Contacts => "winrt.capability.contacts",
                Self::Calendar => "winrt.capability.appointments",
                Self::Bluetooth => "winrt.capability.bluetooth",
                Self::Biometrics => "Windows.Security.Credentials.UI.UserConsentVerifier",
                Self::FileStorage => "winrt.capability.broadFileSystemAccess",
                Self::ClipboardRead => "Windows.ApplicationModel.DataTransfer.Clipboard",
                Self::MotionSensors => "winrt.capability.activity",
                Self::LocalNetwork => "winrt.capability.privateNetworkClientServer",
            },
            PlatformKind::Linux => match *self {
                Self::Camera => "org.freedesktop.portal.Camera",
                Self::Microphone => "org.freedesktop.portal.GlobalShortcuts",
                Self::LocationWhenInUse | Self::LocationAlways => "org.freedesktop.portal.Location",
                Self::Notifications => "org.freedesktop.portal.Notification",
                Self::Photos | Self::FileStorage => "org.freedesktop.portal.FileChooser",
                Self::Contacts | Self::Calendar => "org.freedesktop.portal.Secret",
                Self::Bluetooth => "org.bluez.Adapter1",
                Self::Biometrics => "net.reactivated.Fprint.Device",
                Self::ClipboardRead => "org.freedesktop.portal.Clipboard",
                Self::MotionSensors => "net.hadess.SensorProxy",
                Self::LocalNetwork => "org.freedesktop.Avahi",
            },
            PlatformKind::Web | PlatformKind::Headless => match *self {
                Self::Camera => "camera",
                Self::Microphone => "microphone",
                Self::LocationWhenInUse | Self::LocationAlways => "geolocation",
                Self::Notifications => "notifications",
                Self::Photos | Self::FileStorage => "persistent-storage",
                Self::Contacts => "contacts",
                Self::Calendar => "calendar",
                Self::Bluetooth => "bluetooth",
                Self::Biometrics => "publickey-credentials-get",
                Self::ClipboardRead => "clipboard-read",
                Self::MotionSensors => "accelerometer",
                Self::LocalNetwork => "local-network-access",
            },
        }
    }

    /// Returns whether `platform` natively supports this permission category.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Permission category.
    /// - `PlatformKind` - Target execution platform.
    ///
    /// # Returns
    ///
    /// - `bool` - `true` when supported on `platform`.
    #[must_use]
    pub const fn is_supported_on(&self, platform: PlatformKind) -> bool {
        match platform {
            PlatformKind::Headless => !matches!(*self, Self::MotionSensors),
            PlatformKind::Linux => !matches!(*self, Self::MotionSensors),
            PlatformKind::Web => !matches!(*self, Self::LocationAlways | Self::Calendar),
            PlatformKind::MacOs
            | PlatformKind::Windows
            | PlatformKind::Ios
            | PlatformKind::IpadOs
            | PlatformKind::Android => true,
        }
    }
}

/// Permission inspection and authorization request helpers for `PermissionManager`.
impl PermissionManager {
    /// Checks the current status of `permission` on `platform`.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Permission manager reference.
    /// - `SystemPermission` - Permission category to query.
    /// - `PlatformKind` - Active platform kind.
    ///
    /// # Returns
    ///
    /// - `PermissionStatus` - Resolved authorization status.
    #[must_use]
    pub fn check(&self, permission: SystemPermission, platform: PlatformKind) -> PermissionStatus {
        if !permission.is_supported_on(platform) {
            return PermissionStatus::Unsupported;
        }
        self.get_statuses()
            .get(&permission)
            .copied()
            .unwrap_or(PermissionStatus::NotDetermined)
    }

    /// Records an OS-resolved `PermissionStatus` update for `permission`.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable permission manager.
    /// - `SystemPermission` - Target permission category.
    /// - `PermissionStatus` - New authorization status.
    pub fn set_status(&mut self, permission: SystemPermission, status: PermissionStatus) {
        let _: Option<PermissionStatus> = self.get_mut_statuses().insert(permission, status);
    }

    /// Initiates a cross-platform permission request with a privacy rationale and returns its `PermissionDescriptor`.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable permission manager.
    /// - `SystemPermission` - Permission category to request.
    /// - `String` - Human-readable usage rationale shown in native permission prompts.
    /// - `PlatformKind` - Active platform kind.
    ///
    /// # Returns
    ///
    /// - `PermissionDescriptor` - Complete native key, rationale, and resulting status.
    pub fn request(
        &mut self,
        permission: SystemPermission,
        rationale: String,
        platform: PlatformKind,
    ) -> PermissionDescriptor {
        let native_key: String = permission.native_key(platform).to_string();
        let _: Option<String> = self
            .get_mut_rationales()
            .insert(permission, rationale.clone());
        let status: PermissionStatus = if !permission.is_supported_on(platform) {
            PermissionStatus::Unsupported
        } else {
            let current: PermissionStatus = self.check(permission, platform);
            if current == PermissionStatus::NotDetermined {
                self.set_status(permission, PermissionStatus::Granted);
                PermissionStatus::Granted
            } else {
                current
            }
        };
        PermissionDescriptor::new(permission, platform, native_key, rationale, status)
    }
}
