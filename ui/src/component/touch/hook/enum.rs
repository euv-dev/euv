use super::*;

/// A single-finger directional gesture resolved from a completed touch
/// sequence.
///
/// A value is only produced once a touch has started and ended; the
/// recognition thresholds live on [`EuvGestureConfig`].
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum EuvGesture {
    /// The finger travelled past `swipe_threshold` mostly leftwards.
    Left,
    /// The finger travelled past `swipe_threshold` mostly rightwards.
    Right,
    /// The finger travelled past `swipe_threshold` mostly upwards.
    Up,
    /// The finger travelled past `swipe_threshold` mostly downwards.
    Down,
    /// The finger stayed within `tap_slop` of the origin and the touch was
    /// shorter than `long_press_millis`.
    Tap,
    /// The finger stayed within `tap_slop` of the origin but the touch lasted
    /// at least `long_press_millis`.
    LongPress,
}
