use super::*;

/// Implementation of `From` trait for converting `usize` address into `&'static mut HandlerSlot`.
impl From<usize> for &'static mut HandlerSlot {
    /// Converts a memory address into a mutable reference to `HandlerSlot`.
    ///
    /// # Arguments
    ///
    /// - `usize` - The memory address of the `HandlerSlot` instance.
    ///
    /// # Returns
    ///
    /// - `&'static mut HandlerSlot` - A mutable reference at the given address.
    ///
    /// # Safety
    ///
    /// - The address is guaranteed to be a valid `HandlerSlot` instance
    ///   that was previously converted from a reference and is managed by the runtime.
    fn from(address: usize) -> Self {
        unsafe { &mut *(address as *mut HandlerSlot) }
    }
}

/// Static methods for managing framework registries.
///
/// Provides centralized access to event delegation, signal updates, window events,
/// and DOM event handler registries. Every registry is thread-local, so each
/// thread owns an independent set of tables and no method can observe another
/// thread's entries.
impl Registry {
    /// Runs `operation` with a mutable borrow of this thread's `registry`.
    ///
    /// This is the single door through which every registry write passes. It
    /// replaces the old `static mut` + `LazyLock<XCell(UnsafeCell<T>)>` +
    /// `unsafe impl Sync for XCell` form, which handed out
    /// `&'static mut T` to any number of callers at once: two threads could
    /// hold simultaneous `&mut` references to the same `HashMap`, and a
    /// racing `LazyLock` initialisation poisoned the global outright
    /// ("Lazy instance has previously been poisoned").
    ///
    /// The borrow is released before this returns, so no caller can hold
    /// registry access across a call that re-enters it. `try_borrow_mut`
    /// rather than `borrow_mut` means a re-entrant call degrades to
    /// `fallback` instead of panicking mid-update and leaving the registry
    /// half-mutated — in WASM a Rust panic has no unwind boundary, so the
    /// panic would abort the whole instance.
    ///
    /// `try_with` (not `with`) so a call arriving after this thread's
    /// locals have been destroyed returns `fallback` rather than
    /// panicking.
    ///
    /// # Arguments
    ///
    /// - `&'static LocalKey<RefCell<T>>` - The thread-local registry cell.
    /// - `F` - Closure receiving `&mut T`.
    /// - `R` - Value returned when the cell is already mutably borrowed.
    ///
    /// # Returns
    ///
    /// - `R` - The operation's result, or `fallback` if the borrow was refused.
    fn with_registry<T, F, R>(key: &'static LocalKey<RefCell<T>>, operation: F, fallback: R) -> R
    where
        F: FnOnce(&mut T) -> R,
    {
        // The fallback is parked in a `Cell` rather than moved into the
        // closure: both failure paths (cell already mutably borrowed, thread
        // local destroyed) need it, and `R` is not required to be `Copy` or
        // `Clone`. `Cell::set` takes `&self`, so the closure can still write
        // the operation's result back out through a shared borrow.
        let result: Cell<Option<R>> = Cell::new(Some(fallback));
        key.try_with(|cell: &RefCell<T>| {
            if let Ok(mut guard) = cell.try_borrow_mut() {
                result.set(Some(operation(&mut guard)));
            }
        })
        .ok();
        match result.take() {
            Some(value) => value,
            None => unreachable!("with_registry always leaves a result in the cell"),
        }
    }

    /// Runs `read` with a shared borrow of this thread's `registry`.
    ///
    /// Read counterpart to [`Registry::with_registry`]. Same
    /// `try_borrow` / `try_with` degradation: a refused read yields
    /// `fallback` (usually `false` / `None`) rather than panicking, because
    /// a dropped read costs one frame while a panic costs the whole
    /// instance.
    ///
    /// # Arguments
    ///
    /// - `&'static LocalKey<RefCell<T>>` - The thread-local registry cell.
    /// - `F` - Closure receiving `&T`.
    /// - `R` - Value returned when the cell is already borrowed.
    ///
    /// # Returns
    ///
    /// - `R` - The read's result, or `fallback` if the borrow was refused.
    fn read_registry<T, F, R>(key: &'static LocalKey<RefCell<T>>, read: F, fallback: R) -> R
    where
        F: FnOnce(&T) -> R,
    {
        let result: Cell<Option<R>> = Cell::new(Some(fallback));
        key.try_with(|cell: &RefCell<T>| {
            if let Ok(guard) = cell.try_borrow() {
                result.set(Some(read(&guard)));
            }
        })
        .ok();
        match result.take() {
            Some(value) => value,
            None => unreachable!("read_registry always leaves a result in the cell"),
        }
    }

    /// Returns the handler registered for `(euv_id, event_name)`, if any.
    ///
    /// Clones the `NativeEventHandler` and drops the registry borrow before
    /// returning, so the caller may safely invoke the handler: handlers
    /// routinely re-render, which re-enters the registry to register more
    /// slots. The previous implementation cloned the entire
    /// `HandlerRegistryMap` per event; this clones at most one `Rc`.
    ///
    /// # Arguments
    ///
    /// - `usize` - The element's `data-euv-id` value.
    /// - `&'static str` - The event name.
    ///
    /// # Returns
    ///
    /// - `Option<NativeEventHandler>` - The handler, if one is registered.
    pub(crate) fn get_handler(
        euv_id: usize,
        event_name: &'static str,
    ) -> Option<NativeEventHandler> {
        Self::read_registry(
            &HANDLER_REGISTRY,
            |registry: &HandlerRegistryMap| {
                let entry: HandlerEntry = *registry.get(&euv_id)?.get(&event_name)?;
                // SAFETY: an entry is only freed after `take_handler` /
                // `take_element_handlers` has pulled it out of the
                // registry, so a live entry always points at an allocated
                // slot.
                let slot: &HandlerSlot = unsafe { &*entry };
                slot.try_get_handler().as_ref().cloned()
            },
            None,
        )
    }

    /// Returns whether a handler slot already exists for `(euv_id, event_name)`.
    ///
    /// The read half of the check-then-insert pair in
    /// [`Registry::set_handler`] / [`Registry::insert_handler`]; the DOM
    /// work between the two calls cannot be done under a registry borrow.
    ///
    /// # Arguments
    ///
    /// - `usize` - The element's `data-euv-id` value.
    /// - `&'static str` - The event name.
    ///
    /// # Returns
    ///
    /// - `bool` - `true` when a slot is already registered.
    pub(crate) fn has_handler(euv_id: usize, event_name: &'static str) -> bool {
        Self::read_registry(
            &HANDLER_REGISTRY,
            |registry: &HandlerRegistryMap| {
                registry.get(&euv_id).is_some_and(
                    |event_map: &HashMap<&'static str, HandlerEntry>| {
                        event_map.contains_key(&event_name)
                    },
                )
            },
            false,
        )
    }

    /// Installs (or replaces) the handler on an existing handler slot.
    ///
    /// # Arguments
    ///
    /// - `usize` - The element's `data-euv-id` value.
    /// - `&'static str` - The event name.
    /// - `&NativeEventHandler` - The handler to install.
    pub(crate) fn set_handler(
        euv_id: usize,
        event_name: &'static str,
        handler: &NativeEventHandler,
    ) {
        Self::with_registry(
            &HANDLER_REGISTRY,
            |registry: &mut HandlerRegistryMap| {
                let entry: HandlerEntry = match registry.get_mut(&euv_id).and_then(
                    |event_map: &mut HashMap<&'static str, HandlerEntry>| {
                        event_map.get_mut(&event_name)
                    },
                ) {
                    Some(value) => *value,
                    None => return,
                };
                // SAFETY: see `get_handler` — a live entry always points at
                // an allocated slot.
                let slot: &mut HandlerSlot = unsafe { &mut *entry };
                slot.set_handler(Some(handler.clone()));
            },
            (),
        );
    }

