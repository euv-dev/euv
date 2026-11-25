use super::*;

/// Complete rendering pipeline configuration shared across all 7 platforms.
#[derive(Clone, Copy, Data, Debug, Eq, Hash, PartialEq)]
pub struct RenderConfig {
    /// Requested rendering mode (`Auto`, `Software`, `Hardware`, `Hybrid`).
    #[get(type(copy))]
    pub(crate) mode: RenderMode,
    /// Optional explicit graphics backend override.
    #[get(skip)]
    pub(crate) preferred_backend: Option<GraphicsBackend>,
    /// Swapchain synchronization mode.
    #[get(type(copy))]
    pub(crate) present_mode: PresentMode,
    /// Anti-aliasing mode.
    #[get(type(copy))]
    pub(crate) antialiasing: AntialiasingMode,
    /// Target color space.
    #[get(type(copy))]
    pub(crate) color_space: ColorSpace,
    /// Enables damage-region dirty-rect clipping to skip clean pixels.
    #[get(type(copy))]
    pub(crate) dirty_rect_optimization: bool,
    /// Tile edge size in pixels for CPU tile rasterization.
    #[get(type(copy))]
    pub(crate) tile_size: u32,
    /// Target maximum frame rate cap.
    #[get(type(copy))]
    pub(crate) max_fps: u32,
    /// Whether high dynamic range presentation is requested when supported.
    #[get(type(copy))]
    pub(crate) hdr: bool,
}

/// Per-frame performance and rendering telemetry snapshot.
#[derive(Clone, Copy, Data, Debug, Default, Eq, Hash, PartialEq)]
pub struct FrameStats {
    /// Monotonic rendered frame counter.
    #[get(type(copy))]
    pub(crate) frame_index: u64,
    /// Active resolved render mode.
    #[get(type(copy))]
    pub(crate) active_mode: RenderMode,
    /// Active resolved graphics backend.
    #[get(type(copy))]
    pub(crate) active_backend: GraphicsBackend,
    /// Number of scene nodes laid out this frame.
    #[get(type(copy))]
    pub(crate) layout_nodes: usize,
    /// Number of clean subtrees skipped during layout.
    #[get(type(copy))]
    pub(crate) layout_skipped: usize,
    /// Number of display-list commands emitted.
    #[get(type(copy))]
    pub(crate) draw_commands: usize,
    /// Number of GPU instanced draw batches submitted (or software dirty tiles rasterized).
    #[get(type(copy))]
    pub(crate) draw_batches: usize,
    /// Total pixels updated in the framebuffer this frame.
    #[get(type(copy))]
    pub(crate) pixels_shaded: usize,
    /// Deterministic 64-bit FNV-1a hash of the rendered framebuffer (when software/readback active).
    #[get(type(copy))]
    pub(crate) frame_hash: u64,
}
