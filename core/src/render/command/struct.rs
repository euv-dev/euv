use super::*;

/// Reusable backend-agnostic command buffer emitted once per frame for Software, Hardware, or Hybrid execution.
#[derive(Clone, Data, Debug, Default, PartialEq)]
pub struct DisplayList {
    /// Ordered drawing commands.
    pub(crate) commands: Vec<DrawCommand>,
    /// Logical viewport size associated with this display list.
    #[get(type(copy))]
    pub(crate) viewport: Size,
    /// Scale factor mapping logical coordinates to physical pixels.
    #[get(type(copy))]
    pub(crate) scale_factor: f32,
    /// Command index separating base-scene commands `[0..portal_split_index]` from top-layer portal/fixed overlays.
    #[get(type(copy))]
    pub(crate) portal_split_index: usize,
    /// Persistent scratch queue collecting deferred top-layer portal/fixed nodes across frames (cleared after `PaintPass::build`).
    #[get(skip)]
    #[get_mut(skip)]
    #[set(skip)]
    pub(crate) portals_scratch: Vec<(i32, usize, NodeId)>,
    /// Persistent stack-allocated scratch buffer for nested z-index child sorting across frames (cleared after `PaintPass::build`).
    #[get(skip)]
    #[get_mut(skip)]
    #[set(skip)]
    pub(crate) z_sort_scratch: Vec<(i32, usize, NodeId)>,
}

/// Stateless scene-to-display-list compiler supporting z-order sorting, scroll clipping, and frustum culling.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct PaintPass;

/// Borrowed reusable scratch buffers threaded through recursive `PaintPass::emit_node` calls.
#[derive(Debug)]
pub(crate) struct PaintScratch<'a> {
    /// Optional deferred top-layer portal queue (`None` while painting top-layer portals).
    pub(crate) portals: Option<&'a mut Vec<(i32, usize, NodeId)>>,
    /// Stack-sliced scratch vector reused for nested z-index child sorting.
    pub(crate) z_sort: &'a mut Vec<(i32, usize, NodeId)>,
}
