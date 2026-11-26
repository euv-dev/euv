use super::*;

/// Fluent per-frame parameter bundle for `Renderer::render_with`.
#[derive(Clone, Copy, Data, Debug, PartialEq)]
pub struct FrameRenderParams {
    /// Logical viewport size.
    #[get(type(copy))]
    pub(crate) viewport: Size,
    /// Active physical DPI scale factor.
    #[get(type(copy))]
    pub(crate) scale_factor: f32,
    /// Surface background clear color.
    #[get(type(copy))]
    pub(crate) clear_color: Color,
    /// Layout pass telemetry from the current frame.
    #[get(type(copy))]
    pub(crate) layout_stats: LayoutStats,
    /// Damaged screen region requiring rasterization.
    #[get(type(copy))]
    pub(crate) damage: DamageRegion,
}

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
    /// Whether to compute full-framebuffer FNV-1a pixel hashes on each frame (enabled for headless parity tests).
    #[get(type(copy))]
    pub(crate) compute_pixel_hash: bool,
    /// Telemetry from the most recent frame.
    #[get(type(copy))]
    pub(crate) last_stats: FrameStats,
}
