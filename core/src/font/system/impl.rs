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

    /// Returns the number of lazy native fallback font loads (such as CJK or symbol cascades) performed on the current thread.
    ///
    /// # Returns
    ///
    /// - `usize` - Cumulative lazy fallback font load count on this thread.
    #[must_use]
    pub fn thread_fallback_font_loads() -> usize {
        THREAD_FALLBACK_FONT_LOADS.with(Cell::get)
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
/// Inherent implementation of [`CoreTextFontCache`].
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
            scratch_ctx: null_mut(),
            scratch_buf: Vec::new(),
            scratch_w: 0,
            scratch_h: 0,
            fonts: Vec::new(),
        }
    }
}

/// Automatic CoreFoundation and CoreGraphics handle release on thread exit.
#[cfg(any(target_os = "macos", target_os = "ios"))]
/// Implements [`Drop`] for [`CoreTextFontCache`].
impl Drop for CoreTextFontCache {
    /// Releases the scratch `CGBitmapContextRef`, every cached `CTFontRef`, and the singleton `CGColorSpaceRef`.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Dropping CoreText cache.
    fn drop(&mut self) {
        let Self {
            gray_space,
            scratch_ctx,
            fonts,
            ..
        } = self;
        release_coretext_cache(*gray_space, *scratch_ctx, fonts);
        *scratch_ctx = null_mut();
        *gray_space = null_mut();
    }
}

/// Construction helpers for lazily resolved FreeType face slots.
#[cfg(any(target_os = "linux", target_os = "android"))]
/// Inherent implementation of [`FtLazyFace`].
impl FtLazyFace {
    /// Creates an unprobed lazy face slot incurring zero filesystem or FreeType overhead until first demand.
    ///
    /// # Returns
    ///
    /// - `Self` - Empty unprobed FreeType face slot.
    #[must_use]
    pub(crate) const fn unprobed() -> Self {
        Self {
            probed: false,
            face: null_mut(),
            last_size_px: 0,
        }
    }

    /// Wraps an already opened FreeType face pointer.
    ///
    /// # Arguments
    ///
    /// - `*mut FtFaceRec` - Opened FreeType face handle.
    ///
    /// # Returns
    ///
    /// - `Self` - Ready FreeType face slot.
    #[must_use]
    pub(crate) const fn loaded(face: *mut FtFaceRec) -> Self {
        Self {
            probed: true,
            face,
            last_size_px: 0,
        }
    }
}

/// Construction helpers for the thread-local Web offscreen glyph canvas scratchpad.
#[cfg(target_arch = "wasm32")]
/// Inherent implementation of [`WebFontScratch`].
impl WebFontScratch {
    /// Creates an empty, zero-DOM-allocation Web font scratchpad.
    ///
    /// # Returns
    ///
    /// - `Self` - Empty lazy Web font scratchpad.
    #[must_use]
    pub(crate) const fn new() -> Self {
        Self {
            canvas: None,
            ctx2d: None,
            width: 0,
            height: 0,
            last_font: String::new(),
        }
    }
}

/// Construction helpers for the thread-local Win32 GDI font handle cache.
#[cfg(target_os = "windows")]
/// Inherent implementation of [`Win32FontCache`].
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
/// Implements [`Drop`] for [`Win32FontCache`].
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
