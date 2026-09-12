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
    #[set(pub(crate))]
    pub(crate) callback: Option<Box<dyn FnMut()>>,
    /// Whether this slot has been marked for removal.
    #[get(pub, type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) removed: bool,
    /// Whether this slot has pending changes that need dispatching.
    /// Only dirty slots are invoked during dispatch, avoiding O(N)
    /// broadcast to all dynamic nodes when only one signal changed.
    #[get(pub, type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) dirty: bool,
}

/// A typed binding from a bridge `Signal<String>` to a specific DOM
/// mutation.
///
/// Replaces the older bridge-signal + `BridgeRefsCell::track` chain for
/// per-`{sig}` mount paths. The bridge's listener captures the typed
/// bridge by move and on every set fires a single typed mutation directly
/// without going through `BridgeRefsCell`, `is_connected()`, or an
/// `attr_name.to_string()` clone per signal set. The bridge struct is
/// stored in `ATTRIBUTE_BRIDGES` keyed by the bridge signal's address and
/// freed by `Registry::cleanup_attribute_bridge` at the same time
/// `Signal::<String>::clear_listeners` releases the bridge signal's
/// listener closure.
///
/// Variants:
/// - `SetAttribute` — write `attr_name = value` on an Element. Used by
///   `AttributeValue::Signal` mount paths.
/// - `SetInnerHtml` — replace `innerHTML` on an Element. Used by
///   `AttributeValue::InnerHtmlSignal` mount paths.
/// - `SetTextContent` — replace the text on a `Text` node. Used by the
///   text-signal mount path in `create_dom_with_doc`.
///
/// A `Sync` wrapper for single-threaded global `HashMap` access.
///
/// SAFETY: This type is only safe to use in single-threaded contexts
/// (e.g., WASM). It implements `Sync` to allow usage as a `static mut`
/// variable, but concurrent access from multiple threads would be
/// undefined behavior.
#[derive(Data, Debug, New)]
pub(crate) struct HandlerRegistryCell(
    /// Interior-mutable storage for the handler registry.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub UnsafeCell<HandlerRegistryMap>,
);

/// A `Sync` wrapper for single-threaded global `HashSet` access.
///
/// SAFETY: This type is only safe to use in single-threaded contexts
/// (e.g., WASM). It implements `Sync` to allow usage as a `static mut`
/// variable, but concurrent access from multiple threads would be
/// undefined behavior.
#[derive(Data, Debug, New)]
pub(crate) struct DelegatedEventsCell(
    /// Interior-mutable storage for the delegated events set.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub UnsafeCell<HashSet<&'static str>>,
);

/// A `Sync` wrapper for single-threaded global `HashMap` access.
///
/// SAFETY: This type is only safe to use in single-threaded contexts
/// (e.g., WASM). It implements `Sync` to allow usage as a `static mut`
/// variable, but concurrent access from multiple threads would be
/// undefined behavior.
#[derive(Data, Debug, New)]
pub(crate) struct SignalUpdateRegistryCell(
    /// Interior-mutable storage for the signal update registry.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub UnsafeCell<HashMap<usize, SignalUpdateEntry>>,
);

/// A `Sync` wrapper for single-threaded global `HashMap<usize, AttributeBridge>` access.
///
/// Stores the typed attribute bridges keyed by bridge signal address.
/// Populated by `Registry::register_attribute_bridge` and drained by
/// `Registry::cleanup_attribute_bridge` (called from
/// `Signal::<String>::clear_listeners`).
///
/// Replaces the older bridge-signal + `BridgeRefsCell::track` chain that
/// allocated a `HashSet<usize>` per bridge (the source-dependency set) and
/// required a `HashMap<usize, HashSet<usize>>` lookup on every
/// `Signal::deactivate` for every bridge ever registered. With this
/// registry the bridge struct is keyed by the bridge's address (already in
/// `data-euv-signal-addrs`) and freed at the same time as the bridge
/// signal — one `HashMap<usize, AttributeBridge>` lookup per cleanup
/// instead of one `HashMap<usize, HashSet<usize>>` lookup per
/// `Signal::deactivate` walk.
///
/// SAFETY: This type is only safe to use in single-threaded contexts
/// (e.g., WASM). It implements `Sync` to allow usage as a `static mut`
/// variable, but concurrent access from multiple threads would be
/// undefined behavior.
#[derive(Data, Debug, New)]
pub(crate) struct AttributeBridgesCell(
    /// Interior-mutable storage for the typed-attribute-bridge registry.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub UnsafeCell<HashMap<usize, AttributeBridge>>,
);

/// A `Sync` wrapper for single-threaded global `HashSet` access used by
/// the OPT 6 dirty-id fast path.
///
/// SAFETY: This type is only safe to use in single-threaded contexts
/// (e.g., WASM). It implements `Sync` to allow usage as a `static mut`
/// variable, but concurrent access from multiple threads would be
/// undefined behavior.
#[derive(Data, Debug, New)]
pub(crate) struct DirtyUpdateIdsCell(
    /// Interior-mutable storage for the dirty-id set.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub UnsafeCell<HashSet<usize>>,
);

/// A `Sync` wrapper for single-threaded global `WindowEventRegistryMap` access.
///
/// SAFETY: This type is only safe to use in single-threaded contexts
/// (e.g., WASM). It implements `Sync` to allow usage as a `static mut`
/// variable, but concurrent access from multiple threads would be
/// undefined behavior.
#[derive(Data, Debug, New)]
pub(crate) struct WindowEventRegistryCell(
    /// Interior-mutable storage for the window event handler registry.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub UnsafeCell<WindowEventRegistryMap>,
);

/// A `Sync` wrapper for single-threaded global `NodeRefRegistryMap` access.
///
/// SAFETY: This type is only safe to use in single-threaded contexts
/// (e.g., WASM). It implements `Sync` to allow usage as a `static mut`
/// variable, but concurrent access from multiple threads would be
/// undefined behavior.
#[derive(Data, Debug, New)]
pub(crate) struct NodeRefRegistryCell(
    /// Interior-mutable storage for the `NodeRef` registry.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub UnsafeCell<NodeRefRegistryMap>,
);

/// A zero-sized struct providing static methods for managing
/// the framework's internal registries (handlers, signal updates,
/// window events, and delegated events).
///
/// All methods are crate-internal associated functions that operate
/// on the global static registries.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub(crate) struct Registry;
