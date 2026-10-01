use super::*;

/// Represents a single touch point from a multi-touch event.
///
/// Each `NativeTouchPoint` corresponds to one finger or stylus currently
/// touching the screen. The `identifier` field distinguishes between
/// simultaneous touch points, enabling multi-finger gesture tracking.
#[derive(Clone, Data, Debug, Default, Eq, PartialEq)]
pub struct NativeTouchPoint {
    /// A unique identifier for this touch point.
    ///
    /// The browser assigns a distinct `identifier` to each active touch.
    /// It remains constant for the duration of the touch (from
    /// `touchstart` to `touchend`), allowing the same finger to be
    /// tracked across `touchmove` events.
    #[get(type(copy))]
    pub identifier: i32,
    /// The X coordinate of the touch relative to the viewport.
    #[get(type(copy))]
    pub client_x: i32,
    /// The Y coordinate of the touch relative to the viewport.
    #[get(type(copy))]
    pub client_y: i32,
    /// The X coordinate of the touch relative to the screen.
    #[get(type(copy))]
    pub screen_x: i32,
    /// The Y coordinate of the touch relative to the screen.
    #[get(type(copy))]
    pub screen_y: i32,
    /// The X coordinate of the touch relative to the target element.
    #[get(type(copy))]
    pub offset_x: i32,
    /// The Y coordinate of the touch relative to the target element.
    #[get(type(copy))]
    pub offset_y: i32,
    /// The X coordinate of the touch relative to the page.
    #[get(type(copy))]
    pub page_x: i32,
    /// The Y coordinate of the touch relative to the page.
    #[get(type(copy))]
    pub page_y: i32,
}

/// Represents a single touch point with high-precision `f64` coordinates.
///
/// Used for pixel-precise interactions such as canvas drawing, where
/// sub-pixel accuracy matters. The `identifier` field distinguishes
/// between simultaneous touch points, enabling multi-finger gesture
/// tracking.
#[derive(Clone, Data, Debug, Default, PartialEq)]
pub struct NativeTouchPointF64 {
    /// A unique identifier for this touch point.
    #[get(type(copy))]
    pub identifier: i32,
    /// The X coordinate of the touch relative to the viewport.
    #[get(type(copy))]
    pub client_x: f64,
    /// The Y coordinate of the touch relative to the viewport.
    #[get(type(copy))]
    pub client_y: f64,
    /// The X coordinate of the touch relative to the screen.
    #[get(type(copy))]
    pub screen_x: f64,
    /// The Y coordinate of the touch relative to the screen.
    #[get(type(copy))]
    pub screen_y: f64,
    /// The X coordinate of the touch relative to the target element.
    #[get(type(copy))]
    pub offset_x: f64,
    /// The Y coordinate of the touch relative to the target element.
    #[get(type(copy))]
    pub offset_y: f64,
    /// The X coordinate of the touch relative to the page.
    #[get(type(copy))]
    pub page_x: f64,
    /// The Y coordinate of the touch relative to the page.
    #[get(type(copy))]
    pub page_y: f64,
}

/// A two-finger pinch described by the distance and midpoint of the two
/// touch points.
///
/// The pair is emitted on every `touchmove` while exactly two fingers are
/// down, so consumers can drive a continuous zoom rather than reacting to a
/// discrete end-of-gesture event.
#[derive(Clone, Copy, Data, Debug, Default, PartialEq)]
pub struct EuvPinch {
    /// Distance in CSS pixels between the two touch points.
    #[get(type(copy))]
    pub distance: f64,
    /// Distance in CSS pixels when the pinch started, or the previous
    /// distance when continuing an in-progress pinch.
    #[get(type(copy))]
    pub start_distance: f64,
    /// Midpoint X in client coordinates.
    #[get(type(copy))]
    pub center_x: f64,
    /// Midpoint Y in client coordinates.
    #[get(type(copy))]
    pub center_y: f64,
    /// Change in distance since `start_distance`.
    ///
    /// Positive values mean the fingers spread apart (zoom in), negative
    /// values mean they moved together (zoom out).
    #[get(type(copy))]
    pub delta: f64,
}

/// A single-finger drag described by the current position and the movement
/// accumulated since the drag started.
#[derive(Clone, Copy, Data, Debug, Default, PartialEq)]
pub struct EuvDrag {
    /// Current X in client coordinates.
    #[get(type(copy))]
    pub x: f64,
    /// Current Y in client coordinates.
    #[get(type(copy))]
    pub y: f64,
    /// X displacement since the drag started.
    #[get(type(copy))]
    pub delta_x: f64,
    /// Y displacement since the drag started.
    #[get(type(copy))]
    pub delta_y: f64,
    /// Total path length in CSS pixels, not the straight-line distance.
    ///
    /// A finger that traces a loop has a large path length and zero delta,
    /// which is how a drag is told apart from a tap that wandered slightly.
    #[get(type(copy))]
    #[get(type(copy))]
    pub travel: f64,
}

