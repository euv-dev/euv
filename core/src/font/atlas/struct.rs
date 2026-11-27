use super::*;

/// Fast 128-entry ASCII glyph slot lookup table bound to one quantized style signature.
#[derive(Clone, Debug)]
pub(crate) struct AsciiGlyphStyleTable {
    /// Packed `(size_tenths | bold<<16 | monospace<<17 | italic<<18)` style signature.
    pub(crate) style_sig: u32,
    /// Direct slot indices into `GlyphAtlas::entries` for ASCII codepoints `0..128` (`u32::MAX` when uncached).
    pub(crate) slots: [u32; 128],
}

/// Memoized glyph rasterization cache shared across frames with O(1) slot indexing, ASCII direct tables, bounded LRU budget, and hit/miss telemetry.
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
    /// Per-entry monotonic access ticks aligned 1:1 with `entries` for bounded LRU eviction.
    #[get(skip)]
    #[get_mut(skip)]
    #[set(skip)]
    pub(crate) entry_last_used: Vec<u64>,
    /// Compact per-style `[u32; 128]` ASCII lookup tables eliminating hash probes on ASCII text runs.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) ascii_tables: Vec<AsciiGlyphStyleTable>,
    /// Monotonic counter incremented when new glyphs are inserted or evicted for GPU texture sync.
    #[get(type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) revision: u64,
    /// Configured resident memory budget in bytes (`0` selects `GLYPH_ATLAS_DEFAULT_BUDGET_BYTES`).
    #[get(type(copy))]
    #[get_mut(skip)]
    #[set(pub(crate))]
    pub(crate) budget_bytes: usize,
    /// Current resident memory usage in bytes across all cached glyph bitmaps and metadata.
    #[get(type(copy))]
    #[get_mut(skip)]
    #[set(skip)]
    pub(crate) resident_bytes: usize,
    /// Monotonic logical clock incremented on cache accesses for LRU ordering.
    #[get(skip)]
    #[get_mut(skip)]
    #[set(skip)]
    pub(crate) lru_clock: u64,
    /// Cumulative number of cache hits served without rasterizing.
    #[get(type(copy))]
    #[get_mut(skip)]
    #[set(skip)]
    pub(crate) cache_hits: u64,
    /// Cumulative number of cache misses requiring rasterization.
    #[get(type(copy))]
    #[get_mut(skip)]
    #[set(skip)]
    pub(crate) cache_misses: u64,
    /// Cumulative number of cold glyphs evicted to stay within `budget_bytes`.
    #[get(type(copy))]
    #[get_mut(skip)]
    #[set(skip)]
    pub(crate) evictions: u64,
    /// Guard pinning arena slot indices for the duration of a single frame's `prepare_frame_range`.
    #[get(pub(crate), type(copy))]
    #[get_mut(skip)]
    #[set(pub(crate))]
    pub(crate) slots_pinned: bool,
}
