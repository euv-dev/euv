use super::*;

/// Path-length-aware pointer/touch gesture classifier preventing circular drag-to-origin false taps.
#[derive(Clone, Copy, Data, Debug, Default, PartialEq)]
pub struct GestureRecognizer {
    /// Whether a gesture stroke is currently active.
    #[get(type(copy))]
    pub(crate) active: bool,
    /// Initial contact point.
    #[get(type(copy))]
    pub(crate) start: Point,
    /// Most recent sampled point.
    #[get(type(copy))]
    pub(crate) last: Point,
    /// Cumulative Euclidean path length traveled during the stroke.
    #[get(type(copy))]
    pub(crate) total_path: f32,
}
