use super::*;

/// Canonical cross-platform application lifecycle state tracked identically on all 7 targets.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum AppState {
    /// Application is visible in the foreground and receiving interactive input.
    #[default]
    Active,
    /// Application is transitioning, occluded, or unfocused while still partially visible.
    Inactive,
    /// Application is hidden or suspended in the background.
    Background,
    /// Application is terminating or has shut down.
    Terminated,
}

/// Normalized device or window screen orientation across mobile, tablet, desktop, and web.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum ScreenOrientation {
    /// Standard upright portrait orientation (`height >= width`).
    #[default]
    Portrait,
    /// Standard horizontal landscape orientation (`width > height`).
    Landscape,
    /// Upside-down portrait orientation.
    PortraitUpsideDown,
    /// Landscape orientation rotated counter-clockwise.
    LandscapeLeft,
    /// Landscape orientation rotated clockwise.
    LandscapeRight,
}

/// Normalized operating system memory pressure severity.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum MemoryPressure {
    /// Healthy memory headroom.
    #[default]
    Normal,
    /// Moderate OS memory trim request; release non-essential caches.
    Warning,
    /// Critical OS low-memory alert; release all regeneratable caches immediately.
    Critical,
}

/// Normalized network reachability and transport classification.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum NetworkStatus {
    /// Network connectivity is available (generic online transport).
    #[default]
    Online,
    /// Device has no active network route.
    Offline,
    /// Connected over Wi-Fi / local wireless LAN.
    Wifi,
    /// Connected over cellular mobile data.
    Cellular,
    /// Connectivity state could not be determined by the host OS.
    Unknown,
}

/// Unified cross-platform system and lifecycle event smoothing all 7-platform OS differences.
#[derive(Clone, Debug, PartialEq)]
pub enum SystemEvent {
    /// Active window or routed page became visually presented.
    Appear,
    /// Active window or routed page ceased to be visually presented.
    Disappear,
    /// Application returned to active foreground execution.
    Foreground,
    /// Application moved into suspended/hidden background execution.
    Background,
    /// Application lost active interaction status (e.g. control center, app switcher, or deactivation).
    Inactive,
    /// Application is terminating gracefully.
    Terminate,
    /// Window keyboard/input focus state changed.
    FocusChanged(bool),
    /// Viewport logical dimensions or DPI scale factor changed.
    Resize {
        /// Logical viewport width in device-independent pixels.
        width: f32,
        /// Logical viewport height in device-independent pixels.
        height: f32,
        /// Display DPI scale factor.
        scale_factor: f32,
    },
    /// Safe-area insets changed (notch, Dynamic Island, system bars, Stage Manager).
    SafeAreaChanged(SafeAreaInsets),
    /// Device or window orientation changed.
    OrientationChanged(ScreenOrientation),
    /// Host OS/browser light or dark theme preference changed.
    ThemeChanged(SystemTheme),
    /// Host OS/browser preferred BCP-47 locale changed.
    LocaleChanged(String),
    /// Soft virtual keyboard / IME visibility or covered height changed.
    KeyboardVisibilityChanged {
        /// Whether the soft virtual keyboard is currently shown.
        visible: bool,
        /// Keyboard height in logical pixels (`0.0` when hidden).
        height: f32,
    },
    /// Operating system signaled memory pressure.
    MemoryWarning(MemoryPressure),
    /// Operating system toggled battery saver / low-power mode.
    LowPowerModeChanged(bool),
    /// Host network connectivity status changed.
    NetworkStatusChanged(NetworkStatus),
    /// Hardware/gesture system back navigation was triggered.
    BackPressed,
    /// Host OS delivered a custom-scheme or universal deep link URI.
    DeepLink(String),
    /// System clipboard content changed.
    ClipboardChanged,
}
