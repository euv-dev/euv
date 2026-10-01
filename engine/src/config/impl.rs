use super::*;

/// Implements `Default` for `RenderConfig` with sensible default values.
impl Default for RenderConfig {
    /// Constructs a default [`RenderConfig`] value.
    ///
    /// # Returns
    ///
    /// - `RenderConfig` - A default-constructed instance with the documented initial state.
    fn default() -> RenderConfig {
        RenderConfig {
            backend: RenderBackendType::default(),
            canvas_selector: CONFIG_DEFAULT_CANVAS_SELECTOR.to_string(),
            width: CONFIG_DEFAULT_CANVAS_WIDTH,
            height: CONFIG_DEFAULT_CANVAS_HEIGHT,
            quality: RenderQuality::default(),
            antialias: CONFIG_DEFAULT_ANTIALIAS,
            power_preference: GpuPowerPreference::default(),
            ssaa_scale_factor: CONFIG_DEFAULT_SSAA_SCALE_FACTOR,
        }
    }
}

/// Implements `Default` for `EngineConfig` with default render and scheduler configs.
impl Default for EngineConfig {
    /// Constructs a default [`EngineConfig`] value.
    ///
    /// # Returns
    ///
    /// - `EngineConfig` - A default-constructed instance with the documented initial state.
    fn default() -> EngineConfig {
        EngineConfig {
            render: RenderConfig::default(),
            scheduler: SchedulerConfig::default(),
        }
    }
}

/// Implements construction helpers for `RenderConfig`.
impl RenderConfig {
    /// Creates a rendering configuration for the Canvas 2D backend.
    ///
    /// # Arguments
    ///
    /// - `S` - The CSS selector for the canvas element.
    /// - `f64` - The viewport width in CSS pixels.
    /// - `f64` - The viewport height in CSS pixels.
    ///
    /// # Returns
    ///
    /// - `RenderConfig` - The Canvas 2D rendering configuration.
    pub fn canvas2d<S>(canvas_selector: S, width: f64, height: f64) -> RenderConfig
    where
        S: AsRef<str>,
    {
        RenderConfig {
            backend: RenderBackendType::Canvas2D,
            canvas_selector: canvas_selector.as_ref().to_string(),
            width,
            height,
            ..RenderConfig::default()
        }
    }

    /// Creates a rendering configuration for the WebGPU backend.
    ///
    /// # Arguments
    ///
    /// - `S` - The CSS selector for the canvas element.
    /// - `f64` - The viewport width in CSS pixels.
    /// - `f64` - The viewport height in CSS pixels.
    ///
    /// # Returns
    ///
    /// - `RenderConfig` - The WebGPU rendering configuration.
    pub fn webgpu<S>(canvas_selector: S, width: f64, height: f64) -> RenderConfig
    where
        S: AsRef<str>,
    {
        RenderConfig {
            backend: RenderBackendType::WebGpu,
            canvas_selector: canvas_selector.as_ref().to_string(),
            width,
            height,
            ..RenderConfig::default()
        }
    }

    /// Creates a rendering configuration for the WebGL 2 backend.
    ///
    /// # Arguments
    ///
    /// - `S` - The CSS selector for the canvas element.
    /// - `f64` - The viewport width in CSS pixels.
    /// - `f64` - The viewport height in CSS pixels.
    ///
    /// # Returns
    ///
    /// - `RenderConfig` - The WebGL 2 rendering configuration.
    pub fn webgl<S>(canvas_selector: S, width: f64, height: f64) -> RenderConfig
    where
        S: AsRef<str>,
    {
        RenderConfig {
            backend: RenderBackendType::WebGl,
            canvas_selector: canvas_selector.as_ref().to_string(),
            width,
            height,
            ..RenderConfig::default()
        }
    }
}

/// Implements construction helpers for `EngineConfig`.
impl EngineConfig {
    /// Creates an engine configuration with the given render config and default scheduler.
    ///
    /// # Arguments
    ///
    /// - `RenderConfig` - The rendering configuration.
    ///
    /// # Returns
    ///
    /// - `EngineConfig` - The engine configuration.
    pub fn create(render: RenderConfig) -> EngineConfig {
        EngineConfig {
            render,
            scheduler: SchedulerConfig::default(),
        }
    }

    /// Sets the scheduler configuration and returns the updated config.
    ///
    /// # Arguments
    ///
    /// - `SchedulerConfig` - The scheduler configuration.
    ///
    /// # Returns
    ///
    /// - `EngineConfig` - The updated engine configuration.
    pub fn with_scheduler(mut self, scheduler: SchedulerConfig) -> EngineConfig {
        self.set_scheduler(scheduler);
        self
    }
}

/// Implements WebGPU power preference conversion for `GpuPowerPreference`.
impl GpuPowerPreference {
    /// Converts this power preference to the WebGPU string value.
    ///
    /// Used to set the `powerPreference` field on `GpuRequestAdapterOptions`
    /// via `Reflect::set`, avoiding a direct dependency on the
    /// `web_sys::GpuPowerPreference` type.
    ///
    /// # Returns
    ///
    /// - `&'static str` - The WebGPU power preference string (e.g., `"low-power"`).
    pub fn to_web_sys_string(&self) -> &'static str {
        match self {
            GpuPowerPreference::LowPower => GPU_POWER_PREFERENCE_LOW_POWER,
            GpuPowerPreference::HighPerformance => GPU_POWER_PREFERENCE_HIGH_PERFORMANCE,
        }
    }
}
