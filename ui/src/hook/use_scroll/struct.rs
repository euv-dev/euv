use super::*;

/// Reactive state for the window's scroll position.
///
/// The handle carries the horizontal and vertical scroll offsets and the
/// teardown that removes the window listener.
///
/// The offsets are `Signal<f64>`: a render closure that reads them
/// re-renders when the page scrolls.
#[derive(Clone, CustomDebug, Data)]
pub struct UseScroll {
    /// The horizontal scroll offset in CSS pixels.
    #[get(type(copy))]
    pub x: Signal<f64>,
    /// The vertical scroll offset in CSS pixels.
    #[get(type(copy))]
    pub y: Signal<f64>,
    /// The teardown that removes the window listener.
    #[get(pub(crate))]
    #[debug(skip)]
    pub(crate) teardown: Teardown,
}
