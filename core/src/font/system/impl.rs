use super::*;

/// Default `NativeSystemFont` uses deterministic cross-platform rasterization.
impl Default for NativeSystemFont {
    /// Creates a deterministic `NativeSystemFont`.
    ///
    /// # Returns
    ///
    /// - `Self` - Deterministic font provider.
    fn default() -> Self {
        Self::deterministic()
    }
}

/// Construction and mode helpers for `NativeSystemFont`.
impl NativeSystemFont {
    /// Creates a `NativeSystemFont` probing the active operating system's native font engine.
    ///
    /// # Returns
    ///
    /// - `Self` - Native OS font provider with automatic deterministic fallback.
    #[must_use]
    pub fn system() -> Self {
        Self {
            info: probe_system_font(),
            deterministic_only: false,
        }
    }

    /// Creates a deterministic cross-platform font provider guaranteeing identical pixels on every OS.
    ///
    /// # Returns
    ///
    /// - `Self` - Deterministic bitmap/stroke font provider.
    #[must_use]
    pub fn deterministic() -> Self {
        Self {
            info: SystemFontInfo::new(
                FontRenderBackend::DeterministicBitmap,
                FONT_FAMILY_DETERMINISTIC.to_string(),
                true,
                false,
            ),
            deterministic_only: true,
        }
    }
}

/// `SystemFontProvider` implementation delegating to OS native APIs with deterministic fallback.
impl SystemFontProvider for NativeSystemFont {
    /// Returns the active `SystemFontInfo`.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Font provider reference.
    ///
    /// # Returns
    ///
    /// - `SystemFontInfo` - Provider capability descriptor.
    fn font_info(&self) -> SystemFontInfo {
        self.get_info().clone()
    }

    /// Rasterizes `key` via the native OS font engine, falling back to deterministic rasterization.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Font provider reference.
    /// - `GlyphKey` - Quantized glyph descriptor.
    ///
    /// # Returns
    ///
    /// - `Option<RasterizedGlyph>` - Rasterized alpha8 coverage bitmap.
    fn rasterize(&self, key: GlyphKey) -> Option<RasterizedGlyph> {
        if !self.get_deterministic_only()
            && let Some(native_glyph) = rasterize_native_glyph(key)
        {
            return Some(native_glyph);
        }
        Some(rasterize_glyph(key))
    }
}
