use super::*;

/// Optional process-wide native glyph rasterizer hook registered by mobile/OS host bridges.
pub(crate) static EXTERNAL_GLYPH_RASTERIZER: AtomicPtr<()> = AtomicPtr::new(null_mut());

#[cfg(any(target_os = "macos", target_os = "ios"))]
thread_local! {
    /// Per-thread cache of Apple CoreText `CTFontRef` handles and grayscale `CGColorSpaceRef`.
    pub(crate) static CORETEXT_FONT_CACHE: RefCell<CoreTextFontCache> = const { RefCell::new(CoreTextFontCache::new()) };
}

#[cfg(target_os = "windows")]
thread_local! {
    /// Per-thread cache of Windows Win32 `HDC`, `HFONT` handles, and outline scratch storage.
    pub(crate) static WIN32_FONT_CACHE: RefCell<Win32FontCache> = const { RefCell::new(Win32FontCache::new()) };
}

#[cfg(any(target_os = "linux", target_os = "android"))]
thread_local! {
    /// Cached per-thread FreeType library and loaded system font faces (`None` = uninitialized, `Some(None)` = unavailable).
    pub(crate) static FREETYPE_RUNTIME_CACHE: RefCell<Option<Option<FreeTypeRuntime>>> = const { RefCell::new(None) };
}
