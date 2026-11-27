/// Logical pixel leading clearance separating the empty-input caret from focused placeholder text.
pub(crate) const PLACEHOLDER_FOCUSED_CARET_CLEARANCE_PX: f32 = 2.5;

/// Power-of-two direct-mapped slot count for cross-frame retained `DrawCommand` subtree fragments.
pub(crate) const PAINT_FRAGMENT_CACHE_SLOTS: usize = 512;

/// Maximum direct leaf child count eligible for O(k) shallow subtree fragment caching in `PaintPass`.
pub(crate) const PAINT_FRAGMENT_MAX_CHILDREN: usize = 24;

/// 64-bit FxHash mixing multiplier for fast deterministic subtree fragment fingerprinting.
pub(crate) const PAINT_FRAGMENT_FX_MULT: u64 = 0x517C_C1B7_2722_0A95;
