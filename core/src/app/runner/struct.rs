use super::*;

/// Top-level cross-platform application configuration.
#[derive(Clone, Data, Debug, PartialEq)]
pub struct AppConfig {
    /// Primary window creation settings.
    pub(crate) window: WindowConfig,
    /// Clear background color drawn beneath the root scene node.
    #[get(type(copy))]
    pub(crate) clear_color: Color,
}

/// Unified cross-platform application runtime orchestrating platform events, reactivity, layout, and rendering.
#[derive(CustomDebug, Data)]
pub struct App {
    /// Application configuration.
    pub(crate) config: AppConfig,
    /// Active direct OS or headless platform backend.
    #[debug(skip)]
    #[get(skip)]
    pub(crate) platform: Box<dyn PlatformBackend>,
    /// Primary window live state.
    #[get(type(copy))]
    pub(crate) window_state: WindowState,
    /// Shared scene tree handle.
    pub(crate) scene: SceneHandle,
    /// Framework context carrying reactive breakpoint, theme, gesture, and safe-area signals.
    pub(crate) context: Context,
    /// Deterministic cross-platform layout engine.
    pub(crate) layout_engine: LayoutEngine,
    /// Multi-mode rendering pipeline (`Auto`, `Software`, `Hardware`, `Hybrid`).
    pub(crate) renderer: Renderer,
    /// Queued imperative window commands awaiting dispatch.
    pub(crate) command_queue: WindowCommandQueue,
    /// Local key-value storage service.
    pub(crate) storage: StorageStore,
    /// Flag indicating whether the application loop should keep running.
    #[get(type(copy))]
    pub(crate) running: bool,
    /// Pending damage region for the next rendered frame.
    #[get(type(copy))]
    pub(crate) pending_damage: DamageRegion,
    /// Real-time polled keyboard, pointer, and scroll state for game loops and interactive viewports.
    pub(crate) input_state: InputState,
    /// Fixed-timestep accumulator and FPS telemetry clock for real-time game rendering.
    #[get(type(copy))]
    pub(crate) frame_clock: FrameClock,
    /// Platform-native services hub (permissions, protocol registration, Dynamic Island, haptics, tray, A11y).
    pub(crate) services: PlatformServiceHub,
    /// Reactive signal carrying the most recently received custom-scheme or universal deep link.
    #[get(type(copy))]
    pub(crate) deep_link: Signal<Option<DeepLinkEvent>>,
    /// Shared dynamic update hub for live component-level and page-level OTA updates.
    pub(crate) dynamic_hub: DynamicUpdateHub,
    /// Last scene revision that was rasterized and presented, enabling zero-cost idle frame skipping.
    #[get(type(copy))]
    pub(crate) last_rendered_revision: u64,
    /// Most recently submitted OS/Web cursor icon to avoid redundant platform cursor calls.
    #[get(type(copy))]
    pub(crate) last_cursor: CursorIcon,
    /// Attached real-time game engine hosts receiving automatic OS input forwarding.
    #[debug(skip)]
    pub(crate) game_hosts: Vec<GameEngineHost>,
    /// Active touch pointer identifier currently driving single-finger pan scrolling.
    #[get(skip)]
    pub(crate) touch_drag_pointer_id: Option<u64>,
    /// Starting touch coordinate for mobile pan-to-scroll detection.
    #[get(skip)]
    pub(crate) touch_drag_origin: Option<Point>,
    /// Previous touch coordinate for incremental mobile scroll deltas.
    #[get(skip)]
    pub(crate) touch_drag_last: Option<Point>,
    /// Whether the active touch gesture has crossed tap slop into pan-scrolling.
    #[get(type(copy))]
    pub(crate) touch_dragging: bool,
    /// Cumulative post-slop drag displacement in logical pixels for the active touch scroll gesture.
    #[get(type(copy))]
    pub(crate) touch_drag_travel: f32,
    /// Locked scroll axis for the active touch drag (`0` = unlocked, `1` = vertical, `2` = horizontal, `3` = free).
    #[get(type(copy))]
    pub(crate) touch_scroll_axis: u8,
    /// Touch pan displacement accumulated during the current frame for jitter-free velocity estimation.
    #[get(type(copy))]
    pub(crate) touch_vel_accum_delta: Point,
    /// Rolling window of recent `(dt_secs, dx, dy)` frame samples preventing thumb-lift backward fling reversals.
    pub(crate) touch_vel_samples: Vec<(f32, f32, f32)>,
    /// Elapsed idle seconds since the last touch pan movement while a finger remains held down.
    #[get(type(copy))]
    pub(crate) touch_vel_idle_secs: f32,
    /// Whether the active pointer press grabbed a scrollbar track for thumb dragging.
    #[get(type(copy))]
    pub(crate) scrollbar_dragging: bool,
    /// Active scrollbar thumb drag axis (`0` = none, `1` = vertical, `2` = horizontal).
    #[get(type(copy))]
    pub(crate) scrollbar_drag_axis: u8,
    /// Active selectable `NodeKind::Text` node currently undergoing pointer drag selection.
    #[get(skip)]
    pub(crate) text_selecting_node: Option<NodeId>,
    /// Starting character index anchored at pointer-down for drag text selection.
    #[get(type(copy))]
    pub(crate) text_select_anchor_char: usize,
    /// Consecutive click count on the same selectable text node (`1` = caret, `2` = word, `3` = full node).
    #[get(type(copy))]
    pub(crate) text_click_count: u8,
    /// Most recently clicked selectable text node for multi-click detection.
    #[get(skip)]
    pub(crate) text_last_click_node: Option<NodeId>,
    /// Coordinates of the most recent click on a selectable text node.
    #[get(skip)]
    pub(crate) text_last_click_pos: Option<Point>,
    /// Elapsed seconds since the most recent click on a selectable text node.
    #[get(type(copy))]
    pub(crate) text_last_click_age_secs: f32,
    /// Active multi-touch / pointer contact map keyed by `pointer_id` for pinch & swipe recognition.
    pub(crate) active_pointers: HashMap<u64, Point>,
    /// Device kind recorded per active `pointer_id` for deterministic touch vs mouse separation.
    pub(crate) active_pointer_devices: HashMap<u64, PointerDevice>,
    /// Initial contact point of the active single-pointer gesture stroke.
    #[get(skip)]
    pub(crate) gesture_origin: Option<Point>,
    /// Baseline two-finger distance recorded when a pinch gesture starts.
    #[get(type(copy))]
    pub(crate) pinch_base_dist: f32,
    /// Accumulated hold duration in seconds for long-press gesture detection.
    #[get(type(copy))]
    pub(crate) gesture_hold_secs: f32,
    /// Whether `GestureEventKind::LongPress` has already fired for the current press.
    #[get(type(copy))]
    pub(crate) long_press_fired: bool,
    /// Coordinates of the most recent completed clean tap for double-tap detection.
    #[get(skip)]
    pub(crate) last_tap_pos: Option<Point>,
    /// Elapsed seconds since the most recent completed clean tap.
    #[get(type(copy))]
    pub(crate) last_tap_age_secs: f32,
    /// Monotonic gesture event sequence counter.
    #[get(type(copy))]
    pub(crate) gesture_seq: u64,
    /// Inertial touch-scroll fling velocity in logical pixels per second.
    #[get(type(copy))]
    pub(crate) scroll_velocity: Point,
    /// Pointer coordinates where inertial touch-scroll fling originated for hit-target resolution.
    #[get(type(copy))]
    pub(crate) scroll_fling_pos: Point,
    /// Locked innermost scrollable node captured at touch-down to prevent nested scroll penetration.
    #[get(skip)]
    pub(crate) touch_scroll_target: Option<NodeId>,
    /// Remaining smooth-scroll interpolation delta for discrete mouse-wheel notches.
    #[get(type(copy))]
    pub(crate) smooth_scroll_pending: Point,
    /// Pointer coordinates captured when smooth wheel scrolling started.
    #[get(type(copy))]
    pub(crate) smooth_scroll_pos: Point,
    /// Locked scroll container target for active smooth wheel interpolation.
    #[get(skip)]
    pub(crate) smooth_scroll_target: Option<NodeId>,
    /// Re-entrancy guard preventing nested `step_frame` execution during macOS live-resize callbacks.
    #[get(type(copy))]
    pub(crate) in_step_frame: bool,
    /// Accumulated blink phase timer in seconds for the focused `NodeKind::Input` caret.
    #[get(type(copy))]
    pub(crate) caret_blink_secs: f32,
    /// When `true`, global swipe gestures are suppressed because the active stroke started in a canvas, slider, or isolated region.
    #[get(type(copy))]
    pub(crate) gesture_suppressed: bool,
    /// Elapsed seconds accumulated for smoothed live FPS telemetry publication.
    #[get(type(copy))]
    pub(crate) fps_accum_secs: f32,
    /// Rendered frame count accumulated inside the current FPS sampling window.
    #[get(type(copy))]
    pub(crate) fps_accum_frames: u32,
    /// Accumulated frame time in seconds used to pace frames when an explicit FPS cap is configured.
    #[get(type(copy))]
    pub(crate) fps_budget_accum: f32,
    /// Whether an editable `NodeKind::Input` currently holds focus and requests the system IME keyboard.
    #[get(type(copy))]
    pub(crate) last_ime_active: bool,
    /// Handle of the `NodeKind::Input` that most recently activated the system IME keyboard.
    #[get(skip)]
    pub(crate) last_ime_node: Option<NodeId>,
    /// Damage region rasterized during the most recent rendered frame for partial native surface presentation.
    #[get(type(copy))]
    pub(crate) last_frame_damage: DamageRegion,
    /// Detected native hardware display refresh rate in Hertz (`60.0` fallback, `120.0`/`144.0`/`180.0`+ on high-refresh screens).
    #[get(type(copy))]
    pub(crate) display_refresh_rate_hz: f32,
    /// Whether motion animations, smooth scrolling, inertia, or active games were running on the previous frame.
    #[get(type(copy))]
    pub(crate) was_animating: bool,
    /// Cross-platform lifecycle and system event normalizer deduplicating OS notifications.
    pub(crate) system_event_normalizer: SystemEventNormalizer,
}
