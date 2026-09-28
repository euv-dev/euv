use super::*;

/// Marks `CurrentHookContextCell` as `Sync` for single-threaded WASM contexts.
///
/// SAFETY: `CurrentHookContextCell` is only used in single-threaded WASM contexts.
/// Concurrent access from multiple threads would be undefined behavior.
unsafe impl Sync for CurrentHookContextCell {}

/// Marks `MicrotaskCacheCell` as `Sync` for single-threaded WASM contexts.
///
/// SAFETY: only mutated through `UnsafeCell` interior-mutability on
/// the WASM single-threaded runtime.
unsafe impl Sync for MicrotaskCacheCell {}

/// Static methods for scheduling signal update dispatch and batching.
///
/// Provides centralized scheduling for reactive updates, ensuring efficient
/// batching and dispatch of signal changes to dependent dynamic nodes.
impl Scheduler {
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
        let queued_microtask: bool = MICROTASK_CACHE.with(|cache: &MicrotaskCacheCell| {
            let cache_ptr: *mut MicrotaskCache = cache.get_0().get();
            let cache_ref: &MicrotaskCache = unsafe { &*cache_ptr };
            if cache_ref.queue_microtask.is_none() {
                if let Some(window_value_inner) = window() {
                    let queue_microtask_value: JsValue =
                        Reflect::get(&window_value_inner, &JsValue::from_str(QUEUE_MICROTASK))
                            .unwrap_or(JsValue::UNDEFINED);
                    if let Ok(queue_microtask) = queue_microtask_value.dyn_into::<Function>() {
                        unsafe {
                            (*cache_ptr).queue_microtask = Some(queue_microtask);
                        }
                    }
                }
                let cache_ref: &MicrotaskCache = unsafe { &*cache_ptr };
                if let Some(queue_microtask) = &cache_ref.queue_microtask {
                    // SAFETY: `DISPATCH_CLOSURE` lives for the duration of
                    // the program (it is leaked via `Closure::wrap` /
                    // `Closure::forget` semantics inside the macro).
                    let dispatch_function: &Function =
                        DISPATCH_CLOSURE.with(|closure: &Closure<dyn FnMut()>| unsafe {
                            &*(closure.as_ref() as *const _ as *const Function)
                        });
                    return queue_microtask
                        .call1(&window_value, dispatch_function)
                        .is_ok();
                }
                return false;
            }
            let queue_microtask: &Function = match cache_ref.queue_microtask.as_ref() {
                Some(queue_microtask) => queue_microtask,
                None => return false,
            };
            let dispatch_function: &Function =
                DISPATCH_CLOSURE.with(|closure: &Closure<dyn FnMut()>| unsafe {
                    &*(closure.as_ref() as *const _ as *const Function)
                });
            queue_microtask
                .call1(&window_value, dispatch_function)
                .is_ok()
        });
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
    /// - `F: FnOnce() -> R` - The closure to execute with batching enabled.
    ///
    /// # Returns
    ///
    /// - `R` - The result of the closure execution.
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
            // borrow is released before we mutate `SIGNAL_UPDATE_REGISTRY`
            // in the loop body below. (`HashSet::drain` requires the
            // `RangeFull` pattern which Rust 2024 reserves as the
            // struct-update syntax shorthand.)
            let dirty_keys: HashSet<usize> = take(Registry::get_mut_dirty_update_ids());
            if dirty_keys.is_empty() {
                break;
            }
            for key in dirty_keys {
                let entry: SignalUpdateEntry =
                    match Registry::get_mut_update_registry().remove(&key) {
                        Some(removed_entry) => removed_entry,
                        None => continue,
                    };
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
                    let slot: &mut SignalUpdateSlot = unsafe { &mut *entry };
                    if !slot.get_removed() {
                        slot.set_callback(Some(callback));
                    }
                }
                let slot: &SignalUpdateSlot = unsafe { &*entry };
                if slot.get_removed() {
                    unsafe {
                        let _: Box<SignalUpdateSlot> = Box::from_raw(entry);
                    }
                    continue;
                }
                let registry: &mut HashMap<usize, SignalUpdateEntry> =
                    Registry::get_mut_update_registry();
                if registry.contains_key(&key) {
                    unsafe {
                        let _: Box<SignalUpdateSlot> = Box::from_raw(entry);
                    }
                    continue;
                }
                registry.insert(key, entry);
            }
            iterations += 1;
            if iterations >= MAX_ITERATIONS {
                break;
            }
        }
        SIGNAL_UPDATE_DISPATCHING.store(false, Ordering::Relaxed);
    }
}
