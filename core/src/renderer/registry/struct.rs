use super::*;

/// A wrapper around `Option<NativeEventHandler>` that enables `From<usize>` conversions.
///
/// For non-bubbling events, also stores the JavaScript `Function` reference
/// and `Element` needed to call `removeEventListener` during cleanup.
#[derive(CustomDebug, Data, New)]
pub(crate) struct HandlerSlot {
    /// The optional event handler stored in this slot.
    #[debug(skip)]
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) handler: Option<NativeEventHandler>,
    /// The JavaScript `Function` reference for non-bubbling event listeners.
    ///
    /// When a non-bubbling event is attached directly on an element via
    /// `addEventListener`, the closure's JS `Function` must be kept alive so
    /// it can be passed to `removeEventListener` during cleanup. For bubbling
    /// events that use global delegation, this is `None`.
    #[debug(skip)]
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) listener_function: Option<JsValue>,
    /// The DOM element on which a non-bubbling event listener was registered.
    ///
    /// Stored here so that `removeEventListener` can be called during cleanup
    /// without needing to re-look-up the element. `None` for bubbling events.
    #[debug(skip)]
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) element: Option<Element>,
}

/// Stores a signal update callback and its cleanup flag.
#[derive(CustomDebug, Data, New)]
pub(crate) struct SignalUpdateSlot {
    /// The callback to invoke when signal update events fire.
    #[debug(skip)]
    #[get(skip)]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) callback: Option<Box<dyn FnMut()>>,
    /// Whether this slot has been marked for removal.
    #[get(type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) removed: bool,
    /// Whether this slot has pending changes that need dispatching.
    /// Only dirty slots are invoked during dispatch, avoiding O(N)
    /// broadcast to all dynamic nodes when only one signal changed.
    #[get(type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) dirty: bool,
}

/// A zero-sized struct providing static methods for managing
/// the framework's internal registries (handlers, signal updates,
/// window events, and delegated events).
///
/// All methods are crate-internal associated functions that operate
/// on the global static registries.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub(crate) struct Registry;
