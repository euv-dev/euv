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
    /// Reusable scene-to-display-list compiler handle.
    #[get(skip)]
    #[get_mut(skip)]
    #[set(skip)]
    pub(crate) paint_pass: PaintPass,
    /// Cross-frame retained `DisplayList` and subtree `DrawCommand` fragment cache.
    #[get(pub(crate))]
    #[get_mut(skip)]
    #[set(skip)]
    pub(crate) paint_fragment_cache: PaintFragmentCache,
    /// Reusable frame display list.
    pub(crate) display_list: DisplayList,
    /// Monotonic frame counter.
    #[get(type(copy))]
    pub(crate) frame_index: u64,
    /// Whether to compute full-framebuffer FNV-1a pixel hashes on each frame (enabled for headless parity tests).
    #[get(type(copy))]
    pub(crate) compute_pixel_hash: bool,
    /// Active render mode of the most recent hashed frame (`None` before first render or after mode switch).
    #[get(skip)]
    #[get_mut(skip)]
    #[set(skip)]
    pub(crate) last_hashed_mode: Option<RenderMode>,
    /// Physical framebuffer dimensions `(phys_w, phys_h)` of the most recent hashed frame.
    #[get(skip)]
    #[get_mut(skip)]
    #[set(skip)]
    pub(crate) last_hashed_dims: (u32, u32),
    /// Telemetry from the most recent frame.
    #[get(type(copy))]
    pub(crate) last_stats: FrameStats,
}
