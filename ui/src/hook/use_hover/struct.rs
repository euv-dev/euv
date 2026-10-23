use super::*;

/// Reactive state for a hover watcher.
///
/// The handle carries the element the hook watches — a [`NodeRef`] the
/// caller attaches to its own markup — and the signal that reports
/// whether the pointer is currently over it.
///
/// The watcher listens on the document rather than on the element
/// itself, for the same reason the click-outside watcher does: the
/// element does not exist while the first render is being built, and a
/// document listener can then ask whether the pointer's node sits
/// inside the watched element.
#[derive(Clone, CustomDebug, Data)]
pub struct UseHover {
    /// The element the hook watches.
    ///
    /// Attach it to the caller's markup with the `ref:` attribute.
    pub target: NodeRef<Element>,
    /// Whether the pointer is currently over the watched element.
    #[get(type(copy))]
    pub hovered: Signal<bool>,
    /// The teardown that removes the document listeners.
    #[get(pub(crate))]
    #[debug(skip)]
    pub(crate) teardown: Teardown,
}
