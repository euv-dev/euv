use super::*;

/// Optional process-wide native glyph rasterizer hook registered by mobile/OS host bridges.
pub(crate) static EXTERNAL_GLYPH_RASTERIZER: AtomicPtr<()> = AtomicPtr::new(null_mut());

/// Cached FreeType runtime availability probe avoiding repeated `dlopen`/`dlclose` and path stats.
#[cfg(any(target_os = "linux", target_os = "android"))]
pub(crate) static FREETYPE_AVAILABLE_CACHE: OnceLock<bool> = OnceLock::new();

thread_local! {
    /// Per-thread count of lazy native fallback font loads (CJK / symbol cascades) for telemetry and testing.
    pub(crate) static THREAD_FALLBACK_FONT_LOADS: Cell<usize> = const { Cell::new(0) };
}

#[cfg(any(target_os = "macos", target_os = "ios"))]
thread_local! {
    /// Per-thread cache of Apple CoreText `CTFontRef` handles, grayscale `CGColorSpaceRef`, and scratch bitmap context.
    pub(crate) static CORETEXT_FONT_CACHE: RefCell<CoreTextFontCache> = const { RefCell::new(CoreTextFontCache::new()) };
}

#[cfg(target_os = "windows")]
thread_local! {
    /// Per-thread cache of Windows Win32 `HDC`, `HFONT` handles, and outline scratch storage.
    pub(crate) static WIN32_FONT_CACHE: RefCell<Win32FontCache> = const { RefCell::new(Win32FontCache::new()) };
}

#[cfg(any(target_os = "linux", target_os = "android"))]
thread_local! {
    /// Cached per-thread FreeType library and lazily loaded system font faces (`None` = uninitialized, `Some(None)` = unavailable).
    pub(crate) static FREETYPE_RUNTIME_CACHE: RefCell<Option<Option<FreeTypeRuntime>>> = const { RefCell::new(None) };
}

#[cfg(target_arch = "wasm32")]
thread_local! {
    /// Reusable offscreen `<canvas>` and 2D context for zero-DOM-churn Web glyph rasterization.
    pub(crate) static WEB_FONT_SCRATCH: RefCell<WebFontScratch> = const { RefCell::new(WebFontScratch::new()) };
}
