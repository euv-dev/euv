use super::*;

/// Simulated window record held by `HeadlessPlatform`.
#[derive(Clone, Data, Debug)]
pub struct HeadlessWindowRecord {
    /// Current window configuration.
    pub(crate) config: WindowConfig,
    /// Current live window metrics and state.
    #[get(type(copy))]
    pub(crate) state: WindowState,
    /// Active cursor icon hint.
    #[get(type(copy))]
    pub(crate) cursor: CursorIcon,
    /// Last presented RGBA8 framebuffer snapshot.
    pub(crate) last_frame: Framebuffer,
    /// Count of frames presented to this window.
    #[get(type(copy))]
    pub(crate) present_count: u64,
    /// Whether the system soft keyboard / IME is currently active for this window.
    #[get(type(copy))]
    pub(crate) ime_active: bool,
    /// Logical screen rectangle of the active IME input anchor.
    #[get(type(copy))]
    pub(crate) ime_rect: Rect,
}

/// Deterministic offscreen platform backend available on all targets for CI and pixel-parity testing.
#[derive(Clone, Data, Debug)]
pub struct HeadlessPlatform {
    /// Emulated platform identity (defaults to `PlatformKind::Headless`, can emulate any of the 7 targets).
    #[get(type(copy))]
    pub(crate) emulated_kind: PlatformKind,
    /// Simulated DPI scale factor applied to newly created windows.
    #[get(type(copy))]
    pub(crate) scale_factor: f32,
    /// Simulated safe-area insets applied to newly created windows.
    #[get(type(copy))]
    pub(crate) safe_area: SafeAreaInsets,
    /// Simulated OS theme mode.
    #[get(type(copy))]
    pub(crate) theme: ThemeMode,
    /// Simulated display refresh rate in Hertz (`60.0` default; supports `90.0`/`120.0`/`144.0`/`240.0` testing).
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) refresh_rate_hz: f32,
    /// Monotonic window ID allocator.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) next_id: u64,
    /// Active simulated windows indexed by `WindowId`.
    pub(crate) windows: HashMap<WindowId, HeadlessWindowRecord>,
    /// Injected event queue drained on `poll_events()`.
    pub(crate) pending_events: Vec<WindowEvent>,
    /// In-memory clipboard storage.
    #[get(skip)]
    pub(crate) clipboard: Option<String>,
    /// Whether the frame pump is currently paused for background freeze.
    #[get(type(copy))]
    pub(crate) pump_paused: bool,
    /// Count of blocking `wait_for_events()` invocations recorded during background sleep.
    #[get(type(copy))]
    pub(crate) wait_calls: u64,
}
