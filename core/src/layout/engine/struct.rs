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
    /// Bounded 2-way set-associative cache of measured text sizes for layout passes.
    #[get(skip)]
    #[get_mut(pub(crate))]
    #[set(skip)]
    pub(crate) text_size_cache: TextSizeCache,
}
