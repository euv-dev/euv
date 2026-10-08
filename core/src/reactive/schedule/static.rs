use super::*;

/// Scheduling flag to batch signal updates within a single tick.
///
/// Set to `true` when `schedule_update()` queues a microtask,
/// and reset to `false` when the dispatch callback fires.
pub(crate) static SCHEDULED: AtomicBool = AtomicBool::new(false);

/// Suppress flag to prevent `schedule_update()` from dispatching
/// during internal operations such as `batch`.
pub(crate) static SUPPRESS_SCHEDULE: AtomicBool = AtomicBool::new(false);

thread_local! {
    /// The currently active `HookContext` for this thread.
    ///
    /// This is thread-local, not a process global, because `HookContext::with`
    /// implements save/restore: it takes the previous value, installs the new
    /// one, and puts the old one back afterwards. A process-wide global makes
    /// that pattern unsound as soon as two threads interleave — thread A
    /// saves, thread B saves, thread A restores, thread B restores the value
    /// A had already put back, and the `Rc` is dropped twice, aborting the
    /// process with a refcount underflow. Per-thread storage makes the
    /// save/restore pair atomic with respect to other threads by
    /// construction.
    pub(crate) static CURRENT_HOOK_CONTEXT: RefCell<Option<HookContextRc>> =
        const { RefCell::new(None) };
}

/// The dynamic node ID currently being rendered/set up.
///
/// When a DynamicNode is being initialized or re-rendered, this is set to
/// the dynamic_id so that any signal `get()` calls during that render
/// can register the dependency. `usize::MAX` means "no tracking active".
///
/// SAFETY: Must only be accessed from the main thread (WASM single-threaded context).
pub(crate) static CURRENT_TRACKING_DYNAMIC_ID: AtomicUsize = AtomicUsize::new(usize::MAX);

thread_local! {
    /// The persistent dispatch `Closure`, kept alive for the lifetime of the
    /// program so it can be handed to `setTimeout` repeatedly.
    ///
    /// The closure resets the `SCHEDULED` flag and then runs the queued signal
    /// update callbacks. Resetting `SCHEDULED` here is what allows the next
    /// `schedule_update` call to schedule a fresh dispatch; if
    /// this never ran, the flag would stay `true` forever and every reactive
    /// update would be silently dropped.
    pub(crate) static DISPATCH_CLOSURE: Closure<dyn FnMut()> =
        Closure::wrap(Box::new(|| {
            SCHEDULED.store(false, Ordering::Relaxed);
            Scheduler::dispatch_updates();
        }));

    /// OPT 7: thread-local cache for the `Function` reference to
    /// `window.queueMicrotask`, resolved lazily on first use. The
    /// `Function::call1` in `Scheduler::update` skips the
    /// `Reflect::get` / `dyn_into` lookup on every signal update.
    ///
    /// Storage is a plain `RefCell<MicrotaskCache>` rather than the
    /// previous `MicrotaskCacheCell(UnsafeCell<MicrotaskCache>)` +
    /// `unsafe impl Sync for MicrotaskCacheCell {}`. The wrapper existed
    /// only to let the cache live in a `static`; as a `thread_local!` the
    /// `RefCell` gives the same lazy-populate-once behaviour while
    /// restoring the borrow check the `unsafe` had bypassed. It holds no
    /// raw pointer and no `!Send` payload, so thread-local storage is a
    /// pure win rather than a restriction.
    pub(crate) static MICROTASK_CACHE: RefCell<MicrotaskCache> =
        RefCell::new(MicrotaskCache { queue_microtask: None });
}
