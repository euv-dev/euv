use super::*;

/// Per-frame telemetry emitted by `Renderer3D` for live FPS/RT/LOD/culling diagnostics.
#[derive(Clone, Copy, Data, Debug, Default, PartialEq)]
pub struct RenderStats3D {
    /// Internal rasterized framebuffer width in pixels.
    #[get(type(copy))]
    pub(crate) render_width: u32,
    /// Internal rasterized framebuffer height in pixels.
    #[get(type(copy))]
    pub(crate) render_height: u32,
    /// Active objects rendered inside frustum & chunk stream radius.
    #[get(type(copy))]
    pub(crate) visible_objects: usize,
    /// Objects rejected by 6-plane frustum culling or chunk draw distance.
    #[get(type(copy))]
    pub(crate) culled_objects: usize,
    /// Deduplicated GPU instancing draw batches.
    #[get(type(copy))]
    pub(crate) instanced_batches: usize,
    /// Triangles rasterized after backface culling.
    #[get(type(copy))]
    pub(crate) triangles_drawn: usize,
    /// BVH shadow/reflection/GI rays traced during this frame.
    #[get(type(copy))]
    pub(crate) rays_traced: usize,
    /// Total BVH nodes active in the acceleration structure.
    #[get(type(copy))]
    pub(crate) bvh_nodes: usize,
    /// Objects rendered at simplified distant LOD (`LOD1+`).
    #[get(type(copy))]
    pub(crate) lod_simplified_objects: usize,
}

/// Zero-allocation PBR + BVH Ray-Tracing + FXAA 3D software/hybrid renderer outputting directly to `ImageBuffer`.
#[derive(Clone, Data, Debug, Default)]
pub struct Renderer3D {
    /// Pre-allocated RGBA8 pixel output buffer.
    pub(crate) color_buffer: Vec<u8>,
    /// Pre-allocated RGBA8 FXAA post-processing output buffer.
    pub(crate) aa_buffer: Vec<u8>,
    /// Pre-allocated floating-point depth buffer (`Z-Buffer`).
    pub(crate) depth_buffer: Vec<f32>,
    /// Last rendered internal width.
    #[get(type(copy))]
    pub(crate) width: u32,
    /// Last rendered internal height.
    #[get(type(copy))]
    pub(crate) height: u32,
    /// Telemetry from the most recent frame.
    #[get(type(copy))]
    pub(crate) last_stats: RenderStats3D,
}
