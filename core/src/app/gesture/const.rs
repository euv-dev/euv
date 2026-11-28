/// Minimum logical touch displacement in pixels before gesture disambiguation resolves out of `Pending`.
pub(crate) const GESTURE_TOUCH_SLOP_DP: f32 = 10.0;

/// Fallback logical touch displacement in pixels at which ambiguous 45-degree motion commits to the larger axis.
pub(crate) const GESTURE_DIAGONAL_RESOLVE_DP: f32 = 15.0;

/// Minimum dominant-to-cross-axis displacement ratio required for clean axis-locked gesture disambiguation.
pub(crate) const GESTURE_AXIS_HYSTERESIS_RATIO: f32 = 1.25;

/// Minimum inertial fling speed in logical pixels per second at which a `PointerDown` arrests the fling and suppresses `Click`.
pub(crate) const GESTURE_FLING_TAP_STOP_SPEED_DP_S: f32 = 25.0;

/// Hover-suppression cooldown in seconds following a wheel or trackpad `WindowEvent::Scroll` burst.
pub(crate) const GESTURE_WHEEL_ISOLATION_SECS: f32 = 0.14;

/// Maximum inactivity duration in seconds before an un-released OS-swallowed pointer contact is pruned on a fresh primary contact.
pub(crate) const GESTURE_STALE_POINTER_TIMEOUT_SECS: f32 = 2.0;

/// Minimum pointer displacement in logical pixels before a pressed direct-drag control promotes from `Pending` to `DirectDrag`.
pub(crate) const GESTURE_DIRECT_DRAG_PROMOTE_DP: f32 = 2.0;