/// Tunable thresholds for [`EuvGestureRecognizer`].
///
/// Every field is public so a caller can tighten recognition for a
/// carousel without affecting the default used elsewhere.
#[derive(Clone, Copy, Data, Debug, PartialEq)]
pub struct EuvGestureConfig {
    /// Minimum horizontal or vertical travel, in CSS pixels, before a touch
    /// is classified as a swipe instead of a tap.
    #[get(type(copy))]
    pub swipe_threshold: f64,
    /// Maximum travel, in CSS pixels, for a touch to still count as a tap
    /// or long press.
    #[get(type(copy))]
    pub tap_slop: f64,
    /// Duration, in milliseconds, that turns a stationary touch into a long
    /// press.
    #[get(type(copy))]
    pub long_press_millis: f64,
    /// Minimum relative scale change before a pinch step is reported as
    /// meaningful. Suppresses sub-percent jitter between adjacent moves.
    #[get(type(copy))]
    pub pinch_threshold: f64,
}

/// Live gesture state produced by [`EuvGestureRecognizer::use_gesture`].
///
/// The three continuous signals are always mounted. The discrete
/// `last_gesture` signal only changes when a gesture completes, so a view
/// that renders it does not re-render on every `touchmove`.
#[derive(Clone, Data, New)]
pub struct EuvGestureState {
    /// The most recently completed single-finger gesture, or `None` when no
    /// gesture has completed since the hook was mounted.
    #[get(type(copy))]
    pub last_gesture: Signal<Option<EuvGesture>>,
    /// The in-progress drag, or `None` when no finger is dragging.
    #[get(type(copy))]
    pub drag: Signal<Option<EuvDrag>>,
    /// The in-progress two-finger pinch, or `None` when fewer than two
    /// fingers are down.
    #[get(type(copy))]
    pub pinch: Signal<Option<EuvPinch>>,
    /// Handler for `ontouchstart`; begins a new gesture sequence.
    pub on_start: Option<Rc<dyn Fn(Event)>>,
    /// Handler for `ontouchmove`; updates the drag and pinch signals.
    pub on_move: Option<Rc<dyn Fn(Event)>>,
    /// Handler for `ontouchend`; completes the gesture.
    pub on_end: Option<Rc<dyn Fn(Event)>>,
    /// Handler for `ontouchcancel`; abandons the gesture.
    pub on_cancel: Option<Rc<dyn Fn(Event)>>,
}

/// Stateless gesture classifier driven by [`EuvGestureState`].
///
/// The recognizer holds only the configured thresholds; all mutable
/// per-gesture progress lives in signals owned by the hook, so a view can
/// read the latest result without the classifier itself being reactive.
///
/// # Examples
///
/// ```
/// use euv_ui::EuvGestureRecognizer;
///
/// let recognizer: EuvGestureRecognizer = EuvGestureRecognizer::new();
/// assert_eq!(recognizer.classify(120.0, 5.0, 80.0, 120.5), Some(euv_ui::EuvGesture::Right));
/// ```
#[derive(Clone, Copy, Data)]
pub struct EuvGestureRecognizer {
    /// The thresholds applied to every classification decision.
    #[get(type(copy))]
    pub config: EuvGestureConfig,
}

/// Per-touch progress for one in-flight gesture sequence.
///
/// This is deliberately not a signal: it changes on every `touchmove` and
/// nothing in a view should re-render until a gesture actually completes.
#[derive(Clone, Copy, Data, Debug, Default)]
pub struct GestureProgress {
    /// X of the first finger when the sequence began.
    #[get(type(copy))]
    pub start_x: f64,
    /// Y of the first finger when the sequence began.
    #[get(type(copy))]
    pub start_y: f64,
    /// X of the most recently seen position of the first finger.
    #[get(type(copy))]
    pub last_x: f64,
    /// Y of the most recently seen position of the first finger.
    #[get(type(copy))]
    pub last_y: f64,
    /// Accumulated path length in CSS pixels.
    pub travel: f64,
    /// Distance between the two fingers when a pinch began.
    #[get(type(copy))]
    pub pinch_start_distance: f64,
    /// Timestamp of `touchstart`, in milliseconds.
    #[get(type(copy))]
    pub started_at: f64,
    /// Whether a sequence is currently being tracked.
    #[get(type(copy))]
    pub active: bool,
}

/// The live position of the tracked finger, in client coordinates.
///
/// A plain value type rather than a borrow of the event's touch list, so the
/// recognizer can be exercised without constructing DOM events.
#[derive(Clone, Copy, Data, Debug, Default, PartialEq)]
pub struct GesturePoint {
    /// X in client coordinates.
    #[get(type(copy))]
    pub client_x: f64,
    /// Y in client coordinates.
    #[get(type(copy))]
    pub client_y: f64,
}
