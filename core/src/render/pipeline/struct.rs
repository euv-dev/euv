use super::*;

/// Unified multi-mode rendering facade orchestrating Auto, Software, Hardware, and Hybrid execution.
#[derive(Data, Debug)]
pub struct Renderer {
    /// Active render configuration.
    #[get(type(copy))]
    pub(crate) config: RenderConfig,
    /// Resolved active render mode (never `Auto`).
    #[get(type(copy))]
    pub(crate) active_mode: RenderMode,
    /// Resolved active graphics backend.
    #[get(type(copy))]
    pub(crate) active_backend: GraphicsBackend,
    /// Dedicated CPU software renderer instance.
    pub(crate) software: SoftwareRenderer,
    /// Dedicated hardware GPU instanced renderer instance.
    pub(crate) hardware: HardwareRenderer,
    /// Dedicated hybrid CPU-tile + GPU-compositor renderer instance.
    pub(crate) hybrid: HybridRenderer,
    /// Reusable frame display list.
    pub(crate) display_list: DisplayList,
    /// Monotonic frame counter.
    #[get(type(copy))]
    pub(crate) frame_index: u64,
    /// Telemetry from the most recent frame.
    #[get(type(copy))]
    pub(crate) last_stats: FrameStats,
}
