use super::*;

/// Complete runtime-adjustable AAA 3D rendering, resolution, LOD, streaming, and ray-tracing configuration.
#[derive(Clone, Copy, Data, Debug, PartialEq)]
pub struct RenderQualityConfig {
    /// Active master quality preset.
    #[get(type(copy))]
    pub(crate) preset: QualityPreset,
    /// Target logical output resolution tier.
    #[get(type(copy))]
    pub(crate) resolution_preset: ResolutionPreset,
    /// Internal render resolution multiplier (`0.35..=2.0`).
    #[get(type(copy))]
    pub(crate) resolution_scale: f32,
    /// Enables automatic dynamic resolution scaling (DRS) toward `target_fps`.
    #[get(type(copy))]
    pub(crate) dynamic_resolution: bool,
    /// Target frame rate budget for DRS (`60.0` or `120.0`).
    #[get(type(copy))]
    pub(crate) target_fps: f32,
    /// Active BVH ray-tracing pipeline tier.
    #[get(type(copy))]
    pub(crate) ray_tracing: RayTracingMode,
    /// Maximum ray-traced reflection bounce depth (`0..=3`).
    #[get(type(copy))]
    pub(crate) reflection_bounces: u8,
    /// Upscaler pipeline selection.
    #[get(type(copy))]
    pub(crate) upscaler: UpscalerMode,
    /// Post-process anti-aliasing mode.
    #[get(type(copy))]
    pub(crate) anti_aliasing: AntiAliasingMode,
    /// Maximum open-world draw distance in meters.
    #[get(type(copy))]
    pub(crate) draw_distance: f32,
    /// LOD bias multiplier (`< 1.0` higher detail, `> 1.0` faster aggressive LOD).
    #[get(type(copy))]
    pub(crate) lod_bias: f32,
    /// Enables 6-plane hierarchical view-frustum culling.
    #[get(type(copy))]
    pub(crate) frustum_culling: bool,
    /// Enables distance-based open-world chunk streaming.
    #[get(type(copy))]
    pub(crate) chunk_streaming: bool,
    /// Enables atmospheric height & distance volumetric fog.
    #[get(type(copy))]
    pub(crate) volumetric_fog: bool,
    /// Enables GPU instancing batch deduplication.
    #[get(type(copy))]
    pub(crate) gpu_instancing: bool,
}
