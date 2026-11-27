use super::*;

/// Flat generation-indexed arena holding the entire retained UI scene graph.
#[derive(CustomDebug, Data)]
pub struct SceneTree {
    /// Contiguous node storage vector.
    pub(crate) nodes: Vec<SceneNode>,
    /// Recycled slot indices for O(1) node allocation.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) free_list: Vec<u32>,
    /// Incrementally maintained count of live nodes for O(1) `live_node_count()`.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) live_count: usize,
    /// Root scene node handle.
    #[get(type(copy))]
    pub(crate) root: NodeId,
    /// Focus, hover, and pointer-press tracking state.
    #[get(type(copy))]
    pub(crate) focus: FocusManager,
    /// Monotonic structural/visual mutation counter.
    #[get(type(copy))]
    pub(crate) revision: u64,
    /// Active paint/transform node transitions ticked on every frame until settled.
    pub(crate) transitions: Vec<NodeTransition>,
    /// Blinking visibility state of the focused text input caret (`true` = visible phase).
    #[get(type(copy))]
    pub(crate) caret_visible: bool,
    /// Incrementally maintained registry of live `NodeKind::Portal` and `Position::Fixed` nodes.
    #[get(skip)]
    #[get_mut(pub(crate))]
    #[set(skip)]
    pub(crate) portal_nodes: Vec<NodeId>,
    /// Bounded list of nodes marked paint/scroll dirty since the last `collect_paint_damage` pass.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) dirty_paint_nodes: Vec<NodeId>,
    /// Flag raised when structural/layout mutations or >64 dirty nodes require full viewport damage.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) full_damage_pending: bool,
    /// Flag raised when `dirty_paint_nodes` exceeded `DIRTY_PAINT_FULL_THRESHOLD` since the last collection.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) full_damage_overflow: bool,
    /// Active native text selection state on a selectable `NodeKind::Text` node, if any.
    #[get(skip)]
    #[get_mut(skip)]
    #[set(skip)]
    pub(crate) text_selection: Option<SelectionState>,
}

/// Shared handle wrapping a `SceneTree` so reactive `Effect` closures can patch nodes directly.
#[derive(Clone, CustomDebug)]
pub struct SceneHandle {
    /// Shared interior-mutable reference to the thread-local scene tree.
    pub(crate) inner: Rc<RefCell<SceneTree>>,
}

/// Ambient framework-managed UI context binding the active `SceneHandle`, responsive signals, and platform metadata.
#[derive(Clone, CustomDebug, Data)]
pub struct Context {
    /// Shared scene tree handle for creating and binding nodes.
    pub(crate) scene: SceneHandle,
    /// Current viewport logical breakpoint signal for responsive layouts.
    #[get(type(copy))]
    pub(crate) breakpoint: Signal<Breakpoint>,
    /// Current logical viewport dimensions signal updated immediately on window resize.
    #[get(type(copy))]
    pub(crate) viewport_size: Signal<Size>,
    /// Current OS/app theme mode signal (`Light` / `Dark`).
    #[get(type(copy))]
    pub(crate) theme_mode: Signal<ThemeMode>,
    /// Active platform kind for capability-aware adaptation.
    #[get(type(copy))]
    pub(crate) platform: PlatformKind,
    /// Active safe-area insets signal.
    #[get(type(copy))]
    pub(crate) safe_area: Signal<SafeAreaInsets>,
    /// Reactive stream of recognized cross-platform gestures (`Tap`, `DoubleTap`, `LongPress`, `Swipe*`, `Pan`, `Pinch`).
    #[get(type(copy))]
    pub(crate) gesture: Signal<Option<GestureEvent>>,
    /// Live horizontal touch-swipe drag displacement in logical pixels (`0.0` when idle) for interactive page-swipe feedback.
    #[get(type(copy))]
    pub(crate) swipe_drag_offset: Signal<f32>,
    /// Monotonic counter incremented on OS Back button / Escape / edge-back triggers for unified overlay & router stack dismissal.
    #[get(type(copy))]
    pub(crate) system_back: Signal<u64>,
    /// Shared dynamic update hub enabling ambient component-level and page-level live updates.
    pub(crate) dynamic_hub: DynamicUpdateHub,
    /// Optional target FPS cap across all platforms (`None` = unlimited maximum FPS by default).
    #[get(type(copy))]
    pub(crate) fps_limit: Signal<Option<u32>>,
    /// Smoothed live rendered frames-per-second telemetry signal.
    #[get(type(copy))]
    pub(crate) live_fps: Signal<f32>,
    /// Idle-aware active presented frames-per-second signal (`0.0` while idle).
    #[get(type(copy))]
    pub(crate) present_fps: Signal<f32>,
    /// Reactive flag indicating whether the frame loop is currently idle (`true` when skipping clean frames).
    #[get(type(copy))]
    pub(crate) is_idle: Signal<bool>,
    /// Native hardware display refresh rate in Hertz reported by the active OS/browser host.
    #[get(type(copy))]
    pub(crate) display_refresh_rate: Signal<f32>,
    /// Live snapshot of all currently active touch/pointer contacts `(pointer_id, position)` for multi-touch UIs.
    #[get(type(copy))]
    pub(crate) active_touches: Signal<Vec<(u64, Point)>>,
    /// Reactive flag raised when the application requests graceful termination (e.g. double-back-to-exit at root).
    #[get(type(copy))]
    pub(crate) exit_requested: Signal<bool>,
    /// Canonical cross-platform application lifecycle state signal (`Active`, `Inactive`, `Background`, `Terminated`).
    #[get(type(copy))]
    pub(crate) app_state: Signal<AppState>,
    /// Live network reachability status signal (`Online`, `Offline`, `Wifi`, `Cellular`, `Unknown`).
    #[get(type(copy))]
    pub(crate) network_status: Signal<NetworkStatus>,
    /// Whether the soft virtual keyboard / IME is currently presented.
    #[get(type(copy))]
    pub(crate) keyboard_visible: Signal<bool>,
    /// Current soft virtual keyboard height in logical pixels (`0.0` when hidden).
    #[get(type(copy))]
    pub(crate) keyboard_height: Signal<f32>,
    /// Current screen or window orientation signal.
    #[get(type(copy))]
    pub(crate) orientation: Signal<ScreenOrientation>,
    /// Whether OS battery saver / low-power mode is currently enabled.
    #[get(type(copy))]
    pub(crate) low_power_mode: Signal<bool>,
    /// Active OS/browser preferred BCP-47 locale string signal.
    #[get(type(copy))]
    pub(crate) locale: Signal<String>,
    /// Whether the primary application window currently holds keyboard/input focus.
    #[get(type(copy))]
    pub(crate) window_focused: Signal<bool>,
    /// Monotonic runtime clock in milliseconds advanced deterministically by `App` frame steps.
    #[debug(skip)]
    #[get(skip)]
    pub(crate) clock_ms: Rc<Cell<u64>>,
}

/// RAII scope guard restoring the previous ambient `Context` on drop.
#[derive(CustomDebug, New)]
pub struct ContextGuard {
    /// Previously bound ambient context to restore when leaving scope.
    pub(crate) previous: Option<Context>,
}
