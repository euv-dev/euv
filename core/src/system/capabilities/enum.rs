/// Active presentation surface for iOS Dynamic Island / Live Activities and cross-platform fallbacks.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum IslandPresentationMode {
    /// iOS Dynamic Island compact pill (leading + trailing views).
    Compact,
    /// iOS Dynamic Island minimal circular badge when multiple live activities compete.
    Minimal,
    /// iOS Dynamic Island expanded full-width card on long press.
    Expanded,
    /// iOS / iPadOS Lock Screen Live Activity banner.
    LockScreen,
    /// Android Live Updates / Ongoing Notification or desktop toast fallback.
    FallbackNotification,
    /// macOS Menu Bar, Windows Taskbar badge, Linux StatusNotifier, or Web title pill fallback.
    #[default]
    FallbackStatusBar,
}

/// Tactile haptic feedback waveform normalized across iOS, iPadOS, Android, macOS, and Web.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum HapticStyle {
    /// Light subtle tap (`UIImpactFeedbackGenerator.light`).
    #[default]
    Light,
    /// Standard medium impact (`UIImpactFeedbackGenerator.medium`).
    Medium,
    /// Strong heavy impact (`UIImpactFeedbackGenerator.heavy`).
    Heavy,
    /// Crisp selection tick (`UISelectionFeedbackGenerator`).
    Selection,
    /// Positive completion confirmation (`UINotificationFeedbackGenerator.success`).
    Success,
    /// Cautionary warning pulse.
    Warning,
    /// Error double-pulse feedback.
    Error,
}

/// Hardware biometric authentication provider exposed by the active platform.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum BiometricKind {
    /// No biometric hardware available.
    #[default]
    None,
    /// Apple Face ID (iPhone / iPad Pro).
    FaceId,
    /// Apple Touch ID (Mac / iPad).
    TouchId,
    /// Android `BiometricPrompt` fingerprint / face class.
    Fingerprint,
    /// Windows Hello (`UserConsentVerifier`).
    WindowsHello,
    /// WebAuthn / FIDO2 Passkey authenticator.
    Passkey,
}
