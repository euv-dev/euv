/// Cross-platform fullscreen presentation mode.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum FullscreenMode {
    /// Standard windowed mode.
    #[default]
    Windowed,
    /// Borderless desktop fullscreen or mobile edge-to-edge immersive mode.
    Borderless,
    /// Exclusive hardware fullscreen mode where supported by the OS.
    Exclusive,
}

/// Mobile and tablet screen orientation constraint.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum OrientationLock {
    /// Follow device sensor / user multitasking orientation.
    #[default]
    Auto,
    /// Lock to portrait orientation.
    Portrait,
    /// Lock to landscape orientation.
    Landscape,
}

/// Mobile / iPadOS system status bar appearance style.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum StatusBarStyle {
    /// Follow active application theme.
    #[default]
    Auto,
    /// Light status bar icons for dark backgrounds.
    LightContent,
    /// Dark status bar icons for light backgrounds.
    DarkContent,
    /// Hide system status bar (immersive fullscreen).
    Hidden,
}
