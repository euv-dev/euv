use super::*;

/// Tightly packed GPU instance payload uploaded directly to Metal, Vulkan, D3D12, WebGPU, or WebGL2 buffers.
#[repr(C)]
#[derive(Clone, Copy, Data, Debug, Default, PartialEq)]
pub struct GpuQuadInstance {
    /// Destination bounds `[x, y, width, height]` in physical pixels.
    #[get(type(copy))]
    pub(crate) rect: [f32; 4],
    /// Clip bounds `[x0, y0, x1, y1]` in physical pixels.
    #[get(type(copy))]
    pub(crate) clip: [f32; 4],
    /// Primary normalized RGBA color.
    #[get(type(copy))]
    pub(crate) color: [f32; 4],
    /// Secondary color (border color or gradient end color).
    #[get(type(copy))]
    pub(crate) secondary_color: [f32; 4],
    /// Corner radii `[tl, tr, br, bl]` in physical pixels.
    #[get(type(copy))]
    pub(crate) radii: [f32; 4],
    /// Border width or blur radius in physical pixels.
    #[get(type(copy))]
    pub(crate) stroke_or_blur: f32,
    /// Gradient angle in radians.
    #[get(type(copy))]
    pub(crate) angle_rad: f32,
    /// Encoded `GpuPrimitiveKind` discriminant.
    #[get(type(copy))]
    pub(crate) kind: u32,
    /// Padding for 16-byte GPU std140/std430 alignment.
    #[get(type(copy))]
    pub(crate) reserved: u32,
}

/// Coalesced instanced GPU draw batch submitted as a single hardware draw call.
#[derive(Clone, Data, Debug, Default, PartialEq)]
pub struct GpuBatch {
    /// Active scissor rectangle in physical pixels.
    #[get(type(copy))]
    pub(crate) scissor: Rect,
    /// Instanced quad stream belonging to this batch.
    pub(crate) instances: Vec<GpuQuadInstance>,
}

/// Recording GPU driver used on headless targets and as a hardware-contract validator.
#[derive(Clone, Data, Debug, Default)]
pub struct RecordingGpuDriver {
    /// Emulated or bound graphics backend tag.
    #[get(type(copy))]
    pub(crate) backend_kind: GraphicsBackend,
    /// Total draw batches submitted across frames.
    #[get(type(copy))]
    pub(crate) total_draw_calls: usize,
    /// Total GPU quad instances submitted in the most recent frame.
    #[get(type(copy))]
    pub(crate) last_instance_count: usize,
    /// Last clear color submitted.
    #[get(type(copy))]
    pub(crate) last_clear: Color,
}

/// Per-batch coalescing metadata tracking whether all queued instances are unclipped within `bounds`.
#[derive(Clone, Copy, Debug, Default)]
pub(crate) struct GpuBatchCoalesceState {
    /// True only while every instance in the active tail batch lies strictly inside its safe unclipped region.
    pub(crate) all_unclipped: bool,
    /// Conservative physical bounding box covering all instances in the active tail batch.
    pub(crate) bounds: Rect,
}

/// Active physical clip scope pairing the scissor rectangle with its corner-free interior safe rectangle.
#[derive(Clone, Copy, Debug, Default)]
pub(crate) struct GpuClipEntry {
    /// Intersected physical scissor rectangle.
    pub(crate) scissor: Rect,
    /// Conservative interior safe rectangle strictly clear of any active rounded-clip corners.
    pub(crate) safe_inner: Rect,
}

/// Hardware GPU renderer compiling `DisplayList` streams into order-preserving coalesced instanced `GpuBatch` calls.
#[derive(Data, Debug, Default)]
pub struct HardwareRenderer {
    /// Active physical OS GPU driver handling shader compilation and batch submission.
    pub(crate) driver: NativeGpuDriver,
    /// Reusable batch scratch buffer avoiding per-frame allocations.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) batches: Vec<GpuBatch>,
    /// Count of active `GpuBatch` slots populated for the latest frame so inner `Vec` capacities persist across frames.
    #[get(type(copy))]
    #[get_mut(skip)]
    #[set(pub(crate))]
    pub(crate) active_batches: usize,
    /// Persistent physical clip-rectangle and safe-inner stack reused across frames without heap reallocation.
    #[get(skip)]
    #[get_mut(skip)]
    #[set(skip)]
    pub(crate) clip_stack: Vec<GpuClipEntry>,
    /// Number of draw batches submitted in the most recent frame.
    #[get(type(copy))]
    #[get_mut(skip)]
    #[set(skip)]
    pub(crate) last_submitted_batches: usize,
    /// Number of adjacent cross-clip batch splits eliminated via order-preserving coalescing in the latest frame.
    #[get(type(copy))]
    #[get_mut(skip)]
    #[set(skip)]
    pub(crate) last_coalesced_merges: usize,
    /// Number of completely off-clip instances culled prior to GPU upload in the latest frame.
    #[get(type(copy))]
    #[get_mut(skip)]
    #[set(skip)]
    pub(crate) last_culled_instances: usize,
    /// Number of `GpuQuadInstance` records uploaded in the latest frame.
    #[get(type(copy))]
    #[get_mut(skip)]
    #[set(skip)]
    pub(crate) last_uploaded_instances: usize,
    /// Total instance payload bytes uploaded in the latest frame (`last_uploaded_instances * GPU_QUAD_INSTANCE_BYTES`).
    #[get(type(copy))]
    #[get_mut(skip)]
    #[set(skip)]
    pub(crate) last_uploaded_bytes: usize,
    /// Optional software readback mirror used for deterministic pixel-parity testing.
    pub(crate) readback: SoftwareRenderer,
    /// Whether readback mirroring is enabled.
    #[get(type(copy))]
    pub(crate) enable_readback: bool,
}
