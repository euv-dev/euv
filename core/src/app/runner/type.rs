use super::*;

/// Started pointer text-selection payload `(node_id, anchor_char, click_count, drag_active)`.
pub(crate) type StartedTextSelection = (NodeId, usize, u8, bool);

/// Scroll-chain step evaluation `(scroll_update, next_parent, blocks_scroll, virtual_stop)`.
pub(crate) type ScrollStepOutcome = (
    Option<(Point, f32, f32)>,
    Option<NodeId>,
    bool,
    Option<bool>,
);

/// Platform-portable opt-in frame phase timestamp marker (`Option<Instant>` natively, zero-sized on WASM).
#[cfg(not(target_arch = "wasm32"))]
pub(crate) type PhaseMark = Option<Instant>;

/// Platform-portable opt-in frame phase timestamp marker (`Option<()>` on WASM where `Instant::now` is unavailable).
#[cfg(target_arch = "wasm32")]
pub(crate) type PhaseMark = Option<()>;
