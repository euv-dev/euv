/// Master AAA graphics quality tier adjustable live at runtime.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub enum QualityPreset {
    /// Maximum frame rate on low-power mobile GPUs (aggressive LOD, RT off).
    Low,
    /// Balanced performance and fidelity (medium draw distance, RT shadows optional).
    Medium,
    /// High-resolution desktop & flagship mobile default (PBR + RT shadows).
    #[default]
    High,
    /// Ultra open-world fidelity (RT shadows + reflections, volumetric fog, extended LOD).
    Ultra,
    /// Showcase cinematic path-traced GI + full resolution + max draw distance.
    Cinematic,
}

/// Target rendering resolution preset coupled with dynamic resolution scaling.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub enum ResolutionPreset {
    /// 1280x720 HD target.
    Hd720,
    /// 1920x1080 Full HD target.
    #[default]
    Fhd1080,
    /// 2560x1440 QHD target.
    Qhd1440,
    /// 3840x2160 4K UHD target.
    Uhd4k,
}

/// Hardware/BVH ray-tracing pipeline mode adjustable live at runtime.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub enum RayTracingMode {
    /// Rasterized PBR shading with analytic directional lighting only.
    Off,
    /// BVH-accelerated ray-traced directional sun shadows.
    #[default]
    ShadowsOnly,
    /// BVH ray-traced shadows plus glossy/mirror PBR surface reflections.
    ShadowsAndReflections,
    /// Full ray-traced shadows, reflections, and hemisphere ambient occlusion / 1-bounce GI.
    FullPathTracedGi,
}

/// Spatial or temporal upscaling mode for maintaining high FPS at 4K output.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub enum UpscalerMode {
    /// Native 1:1 pixel shading without upscaling.
    Native,
    /// Edge-preserving spatial upscaling (FSR-style sharpening).
    #[default]
    SpatialFsr,
    /// Temporal accumulation upscaling (TAAU/DLSS-style pipeline).
    TemporalTaa,
}

/// Post-processing anti-aliasing mode.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub enum AntiAliasingMode {
    /// No post-process anti-aliasing.
    Off,
    /// Fast approximate edge-smoothing anti-aliasing.
    #[default]
    Fxaa,
    /// Temporal anti-aliasing.
    Taa,
}
