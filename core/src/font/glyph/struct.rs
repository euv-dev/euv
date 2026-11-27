use super::*;

/// Cache key uniquely identifying a rasterized glyph bitmap.
#[derive(Clone, Copy, Debug, Default, Eq, Getter, Hash, New, Ord, PartialEq, PartialOrd)]
pub struct GlyphKey {
    /// Unicode scalar value of the character.
    #[get(type(copy))]
    pub(crate) codepoint: char,
    /// Quantized pixel height in tenths of a pixel.
    #[get(type(copy))]
    pub(crate) size_tenths: u16,
    /// Whether bold stroke thickening is active.
    #[get(type(copy))]
    pub(crate) bold: bool,
    /// Whether a fixed-pitch monospace typeface is requested.
    #[new(skip)]
    #[get(type(copy))]
    pub(crate) monospace: bool,
    /// Whether an italic typeface is requested.
    #[new(skip)]
    #[get(type(copy))]
    pub(crate) italic: bool,
}

/// Rasterized alpha-coverage glyph bitmap ready for software blitting or GPU atlas upload.
#[derive(Clone, Debug, Default, Eq, Getter, New, PartialEq)]
pub struct RasterizedGlyph {
    /// Bitmap width in physical pixels.
    #[get(type(copy))]
    pub(crate) width: u32,
    /// Bitmap height in physical pixels.
    #[get(type(copy))]
    pub(crate) height: u32,
    /// Horizontal advance in physical pixels.
    #[get(type(copy))]
    pub(crate) advance: u32,
    /// Alpha coverage values in row-major order (`0..=255`).
    pub(crate) coverage: Vec<u8>,
}
