use super::*;

/// Optional process-wide native glyph rasterizer hook registered by mobile/OS host bridges.
pub(crate) static EXTERNAL_GLYPH_RASTERIZER: AtomicPtr<()> = AtomicPtr::new(null_mut());
