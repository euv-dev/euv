use super::*;

/// Reactive state for an element's visibility.
///
/// The handle carries the element the hook watches — a [`NodeRef`] the
/// caller attaches to its own markup — and the signals that report
/// whether the element is intersecting the viewport and how much of it
/// is visible.
///
/// This is the cheap way to defer the work of an off-screen element:
/// the signal only changes when the element actually crosses the
/// viewport, so a render closure that reads it stays quiet while the
/// element scrolls past.
#[derive(Clone, CustomDebug, Data)]
pub struct UseIntersectionObserver {
    /// The element the hook watches.
    ///
    /// Attach it to the caller's markup with the `ref:` attribute.
    pub target: NodeRef<Element>,
    /// Whether the element is currently intersecting the viewport.
    #[get(type(copy))]
    pub visible: Signal<bool>,
    /// The fraction of the element that is visible, from 0 to 1.
    #[get(type(copy))]
    pub ratio: Signal<f64>,
    /// The teardown that disconnects the observer.
    #[get(pub(crate))]
    #[debug(skip)]
    pub(crate) teardown: Teardown,
}
