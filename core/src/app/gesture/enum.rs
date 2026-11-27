use super::*;

/// Exclusive gesture owner currently holding the active pointer stream in `GestureArena`.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub enum ActiveGesture {
    /// No pointer gesture is currently active.
    #[default]
    Idle,
    /// Primary contact began and stays within touch slop awaiting tap or directional disambiguation.
    Pending,
    /// Exclusive vertical scroll ownership claimed by the target scrollable container.
    VerticalScroll(NodeId),
    /// Exclusive horizontal scroll ownership claimed by the target scrollable container.
    HorizontalScroll(NodeId),
    /// Exclusive horizontal route or page swipe gesture driving `Context::swipe_drag_offset`.
    RouteSwipe,
    /// Exclusive direct drag interaction claimed by a slider, drawer/sheet grabber, or canvas control.
    DirectDrag(NodeId),
    /// Exclusive desktop scrollbar thumb drag claimed by a scroll container.
    ScrollbarDrag(NodeId),
    /// Exclusive pointer text-selection drag claimed by a selectable `NodeKind::Text` node.
    TextSelection(NodeId),
    /// Active multi-touch pinch gesture involving two or more simultaneous contacts.
    Pinch,
}

/// Arbitration outcome produced when evaluating touch movement against `GestureArena`.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum GestureResolution {
    /// Contact remains within touch slop or awaits dominant-axis hysteresis.
    StillPending,
    /// An exclusive gesture won ownership and competing press/tap/hover targets must cancel immediately.
    Won(ActiveGesture),
    /// Contact exceeded slop with no scroll or swipe winner, cancelling pending tap/click cleanly.
    CancelPending,
}
