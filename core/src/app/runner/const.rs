/// Minimum touch contact travel in logical pixels before a tap transitions into pan scrolling.
pub(crate) const TOUCH_SLOP_DP: f32 = 10.0;

/// Minimum post-slop touch drag travel in logical pixels required before releasing inertial fling momentum.
pub(crate) const TOUCH_MIN_FLING_DRAG_DP: f32 = 16.0;

/// Minimum release velocity in logical pixels per second required to launch touch scroll inertia.
pub(crate) const TOUCH_MIN_FLING_SPEED_DP_S: f32 = 95.0;

/// Native action argument instructing the host OS to display the system soft keyboard / IME.
pub(crate) const IME_ACTION_SHOW: &str = "show";

/// Native action argument instructing the host OS to dismiss the system soft keyboard / IME.
pub(crate) const IME_ACTION_HIDE: &str = "hide";
