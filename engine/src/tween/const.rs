/// The default playback direction of a tween (forwards).
pub(crate) const TWEEN_DIRECTION_FORWARD: f64 = 1.0;

/// The reversed playback direction used by ping-pong tweens.
pub(crate) const TWEEN_DIRECTION_BACKWARD: f64 = -1.0;

/// The struct name used by the `Debug` implementation of `Tween`.
pub(crate) const TWEEN_DEBUG_NAME: &str = "Tween";

/// The `Debug` field name of the tween start value.
pub(crate) const TWEEN_FIELD_FROM: &str = "from";

/// The `Debug` field name of the tween end value.
pub(crate) const TWEEN_FIELD_TO: &str = "to";

/// The `Debug` field name of the tween duration.
pub(crate) const TWEEN_FIELD_DURATION: &str = "duration";

/// The `Debug` field name of the tween easing curve.
pub(crate) const TWEEN_FIELD_EASING: &str = "easing";

/// The `Debug` field name of the tween start delay.
pub(crate) const TWEEN_FIELD_DELAY: &str = "delay";

/// The `Debug` field name of the tween elapsed time.
pub(crate) const TWEEN_FIELD_ELAPSED: &str = "elapsed";

/// The `Debug` field name of the tween playback state.
pub(crate) const TWEEN_FIELD_STATE: &str = "state";

/// The `Debug` field name of the tween completion mode.
pub(crate) const TWEEN_FIELD_MODE: &str = "mode";

/// The `Debug` field name of the tween playback direction.
pub(crate) const TWEEN_FIELD_DIRECTION: &str = "direction";