    /// Stores a freshly built handler slot, returning any slot it replaced.
    ///
    /// The caller must have already established via
    /// [`Registry::has_handler`] that no slot exists, so this is a pure
    /// insert. The `Box` is allocated before the registry is touched,
    /// keeping the borrow free of re-entrant work; on a refused borrow the
    /// raw pointer is handed back to the caller rather than leaked.
    ///
    /// # Arguments
    ///
    /// - `usize` - The element's `data-euv-id` value.
    /// - `&'static str` - The event name.
    /// - `HandlerSlot` - The slot to store.
    ///
    /// # Returns
    ///
    /// - `Option<HandlerEntry>` - The replaced slot, if any.
    pub(crate) fn insert_handler(
        euv_id: usize,
        event_name: &'static str,
        slot: HandlerSlot,
    ) -> Option<HandlerEntry> {
        let pending: RefCell<Option<HandlerEntry>> =
            RefCell::new(Some(Box::into_raw(Box::new(slot))));
        let replaced: Option<HandlerEntry> = Self::with_registry(
            &HANDLER_REGISTRY,
            |registry: &mut HandlerRegistryMap| {
                let entry: HandlerEntry = pending.borrow_mut().take()?;
                registry
                    .entry(euv_id)
                    .or_default()
                    .insert(event_name, entry)
            },
            None,
        );
        replaced.or_else(|| pending.into_inner())
    }

