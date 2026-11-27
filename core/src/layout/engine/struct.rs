use super::*;

/// Deterministic cross-platform Flexbox, Grid, Block, and Positioned layout engine.
#[derive(Clone, Data, Debug, Default)]
pub struct LayoutEngine {
    /// Deterministic text measurer shared with the rendering pass.
    #[get(type(copy))]
    pub(crate) measurer: DeterministicTextMeasurer,
    /// Statistics captured from the most recent layout pass.
    #[get(type(copy))]
    pub(crate) last_stats: LayoutStats,
    /// Reusable flat stack storing copied child handles across recursive `layout_node` frames.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(skip)]
    pub(crate) children_scratch: Vec<NodeId>,
    /// Reusable flat stack storing normal-flow child handles across recursive `layout_children` frames.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(skip)]
    pub(crate) flow_scratch: Vec<NodeId>,
    /// Reusable flat stack storing absolute/fixed overlay child handles across recursive `layout_children` frames.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(skip)]
    pub(crate) abs_scratch: Vec<NodeId>,
    /// Reusable flat stack storing measured flex child records across recursive `layout_flex` frames.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(skip)]
    pub(crate) flex_items_scratch: Vec<FlexItemRecord>,
    /// Reusable flat stack storing contiguous `(start, end)` line slices into `flex_items_scratch`.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(skip)]
    pub(crate) flex_lines_scratch: Vec<(usize, usize)>,
    /// Reusable stack buffer for iterative zero-allocation subtree translations.
    #[get(skip)]
    #[get_mut(pub(crate))]
    #[set(skip)]
    pub(crate) translate_scratch: Vec<NodeId>,
    /// Bounded 2-way set-associative cache of measured text sizes for layout passes.
    #[get(skip)]
    #[get_mut(pub(crate))]
    #[set(skip)]
    pub(crate) text_size_cache: TextSizeCache,
    /// Whether the active layout pass is recording pre-layout geometry snapshots for partial damage diffing.
    #[get(skip)]
    #[get_mut(skip)]
    #[set(skip)]
    pub(crate) track_damage: bool,
    /// Active fallback trigger recorded during a tracked layout pass (`LAYOUT_FALLBACK_*`).
    #[get(skip)]
    #[get_mut(skip)]
    #[set(skip)]
    pub(crate) damage_fallback: u8,
    /// Bounded pre-layout geometry snapshots collected during surgical partial-layout damage diffing.
    #[get(skip)]
    #[get_mut(skip)]
    #[set(skip)]
    pub(crate) layout_snapshots: Vec<LayoutNodeSnapshot>,
}

/// Pre-layout geometry snapshot used for surgical partial-layout damage diffing.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub(crate) struct LayoutNodeSnapshot {
    /// Target scene node handle.
    pub(crate) id: NodeId,
    /// Pre-layout border-box rectangle in layout coordinates.
    pub(crate) old_bounds: Rect,
    /// Pre-layout content-box rectangle in layout coordinates.
    pub(crate) old_content_bounds: Rect,
    /// Pre-layout intrinsic content dimensions.
    pub(crate) old_content_size: Size,
    /// Whether `old_screen` holds a valid pre-layout conservative logical screen rectangle.
    pub(crate) has_old_screen: bool,
    /// Pre-layout conservative logical screen bounds (including shadow, border, scroll, and transform).
    pub(crate) old_screen: Rect,
}

/// Outcome of surgical pre/post-layout conservative bounds diffing.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub(crate) struct LayoutDamageReport {
    /// Tight physical damage rectangles unioning old and new bounds of geometry-changed nodes.
    pub(crate) damage: DamageRegion,
    /// Fallback trigger (`LAYOUT_FALLBACK_*`) requesting full-viewport damage when non-zero.
    pub(crate) fallback_kind: u8,
}
