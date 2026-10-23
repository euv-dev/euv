use super::*;

/// Reactive state for an element's size.
///
/// The handle carries the element the hook watches — a [`NodeRef`] the
/// caller attaches to its own markup — and the signals that report its
/// content-box width and height.
///
/// The size is a `Signal<f64>`: a render closure that reads it
/// re-renders when the element is resized, and nothing else in the tree
/// is disturbed.
#[derive(Clone, CustomDebug, Data)]
pub struct UseResizeObserver {
    /// The element the hook watches.
    ///
    /// Attach it to the caller's markup with the `ref:` attribute.
    pub target: NodeRef<Element>,
    /// The element's content-box width in CSS pixels.
    #[get(type(copy))]
    pub width: Signal<f64>,
    /// The element's content-box height in CSS pixels.
    #[get(type(copy))]
    pub height: Signal<f64>,
    /// The teardown that disconnects the observer.
    #[get(pub(crate))]
    #[debug(skip)]
    pub(crate) teardown: Teardown,
}
