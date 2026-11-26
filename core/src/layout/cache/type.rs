use super::*;

/// Cloned node snapshot extracted before recursive child layout traversal.
pub(crate) type LayoutNodeSnapshot = (
    Style,
    NodeKind,
    DirtyFlags,
    Option<AvailableSpace>,
    Rect,
    Vec<NodeId>,
);
