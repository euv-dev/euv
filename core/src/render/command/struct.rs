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
}

/// Stateless scene-to-display-list compiler supporting z-order sorting, scroll clipping, and frustum culling.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct PaintPass;
