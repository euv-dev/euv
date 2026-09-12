use super::*;

/// Type alias for the handler registry value.
///
/// Stores a raw pointer to a heap-allocated `HandlerSlot`. The allocation
/// is owned by the registry and freed during cleanup. Direct pointer access
/// avoids `Rc<RefCell<>>` overhead in the event dispatch hot path.
pub type HandlerEntry = *mut HandlerSlot;

/// Type alias for the signal update registry value.
///
/// Stores a raw pointer to a heap-allocated `SignalUpdateSlot`. The allocation
/// is owned by the registry and freed during cleanup or sweep. Direct pointer
/// access avoids `Rc<RefCell<>>` overhead in the signal dispatch hot path.
pub type SignalUpdateEntry = *mut SignalUpdateSlot;

/// Type alias for the handler registry map.
///
/// Nested by element ID first so that removing all handlers for one element is a
/// single `HashMap::remove(&euv_id)` instead of a full-registry scan. Uses
/// `&'static str` for event names to avoid allocation on every dispatch lookup.
/// Known event names are compile-time constants; custom names are leaked once via `as_str()`.
pub type HandlerRegistryMap = HashMap<usize, HashMap<&'static str, HandlerEntry>>;

/// Type alias for a single window event handler entry in the proxy registry.
///
/// Each entry holds a unique handler ID and a raw pointer to a heap-allocated
/// callback. The ID allows targeted removal during cleanup without disrupting
/// other handlers.
pub type WindowEventHandlerEntry = (usize, *mut Box<dyn FnMut()>);

/// Type alias for the window event proxy registry map.
///
/// Maps event names to a list of handler entries. All handlers for the same
/// event name share a single `window.addEventListener` listener (the proxy),
/// which iterates this list and invokes each callback on every event.
pub type WindowEventRegistryMap = HashMap<String, Vec<WindowEventHandlerEntry>>;

/// Type alias for a single `NodeRef` registration in the unmount-clear registry.
///
/// NP-3: holds a clone of the `NodeRef`'s interior cell so `cleanup_subtree`
/// can call `clear()` on every handle that pointed at a now-unmounted element.
pub type NodeRefEntry = Rc<UnsafeCell<Option<JsValue>>>;

/// Type alias for the `NodeRef` unmount-clear registry.
///
/// Maps `euv_id` (the same id used by the handler registry and `data-euv-id`
/// attribute) to the list of `NodeRef` interior cells registered against that
/// element. Removing an element from the DOM causes a single
/// `HashMap::remove(&euv_id)` followed by iterating the entries to call
/// `clear()` on each cell.
pub type NodeRefRegistryMap = HashMap<usize, Vec<NodeRefEntry>>;

/// Type alias for a single binding-teardown thunk.
///
/// Produced by the mount path when a signal is subscribed directly to a DOM
/// element (attribute / `inner_html` bindings). Running the thunk detaches
/// exactly one subscription via [`Signal::unsubscribe`], so removing a DOM
/// subtree tears its bindings down without touching the source signal's
/// other listeners or its `alive` flag.
pub type BindingCleanup = Box<dyn FnOnce()>;

/// Type alias for the binding-cleanup registry.
///
/// Maps `euv_id` to the teardown thunks of every signal binding installed on
/// that element. Drained by `cleanup_subtree` when the element leaves the
/// DOM.
pub type BindingCleanupsMap = HashMap<usize, Vec<BindingCleanup>>;
