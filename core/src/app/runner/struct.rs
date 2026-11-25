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
}
