use super::*;

/// Started pointer text-selection payload `(node_id, anchor_char, click_count, drag_active)`.
pub(crate) type StartedTextSelection = (NodeId, usize, u8, bool);

/// Result of pointer-down scene mutation `(callbacks, scrollbar_axis, tapped_input, started_selection)`.
pub(crate) type PointerDownOutcome = (
    Vec<(UiEventHandler, UiEvent)>,
    u8,
    bool,
    Option<StartedTextSelection>,
);

/// Scroll-chain step evaluation `(scroll_update, next_parent, blocks_scroll, virtual_stop)`.
pub(crate) type ScrollStepOutcome = (
    Option<(Point, f32, f32)>,
    Option<NodeId>,
    bool,
    Option<bool>,
);
