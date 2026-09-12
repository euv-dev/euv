use super::*;

/// Inner state of a signal, holding the value and subscribed listeners.
///
/// This struct is not exposed directly; use `Signal` instead.
#[derive(CustomDebug, Data, New)]
pub(crate) struct SignalInner<T>
where
    T: Clone,
{
    /// The current value of the signal.
    #[debug(skip)]
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) value: T,
    /// Callbacks to invoke when the value changes.
    #[debug(skip)]
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) listeners: Vec<Box<dyn FnMut()>>,
    /// Whether this signal is still active. Set to `false` by `deactivate()`
    /// (and `clear_signal_listeners`) to make subsequent `set()` calls
    /// complete no-ops (no value update, no listener invocation, no
    /// dispatch scheduling), ensuring stale closures like orphaned
    /// `setInterval` handlers or pending `spawn_local` futures become harmless.
    #[get(pub, type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) alive: bool,
    /// IDs of dynamic nodes that depend on this signal for precise dirty marking.
    /// When this signal changes, only these dynamic nodes are marked dirty
    /// instead of broadcasting to all registered dynamic nodes.
    #[debug(skip)]
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    #[new(skip)]
    pub(crate) dependents: Vec<usize>,
    /// Flag indicating that `replace_subscribe` was called during
    /// `update_and_notify`'s swap-out phase. When a listener callback
    /// re-registers listeners via `replace_subscribe`, it intends to
    /// replace all existing listeners — but the old listeners have
    /// already been swapped out into the local variable. Without this
    /// flag, `update_and_notify` would incorrectly merge the old
    /// listeners back with the new ones, defeating `replace_subscribe`'s
    /// replacement semantics and causing listener accumulation.
    #[get(pub, type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    #[new(skip)]
    pub(crate) listeners_replaced: bool,
}

/// A reactive signal handle.
///
/// Allows reading, writing, and subscribing to changes.
/// Implements `Clone` and `Copy` for ergonomic use; all copies share the same
/// underlying state. The inner state is heap-allocated via `Box` and accessed
/// through a raw pointer stored as a `usize`. The allocation is tracked in a
/// global registry for lifecycle management. The `Copy` semantics are safe
/// because only the pointer address is copied — the actual heap allocation
/// is owned by the registry.
#[derive(CustomDebug, Data, Eq, Hash, New, Ord, PartialEq, PartialOrd)]
pub struct Signal<T>
where
    T: Clone + PartialEq + 'static,
{
    /// Address of the heap-allocated inner state (`*mut SignalInner<T>`).
    #[debug(skip)]
    #[get(pub, type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) inner: usize,
    /// Marker for the generic type parameter (uses fn pointer to be `Copy`
    /// regardless of `T`).
    #[debug(skip)]
    #[get(pub, type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) _marker: PhantomData<fn() -> T>,
}

/// A `Sync` wrapper for single-threaded global `Signal` access.
///
/// SAFETY: This type is only safe to use in single-threaded contexts
/// (e.g., WASM). It implements `Sync` to allow usage as a `static`
/// variable, but concurrent access from multiple threads would be
/// undefined behavior.
#[derive(CustomDebug, Data, New)]
pub struct SignalCell<T>
where
    T: Clone + PartialEq + 'static,
{
    /// Interior-mutable storage for an optional signal handle.
    #[debug(skip)]
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) inner: UnsafeCell<Option<Signal<T>>>,
}

/// A `Sync` wrapper for single-threaded global `HashMap` access.
///
/// SAFETY: This type is only safe to use in single-threaded contexts
/// (e.g., WASM). It implements `Sync` to allow usage as a `static`
/// variable, but concurrent access from multiple threads would be
/// undefined behavior.
#[derive(Data, Debug, New)]
pub(crate) struct BridgeRefsCell(
    /// Interior-mutable storage for the bridge dependency reverse-index.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub UnsafeCell<HashMap<usize, HashSet<usize>>>,
);

/// A handle to a leaked `FnMut()` closure, stored as the closure's heap address.
///
/// The closure is double-boxed (`Box<Box<dyn FnMut()>>`) and leaked, so its
/// memory outlives any `FireHandle` copy and can be safely invoked from any
/// context via the raw pointer. The handle is `Copy` because it only holds
/// the address — repeated invocations on captured copies all resolve to the
/// same underlying closure.
///
/// This type replaces the inline `Box::leak(... as *mut Box<dyn FnMut()> as usize)`
/// pattern that was used by `watch!`/`computed!` macros and the virtual list
/// component, encapsulating the unsized coercion, double-boxing, and raw
/// pointer arithmetic behind `From`/`Into` conversions and a dedicated
/// `fire` method.
#[derive(Clone, Copy, CustomDebug, Data, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct FireHandle {
    /// Address of the leaked `Box<dyn FnMut()>` allocation.
    #[get(pub, type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) inner: usize,
}

/// Typed slab allocator for `SignalInner<T>`.
///
/// The slab owns all `SignalInner<T>` allocations for the program's
/// lifetime. `Signal<T>` carries only a `usize` slot index, so signals are
/// trivially `Copy` and stay cheap to clone. Reclamation is explicit via
/// `Signal::deactivate`, which calls [`SignalSlab::free`]; `Copy`
/// semantics intentionally prevent an implicit `Drop` from double-freeing.
///
/// Layout:
/// - `entries: Vec<SignalSlot>` — slot storage, indexed 0..len.
/// - `free_head: usize` — head of the free-slot stack, `usize::MAX` when empty.
///
/// Allocation is O(1) (free-list pop or Vec push). Free is O(1) (push to
/// free-list head, drop the boxed inner). Lookup is O(1) bounds-checked
/// `Vec` indexing.
pub(crate) struct SignalSlab {
    /// Slot storage. Index 0..len.
    pub(crate) entries: Vec<SignalSlot>,
    /// Head of the free-slot stack; `usize::MAX` when no free slots exist.
    pub(crate) free_head: usize,
}
