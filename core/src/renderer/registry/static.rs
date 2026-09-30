use super::*;

/// Global auto-incrementing ID counter for DOM elements.
pub static NEXT_EUV_ID: AtomicUsize = AtomicUsize::new(0);

/// Global auto-incrementing ID counter for DynamicNode placeholder elements.
pub static NEXT_EUV_DYNAMIC_ID: AtomicUsize = AtomicUsize::new(0);

/// Whether `dispatch_updates` is currently executing.
pub static SIGNAL_UPDATE_DISPATCHING: AtomicBool = AtomicBool::new(false);

/// Global auto-incrementing ID counter for window event handler entries.
pub static NEXT_WINDOW_HANDLER_ID: AtomicUsize = AtomicUsize::new(0);

thread_local! {
    /// Set of dynamic node IDs marked dirty since the last dispatch drain.
    ///
    /// OPT 6: the dispatcher's hot path used to scan the entire signal
    /// update registry (`HashMap<usize, SignalUpdateEntry>`) on every
    /// tick to find slots whose `dirty` flag was set. For a SPA with N
    /// dynamic nodes and only a handful of changed signals per tick, this
    /// was an `O(N)` scan with a `*const SignalUpdateSlot` pointer
    /// dereference + branch per entry. The dirty-set replaces it with a
    /// single `HashSet::drain()` over the IDs that were actually marked
    /// dirty — a `O(脏节点数)` operation that does no per-slot pointer
    /// traversal until the matching slot is found.
    ///
    /// Populated by `Registry::mark_dirty` and drained by
    /// `Scheduler::dispatch_updates`. A redundant `dirty = true` set is a
    /// no-op thanks to `HashSet` semantics. The set survives the dispatch
    /// itself (the dirty flag is reset inside the loop); only the
    /// `mark_dirty` path inserts into it.
    ///
    /// Storage is thread-local rather than the previous `static mut` +
    /// `LazyLock` + `UnsafeCell`. The old form handed out
    /// `&'static mut HashSet<usize>` from a global that was never
    /// actually `Sync` — the only reason it compiled was an
    /// `unsafe impl Sync for DirtyUpdateIdsCell {}` whose SAFETY comment
    /// only held for the single-threaded WASM runtime, not for the
    /// multi-threaded host that `cargo test` links against. Two threads
    /// racing to initialise the `LazyLock` poisoned it ("Lazy instance
    /// has previously been poisoned"), and concurrent `insert` into one
    /// `HashSet` corrupted its bucket array. Thread-local storage makes
    /// each thread's dirty set structurally invisible to the others, and
    /// `RefCell` restores the borrow check the `unsafe` had bypassed.
    pub static DIRTY_UPDATE_IDS: RefCell<HashSet<usize>> = RefCell::new(HashSet::new());

    /// Global handler registry, mapping (element_id, event_name) to HandlerEntry.
    ///
    /// Thread-local for the same reason as [`DIRTY_UPDATE_IDS`]: the old
    /// `static mut LazyLock<HandlerRegistryCell>` handed out
    /// `&'static mut HandlerRegistryMap` behind an unsound `unsafe impl
    /// Sync`, so a parallel test run could observe two live
    /// `&mut` references to the same map.
    pub static HANDLER_REGISTRY: RefCell<HandlerRegistryMap> = RefCell::new(HashMap::new());

    /// Global set of event names that have already been delegated at the window level.
    ///
    /// Thread-local for the same reason as [`DIRTY_UPDATE_IDS`]: the old
    /// `static mut LazyLock<DelegatedEventsCell>` poisoned on a racing
    /// initialisation and aliased a `HashSet` across threads.
    pub static DELEGATED_EVENTS: RefCell<HashSet<&'static str>> = RefCell::new(HashSet::new());

    /// Global signal update callback registry, mapping keys to SignalUpdateEntry.
    ///
    /// Thread-local for the same reason as [`DIRTY_UPDATE_IDS`]: the old
    /// `static mut LazyLock<SignalUpdateRegistryCell>` poisoned on a
    /// racing initialisation and aliased a `HashMap` across threads.
    pub static SIGNAL_UPDATE_REGISTRY: RefCell<HashMap<usize, SignalUpdateEntry>> =
        RefCell::new(HashMap::new());

    /// Global window event proxy registry, mapping event names to handler lists.
    ///
    /// Thread-local for the same reason as [`DIRTY_UPDATE_IDS`]: the old
    /// `static mut LazyLock<WindowEventRegistryCell>` poisoned on a racing
    /// initialisation and aliased a `HashMap` across threads.
    pub static WINDOW_EVENT_REGISTRY: RefCell<WindowEventRegistryMap> =
        RefCell::new(HashMap::new());

    /// Global `NodeRef` registry used to clear `NodeRef` handles when the
    /// DOM element they point to is unmounted.
    ///
    /// NP-3: each time a `ref:` attribute fires, the mount path registers
    /// the `NodeRef`'s shared interior cell into this map under the
    /// element's `euv_id`. `cleanup_subtree` then drains the entries for
    /// that id and calls `NodeRef::clear` so `get()` / `get_cloned()` return
    /// `None` after the underlying DOM subtree is gone.
    ///
    /// Thread-local for the same reason as [`DIRTY_UPDATE_IDS`]: the old
    /// `static mut LazyLock<NodeRefRegistryCell>` poisoned on a racing
    /// initialisation and aliased a `HashMap` across threads.
    pub static NODEREF_REGISTRY: RefCell<NodeRefRegistryMap> = RefCell::new(HashMap::new());

    /// Global binding-cleanup registry, mapping `euv_id` to the teardown thunks
    /// of the signal bindings installed on that element.
    ///
    /// Populated by `Registry::push_binding_cleanup` (called from the signal
    /// attribute / `inner_html` mount paths in `Renderer::create_dom_with_doc`
    /// and from the late-binding path in `patch_attributes`) and drained by
    /// `cleanup_subtree`, which runs each thunk so the subscription is detached
    /// via [`Signal::unsubscribe`].
    ///
    /// Thread-local for the same reason as [`DIRTY_UPDATE_IDS`]: the old
    /// `static mut LazyLock<BindingCleanupsCell>` poisoned on a racing
    /// initialisation and aliased a `HashMap` across threads.
    pub static BINDING_CLEANUPS: RefCell<BindingCleanupsMap> = RefCell::new(HashMap::new());
}
