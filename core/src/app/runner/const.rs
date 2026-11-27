/// Minimum touch contact travel in logical pixels before a tap transitions into pan scrolling.
pub(crate) const TOUCH_SLOP_DP: f32 = 12.0;

/// Minimum directional displacement in logical pixels required to recognize an intentional swipe gesture.
pub(crate) const SWIPE_MIN_DISTANCE_DP: f32 = 84.0;

/// Minimum primary-to-cross-axis displacement ratio required to classify a drag as an intentional directional swipe.
pub(crate) const SWIPE_DIRECTION_RATIO: f32 = 2.0;

/// Maximum perpendicular drift in logical pixels allowed during a directional swipe gesture.
pub(crate) const SWIPE_MAX_CROSS_AXIS_DP: f32 = 52.0;

/// Minimum post-slop touch drag travel in logical pixels required before releasing inertial fling momentum.
pub(crate) const TOUCH_MIN_FLING_DRAG_DP: f32 = 16.0;

/// Minimum release velocity in logical pixels per second required to launch touch scroll inertia.
pub(crate) const TOUCH_MIN_FLING_SPEED_DP_S: f32 = 95.0;

/// Native action argument instructing the host OS to display the system soft keyboard / IME.
pub(crate) const IME_ACTION_SHOW: &str = "show";

/// Native action argument instructing the host OS to dismiss the system soft keyboard / IME.
pub(crate) const IME_ACTION_HIDE: &str = "hide";
