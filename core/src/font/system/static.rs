use super::*;

/// Optional process-wide native glyph rasterizer hook registered by mobile/OS host bridges.
pub(crate) static EXTERNAL_GLYPH_RASTERIZER: AtomicPtr<()> = AtomicPtr::new(null_mut());

#[cfg(any(target_os = "linux", target_os = "android"))]
thread_local! {
    /// Cached per-thread FreeType library and loaded system font faces (`None` = uninitialized, `Some(None)` = unavailable).
    pub(crate) static FREETYPE_RUNTIME_CACHE: RefCell<Option<Option<FreeTypeRuntime>>> = const { RefCell::new(None) };
}
