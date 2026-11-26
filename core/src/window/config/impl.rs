use super::*;

/// Default display info (`1280x800`, `1.0x` scale, `60Hz`, `sRGB`).
impl Default for DisplayInfo {
    /// Creates standard desktop fallback display metadata.
    ///
    /// # Returns
    ///
    /// - `Self` - Default `DisplayInfo`.
    fn default() -> Self {
        Self::new(
            DEFAULT_WINDOW_WIDTH,
            DEFAULT_WINDOW_HEIGHT,
            1.0,
            DEFAULT_REFRESH_RATE_HZ,
            ColorSpace::Srgb,
        )
    }
}

/// Default `WindowConfig` with cross-platform sensible defaults.
impl Default for WindowConfig {
    /// Creates a standard resizable `1280x800` decorated window configuration.
    ///
    /// # Returns
    ///
    /// - `Self` - Default `WindowConfig`.
    fn default() -> Self {
        Self {
            title: DEFAULT_WINDOW_TITLE.to_string(),
            width: DEFAULT_WINDOW_WIDTH,
            height: DEFAULT_WINDOW_HEIGHT,
            min_size: None,
            max_size: None,
            position: None,
            resizable: true,
            decorations: true,
            transparent: false,
            always_on_top: false,
            fullscreen: FullscreenMode::Windowed,
            immersive: false,
            visible: true,
            render: RenderConfig::default(),
            orientation: OrientationLock::Auto,
            status_bar: StatusBarStyle::Auto,
            preferred_theme: None,
            icon: None,
            icon_path: None,
        }
    }
}

/// Conversion and helper methods for `SafeAreaInsets`.
impl SafeAreaInsets {
    /// Converts safe-area insets directly into layout `Edges`.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Safe-area insets.
    ///
    /// # Returns
    ///
    /// - `Edges` - Equivalent padding edges.
    #[must_use]
    pub fn to_edges(&self) -> Edges {
        Edges::new(
            self.get_top(),
            self.get_right(),
            self.get_bottom(),
            self.get_left(),
        )
    }
}

/// Convenience setters and safe optional accessors for `WindowConfig`.
impl WindowConfig {
    /// Sets initial logical `(width, height)` clamped to at least `1x1`.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable window config.
    /// - `u32` - Logical width.
    /// - `u32` - Logical height.
    ///
    /// # Returns
    ///
    /// - `&mut Self` - Mutable reference for fluent setter chaining.
    pub fn set_size(&mut self, width: u32, height: u32) -> &mut Self {
        self.set_width(width.max(1)).set_height(height.max(1))
    }

    /// Returns the optional minimum size tuple.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Window configuration.
    ///
    /// # Returns
    ///
    /// - `Option<(u32, u32)>` - Minimum logical dimensions if set.
    #[must_use]
    pub fn min_size_value(&self) -> Option<(u32, u32)> {
        let Self { min_size, .. } = self;
        *min_size
    }

    /// Returns the optional maximum size tuple.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Window configuration.
    ///
    /// # Returns
    ///
    /// - `Option<(u32, u32)>` - Maximum logical dimensions if set.
    #[must_use]
    pub fn max_size_value(&self) -> Option<(u32, u32)> {
        let Self { max_size, .. } = self;
        *max_size
    }

    /// Returns the optional initial window position.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Window configuration.
    ///
    /// # Returns
    ///
    /// - `Option<(i32, i32)>` - Initial screen position if set.
    #[must_use]
    pub fn position_value(&self) -> Option<(i32, i32)> {
        let Self { position, .. } = self;
        *position
    }

    /// Returns the optional preferred theme override.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Window configuration.
    ///
    /// # Returns
    ///
    /// - `Option<ThemeMode>` - Preferred theme mode if set.
    #[must_use]
    pub fn preferred_theme_value(&self) -> Option<ThemeMode> {
        let Self {
            preferred_theme, ..
        } = self;
        *preferred_theme
    }

    /// Returns the optional decoded RGBA8 application icon bitmap.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Window configuration.
    ///
    /// # Returns
    ///
    /// - `Option<ImageBuffer>` - Configured RGBA8 icon buffer if set.
    #[must_use]
    pub fn icon_value(&self) -> Option<ImageBuffer> {
        let Self { icon, .. } = self;
        icon.clone()
    }

    /// Returns the optional application icon asset path.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Window configuration.
    ///
    /// # Returns
    ///
    /// - `Option<String>` - Configured icon asset path if set.
    #[must_use]
    pub fn icon_path_value(&self) -> Option<String> {
        let Self { icon_path, .. } = self;
        icon_path.clone()
    }
}

/// State construction and geometry queries for `WindowState`.
impl WindowState {
    /// Initializes a `WindowState` from a `WindowId`, `WindowConfig`, and DPI scale factor.
    ///
    /// # Arguments
    ///
    /// - `WindowId` - Assigned window handle.
    /// - `&WindowConfig` - Initial window configuration.
    /// - `f32` - Active display scale factor.
    ///
    /// # Returns
    ///
    /// - `Self` - Initialized live window state.
    #[must_use]
    pub fn from_config(id: WindowId, config: &WindowConfig, scale_factor: f32) -> Self {
        let safe_scale: f32 = scale_factor.max(0.5);
        let logical_w: f32 = config.get_width().max(1) as f32;
        let logical_h: f32 = config.get_height().max(1) as f32;
        let phys_w: u32 = ((logical_w * safe_scale).round() as u32).max(1);
        let phys_h: u32 = ((logical_h * safe_scale).round() as u32).max(1);
        let display: DisplayInfo = DisplayInfo::new(
            phys_w,
            phys_h,
            safe_scale,
            DEFAULT_REFRESH_RATE_HZ,
            config.get_render().get_color_space(),
        );
        Self {
            id,
            logical_width: logical_w,
            logical_height: logical_h,
            physical_width: phys_w,
            physical_height: phys_h,
            scale_factor: safe_scale,
            safe_area: SafeAreaInsets::default(),
            display,
            focused: true,
            minimized: false,
            maximized: false,
            fullscreen: config.get_fullscreen(),
            system_theme: config.preferred_theme_value().unwrap_or(ThemeMode::Light),
        }
    }

    /// Returns the logical viewport `Size`.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Window state.
    ///
    /// # Returns
    ///
    /// - `Size` - Logical width and height.
    #[must_use]
    pub fn logical_size(&self) -> Size {
        Size::new(self.get_logical_width(), self.get_logical_height())
    }

    /// Returns the responsive `Breakpoint` for the current logical width.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Window state.
    ///
    /// # Returns
    ///
    /// - `Breakpoint` - Active viewport breakpoint.
    #[must_use]
    pub fn breakpoint(&self) -> Breakpoint {
        Breakpoint::from_width(self.get_logical_width())
    }
}
