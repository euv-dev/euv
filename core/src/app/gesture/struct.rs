use super::*;

/// Deterministic single-source-of-truth gesture arbitration state machine enforcing strict input priority tiers.
#[derive(Clone, Data, Debug)]
pub(crate) struct GestureArena {
    /// Active exclusive gesture owner (`Idle` when no pointer sequence is underway).
    #[get(pub(crate), type(copy))]
    #[set(pub(crate))]
    pub(crate) active: ActiveGesture,
    /// Primary pointer identifier currently arbitrated by the arena.
    #[get(skip)]
    #[set(pub(crate))]
    pub(crate) primary_pointer_id: Option<u64>,
    /// Input device class of the primary contact.
    #[get(pub(crate), type(copy))]
    #[set(pub(crate))]
    pub(crate) primary_device: PointerDevice,
    /// Logical coordinates where the primary contact started.
    #[get(pub(crate), type(copy))]
    #[set(pub(crate))]
    pub(crate) origin: Point,
    /// Most recently observed logical coordinates for the primary contact.
    #[get(pub(crate), type(copy))]
    #[set(pub(crate))]
    pub(crate) last_pos: Point,
    /// Leaf scene node initially hit at `PointerDown`.
    #[get(skip)]
    #[set(pub(crate))]
    pub(crate) origin_hit: Option<NodeId>,
    /// Interactive scene node pressed at `PointerDown`.
    #[get(skip)]
    #[set(pub(crate))]
    pub(crate) pressed_target: Option<NodeId>,
    /// Eligible direct-drag control (`Canvas` or `on_pointer_down` + `on_pointer_move`) under `origin_hit`.
    #[get(skip)]
    #[set(pub(crate))]
    pub(crate) direct_drag_target: Option<NodeId>,
    /// Tier 0 topmost visible `Portal` or `Position::Fixed` barrier enclosing `origin_hit`, if any.
    #[get(skip)]
    #[set(pub(crate))]
    pub(crate) overlay_barrier: Option<NodeId>,
    /// Whether `Click` and `Tap` emission must be suppressed when the current pointer sequence ends.
    #[get(pub(crate), type(copy))]
    #[set(pub(crate))]
    pub(crate) suppress_click: bool,
    /// Whether this touch sequence arrested an active inertial scroll fling on `PointerDown`.
    #[get(pub(crate), type(copy))]
    #[set(pub(crate))]
    pub(crate) arrested_fling: bool,
    /// Whether horizontal route swipe is permitted when no horizontal scroll ancestor claims the drag.
    #[get(pub(crate), type(copy))]
    #[set(pub(crate))]
    pub(crate) route_swipe_allowed: bool,
    /// Monotonic runtime timestamp in milliseconds recorded at `PointerDown`.
    #[get(pub(crate), type(copy))]
    #[set(pub(crate))]
    pub(crate) started_ms: u64,
    /// Remaining wheel/trackpad scroll-burst isolation cooldown in seconds suppressing hover churn.
    #[get(pub(crate), type(copy))]
    #[set(pub(crate))]
    pub(crate) wheel_isolation_secs: f32,
    /// Per-pointer elapsed idle seconds since the last down or move event for stale-contact recovery.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) pointer_idle_secs: HashMap<u64, f32>,
}

/// Scene-resolved targets and callbacks computed inside a single `PointerDown` write borrow.
#[derive(Default)]
pub(crate) struct PointerDownPlan {
    /// Callbacks (`PointerEnter`, `PointerLeave`, `Focus`, `Blur`, `PointerDown`, virtual `Scroll`) to invoke outside the tree borrow.
    pub(crate) callbacks: Vec<(UiEventHandler, UiEvent)>,
    /// Scrollbar drag axis when a desktop scrollbar track was grabbed (`SCROLLBAR_AXIS_NONE` otherwise).
    pub(crate) scrollbar_axis: u8,
    /// Scrollbar container node handle when `scrollbar_axis != SCROLLBAR_AXIS_NONE`.
    pub(crate) scrollbar_node: Option<NodeId>,
    /// Whether the press landed on an enabled editable `NodeKind::Input`.
    pub(crate) tapped_enabled_input: bool,
    /// Started text-selection metadata for primary desktop text clicks.
    pub(crate) text_sel_started: Option<StartedTextSelection>,
    /// Leaf scene node initially hit by `hit_test`.
    pub(crate) origin_hit: Option<NodeId>,
    /// Interactive node marked pressed on `PointerDown`.
    pub(crate) press_target: Option<NodeId>,
    /// Eligible direct-drag control (`Canvas` or `on_pointer_down` + `on_pointer_move`) under `origin_hit`.
    pub(crate) direct_drag_target: Option<NodeId>,
    /// Enclosing Tier 0 `Portal` or `Position::Fixed` barrier, if any.
    pub(crate) overlay_barrier: Option<NodeId>,
    /// Whether horizontal route swipe is permitted if no horizontal scroll container claims the drag.
    pub(crate) route_swipe_allowed: bool,
    /// Whether global `Context::gesture` synthesis should be suppressed for this contact (canvas, direct drag, stop propagation).
    pub(crate) suppress_global_gestures: bool,
    /// Whether this press landed inside an actively flinging scroll container and arrested inertial momentum.
    pub(crate) arrested_fling: bool,
}
