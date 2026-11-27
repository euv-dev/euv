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

/// Construction helpers for the thread-local Apple CoreText handle cache.
#[cfg(any(target_os = "macos", target_os = "ios"))]
impl CoreTextFontCache {
    /// Creates an empty, zero-allocation CoreText font handle cache.
    ///
    /// # Returns
    ///
    /// - `Self` - Empty thread-local CoreText handle cache.
    #[must_use]
    pub(crate) const fn new() -> Self {
        Self {
            gray_space: null_mut(),
            fonts: Vec::new(),
        }
    }
}

/// Automatic CoreFoundation and CoreGraphics handle release on thread exit.
#[cfg(any(target_os = "macos", target_os = "ios"))]
impl Drop for CoreTextFontCache {
    /// Releases every cached `CTFontRef` and the singleton `CGColorSpaceRef`.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Dropping CoreText cache.
    fn drop(&mut self) {
        let Self { gray_space, fonts } = self;
        release_coretext_cache(*gray_space, fonts);
        *gray_space = null_mut();
    }
}

/// Construction helpers for the thread-local Win32 GDI font handle cache.
#[cfg(target_os = "windows")]
impl Win32FontCache {
    /// Creates an empty, zero-allocation Win32 GDI font handle cache.
    ///
    /// # Returns
    ///
    /// - `Self` - Empty thread-local Win32 GDI handle cache.
    #[must_use]
    pub(crate) const fn new() -> Self {
        Self {
            hdc: null_mut(),
            fonts: Vec::new(),
            raw_buf: Vec::new(),
        }
    }
}

/// Automatic Win32 GDI `HFONT` and memory `HDC` release on thread exit.
#[cfg(target_os = "windows")]
impl Drop for Win32FontCache {
    /// Deletes every cached `HFONT` and the persistent memory `HDC`.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Dropping Win32 GDI cache.
    fn drop(&mut self) {
        let Self { hdc, fonts, .. } = self;
        release_win32_cache(*hdc, fonts);
        *hdc = null_mut();
    }
}
