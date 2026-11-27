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
}
