use super::*;

/// Fast 128-entry ASCII glyph slot lookup table bound to one quantized style signature.
#[derive(Clone, Debug)]
pub(crate) struct AsciiGlyphStyleTable {
    /// Packed `(size_tenths | bold<<16 | monospace<<17 | italic<<18)` style signature.
    pub(crate) style_sig: u32,
    /// Direct slot indices into `GlyphAtlas::entries` for ASCII codepoints `0..128` (`u32::MAX` when uncached).
    pub(crate) slots: [u32; 128],
}

/// Memoized glyph rasterization cache shared across frames with O(1) slot indexing and ASCII direct tables.
#[derive(Clone, Data, Debug, Default)]
pub struct GlyphAtlas {
    /// Open-addressed power-of-two lookup table packing `(slot << 40) | packed_key` (`u64::MAX` when empty).
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) glyph_index: Vec<u64>,
    /// Contiguous flat arena of `(GlyphKey, RasterizedGlyph)` pairs indexed directly by `u32` slot.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) entries: Vec<(GlyphKey, RasterizedGlyph)>,
    /// Compact per-style `[u32; 128]` ASCII lookup tables eliminating HashMap probes on ASCII text runs.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) ascii_tables: Vec<AsciiGlyphStyleTable>,
    /// Monotonic counter incremented when new glyphs are inserted for GPU texture sync.
    #[get(type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) revision: u64,
}
