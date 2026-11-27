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

/// Opt-in per-phase microsecond breakdown for a single `App::step_frame` execution.
#[derive(Clone, Copy, Data, Debug, Default, Eq, Hash, PartialEq)]
pub struct FramePhaseMetrics {
    /// Window command flushing, event polling, and input dispatch duration in microseconds.
    #[get(type(copy))]
    pub(crate) input_us: u64,
    /// Reactive effect flushing, animation/gesture/scroll ticking, and damage collection duration in microseconds.
    #[get(type(copy))]
    pub(crate) reactive_us: u64,
    /// `LayoutEngine::compute` pass duration in microseconds.
    #[get(type(copy))]
    pub(crate) layout_us: u64,
    /// Display-list compilation duration in microseconds when timed separately (`0` when combined into `raster_us`).
    #[get(type(copy))]
    pub(crate) paint_list_us: u64,
    /// Unified render pass (`Renderer::render_frame`) duration in microseconds.
    #[get(type(copy))]
    pub(crate) raster_us: u64,
    /// Native surface / framebuffer region presentation duration in microseconds.
    #[get(type(copy))]
    pub(crate) present_us: u64,
}

/// Unified per-frame diagnostic telemetry covering phase timings, damage ratio, redraw cause, and idle-aware FPS.
#[derive(Clone, Copy, Data, Debug, Default, PartialEq)]
pub struct FrameTelemetry {
    /// Monotonic frame identifier matching `FrameStats::get_frame_index()`.
    #[get(type(copy))]
    pub(crate) frame_id: u64,
    /// Microsecond breakdown across pipeline phases (zeroed when telemetry timing is disabled).
    #[get(type(copy))]
    pub(crate) phases: FramePhaseMetrics,
    /// Fraction of physical viewport area rasterized (`0.0` when idle, `(0.0..=1.0]` when rendered).
    #[get(type(copy))]
    pub(crate) dirty_area_ratio: f32,
    /// Diagnostic reason when the frame promoted to full-viewport redraw (`FullRedrawReason::None` for partial or idle).
    #[get(type(copy))]
    pub(crate) full_redraw_reason: FullRedrawReason,
    /// Effective target cadence in Hertz (`min(fps_limit, display_refresh_rate_hz)` or `display_refresh_rate_hz`).
    #[get(type(copy))]
    pub(crate) target_hz: f32,
    /// Active presented frames per second (`0.0` while idle, smoothed active present cadence while rendering).
    #[get(type(copy))]
    pub(crate) present_fps: f32,
    /// Whether this frame skipped rendering because the scene was clean or rate-capped.
    #[get(type(copy))]
    pub(crate) is_idle: bool,
    /// Number of scene nodes recomputed during layout.
    #[get(type(copy))]
    pub(crate) layout_nodes: usize,
    /// Number of display-list draw commands executed.
    #[get(type(copy))]
    pub(crate) draw_commands: usize,
    /// Number of physical pixels shaded.
    #[get(type(copy))]
    pub(crate) pixels_shaded: usize,
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
    /// Fixed 8-entry stack ring buffer of `(dt_secs, dx, dy)` frame samples preventing thumb-lift backward fling reversals.
    #[get(skip)]
    #[get_mut(skip)]
    #[set(skip)]
    pub(crate) touch_vel_ring: [(f32, f32, f32); 8],
    /// Ring buffer index of the oldest live velocity sample in `0..8`.
    #[get(type(copy))]
    pub(crate) touch_vel_head: usize,
    /// Number of live samples currently stored in `touch_vel_ring` (`0..=8`).
    #[get(type(copy))]
    pub(crate) touch_vel_len: usize,
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
    /// Opt-in flag enabling per-phase microsecond timing instrumentation inside `step_frame` (zero syscall overhead when `false`).
    #[get(type(copy))]
    #[set(skip)]
    pub(crate) telemetry_enabled: bool,
    /// Most recently recorded `FrameTelemetry` snapshot.
    #[get(type(copy))]
    pub(crate) last_frame_telemetry: FrameTelemetry,
    /// Pending diagnostic reason associated with `pending_damage` when full-viewport damage was queued before `step_frame`.
    #[get(type(copy))]
    pub(crate) pending_full_redraw_reason: FullRedrawReason,
    /// Logical viewport size rasterized during the most recent rendered frame.
    #[get(type(copy))]
    pub(crate) last_rendered_viewport: Size,
    /// Display DPI scale factor rasterized during the most recent rendered frame.
    #[get(type(copy))]
    pub(crate) last_rendered_scale: f32,
    /// Active presented frames-per-second rate (`0.0` once idle beyond the sampling window).
    #[get(type(copy))]
    pub(crate) active_present_fps: f32,
    /// Continuous idle seconds elapsed since the most recent rendered frame.
    #[get(type(copy))]
    pub(crate) idle_secs_since_present: f32,
    /// Whether the most recent `step_frame` call skipped rendering due to a clean idle scene.
    #[get(type(copy))]
    pub(crate) is_idle_frame: bool,
    /// Toggle enabling surgical pre/post-layout conservative bounds diffing instead of full-screen layout fallback (default `true`).
    #[get(type(copy))]
    #[set(skip)]
    pub(crate) partial_layout_damage: bool,
}

/// Fused single-borrow pre-tick snapshot collected at the start of `App::step_frame`.
#[derive(Debug, Default)]
pub(crate) struct FramePreTick {
    /// Whether a live `NodeKind::Input` currently holds keyboard focus.
    pub(crate) has_focused_input: bool,
    /// Whether `ime_node_id` holds a visible non-disabled input node requesting OS IME presentation.
    pub(crate) has_ime_node: bool,
    /// Focused input node handle requesting OS IME presentation when `has_ime_node` is `true`.
    pub(crate) ime_node_id: NodeId,
    /// Screen-space logical bounds of `ime_node_id`.
    pub(crate) ime_rect: Rect,
    /// Dirty viewport rectangles reported by active `GameEngineHost` instances.
    pub(crate) game_damage_rects: Vec<Rect>,
    /// Visible top-layer portals `(is_dirty, screen_bounds)` collected from `SceneTree::portal_nodes()`.
    pub(crate) visible_portals: Vec<(bool, Rect)>,
    /// Indices into `App::game_hosts` that are actively rendering this frame.
    pub(crate) active_host_indices: Vec<usize>,
    /// Resolved frame animation timestep in seconds.
    pub(crate) step_dt: f32,
    /// Whether one or more `NodeTransition`s ticked inside the fused borrow.
    pub(crate) anim_ticked: bool,
    /// Whether active transitions were deferred until after `GameEngineHost::step`.
    pub(crate) pending_game_transitions: bool,
}

/// Stack-first ancestor chain buffer avoiding heap allocations up to 32 tree levels deep.
#[derive(Debug)]
pub(crate) struct AncestorChain {
    /// Inline stack buffer holding the first 32 ancestors (leaf first).
    pub(crate) inline: [NodeId; 32],
    /// Number of valid entries in `inline` when `spill` is empty.
    pub(crate) len: usize,
    /// Heap overflow buffer used only when ancestor depth exceeds 32.
    pub(crate) spill: Vec<NodeId>,
}
