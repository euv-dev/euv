/// The default width of the glyph atlas, in pixels.
///
/// The atlas holds the rasterised glyphs of a chart. A thousand distinct
/// glyphs need far less than a megapixel, so 1024×1024 covers almost
/// every chart without a second shelf.
pub(crate) const DEFAULT_ATLAS_SIZE: u32 = 1024;

/// The padding around each glyph in the atlas, in pixels.
///
/// Without the padding, the bilinear filter of a magnified glyph would
/// bleed the neighbouring glyph's pixels into it.
pub(crate) const GLYPH_PADDING: f32 = 1.0;

/// The tag of the offscreen canvas the text is measured on.
pub(crate) const CANVAS_TAG: &str = "canvas";
