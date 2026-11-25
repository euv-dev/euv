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
}
