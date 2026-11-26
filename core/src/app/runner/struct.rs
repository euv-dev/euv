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
    /// Build context carrying reactive breakpoint, theme, and safe-area signals.
    pub(crate) build_context: BuildContext,
    /// Deterministic cross-platform layout engine.
    #[get(type(copy))]
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
    /// Starting touch coordinate for mobile pan-to-scroll detection.
    #[get(skip)]
    pub(crate) touch_drag_origin: Option<Point>,
    /// Previous touch coordinate for incremental mobile scroll deltas.
    #[get(skip)]
    pub(crate) touch_drag_last: Option<Point>,
    /// Whether the active touch gesture has crossed tap slop into pan-scrolling.
    #[get(type(copy))]
    pub(crate) touch_dragging: bool,
    /// Whether the active pointer press grabbed a right-edge scrollbar track for thumb dragging.
    #[get(type(copy))]
    pub(crate) scrollbar_dragging: bool,
    /// Active multi-touch / pointer contact map keyed by `pointer_id` for pinch & swipe recognition.
    pub(crate) active_pointers: HashMap<u64, Point>,
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
}
