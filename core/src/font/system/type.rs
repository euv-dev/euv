use super::*;

/// Function pointer signature for platform-host supplied anti-aliased glyph rasterizers (e.g. Android JNI `Paint`/`Canvas`).
pub type ExternalGlyphRasterizer = fn(GlyphKey) -> Option<RasterizedGlyph>;