    /// Removes the handler slot registered for `(euv_id, event_name)`.
    ///
    /// The `&mut HandlerSlot` the raw pointer denotes is returned by
    /// value, so the caller can run the DOM teardown
    /// (`removeEventListener`) and free the `Box` with the registry
    /// borrow already released.
    ///
    /// # Arguments
    ///
    /// - `usize` - The element's `data-euv-id` value.
    /// - `&'static str` - The event name.
    ///
    /// # Returns
    ///
    /// - `Option<HandlerEntry>` - The removed slot, if one existed.
    pub(crate) fn take_handler(euv_id: usize, event_name: &'static str) -> Option<HandlerEntry> {
        Self::with_registry(
            &HANDLER_REGISTRY,
            |registry: &mut HandlerRegistryMap| {
                registry.get_mut(&euv_id).and_then(
                    |event_map: &mut HashMap<&'static str, HandlerEntry>| {
                        event_map.remove(&event_name)
                    },
                )
            },
            None,
        )
    }

    /// Removes and returns every handler slot registered for `euv_id`.
    ///
    /// # Arguments
    ///
    /// - `usize` - The element's `data-euv-id` value.
    ///
    /// # Returns
    ///
    /// - `Vec<(&'static str, HandlerEntry)>` - The removed `(name, slot)` pairs.
    pub(crate) fn take_element_handlers(euv_id: usize) -> Vec<(&'static str, HandlerEntry)> {
        Self::with_registry(
            &HANDLER_REGISTRY,
            |registry: &mut HandlerRegistryMap| {
                registry
                    .remove(&euv_id)
                    .unwrap_or_default()
                    .into_iter()
                    .collect()
            },
            Vec::new(),
        )
    }

    /// Detaches a non-bubbling listener's DOM wiring and frees its slot.
    ///
    /// Runs `removeEventListener` on the element / `Function` pair the slot
    /// recorded at mount time, drops the handler, and reclaims the `Box`.
    /// The registry borrow is already released by the time this is called,
    /// which matters because `removeEventListener` crosses into JS and a JS
    /// callback could re-enter the registry.
    ///
    /// # Arguments
    ///
    /// - `&'static str` - The event name.
    /// - `HandlerEntry` - The removed slot.
    pub(crate) fn free_handler_slot(event_name: &'static str, entry: HandlerEntry) {
        let slot: &mut HandlerSlot = unsafe { &mut *entry };
        if let Some(element) = slot.try_get_element().as_ref().cloned()
            && let Some(listener_function) = slot.get_mut_listener_function().take()
        {
            let listener: &Function = listener_function.unchecked_ref::<Function>();
            let _: Result<(), JsValue> =
                element.remove_event_listener_with_callback(event_name, listener);
        }
        slot.set_handler(None);
        unsafe {
            let _: Box<HandlerSlot> = Box::from_raw(entry);
        }
    }

    /// Dispatches a delegated event by walking up from `event.target` to
    /// find the nearest element with a `data-euv-id` attribute, then
    /// invoking the matching handler from the global registry.
    ///
    /// The ancestor walk runs in a single injected global
    /// (`__euvEventIdChain`, installed once by `Mount::setup`), which walks
    /// `event.composedPath()` and returns every `data-euv-id` on the chain
    /// in one call. Per-event cost is therefore **one** WASM↔JS crossing
    /// for the whole chain, not two per ancestor layer.
    ///
    /// Before OPT 40 this was a Rust-side loop issuing `get_attribute` +
    /// `parent_node` per layer: a click inside the 402-node example event
    /// page measured **71 `getAttribute` calls**. Injecting a global at
    /// startup rather than using `#[wasm_bindgen(inline_js)]` keeps the
    /// deployed artefact count stable — every `inline_js` item would emit
    /// its own `pkg/snippets/.../inlineN.js`, so the file count would grow
    /// with the number of features. The Rust loop is retained as a
    /// correctness fallback for hosts where the injection cannot run.
    ///
    /// `max_depth` caps the ancestor walk at this many `parent_element`
    /// hops. The walk counts `event.target()` itself as depth 0. Pass
    /// `0` for an unbounded walk (the original behaviour) — note the JS
    /// glue treats `0` as "walk until `<html>`" per the call-site
    /// convention below; events in `HIGH_FREQUENCY_EVENTS` get a bounded
    /// cap so their per-event cost stays proportional to a small constant
    /// rather than DOM depth.
    ///
    /// # Arguments
    ///
    /// - `&Event` - The DOM event to dispatch.
    /// - `&'static str` - The event name (e.g., "click", "input").
    /// - `usize` - Upper bound on ancestor walk depth; `0` for unbounded.
    fn dispatch_delegated_event(event: &Event, event_name: &'static str, max_depth: usize) {
        // Clone the event into an owned `JsValue` so it can be handed both
        // to the JS id-chain walk and (cloned once more) to the winning
        // handler. The underlying DOM `Event` is reference-counted by
        // wasm-bindgen so the clone is cheap.
        let event_value: JsValue = event.clone().into();
        let id_chain: Float64Array = euv_event_collect_id_chain(&event_value, max_depth);
        // One bulk copy for the whole chain instead of one `Array.get`
        // crossing per marked ancestor (ids are `< 2^53`, exact in f64).
        let mut chain: Vec<f64> = vec![0.0; id_chain.length() as usize];
        id_chain.copy_to(&mut chain);
        for id_value in chain {
            let euv_id: usize = id_value as usize;
            // Scoped lookup: clone the handler out of the live registry
            // and drop the registry borrow BEFORE invoking. Handlers
            // routinely re-render and thereby mutate the registry, so the
            // lookup must not alias the mutable access that `handle`
            // may perform. This replaces the previous full
            // `HandlerRegistryMap` clone per event with at most one
            // `Rc` clone of the winning handler.
            let handler_found: Option<NativeEventHandler> = Self::get_handler(euv_id, event_name);
            if let Some(active_handler) = handler_found {
                let event_for_handler: Event = event_value.clone().unchecked_into();
                active_handler.handle(event_for_handler);
                return;
            }
        }
    }

    /// Computes the JS-glue walk depth cap for an event name.
    ///
    /// Returns `0` for unbounded walks (the JS glue in `glue.rs` treats
    /// `0` as "walk until `<html>`"). Events listed in
    /// `HIGH_FREQUENCY_EVENTS` get a bounded cap (see
    /// `MAX_ANCESTOR_DEPTH_FOR_HIGH_FREQ`) so their per-event cost
    /// stays proportional to a small constant rather than DOM depth.
    ///
    /// # Arguments
    ///
    /// - `&str` - The event name (e.g., "click", "input").
    ///
    /// # Returns
    ///
    /// - `usize` - Walk depth cap; `0` means unbounded.
    fn dispatch_max_depth(event_name: &str) -> usize {
        if HIGH_FREQUENCY_EVENTS.contains(&event_name) {
            MAX_ANCESTOR_DEPTH_FOR_HIGH_FREQ
        } else {
            0
        }
    }

    /// Ensures a global capturing-phase listener is registered on `window`
    /// for the given event type.
    ///
    /// Uses event delegation to minimize the number of event listeners attached
    /// to the DOM. All events of the same type are handled by a single window-level
    /// listener that walks the DOM tree to find the appropriate handler.
    ///
    /// # Arguments
    ///
    /// - `&'static str` - The event name to delegate (e.g., "click", "input").
    pub(crate) fn delegation(event_name: &'static str) {
        if Self::is_delegated(event_name) {
            return;
        }
        // Compute the depth cap for this event name once at registration
        // time and capture it in the closure — avoids re-computing on
        // every event dispatch. Events listed in HIGH_FREQUENCY_EVENTS
        // get a bounded walk (see MAX_ANCESTOR_DEPTH_FOR_HIGH_FREQ);
        // everything else gets the original unbounded behaviour, which
        // the JS glue encodes as `0`.
        let max_depth: usize = Self::dispatch_max_depth(event_name);
        let closure: Closure<dyn FnMut(Event)> = Closure::wrap(Box::new(move |event: Event| {
            Self::dispatch_delegated_event(&event, event_name, max_depth);
        }));
        let window: Window = match window() {
            Some(window_instance) => window_instance,
            None => return,
        };
        let _: Result<(), JsValue> = window.add_event_listener_with_callback_and_bool(
            event_name,
            closure.as_ref().unchecked_ref(),
            true,
        );
        closure.forget();
        Self::mark_delegated(event_name);
    }

    /// Returns whether a dynamic node id currently has a live slot.
    ///
    /// # Arguments
    ///
    /// - `&HashMap<usize, SignalUpdateEntry>` - The dynamic slot registry.
    /// - `&usize` - The dynamic node's unique ID.
    ///
    /// # Returns
    ///
    /// - `bool` - `true` when the slot is registered and not marked removed.
    fn is_live_dynamic(registry: &HashMap<usize, SignalUpdateEntry>, dynamic_id: &usize) -> bool {
        registry
            .get(dynamic_id)
            .is_some_and(|entry: &SignalUpdateEntry| {
                // SAFETY: an entry is only freed after `cleanup_dynamic_node` /
                // `take_dynamic` has pulled the id out of the registry, so a
                // live entry always points at an allocated slot.
                let slot: &SignalUpdateSlot = unsafe { &**entry };
                !slot.get_removed()
            })
    }

    /// Marks the specified dynamic node IDs as dirty, scheduling them for re-render.
    ///
    /// Called when a signal changes to notify all dependent dynamic nodes
    /// that they need to update their DOM representation.
    ///
    /// OPT 6: also inserts each id into `DIRTY_UPDATE_IDS` so the
    /// dispatcher's `drain()` loop only visits dynamic nodes that actually
    /// changed, instead of scanning the whole registry. The previous
    /// `has_dirty` implementation iterated every registry entry and
    /// dereferenced a raw pointer per entry just to read the `dirty` flag.
    ///
    /// # Arguments
    ///
    /// - `&[usize]` - The dynamic node IDs to mark as dirty.
    pub(crate) fn mark_dirty(dynamic_ids: &[usize]) {
        // Pass 1: insert into the dirty set under its own borrow.
        Self::with_registry(
            &DIRTY_UPDATE_IDS,
            |ids: &mut HashSet<usize>| {
                for dynamic_id in dynamic_ids {
                    ids.insert(*dynamic_id);
                }
            },
            (),
        );
        // Pass 2: drop ids that have no live slot. The liveness test reads
        // the update registry, so it runs as a separate borrow — the two
        // sets are independent cells and this keeps both borrows short.
        let live: HashSet<usize> = Self::read_registry(
            &SIGNAL_UPDATE_REGISTRY,
            |registry: &HashMap<usize, SignalUpdateEntry>| {
                dynamic_ids
                    .iter()
                    .filter(|dynamic_id: &&usize| Self::is_live_dynamic(registry, dynamic_id))
                    .copied()
                    .collect()
            },
            HashSet::new(),
        );
        Self::with_registry(
            &DIRTY_UPDATE_IDS,
            |ids: &mut HashSet<usize>| {
                for dynamic_id in dynamic_ids {
                    if !live.contains(dynamic_id) {
                        ids.remove(dynamic_id);
                    }
                }
            },
            (),
        );
    }

    /// Returns whether the signal update registry contains any dirty slots.
    ///
    /// OPT 6: now an O(脏节点数) check against `DIRTY_UPDATE_IDS` instead of
    /// an O(N) scan of every dynamic node.
    ///
    /// # Returns
    ///
    /// - `bool` - `true` if at least one dynamic node is marked dirty and not removed.
    pub(crate) fn has_dirty() -> bool {
        let dirty_ids: HashSet<usize> = Self::take_dirty_update_ids_peek();
        if dirty_ids.is_empty() {
            return false;
        }
        Self::read_registry(
            &SIGNAL_UPDATE_REGISTRY,
            |registry: &HashMap<usize, SignalUpdateEntry>| {
                dirty_ids
                    .iter()
                    .any(|id: &usize| Self::is_live_dynamic(registry, id))
            },
            false,
        )
    }

    /// Registers a signal update callback for a DynamicNode placeholder.
    ///
    /// Associates a re-render callback with a dynamic node ID so that when
    /// the node is marked dirty, the callback can be invoked to update the DOM.
    ///
    /// # Arguments
    ///
    /// - `usize` - The unique dynamic node ID.
    /// - `Box<dyn FnMut()>` - The callback to invoke when the node needs re-rendering.
    pub(crate) fn register_dynamic(dynamic_id: usize, callback: Box<dyn FnMut()>) {
        let slot: Box<SignalUpdateSlot> =
            Box::new(SignalUpdateSlot::new(Some(callback), false, true));
        let pending: RefCell<Option<SignalUpdateEntry>> = RefCell::new(Some(Box::into_raw(slot)));
        let retired: Option<SignalUpdateEntry> = Self::with_registry(
            &SIGNAL_UPDATE_REGISTRY,
            |registry: &mut HashMap<usize, SignalUpdateEntry>| {
                let entry: SignalUpdateEntry = pending.borrow_mut().take()?;
                registry.insert(dynamic_id, entry)
            },
            None,
        );
        // A refused borrow leaves the slot unregistered but still owned by
        // us; hand the pointer back so it is freed rather than leaked.
        let orphan: Option<SignalUpdateEntry> = retired.or_else(|| pending.into_inner());
        if let Some(retired_entry) = orphan {
            unsafe {
                let _: Box<SignalUpdateSlot> = Box::from_raw(retired_entry);
            }
        }
    }

    /// Stores a dynamic node slot back after the caller released it.
    ///
    /// # Arguments
    ///
    /// - `usize` - The dynamic node's unique ID.
    /// - `SignalUpdateEntry` - The slot to store.
    pub(crate) fn put_dynamic(dynamic_id: usize, entry: SignalUpdateEntry) {
        Self::with_registry(
            &SIGNAL_UPDATE_REGISTRY,
            |registry: &mut HashMap<usize, SignalUpdateEntry>| {
                registry.insert(dynamic_id, entry);
            },
            (),
        );
    }

    /// Returns whether a dynamic node id is still registered.
    ///
    /// # Arguments
    ///
    /// - `usize` - The dynamic node's unique ID.
    ///
    /// # Returns
    ///
    /// - `bool` - `true` while the id is present in the registry.
    pub(crate) fn has_dynamic(dynamic_id: usize) -> bool {
        Self::read_registry(
            &SIGNAL_UPDATE_REGISTRY,
            |registry: &HashMap<usize, SignalUpdateEntry>| registry.contains_key(&dynamic_id),
            false,
        )
    }

    /// Removes a dynamic node's slot from the registry and returns it.
    ///
    /// # Arguments
    ///
    /// - `usize` - The dynamic node's unique ID.
    ///
    /// # Returns
    ///
    /// - `Option<SignalUpdateEntry>` - The removed slot, if one existed.
    pub(crate) fn take_dynamic(dynamic_id: usize) -> Option<SignalUpdateEntry> {
        Self::with_registry(
            &SIGNAL_UPDATE_REGISTRY,
            |registry: &mut HashMap<usize, SignalUpdateEntry>| registry.remove(&dynamic_id),
            None,
        )
    }

    /// Marks the slot backing a DynamicNode as removed and frees its backing
    /// allocation.
    ///
    /// Intended to be called when the placeholder element is removed
    /// from the DOM by the surrounding diff / patch logic; this
    /// function itself only updates the registry and does not touch
    /// the DOM.
    ///
    /// The `SignalUpdateSlot` is removed from the registry and its
    /// `Box` is freed immediately rather than waiting for the next
    /// dispatch cycle's sweep, so that detached subtrees do not pin
    /// their callback allocations in the registry between unmount and
    /// the next scheduled update. This is safe because the registry is
    /// only mutated from the thread that owns it; if a dispatch is in
    /// progress it is running in a separate microtask turn and cannot
    /// observe a stale entry here.
    ///
    /// OPT 6: also drops the id from `DIRTY_UPDATE_IDS` so the
    /// dispatcher does not re-discover a freed pointer on the next tick.
    ///
    /// # Arguments
    ///
    /// - `usize` - The dynamic node's unique ID.
    pub(crate) fn cleanup_dynamic_node(dynamic_id: usize) {
        Self::with_registry(
            &DIRTY_UPDATE_IDS,
            |ids: &mut HashSet<usize>| {
                ids.remove(&dynamic_id);
            },
            (),
        );
        if let Some(entry) = Self::take_dynamic(dynamic_id) {
            unsafe {
                let _: Box<SignalUpdateSlot> = Box::from_raw(entry);
            }
        }
    }

    /// Drains the dirty-id set, handing the whole batch to the caller.
    ///
    /// `HashSet::drain` requires the `RangeFull` pattern which Rust 2024
    /// reserves as the struct-update syntax shorthand, so the set is
    /// `take`n instead: `with_registry` swaps a fresh empty set in and
    /// returns the old one by value, releasing the borrow before the
    /// dispatch loop mutates the update registry.
    ///
    /// # Returns
    ///
    /// - `HashSet<usize>` - The drained dynamic node ids.
    pub(crate) fn take_dirty_update_ids() -> HashSet<usize> {
        Self::with_registry(
            &DIRTY_UPDATE_IDS,
            |ids: &mut HashSet<usize>| take(ids),
            HashSet::new(),
        )
    }

    /// Returns a snapshot of the pending dirty ids without draining them.
    ///
    /// [`Registry::has_dirty`] is a pure predicate on the caller side, so
    /// it must not consume the set. Cloning the set is cheaper than
    /// cloning the whole update registry would be, and the dirty set only
    /// holds ids that changed since the last tick.
    ///
    /// # Returns
    ///
    /// - `HashSet<usize>` - Snapshot of the pending dirty ids.
    fn take_dirty_update_ids_peek() -> HashSet<usize> {
        Self::read_registry(
            &DIRTY_UPDATE_IDS,
            |ids: &HashSet<usize>| ids.clone(),
            HashSet::new(),
        )
    }

    /// Cleans up all handler entries associated with a DOM element.
    ///
    /// Removes all event handlers registered for the given element ID,
    /// detaching any direct event listeners from the DOM.
    ///
    /// # Arguments
    ///
    /// - `usize` - The element's unique `data-euv-id` value.
    pub(crate) fn cleanup_element(euv_id: usize) {
        // Take the entries out under the registry borrow, then run the DOM
        // teardown with the borrow released: `removeEventListener` crosses
        // into JS and a JS callback could re-enter the registry.
        let entries: Vec<(&'static str, HandlerEntry)> = Self::take_element_handlers(euv_id);
        for (event_name, entry) in entries {
            Self::free_handler_slot(event_name, entry);
        }
    }

    /// Appends a binding-teardown thunk for `euv_id`.
    ///
    /// Called by the signal attribute / `inner_html` mount paths so the
    /// subscription they just installed can be detached exactly once when
    /// the element leaves the DOM. Multiple pushes for the same element
    /// accumulate; `take_binding_cleanups` drains them in one pass.
    ///
    /// # Arguments
    ///
    /// - `usize` - The element's `data-euv-id` value.
    /// - `BindingCleanup` - The teardown thunk to store.
    pub(crate) fn push_binding_cleanup(euv_id: usize, cleanup: BindingCleanup) {
        Self::with_registry(
            &BINDING_CLEANUPS,
            |map: &mut BindingCleanupsMap| {
                map.entry(euv_id).or_default().push(cleanup);
            },
            (),
        );
    }

    /// Removes and returns every binding-teardown thunk for `euv_id`.
    ///
    /// `Some(vec)` when the element had signal bindings; `None` when it
    /// never did (the common case for static subtrees).
    ///
    /// # Arguments
    ///
    /// - `usize` - The element's `data-euv-id` value.
    ///
    /// # Returns
    ///
    /// - `Option<Vec<BindingCleanup>>` - The drained thunks, if any.
    pub(crate) fn take_binding_cleanups(euv_id: usize) -> Option<Vec<BindingCleanup>> {
        Self::with_registry(
            &BINDING_CLEANUPS,
            |map: &mut BindingCleanupsMap| map.remove(&euv_id),
            None,
        )
    }

    /// Returns whether the given event name is a non-bubbling event.
    ///
    /// Non-bubbling events (like "load", "error", "focus") must be attached
    /// directly to elements rather than using event delegation.
    ///
    /// # Arguments
    ///
    /// - `&str` - The event name to check.
    ///
    /// # Returns
    ///
    /// - `bool` - `true` if the event does not bubble up the DOM tree.
    pub(crate) fn is_non_bubbling(event_name: &str) -> bool {
        NON_BUBBLING_EVENTS.contains(&event_name)
    }

    /// Returns whether the event name is already delegated.
    ///
    /// # Arguments
    ///
    /// - `&str` - The event name to check.
    ///
    /// # Returns
    ///
    /// - `bool` - `true` if a window-level listener already exists for this event type.
    pub(crate) fn is_delegated(event_name: &str) -> bool {
        Self::read_registry(
            &DELEGATED_EVENTS,
            |events: &HashSet<&'static str>| events.contains(event_name),
            false,
        )
    }

    /// Marks an event name as delegated in the global set.
    ///
    /// # Arguments
    ///
    /// - `&'static str` - The event name to mark as delegated.
    pub(crate) fn mark_delegated(event_name: &'static str) {
        Self::with_registry(
            &DELEGATED_EVENTS,
            |events: &mut HashSet<&'static str>| {
                events.insert(event_name);
            },
            (),
        );
    }

    /// Registers a callback for a window-level event using the proxy pattern.
    ///
    /// Creates a shared window event listener that dispatches to all registered
    /// callbacks for the same event type. Returns a unique handler ID for later
    /// unregistration.
    ///
    /// # Arguments
    ///
    /// - `&str` - The event name to listen for (e.g., "resize", "hashchange").
    /// - `F` - The callback to invoke when the event fires.
    ///
    /// # Returns
    ///
    /// - `usize` - A unique handler ID that can be used to unregister the callback.
    pub(crate) fn register_window_event<F>(event_name: &str, callback: F) -> usize
    where
        F: FnMut() + 'static,
    {
        let handler_id: usize = NEXT_WINDOW_HANDLER_ID.fetch_add(1, Ordering::Relaxed);
        let boxed: Box<Box<dyn FnMut()>> = Box::new(Box::new(callback));
        let entry: WindowEventHandlerEntry = (handler_id, Box::into_raw(boxed));
        let pending: RefCell<Option<WindowEventHandlerEntry>> = RefCell::new(Some(entry));
        let is_new_event: bool = Self::with_registry(
            &WINDOW_EVENT_REGISTRY,
            |registry: &mut WindowEventRegistryMap| {
                let entry: WindowEventHandlerEntry = match pending.borrow_mut().take() {
                    Some(value) => value,
                    None => return false,
                };
                let is_new: bool = !registry.contains_key(event_name);
                registry
                    .entry(event_name.to_string())
                    .or_default()
                    .push(entry);
                is_new
            },
            false,
        );
        // A refused borrow drops the registration and orphans the callback
        // box; hand the pointer back so it is freed rather than leaked.
        if let Some(orphaned) = pending.into_inner() {
            unsafe {
                let _: Box<Box<dyn FnMut()>> = Box::from_raw(orphaned.1);
            }
        }
        if is_new_event {
            Self::window_event_listener(event_name);
        }
        handler_id
    }

    /// Unregisters a window event handler by its event name and handler ID.
    ///
    /// Removes the callback from the registry and frees its memory.
    ///
    /// # Arguments
    ///
    /// - `&str` - The event name the handler was registered for.
    /// - `usize` - The handler ID returned by `register_window_event`.
    pub(crate) fn unregister_window_event(event_name: &str, handler_id: usize) {
        let removed: Option<WindowEventHandlerEntry> = Self::with_registry(
            &WINDOW_EVENT_REGISTRY,
            |registry: &mut WindowEventRegistryMap| {
                registry.get_mut(event_name).and_then(
                    |handlers: &mut Vec<WindowEventHandlerEntry>| {
                        let index: Option<usize> = handlers
                            .iter()
                            .position(|(id, _ptr): &WindowEventHandlerEntry| *id == handler_id);
                        index.map(|found: usize| handlers.remove(found))
                    },
                )
            },
            None,
        );
        if let Some((_id, callback_ptr)) = removed {
            unsafe {
                let _: Box<Box<dyn FnMut()>> = Box::from_raw(callback_ptr);
            }
        }
    }

    /// Runs every callback registered for `event_name`, outside the registry
    /// borrow.
    ///
    /// The handler list is taken out of the registry by value, so each
    /// callback may freely call `register_window_event` /
    /// `unregister_window_event` (both of which write the registry) without
    /// aliasing the vector this loop walks.
    ///
    /// # Arguments
    ///
    /// - `&str` - The event name whose handlers should run.
    fn fire_window_event(event_name: &str) {
        // Take the handler list out, but leave the KEY in place. A callback
        // is free to re-render, and the re-render path calls
        // `register_window_event` for this same event name. `register` only
        // installs a new native `window.addEventListener` when the key was
        // absent, so removing the key here would make every re-entrant
        // registration believe it was the first one and stack a duplicate
        // native listener on `window` every navigation — after which the
        // cleanup bookkeeping and the live handlers disagree and the route
        // stops updating.
        let entries: Vec<WindowEventHandlerEntry> = Self::with_registry(
            &WINDOW_EVENT_REGISTRY,
            |registry: &mut WindowEventRegistryMap| match registry.get_mut(event_name) {
                Some(handlers) => take(handlers),
                None => Vec::new(),
            },
            Vec::new(),
        );
        for entry in entries {
            let callback_ptr: *mut Box<dyn FnMut()> = entry.1;
            // SAFETY: the entry was removed from the registry above, so we
            // are its sole owner; the box is reinserted or freed below.
            // `callback_ptr` is a `*mut Box<dyn FnMut()>`, so one deref
            // yields the `Box` and calling it works through `Box`'s
            // `FnMut` impl.
            let callback: &mut Box<dyn FnMut()> = unsafe { &mut *callback_ptr };
            callback();
            // OPT 15 continuation: the handler is merged back only if no
            // re-entrant call claimed it while the callback ran. A
            // callback that unregistered itself (directly or via a nested
            // `unregister_window_event`) has already freed the box, so the
            // check must happen before the merge — reinserting it would
            // hand out a dangling pointer.
            if Self::window_event_claimed(event_name, callback_ptr) {
                unsafe {
                    let _: Box<Box<dyn FnMut()>> = Box::from_raw(callback_ptr);
                }
            } else {
                Self::restore_window_event(event_name, entry);
            }
        }
    }

    /// Returns whether a window event callback is still owned by the registry.
    ///
    /// After [`Registry::fire_window_event`] takes the handler list out, a
    /// re-entrant `unregister_window_event` can no longer find the id (it
    /// already left the registry), so the round-trip through the registry
    /// is what tells the dispatch loop whether a handler survived. The
    /// caller reinserts the entry when this returns `false`; when it
    /// returns `true` the handler is gone and its box must be freed.
    ///
    /// # Arguments
    ///
    /// - `&str` - The event name.
    /// - `*mut Box<dyn FnMut()>` - The callback box address.
    ///
    /// # Returns
    ///
    /// - `bool` - `true` when the registry no longer owns this callback.
    fn window_event_claimed(event_name: &str, callback_ptr: *mut Box<dyn FnMut()>) -> bool {
        Self::read_registry(
            &WINDOW_EVENT_REGISTRY,
            |registry: &WindowEventRegistryMap| {
                registry
                    .get(event_name)
                    .is_some_and(|handlers: &Vec<WindowEventHandlerEntry>| {
                        handlers
                            .iter()
                            .any(|(_id, ptr): &WindowEventHandlerEntry| *ptr == callback_ptr)
                    })
            },
            true,
        )
    }

    /// Puts a fired window event callback back on the handler list.
    ///
    /// # Arguments
    ///
    /// - `&str` - The event name.
    /// - `WindowEventHandlerEntry` - The entry to reinsert.
    pub(crate) fn restore_window_event(event_name: &str, entry: WindowEventHandlerEntry) {
        Self::with_registry(
            &WINDOW_EVENT_REGISTRY,
            |registry: &mut WindowEventRegistryMap| {
                registry
                    .entry(event_name.to_string())
                    .or_default()
                    .push(entry);
            },
            (),
        );
    }

    /// Ensures a single `window.addEventListener` listener is registered
    /// for the given event name that dispatches to all registered callbacks.
    ///
    /// # Arguments
    ///
    /// - `&str` - The event name to register the listener for.
    fn window_event_listener(event_name: &str) {
        let event_name_owned: String = event_name.to_string();
        let closure: Closure<dyn FnMut()> = Closure::wrap(Box::new(move || {
            // OPT 15: `fire_window_event` takes the handler list out of
            // the registry by value and iterates it in place, instead of
            // `collect()`-ing the IDs into a Vec and then re-doing a
            // HashMap lookup per ID. The event name is owned by the
            // closure (a `&str` lookup key borrows it — no per-event
            // `String` clone).
            //
            // The registry borrow MUST be released before any callback
            // runs: a callback is free to call `unregister_window_event`
            // or `register_window_event`, and holding a borrow across that
            // would — in the old `&'static mut` form — mutate the very
            // `Vec` this loop is walking.
            Self::fire_window_event(&event_name_owned);
        }));
        let window: Window = match window() {
            Some(window_instance) => window_instance,
            None => return,
        };
        let _: Result<(), JsValue> =
            window.add_event_listener_with_callback(event_name, closure.as_ref().unchecked_ref());
        closure.forget();
    }

    /// Registers a `NodeRef` handle against the given `euv_id` so it can be
    /// cleared when the DOM element is unmounted.
    ///
    /// NP-3: fixes a correctness bug where `NodeRef::get()` would return a
    /// stale `JsValue` after the underlying VDOM subtree was destroyed
    /// (the `NodeRef` was `set` on mount but never cleared on unmount).
    /// The fix records a clone of the `NodeRef`'s shared interior cell
    /// keyed by the element's `euv_id`; `cleanup_noderefs` walks the
    /// entry list and calls `clear()` on each cell so consumers see
    /// `None` again.
    ///
    /// # Arguments
    ///
    /// - `usize` - The element's unique `data-euv-id` value.
    /// - `NodeRefEntry` - A clone of the `NodeRef`'s interior `Rc<UnsafeCell<Option<JsValue>>>`.
    pub(crate) fn register_noderef(euv_id: usize, entry: NodeRefEntry) {
        Self::with_registry(
            &NODEREF_REGISTRY,
            |registry: &mut NodeRefRegistryMap| {
                registry.entry(euv_id).or_default().push(entry);
            },
            (),
        );
    }

    /// Clears every `NodeRef` handle that was registered against `euv_id`
    /// and forgets the list.
    ///
    /// Called from `cleanup_subtree` after `cleanup_element` so consumers
    /// that read a `NodeRef` after the DOM element is removed see `None`
    /// instead of a detached `JsValue`. The `Rc` clones kept in the
    /// registry are dropped here, releasing the shared interior cell when
    /// no other clone remains.
    ///
    /// # Arguments
    ///
    /// - `usize` - The element's unique `data-euv-id` value.
    pub(crate) fn cleanup_noderefs(euv_id: usize) {
        let entries: Vec<NodeRefEntry> = Self::with_registry(
            &NODEREF_REGISTRY,
            |registry: &mut NodeRefRegistryMap| registry.remove(&euv_id).unwrap_or_default(),
            Vec::new(),
        );
        for entry in entries {
            // SAFETY: `NodeRef::clear` is the only mutating accessor on the
            // cell; mounting-time `NodeRef::set` and unmount-time `clear`
            // are serialised by the single-threaded WASM execution model.
            let cell: *mut Option<JsValue> = entry.as_ref().get();
            unsafe {
                let _: Option<JsValue> = (*cell).take();
            }
        }
    }
}
