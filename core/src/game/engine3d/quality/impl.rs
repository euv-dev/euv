use super::*;

/// Default `High` AAA quality configuration with BVH ray-traced shadows and DRS enabled.
impl Default for RenderQualityConfig {
    /// Builds the default `High` runtime graphics profile.
    ///
    /// # Returns
    ///
    /// - `Self` - High quality graphics configuration.
    fn default() -> Self {
        Self::from_preset(QualityPreset::High)
    }
}

/// Resolution dimension helpers for `ResolutionPreset`.
impl ResolutionPreset {
    /// Returns nominal `(width, height)` pixel dimensions for this target tier.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Resolution tier.
    ///
    /// # Returns
    ///
    /// - `(u32, u32)` - Target output dimensions.
    #[must_use]
    pub const fn dimensions(&self) -> (u32, u32) {
        match self {
            Self::Hd720 => (1280, 720),
            Self::Fhd1080 => (1920, 1080),
            Self::Qhd1440 => (2560, 1440),
            Self::Uhd4k => (3840, 2160),
        }
    }
}

/// Preset builders and live dynamic resolution scaling for `RenderQualityConfig`.
impl RenderQualityConfig {
    /// Builds a complete runtime graphics configuration tuned for `preset`.
    ///
    /// # Arguments
    ///
    /// - `QualityPreset` - Desired master quality tier.
    ///
    /// # Returns
    ///
    /// - `Self` - Tuned runtime configuration.
    #[must_use]
    pub fn from_preset(preset: QualityPreset) -> Self {
        match preset {
            QualityPreset::Low => Self {
                preset,
                resolution_preset: ResolutionPreset::Hd720,
                resolution_scale: 0.55,
                dynamic_resolution: true,
                target_fps: 120.0,
                ray_tracing: RayTracingMode::Off,
                reflection_bounces: 0,
                upscaler: UpscalerMode::SpatialFsr,
                anti_aliasing: AntiAliasingMode::Off,
                draw_distance: 140.0,
                lod_bias: 1.5,
                frustum_culling: true,
                chunk_streaming: true,
                volumetric_fog: false,
                gpu_instancing: true,
            },
            QualityPreset::Medium => Self {
                preset,
                resolution_preset: ResolutionPreset::Fhd1080,
                resolution_scale: 0.72,
                dynamic_resolution: true,
                target_fps: 60.0,
                ray_tracing: RayTracingMode::ShadowsOnly,
                reflection_bounces: 0,
                upscaler: UpscalerMode::SpatialFsr,
                anti_aliasing: AntiAliasingMode::Fxaa,
                draw_distance: 220.0,
                lod_bias: 1.15,
                frustum_culling: true,
                chunk_streaming: true,
                volumetric_fog: true,
                gpu_instancing: true,
            },
            QualityPreset::High => Self {
                preset,
                resolution_preset: ResolutionPreset::Fhd1080,
                resolution_scale: 0.85,
                dynamic_resolution: true,
                target_fps: 60.0,
                ray_tracing: RayTracingMode::ShadowsAndReflections,
                reflection_bounces: 1,
                upscaler: UpscalerMode::SpatialFsr,
                anti_aliasing: AntiAliasingMode::Fxaa,
                draw_distance: 320.0,
                lod_bias: 1.0,
                frustum_culling: true,
                chunk_streaming: true,
                volumetric_fog: true,
                gpu_instancing: true,
            },
            QualityPreset::Ultra => Self {
                preset,
                resolution_preset: ResolutionPreset::Qhd1440,
                resolution_scale: 1.0,
                dynamic_resolution: false,
                target_fps: 60.0,
                ray_tracing: RayTracingMode::ShadowsAndReflections,
                reflection_bounces: 2,
                upscaler: UpscalerMode::TemporalTaa,
                anti_aliasing: AntiAliasingMode::Taa,
                draw_distance: 420.0,
                lod_bias: 0.8,
                frustum_culling: true,
                chunk_streaming: true,
                volumetric_fog: true,
                gpu_instancing: true,
            },
            QualityPreset::Cinematic => Self {
                preset,
                resolution_preset: ResolutionPreset::Uhd4k,
                resolution_scale: 1.0,
                dynamic_resolution: false,
                target_fps: 60.0,
                ray_tracing: RayTracingMode::FullPathTracedGi,
                reflection_bounces: 2,
                upscaler: UpscalerMode::Native,
                anti_aliasing: AntiAliasingMode::Taa,
                draw_distance: 520.0,
                lod_bias: 0.6,
                frustum_culling: true,
                chunk_streaming: true,
                volumetric_fog: true,
                gpu_instancing: true,
            },
        }
    }

    /// Dynamically adapts `resolution_scale` based on measured `frame_ms` when DRS is enabled.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable configuration.
    /// - `f32` - Measured render duration of the previous frame in milliseconds.
    pub fn adjust_dynamic_resolution(&mut self, frame_ms: f32) {
        if !self.get_dynamic_resolution() {
            return;
        }
        let budget_ms: f32 = 1000.0 / self.get_target_fps().max(30.0);
        let cur: f32 = self.get_resolution_scale();
        if frame_ms > budget_ms * 1.12 {
            self.set_resolution_scale((cur - 0.04).clamp(0.40, 1.25));
        } else if frame_ms < budget_ms * 0.72 {
            self.set_resolution_scale((cur + 0.02).clamp(0.40, 1.25));
        }
    }
}
