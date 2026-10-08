use super::*;

/// Static methods for scheduling signal update dispatch and batching.
///
/// Provides centralized scheduling for reactive updates, ensuring efficient
/// batching and dispatch of signal changes to dependent dynamic nodes.
impl Scheduler {
    /// Resolves `window.queueMicrotask` to a `Function` handle.
    ///
    /// The lookup crosses into JS, so it is kept out of any `RefCell`
    /// borrow: the caller stores the result afterwards. Returns `None`
    /// when the host does not expose `queueMicrotask` or the cast fails.
    ///
    /// # Returns
    ///
    /// - `Option<Function>` - The resolved handle, or `None` if unavailable.
    fn resolve_queue_microtask() -> Option<Function> {
        let window_value: Window = window()?;
        let queue_microtask_value: JsValue =
            Reflect::get(&window_value, &JsValue::from_str(QUEUE_MICROTASK)).ok()?;
        queue_microtask_value.dyn_into::<Function>().ok()
    }

    /// Returns the persistent dispatch closure as a JS `Function`.
    ///
    /// `DISPATCH_CLOSURE` is a `Closure<dyn FnMut()>`; `Closure::as_ref`
    /// yields a `&JsValue` that is the underlying JS function object, so
    /// the cast is a reinterpretation of the same `JsValue` rather than a
    /// new borrow. The closure is created inside a `thread_local!` and
    /// never dropped, so the returned reference is live for the life of
    /// the thread.
    ///
    /// # Returns
    ///
    /// - `&'static Function` - The dispatch function handle.
    fn dispatch_function() -> &'static Function {
        DISPATCH_CLOSURE.with(|closure: &Closure<dyn FnMut()>| {
            let value: &JsValue = closure.as_ref();
            unsafe { &*(value as *const JsValue as *const Function) }
        })
    }

    /// Invokes `queue_microtask` with the persistent dispatch closure.
    ///
    /// The dispatch `Function` is resolved inside this call, after the
    /// `MICROTASK_CACHE` borrow has already been released, so the
    /// `queueMicrotask` invocation never runs under a live `RefCell`
    /// borrow. `queueMicrotask` schedules a microtask — the callback runs
    /// later, in a separate turn — but a host that runs it synchronously
    /// would otherwise re-enter `MICROTASK_CACHE` and hit a refused
    /// borrow.
    ///
    /// # Arguments
    ///
    /// - `&Window` - The window whose `queueMicrotask` is being called.
    /// - `&Function` - The cached `queueMicrotask` handle.
    ///
    /// # Returns
    ///
    /// - `bool` - `true` when the microtask was queued.
    fn call_queue_microtask(window_value: &Window, queue_microtask: &Function) -> bool {
        let dispatch_function: &Function = Self::dispatch_function();
        queue_microtask
            .call1(window_value, dispatch_function)
            .is_ok()
    }

    /// Whether a JS `Window` is reachable on this host.
    ///
    /// `window()` resolves the JS global through a
    /// process-wide `once_cell::Lazy` inside `js_sys`. On a non-WASM host
    /// (where `cargo test` runs) there is no JS global, so the lookup
    /// **panics** — and because the `Lazy` is process-wide, that one
    /// panic poisons it for every other thread, which then fail with
    /// "Lazy instance has previously been poisoned" in a completely
    /// unrelated test. The panic is the bug, not the poisoning.
    ///
    /// `cfg!(target_arch = "wasm32")` is a compile-time constant, so on
    /// WASM the whole body is optimised away to `true` and this costs
    /// nothing; on the host it returns `false` before any JS call is
    /// made, keeping `Scheduler::update` a pure registry operation.
    ///
    /// # Returns
    ///
    /// - `bool` - `true` when JS globals are reachable.
    fn js_reachable() -> bool {
        cfg!(target_arch = "wasm32")
    }

    /// Schedules a deferred signal update with precise dirty marking.
    ///
    /// Marks the specified dynamic nodes as dirty and queues a microtask
    /// to dispatch updates. Uses `queueMicrotask` if available, falling
    /// back to `setTimeout` or `requestAnimationFrame`.
    ///
    /// OPT 7: the cached `queueMicrotask` `Function` and dispatch
    /// closure `Function` are read once per call from
    /// `MICROTASK_CACHE` / `DISPATCH_CLOSURE`, instead of being looked
    /// up via `Reflect::get(&window, "queueMicrotask")` and
    /// `Closure::as_ref().unchecked_ref::<Function>()` three times per
    /// signal update.
    ///
    /// # Arguments
    ///
    /// - `&[usize]` - The dynamic node IDs that depend on the changed signal.
    pub(crate) fn update(dependents: &[usize]) {
        Registry::mark_dirty(dependents);
        if SUPPRESS_SCHEDULE.load(Ordering::Relaxed) {
            return;
        }
        // Off-WASM there is no JS global to schedule a microtask against, and
        // attempting the lookup panics inside `js_sys`'s process-wide
        // `once_cell::Lazy` — poisoning it for every other thread. Return
        // before touching any JS so host test runs stay green and the dirty
        // marking above still happens.
        if !Self::js_reachable() {
            return;
        }
        if SCHEDULED.load(Ordering::Relaxed) {
            return;
        }
        SCHEDULED.store(true, Ordering::Relaxed);
        let window_value: Window = match window() {
            Some(window_instance) => window_instance,
            None => {
                SCHEDULED.store(false, Ordering::Relaxed);
                return;
            }
        };
        let queued_microtask: bool = MICROTASK_CACHE
            .try_with(|cache: &RefCell<MicrotaskCache>| {
                // Fast path: a cached `queueMicrotask` handle, cloned out so
                // the `RefCell` borrow is released before the call crosses
                // into JS. A refused borrow just falls through to the
                // resolution path below rather than panicking.
                if let Ok(guard) = cache.try_borrow()
                    && let Some(cached) = guard.try_get_queue_microtask().clone()
                {
                    return Self::call_queue_microtask(&window_value, &cached);
                }
                // Slow path: resolve the handle once and cache it. The
                // `Reflect::get` + `dyn_into` lookup crosses into JS, so it
                // runs with no borrow held and the result is stored after.
                let resolved: Option<Function> = Self::resolve_queue_microtask();
                if let Some(queue_microtask) = &resolved
                    && let Ok(mut guard) = cache.try_borrow_mut()
                {
                    guard.set_queue_microtask(Some(queue_microtask.clone()));
                }
                match resolved {
                    Some(queue_microtask) => {
                        Self::call_queue_microtask(&window_value, &queue_microtask)
                    }
                    None => false,
                }
            })
            .unwrap_or(false);
        if queued_microtask {
            return;
        }
        let scheduled: bool = DISPATCH_CLOSURE.with(|dispatch_closure: &Closure<dyn FnMut()>| {
            let dispatch_function: &Function =
                dispatch_closure.as_ref().unchecked_ref::<Function>();
            window_value
                .set_timeout_with_callback_and_timeout_and_arguments_0(dispatch_function, 0)
                .is_ok()
        });
        if scheduled {
            return;
        }
        let requested_frame: bool =
            DISPATCH_CLOSURE.with(|dispatch_closure: &Closure<dyn FnMut()>| {
                let dispatch_function: &Function =
                    dispatch_closure.as_ref().unchecked_ref::<Function>();
                window_value
                    .request_animation_frame(dispatch_function)
                    .is_ok()
            });
        if requested_frame {
            return;
        }
        SCHEDULED.store(false, Ordering::Relaxed);
    }

    /// Batches signal updates within a closure, deferring DOM dispatch.
    ///
    /// Suppresses scheduling during the callback execution, then triggers
    /// a single dispatch after the outermost batch completes. This prevents
    /// redundant re-renders when multiple signals are updated in sequence.
    ///
    /// # Arguments
    ///
    /// - `F` - The closure to execute with batching enabled.
    ///
    /// # Returns
    ///
    /// - `R` - The value the closure returns, forwarded unchanged.
    pub(crate) fn batch<F, R>(callback: F) -> R
    where
        F: FnOnce() -> R,
    {
        let was_outermost: bool = !SUPPRESS_SCHEDULE.load(Ordering::Relaxed);
        SUPPRESS_SCHEDULE.store(true, Ordering::Relaxed);
        let result: R = callback();
        SUPPRESS_SCHEDULE.store(!was_outermost, Ordering::Relaxed);
        if was_outermost && Registry::has_dirty() {
            Self::update(&[]);
        }
        result
    }

    /// Invokes all active callbacks in the signal update registry.
    ///
    /// Guards against re-entrant dispatch with `SIGNAL_UPDATE_DISPATCHING`.
    /// Iterates dirty slots, takes their callbacks, invokes them, and puts
    /// them back. After completing one pass, checks whether new entries
    /// were added during callback execution. If so, performs additional
    /// passes until the registry stabilizes, up to a maximum iteration limit.
    ///
    /// OPT 6: replaces the per-tick `O(累计动态节点数)` registry scan
    /// with an `O(脏节点数)` drain over `DIRTY_UPDATE_IDS`. Each id is
    /// pulled from the set exactly once per dispatch, then removed so a
    /// second pass does not re-fire it. The previous
    /// `sweep_removed_entries` step is gone: every `cleanup_*` path
    /// already pulls its id from both the registry and the dirty set,
    /// so the registry holds no removed entries by the time the next
    /// `mark_dirty` arrives.
    pub(crate) fn dispatch_updates() {
        if SIGNAL_UPDATE_DISPATCHING.load(Ordering::Relaxed) {
            return;
        }
        SIGNAL_UPDATE_DISPATCHING.store(true, Ordering::Relaxed);
        let mut iterations: usize = 0;
        loop {
            // OPT 6: drain the dirty set rather than scanning the registry.
            // `std::mem::take` swaps in a fresh empty set so the dirty-set
            // borrow is released before we mutate the signal update
            // registry in the loop body below. (`HashSet::drain` requires
            // the `RangeFull` pattern which Rust 2024 reserves as the
            // struct-update syntax shorthand.)
            let dirty_keys: HashSet<usize> = Registry::take_dirty_update_ids();
            if dirty_keys.is_empty() {
                break;
            }
            for key in dirty_keys {
                // The slot is taken out of the registry for the duration of
                // the callback. A re-render that unmounts this node runs
                // `cleanup_dynamic_node`, which finds nothing to remove and
                // therefore cannot free the box while the callback is still
                // using it — that is why the "put it back" step below
                // re-checks both `removed` and registry membership.
                // The entry MUST be taken out of the registry for the
                // duration of the callback. `get_dynamic` only copies the
                // pointer, so the key would still be present below and the
                // `has_dynamic` guard would then treat our own untouched
                // entry as "a re-entrant pass already replaced it" — freeing
                // the slot and leaving the dynamic node with no callback.
                // The node then never re-renders again: the first
                // signal-driven update after mount works, every later one is
                // silently dropped.
                let Some(entry) = Registry::take_dynamic(key) else {
                    continue;
                };
                // SAFETY: `take_dynamic` returns the raw pointer the registry
                // stores; the entry is still live because nothing removed
                // it (removal frees the box, and only removal precedes
                // freeing).
                let slot: &mut SignalUpdateSlot = unsafe { &mut *entry };
                if slot.get_removed() {
                    unsafe {
                        let _: Box<SignalUpdateSlot> = Box::from_raw(entry);
                    }
                    continue;
                }
                slot.set_dirty(false);
                let callback: Option<Box<dyn FnMut()>> = slot.get_mut_callback().take();
                if let Some(mut callback) = callback {
                    callback();
                    if !slot.get_removed() {
                        slot.set_callback(Some(callback));
                    }
                }
                if slot.get_removed() {
                    unsafe {
                        let _: Box<SignalUpdateSlot> = Box::from_raw(entry);
                    }
                    continue;
                }
                // Reinsert only if a re-entrant pass did not already put
                // this id back (which would leave the old box unreclaimed
                // and the new one duplicated).
                if Registry::has_dynamic(key) {
                    unsafe {
                        let _: Box<SignalUpdateSlot> = Box::from_raw(entry);
                    }
                    continue;
                }
                Registry::put_dynamic(key, entry);
            }
            iterations += 1;
            if iterations >= MAX_ITERATIONS {
                break;
            }
        }
        SIGNAL_UPDATE_DISPATCHING.store(false, Ordering::Relaxed);
    }
}
