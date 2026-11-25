/// Classified cross-platform pointer/touch/stylus gesture.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum GestureKind {
    /// Clean tap (bounded net displacement and bounded total path length).
    Tap,
    /// Horizontal leftward swipe.
    SwipeLeft,
    /// Horizontal rightward swipe.
    SwipeRight,
    /// Vertical upward swipe.
    SwipeUp,
    /// Vertical downward swipe.
    SwipeDown,
    /// Gesture moved beyond tap bounds without meeting directional swipe criteria.
    Drag,
}
