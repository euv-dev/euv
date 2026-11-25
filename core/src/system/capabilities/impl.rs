use super::*;

/// Safe optional accessors for `LiveActivityContent`.
impl LiveActivityContent {
    /// Returns the optional progress percentage in `0..=100`.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Live activity content reference.
    ///
    /// # Returns
    ///
    /// - `Option<u8>` - Clamped progress percentage, if set.
    #[must_use]
    pub fn progress(&self) -> Option<u8> {
        let Self {
            progress_percent, ..
        } = self;
        progress_percent.map(|v: u8| v.min(100))
    }
}

/// Safe optional accessors for `SystemNotification`.
impl SystemNotification {
    /// Returns the optional app icon badge count.
    ///
    /// # Arguments
    ///
    /// - `&Self` - System notification reference.
    ///
    /// # Returns
    ///
    /// - `Option<u32>` - Badge count if specified.
    #[must_use]
    pub fn badge(&self) -> Option<u32> {
        let Self { badge_count, .. } = self;
        *badge_count
    }
}

/// Platform-aware Dynamic Island, permissions, haptics, tray, sharing, and A11y dispatch for `PlatformServiceHub`.
impl PlatformServiceHub {
    /// Resolves the optimal live-activity presentation surface for `platform`.
    ///
    /// # Arguments
    ///
    /// - `PlatformKind` - Target execution platform.
    ///
    /// # Returns
    ///
    /// - `IslandPresentationMode` - Native Dynamic Island or graceful platform fallback mode.
    #[must_use]
    pub const fn default_island_mode(platform: PlatformKind) -> IslandPresentationMode {
        match platform {
            PlatformKind::Ios => IslandPresentationMode::Compact,
            PlatformKind::IpadOs => IslandPresentationMode::LockScreen,
            PlatformKind::Android => IslandPresentationMode::FallbackNotification,
            PlatformKind::MacOs
            | PlatformKind::Windows
            | PlatformKind::Linux
            | PlatformKind::Web
            | PlatformKind::Headless => IslandPresentationMode::FallbackStatusBar,
        }
    }

    /// Starts a Dynamic Island / Live Activity session on iOS/iPadOS or a graceful fallback on other platforms.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable service hub.
    /// - `String` - Unique activity identifier.
    /// - `LiveActivityContent` - Declarative compact, minimal, and expanded content payload.
    ///
    /// # Returns
    ///
    /// - `LiveActivityHandle` - Handle describing the active live activity and resolved presentation mode.
    pub fn start_live_activity(
        &mut self,
        activity_id: String,
        content: LiveActivityContent,
    ) -> LiveActivityHandle {
        let platform: PlatformKind = self.get_platform();
        let mode: IslandPresentationMode = Self::default_island_mode(platform);
        let handle: LiveActivityHandle =
            LiveActivityHandle::new(activity_id.clone(), platform, mode, content, true);
        self.get_mut_live_activities()
            .retain(|existing: &LiveActivityHandle| existing.get_activity_id() != &activity_id);
        self.get_mut_live_activities().push(handle.clone());
        handle
    }

    /// Updates an active Dynamic Island / Live Activity in place.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable service hub.
    /// - `&str` - Activity identifier.
    /// - `LiveActivityContent` - Updated content payload.
    ///
    /// # Returns
    ///
    /// - `bool` - `true` when an active session matching `activity_id` was updated.
    pub fn update_live_activity(
        &mut self,
        activity_id: &str,
        content: LiveActivityContent,
    ) -> bool {
        for item in self.get_mut_live_activities().iter_mut() {
            if item.get_activity_id() == activity_id && item.get_active() {
                item.set_content(content);
                return true;
            }
        }
        false
    }

    /// Ends an active Dynamic Island / Live Activity session.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable service hub.
    /// - `&str` - Activity identifier.
    ///
    /// # Returns
    ///
    /// - `bool` - `true` when an active session was terminated.
    pub fn end_live_activity(&mut self, activity_id: &str) -> bool {
        for item in self.get_mut_live_activities().iter_mut() {
            if item.get_activity_id() == activity_id && item.get_active() {
                item.set_active(false);
                return true;
            }
        }
        false
    }

