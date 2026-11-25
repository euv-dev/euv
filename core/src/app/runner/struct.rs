use super::*;

/// Top-level cross-platform application configuration.
#[derive(Clone, Data, Debug, PartialEq)]
pub struct AppConfig {
    /// Primary window creation settings.
    pub(crate) window: WindowConfig,
    /// Clear background color drawn beneath the root scene node.
    #[get(type(copy))]
    pub(crate) clear_color: Color,
}

/// Unified cross-platform application runtime orchestrating platform events, reactivity, layout, and rendering.
#[derive(CustomDebug, Data)]
pub struct App {
    /// Application configuration.
    pub(crate) config: AppConfig,
    /// Active direct OS or headless platform backend.
    #[debug(skip)]
    #[get(skip)]
    pub(crate) platform: Box<dyn PlatformBackend>,
    /// Primary window live state.
    #[get(type(copy))]
    pub(crate) window_state: WindowState,
    /// Shared scene tree handle.
    pub(crate) scene: SceneHandle,
    /// Build context carrying reactive breakpoint, theme, and safe-area signals.
    pub(crate) build_context: BuildContext,
    /// Deterministic cross-platform layout engine.
    #[get(type(copy))]
    pub(crate) layout_engine: LayoutEngine,
    /// Multi-mode rendering pipeline (`Auto`, `Software`, `Hardware`, `Hybrid`).
    pub(crate) renderer: Renderer,
    /// Queued imperative window commands awaiting dispatch.
    pub(crate) command_queue: WindowCommandQueue,
    /// Local key-value storage service.
    pub(crate) storage: StorageStore,
    /// Flag indicating whether the application loop should keep running.
    #[get(type(copy))]
    pub(crate) running: bool,
    /// Pending damage region for the next rendered frame.
    #[get(type(copy))]
    pub(crate) pending_damage: DamageRegion,
    /// Real-time polled keyboard, pointer, and scroll state for game loops and interactive viewports.
    pub(crate) input_state: InputState,
    /// Fixed-timestep accumulator and FPS telemetry clock for real-time game rendering.
    #[get(type(copy))]
    pub(crate) frame_clock: FrameClock,
    /// Platform-native services hub (permissions, protocol registration, Dynamic Island, haptics, tray, A11y).
    pub(crate) services: PlatformServiceHub,
    /// Reactive signal carrying the most recently received custom-scheme or universal deep link.
    #[get(type(copy))]
    pub(crate) deep_link: Signal<Option<DeepLinkEvent>>,
    /// Shared dynamic update hub for live component-level and page-level OTA updates.
    pub(crate) dynamic_hub: DynamicUpdateHub,
    /// Last scene revision that was rasterized and presented, enabling zero-cost idle frame skipping.
    #[get(type(copy))]
    pub(crate) last_rendered_revision: u64,
    /// Most recently submitted OS/Web cursor icon to avoid redundant platform cursor calls.
    #[get(type(copy))]
    pub(crate) last_cursor: CursorIcon,
    /// Attached real-time game engine hosts receiving automatic OS input forwarding.
    #[debug(skip)]
    pub(crate) game_hosts: Vec<GameEngineHost>,
}
