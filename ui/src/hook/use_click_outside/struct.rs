use super::*;

/// Reactive state for a click-outside watcher.
///
/// The handle carries the element the hook watches — a [`NodeRef`]
/// the caller attaches to its own markup — and the signal that is
/// raised when a pointer press lands outside that element.
///
/// The watcher listens on the document rather than on the element
/// itself. That is deliberate: the element does not exist while the
/// first render is being built, so a listener attached to it would
/// have nothing to attach to, and a document listener sees the press
/// either way and can then ask whether the pressed node sits inside
/// the watched element.
#[derive(Clone, CustomDebug, Data)]
pub struct UseClickOutside {
    /// The element the hook watches.
    ///
    /// Attach it to the caller's markup with the `ref:` attribute.
    pub target: NodeRef<Element>,
    /// Raised when a press landed outside the watched element.
    #[get(type(copy))]
    pub triggered: Signal<bool>,
    /// The teardown that removes the document listener.
    #[get(pub(crate))]
    #[debug(skip)]
    pub(crate) teardown: Teardown,
}