    /// Triggers a tactile haptic pulse when supported by the active platform.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable service hub.
    /// - `HapticStyle` - Requested tactile waveform.
    ///
    /// # Returns
    ///
    /// - `bool` - `true` when dispatched on a haptic-capable platform.
    pub fn trigger_haptic(&mut self, style: HapticStyle) -> bool {
        let _: HapticStyle = style;
        let caps: PlatformCapabilities = PlatformCapabilities::for_platform(self.get_platform());
        if !caps.get_haptics() {
            return false;
        }
        let next: u64 = self.get_haptic_count().saturating_add(1);
        self.set_haptic_count(next);
        true
    }

    /// Returns the native biometric authentication provider available on the active platform.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Service hub reference.
    ///
    /// # Returns
    ///
    /// - `BiometricKind` - Platform biometric authenticator kind.
    #[must_use]
    pub fn biometric_kind(&self) -> BiometricKind {
        match self.get_platform() {
            PlatformKind::Ios | PlatformKind::IpadOs => BiometricKind::FaceId,
            PlatformKind::MacOs => BiometricKind::TouchId,
            PlatformKind::Android => BiometricKind::Fingerprint,
            PlatformKind::Windows => BiometricKind::WindowsHello,
            PlatformKind::Web => BiometricKind::Passkey,
            PlatformKind::Linux | PlatformKind::Headless => BiometricKind::None,
        }
    }

    /// Installs or updates the desktop system tray / menu bar configuration.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable service hub.
    /// - `SystemTrayConfig` - Tray icon tooltip, title, and menu items.
    ///
    /// # Returns
    ///
    /// - `Result<(), PlatformError>` - `Ok(())` on desktop targets, or `Unsupported` on mobile/Web.
    pub fn configure_tray(&mut self, config: SystemTrayConfig) -> Result<(), PlatformError> {
        let caps: PlatformCapabilities = PlatformCapabilities::for_platform(self.get_platform());
        if !caps.get_system_tray() {
            return Err(PlatformError::Unsupported(format!(
                "System tray is unsupported on {}",
                self.get_platform().as_str()
            )));
        }
        self.set_tray(Some(config));
        Ok(())
    }

    /// Returns the active desktop tray configuration, if installed.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Service hub reference.
    ///
    /// # Returns
    ///
    /// - `Option<&SystemTrayConfig>` - Configured tray metadata.
    #[must_use]
    pub fn tray_config(&self) -> Option<&SystemTrayConfig> {
        let Self { tray, .. } = self;
        tray.as_ref()
    }

    /// Dispatches a native OS notification and records it in telemetry.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable service hub.
    /// - `SystemNotification` - Notification payload.
    pub fn send_notification(&mut self, notification: SystemNotification) {
        self.get_mut_notifications().push(notification);
    }

    /// Queues a screen reader accessibility live announcement.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable service hub.
    /// - `String` - Message text to announce.
    /// - `A11yLiveRegion` - Politeness priority.
    pub fn announce_a11y(&mut self, message: String, politeness: A11yLiveRegion) {
        self.get_mut_announcements()
            .push(A11yAnnouncement::new(message, politeness));
    }

    /// Resolves the native OS share sheet API descriptor for `payload`.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Service hub reference.
    /// - `&SharePayload` - Share sheet title, text, and optional URL.
    ///
    /// # Returns
    ///
    /// - `Result<String, PlatformError>` - Native OS share dispatcher description.
    pub fn share(&self, payload: &SharePayload) -> Result<String, PlatformError> {
        let api: &str = match self.get_platform() {
            PlatformKind::Ios | PlatformKind::IpadOs => "UIActivityViewController",
            PlatformKind::MacOs => "NSSharingServicePicker",
            PlatformKind::Android => "android.content.Intent.ACTION_SEND",
            PlatformKind::Windows => "Windows.ApplicationModel.DataTransfer.DataTransferManager",
            PlatformKind::Linux => "org.freedesktop.portal.OpenURI",
            PlatformKind::Web | PlatformKind::Headless => "navigator.share",
        };
        Ok(format!("{api}: {}", payload.get_title()))
    }
}
