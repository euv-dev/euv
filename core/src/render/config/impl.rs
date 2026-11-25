use super::*;

/// Default rendering configuration (`Auto`, `VSync`, `AnalyticalSdf`, dirty rects enabled).
impl Default for RenderConfig {
    /// Creates the recommended cross-platform `RenderConfig`.
    ///
    /// # Returns
    ///
    /// - `Self` - Default `RenderConfig`.
    fn default() -> Self {
        Self {
            mode: RenderMode::Auto,
            preferred_backend: None,
            present_mode: PresentMode::VSync,
            antialiasing: AntialiasingMode::AnalyticalSdf,
            color_space: ColorSpace::Srgb,
            dirty_rect_optimization: true,
            tile_size: DEFAULT_TILE_SIZE,
            max_fps: DEFAULT_MAX_FPS,
            hdr: false,
        }
    }
}

/// Fluent builder helpers for `RenderConfig`.
impl RenderConfig {
    /// Creates a software-rendering configuration.
    ///
    /// # Returns
    ///
    /// - `Self` - Software-mode `RenderConfig`.
    #[must_use]
    pub fn software() -> Self {
        let mut cfg: Self = Self::default();
        cfg.set_mode(RenderMode::Software);
        cfg.set_preferred_backend(Some(GraphicsBackend::SoftwareCpu));
        cfg
    }

    /// Creates a hardware GPU rendering configuration.
    ///
    /// # Returns
    ///
    /// - `Self` - Hardware-mode `RenderConfig`.
    #[must_use]
    pub fn hardware() -> Self {
        let mut cfg: Self = Self::default();
        cfg.set_mode(RenderMode::Hardware);
        cfg
    }

    /// Creates a hybrid CPU-tile + GPU-compositor rendering configuration.
    ///
    /// # Returns
    ///
    /// - `Self` - Hybrid-mode `RenderConfig`.
    #[must_use]
    pub fn hybrid() -> Self {
        let mut cfg: Self = Self::default();
        cfg.set_mode(RenderMode::Hybrid);
        cfg
    }

    /// Returns the optional preferred graphics backend.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Render config.
    ///
    /// # Returns
    ///
    /// - `Option<GraphicsBackend>` - Explicit backend preference if set.
    #[must_use]
    pub fn preferred_backend_value(&self) -> Option<GraphicsBackend> {
        let Self {
            preferred_backend, ..
        } = *self;
        preferred_backend
    }
}
