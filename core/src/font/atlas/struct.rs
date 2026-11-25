use super::*;

/// Memoized glyph rasterization cache shared across frames to avoid redundant glyph work.
#[derive(Clone, Data, Debug, Default)]
pub struct GlyphAtlas {
    /// Cached rasterized glyph bitmaps indexed by quantized `GlyphKey`.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) glyphs: HashMap<GlyphKey, RasterizedGlyph>,
    /// Monotonic counter incremented when new glyphs are inserted for GPU texture sync.
    #[get(type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) revision: u64,
}
