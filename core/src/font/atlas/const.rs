/// Empty-slot sentinel inside `GlyphAtlas::glyph_index` (`u64::MAX` can never match a packed key).
pub(crate) const GLYPH_INDEX_EMPTY: u64 = u64::MAX;

/// 40-bit mask isolating the lossless packed `GlyphKey` payload inside a `glyph_index` slot.
pub(crate) const GLYPH_INDEX_KEY_MASK: u64 = (1_u64 << 40) - 1;

/// Bit offset of the 24-bit `entries` arena slot index inside a `glyph_index` slot.
pub(crate) const GLYPH_INDEX_SLOT_SHIFT: u32 = 40;

/// Initial power-of-two capacity of the open-addressed `GlyphAtlas::glyph_index` table.
pub(crate) const GLYPH_INDEX_INIT_CAP: usize = 256;

/// Initial reservation capacity of `GlyphAtlas::entries` on first insertion.
pub(crate) const GLYPH_ENTRIES_INIT_CAP: usize = 128;

/// Multiplicative avalanche constant for single-probe `GlyphKey` hashing.
pub(crate) const GLYPH_INDEX_HASH_MULT: u64 = 0x517C_C1B7_2722_0A95;

/// Minimum width and height in pixels for a packed GPU RGBA8 glyph texture atlas.
pub(crate) const ATLAS_MIN_DIM: u32 = 16;

/// Maximum glyph count whose shelf placements stay on the stack during `pack_rgba_atlas`.
pub(crate) const ATLAS_STACK_PLACEMENTS: usize = 32;

/// Default resident memory budget for `GlyphAtlas` (8 MiB) before LRU eviction activates.
pub(crate) const GLYPH_ATLAS_DEFAULT_BUDGET_BYTES: usize = 8 * 1024 * 1024;

/// Minimum configurable resident memory budget for `GlyphAtlas` (1 KiB).
pub(crate) const GLYPH_ATLAS_MIN_BUDGET_BYTES: usize = 1024;

/// Fixed per-entry metadata overhead (struct fields, index slot, and LRU tick) counted toward `GlyphAtlas::resident_bytes`.
pub(crate) const GLYPH_ENTRY_OVERHEAD_BYTES: usize = 64;
