use super::*;

/// Unique handle identifying an OS or web window managed by a platform backend.
#[derive(Clone, Copy, Debug, Default, Eq, Getter, Hash, New, Ord, PartialEq, PartialOrd)]
pub struct WindowId {
    /// Monotonic numeric window identifier.
    #[get(type(copy))]
    pub(crate) value: u64,
}

/// Safe-area insets in logical pixels (iOS notch/Dynamic Island, iPadOS Stage Manager, Android cutouts, Web immersive).
#[derive(Clone, Copy, Data, Debug, Default, New, PartialEq)]
pub struct SafeAreaInsets {
    /// Top safe inset (status bar / notch / Dynamic Island).
    #[get(type(copy))]
    pub(crate) top: f32,
    /// Right safe inset (landscape camera cutout / gesture bar).
    #[get(type(copy))]
    pub(crate) right: f32,
    /// Bottom safe inset (home indicator / gesture pill).
    #[get(type(copy))]
    pub(crate) bottom: f32,
    /// Left safe inset (landscape cutout).
    #[get(type(copy))]
    pub(crate) left: f32,
}

/// Physical monitor or mobile screen metadata.
#[derive(Clone, Copy, Data, Debug, New, PartialEq)]
pub struct DisplayInfo {
    /// Physical screen width in pixels.
    #[get(type(copy))]
    pub(crate) physical_width: u32,
    /// Physical screen height in pixels.
    #[get(type(copy))]
    pub(crate) physical_height: u32,
    /// Scale factor mapping logical pixels to physical pixels (DPI / HiDPI).
    #[get(type(copy))]
    pub(crate) scale_factor: f32,
    /// Nominal refresh rate in hertz.
    #[get(type(copy))]
    pub(crate) refresh_rate_hz: u32,
    /// Active display color gamut.
    #[get(type(copy))]
    pub(crate) color_space: ColorSpace,
}

/// Unified declarative window creation settings normalized across desktop, mobile, tablet, and web.
#[derive(Clone, Data, Debug, PartialEq)]
pub struct WindowConfig {
    /// Window or browser document title.
    pub(crate) title: String,
    /// Initial logical width in pixels.
    #[get(type(copy))]
    pub(crate) width: u32,
    /// Initial logical height in pixels.
    #[get(type(copy))]
    pub(crate) height: u32,
    /// Optional minimum logical `(width, height)`.
    #[get(skip)]
    pub(crate) min_size: Option<(u32, u32)>,
    /// Optional maximum logical `(width, height)`.
    #[get(skip)]
    pub(crate) max_size: Option<(u32, u32)>,
    /// Optional initial screen `(x, y)` position on desktop platforms.
    #[get(skip)]
    pub(crate) position: Option<(i32, i32)>,
    /// Whether the user can resize the window.
    #[get(type(copy))]
    pub(crate) resizable: bool,
    /// Whether OS window decorations (title bar / borders) are shown.
    #[get(type(copy))]
    pub(crate) decorations: bool,
    /// Whether transparent framebuffer compositing is requested.
    #[get(type(copy))]
    pub(crate) transparent: bool,
    /// Whether the window floats above standard windows.
    #[get(type(copy))]
    pub(crate) always_on_top: bool,
    /// Fullscreen / edge-to-edge presentation mode.
    #[get(type(copy))]
    pub(crate) fullscreen: FullscreenMode,
    /// Whether mobile/web edge-to-edge immersive safe-area mode is active.
    #[get(type(copy))]
    pub(crate) immersive: bool,
    /// Whether window is initially visible.
    #[get(type(copy))]
    pub(crate) visible: bool,
    /// Requested rendering pipeline configuration.
    #[get(type(copy))]
    pub(crate) render: RenderConfig,
    /// Mobile / tablet orientation lock policy.
    #[get(type(copy))]
    pub(crate) orientation: OrientationLock,
    /// Mobile / iPadOS status bar appearance.
    #[get(type(copy))]
    pub(crate) status_bar: StatusBarStyle,
    /// Preferred initial theme mode override.
    #[get(skip)]
    pub(crate) preferred_theme: Option<ThemeMode>,
    /// Optional decoded RGBA8 application/window icon bitmap for native chrome and mobile bundles.
    #[get(skip)]
    pub(crate) icon: Option<ImageBuffer>,
    /// Optional path to a PNG/SVG application icon asset consumed by desktop/iOS/Android packagers.
    #[get(skip)]
    pub(crate) icon_path: Option<String>,
}

/// Live runtime metrics and state of an active window.
#[derive(Clone, Copy, Data, Debug, PartialEq)]
pub struct WindowState {
    /// Window handle.
    #[get(type(copy))]
    pub(crate) id: WindowId,
    /// Current logical width in pixels.
    #[get(type(copy))]
    pub(crate) logical_width: f32,
    /// Current logical height in pixels.
    #[get(type(copy))]
    pub(crate) logical_height: f32,
    /// Current physical framebuffer width in pixels.
    #[get(type(copy))]
    pub(crate) physical_width: u32,
    /// Current physical framebuffer height in pixels.
    #[get(type(copy))]
    pub(crate) physical_height: u32,
    /// Current DPI scale factor.
    #[get(type(copy))]
    pub(crate) scale_factor: f32,
    /// Current safe-area insets in logical pixels.
    #[get(type(copy))]
    pub(crate) safe_area: SafeAreaInsets,
    /// Active display metadata.
    #[get(type(copy))]
    pub(crate) display: DisplayInfo,
    /// Whether window currently has OS input focus.
    #[get(type(copy))]
    pub(crate) focused: bool,
    /// Whether window is minimized or backgrounded.
    #[get(type(copy))]
    pub(crate) minimized: bool,
    /// Whether window is maximized.
    #[get(type(copy))]
    pub(crate) maximized: bool,
    /// Active fullscreen mode.
    #[get(type(copy))]
    pub(crate) fullscreen: FullscreenMode,
    /// Active OS dark/light appearance.
    #[get(type(copy))]
    pub(crate) system_theme: ThemeMode,
}
