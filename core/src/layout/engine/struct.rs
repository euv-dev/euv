use super::*;

/// Deterministic cross-platform Flexbox, Grid, Block, and Positioned layout engine.
#[derive(Clone, Copy, Data, Debug, Default)]
pub struct LayoutEngine {
    /// Deterministic text measurer shared with the rendering pass.
    #[get(type(copy))]
    pub(crate) measurer: DeterministicTextMeasurer,
    /// Statistics captured from the most recent layout pass.
    #[get(type(copy))]
    pub(crate) last_stats: LayoutStats,
}
