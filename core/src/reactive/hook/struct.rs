use super::*;

/// Internal storage for hook state, holding boxed `Any` values.
///
/// This struct is not exposed directly; use `HookContext` instead.
/// The `arm_changed` flag tracks whether a `match` arm switch occurred;
/// when toggled, the hook array is cleared to prevent signal leakage
/// between different match arms.
#[derive(CustomDebug, Data, Default, New)]
pub struct HookContextInner {
    /// Storage for hook state values (signals, etc.).
    #[debug(skip)]
    pub hooks: Vec<Box<dyn Any>>,
    /// The match arm index from the last render.
    #[get(type(copy))]
    pub arm_changed: usize,
    /// Current hook index, incremented on each hook call and reset per render.
    #[get(type(copy))]
    pub hook_index: usize,
    /// Cleanup closures registered by hooks (e.g., `use_signal`) that must
    /// be executed when the hook context is cleared due to a `match` arm
    /// switch.
    #[debug(skip)]
    pub cleanups: Vec<Box<dyn FnOnce()>>,
}

/// Manages hook state across render cycles for a DynamicNode.
///
/// Stores boxed `Any` values keyed by hook call order, enabling `use_signal`
/// and similar hooks to persist state between re-renders of the same
/// dynamic node.
///
/// Implements `Clone` for ergonomic use; all clones share the same underlying state.
#[derive(CustomDebug, Data, New)]
pub struct HookContext {
    /// Shared reference to the heap-allocated hook context inner state.
    #[debug(skip)]
    pub inner: Rc<RefCell<HookContextInner>>,
}

/// A handle to a browser interval timer created by `use_interval`.
///
/// Stores the numeric interval ID returned by `window.setInterval` so the
/// timer can be cancelled later via `clear_interval`.
#[derive(Clone, Copy, Data, Debug, Default, Eq, Hash, New, Ord, PartialEq, PartialOrd)]
pub struct IntervalHandle {
    /// The interval ID assigned by the browser.
    #[get(type(copy))]
    pub(crate) interval_id: i32,
}
