use super::*;

/// SAFETY: `HandlerRegistryCell` is only used in single-threaded WASM contexts.
unsafe impl Sync for HandlerRegistryCell {}

/// SAFETY: `DelegatedEventsCell` is only used in single-threaded WASM contexts.
unsafe impl Sync for DelegatedEventsCell {}

/// SAFETY: `SignalUpdateRegistryCell` is only used in single-threaded WASM contexts.
unsafe impl Sync for SignalUpdateRegistryCell {}

/// SAFETY: `DirtyUpdateIdsCell` is only used in single-threaded WASM contexts.
unsafe impl Sync for DirtyUpdateIdsCell {}

/// SAFETY: `WindowEventRegistryCell` is only used in single-threaded WASM contexts.
unsafe impl Sync for WindowEventRegistryCell {}

/// SAFETY: `NodeRefRegistryCell` is only used in single-threaded WASM contexts.
unsafe impl Sync for NodeRefRegistryCell {}

/// SAFETY: `BindingCleanupsCell` is only used in single-threaded WASM contexts.
unsafe impl Sync for BindingCleanupsCell {}

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
/// and DOM event handler registries. All methods are thread-safe for single-threaded
/// WASM contexts.
impl Registry {
    /// Returns a shared reference to the delegated events set.
    ///
    /// # Returns
    ///
    /// - `&'static HashSet<&'static str>` - A shared reference to the global set of delegated event names.
    pub(crate) fn get_delegated_events() -> &'static HashSet<&'static str> {
        unsafe {
            &*(*std::ptr::addr_of!(DELEGATED_EVENTS))
                .deref()
                .get_0()
                .get()
        }
    }

    /// Returns a mutable reference to the delegated events set.
    ///
    /// # Returns
    ///
    /// - `&'static mut HashSet<&'static str>` - A mutable reference to the global set of delegated event names.
    pub(crate) fn get_mut_delegated_events() -> &'static mut HashSet<&'static str> {
        unsafe {
            &mut *(*std::ptr::addr_of_mut!(DELEGATED_EVENTS))
                .deref()
                .get_0()
                .get()
        }
    }

    /// Returns a mutable reference to the signal update registry.
    ///
    /// # Returns
    ///
    /// - `&'static mut HashMap<usize, SignalUpdateEntry>` - A mutable reference to the global signal update registry.
    pub(crate) fn get_mut_update_registry() -> &'static mut HashMap<usize, SignalUpdateEntry> {
        unsafe {
            &mut *(*std::ptr::addr_of_mut!(SIGNAL_UPDATE_REGISTRY))
                .deref()
                .get_0()
                .get()
        }
    }

    /// Returns a mutable reference to the dirty-id set used by the OPT 6
    /// dispatcher fast path.
    ///
    /// # Returns
    ///
    /// - `&'static mut HashSet<usize>` - A mutable reference to the global dirty-id set.
    pub(crate) fn get_mut_dirty_update_ids() -> &'static mut HashSet<usize> {
        unsafe {
            &mut *(*std::ptr::addr_of_mut!(DIRTY_UPDATE_IDS))
                .deref()
                .get_0()
                .get()
        }
    }

    /// Returns a shared reference to the window event registry.
    ///
    /// # Returns
    ///
    /// - `&'static WindowEventRegistryMap` - A shared reference to the global window event registry.
    pub(crate) fn get_window_registry() -> &'static WindowEventRegistryMap {
        unsafe {
            &*(*std::ptr::addr_of!(WINDOW_EVENT_REGISTRY))
                .deref()
                .get_0()
                .get()
        }
    }

    /// Returns a mutable reference to the window event registry.
    ///
    /// # Returns
    ///
    /// - `&'static mut WindowEventRegistryMap` - A mutable reference to the global window event registry.
    pub(crate) fn get_mut_window_registry() -> &'static mut WindowEventRegistryMap {
        unsafe {
            &mut *(*std::ptr::addr_of_mut!(WINDOW_EVENT_REGISTRY))
                .deref()
                .get_0()
                .get()
        }
    }

    /// Returns a mutable reference to the `NodeRef` unmount-clear registry.
    ///
    /// # Returns
    ///
    /// - `&'static mut NodeRefRegistryMap` - A mutable reference to the
    ///   global `NodeRef` registry.
    pub(crate) fn get_mut_noderef_registry() -> &'static mut NodeRefRegistryMap {
        unsafe {
            &mut *(*std::ptr::addr_of_mut!(NODEREF_REGISTRY))
                .deref()
                .get_0()
                .get()
        }
    }

    /// Returns a shared reference to the handler registry.
    ///
    /// # Returns
    ///
    /// - `&'static HandlerRegistryMap` - A shared reference to the global handler registry.
    pub(crate) fn get_handler_registry() -> &'static HandlerRegistryMap {
        unsafe {
            &*(*std::ptr::addr_of!(HANDLER_REGISTRY))
                .deref()
                .get_0()
                .get()
        }
    }

    /// Returns a mutable reference to the handler registry.
    ///
    /// # Returns
    ///
    /// - `&'static mut HandlerRegistryMap` - A mutable reference to the global handler registry.
    pub(crate) fn get_mut_handler_registry() -> &'static mut HandlerRegistryMap {
        unsafe {
            &mut *(*std::ptr::addr_of_mut!(HANDLER_REGISTRY))
                .deref()
                .get_0()
                .get()
        }
    }

    /// Dispatches a delegated event by walking up from `event.target` to
    /// find the nearest element with a `data-euv-id` attribute, then
    /// invoking the matching handler from the global registry.
    ///
    /// The ancestor walk happens entirely in JS via `euv_event_walk_ancestors`
    /// (a `#[wasm_bindgen(inline_js)]` glue function in this module), so the
    /// per-event cost is **one** WASM↔JS crossing for the walk itself,
    /// plus one callback invocation per marked ancestor. The previous
    /// Rust-side loop walked one layer at a time (`get_attribute` +
    /// `parent_element` = 2 JS crossings per layer); a depth-10 click used
    /// to cost 20 crossings; it now costs 1 walk crossing + N callback
    /// crossings where N = number of `data-euv-id` ancestors (typically
    /// 1–3 for nested DOM).
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
        let id_chain: Array = euv_event_collect_id_chain(&event_value, max_depth);
        let chain_len: u32 = id_chain.length();
        for chain_index in 0..chain_len {
            let id_value: JsValue = id_chain.get(chain_index);
            let Some(euv_id_f64) = id_value.as_f64() else {
                continue;
            };
            let euv_id: usize = euv_id_f64 as usize;
            // Scoped lookup: clone the handler out of the live registry
            // and drop the registry borrow BEFORE invoking. Handlers
            // routinely re-render and thereby mutate the registry, so the
            // lookup must not alias the mutable access that `handle`
            // may perform. This replaces the previous full
            // `HandlerRegistryMap` clone per event with at most one
            // `Rc` clone of the winning handler.
            let handler_found: Option<NativeEventHandler> = Self::get_handler_registry()
                .get(&euv_id)
                .and_then(|event_map: &HashMap<&'static str, HandlerEntry>| {
                    event_map.get(&event_name)
                })
                .and_then(|entry: &HandlerEntry| {
                    let slot: &HandlerSlot = unsafe { &**entry };
                    slot.try_get_handler().as_ref().cloned()
                });
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
        let dirty_ids: &mut HashSet<usize> = Self::get_mut_dirty_update_ids();
        for dynamic_id in dynamic_ids {
            dirty_ids.insert(*dynamic_id);
        }
        let registry: &mut HashMap<usize, SignalUpdateEntry> = Self::get_mut_update_registry();
        for dynamic_id in dynamic_ids {
            if let Some(entry) = registry.get(dynamic_id) {
                let slot: &mut SignalUpdateSlot = unsafe { &mut **entry };
                if !slot.get_removed() {
                    slot.set_dirty(true);
                } else {
                    dirty_ids.remove(dynamic_id);
                }
            } else {
                dirty_ids.remove(dynamic_id);
            }
        }
    }

    /// Returns whether the signal update registry contains any dirty slots.
    ///
    /// OPT 6: now an O(1) check against `DIRTY_UPDATE_IDS` instead of an
    /// O(N) scan of every dynamic node.
    ///
    /// # Returns
    ///
    /// - `bool` - `true` if at least one dynamic node is marked dirty and not removed.
    pub(crate) fn has_dirty() -> bool {
        Self::get_mut_dirty_update_ids().iter().any(|id: &usize| {
            let registry: &HashMap<usize, SignalUpdateEntry> = Self::get_mut_update_registry();
            registry.get(id).is_some_and(|entry: &SignalUpdateEntry| {
                let slot: &SignalUpdateSlot = unsafe { &**entry };
                !slot.get_removed()
            })
        })
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
        let entry: SignalUpdateEntry = Box::into_raw(slot);
        if let Some(old_entry) = Self::get_mut_update_registry().insert(dynamic_id, entry) {
            unsafe {
                let _: Box<SignalUpdateSlot> = Box::from_raw(old_entry);
            }
        }
    }

    /// Registers a signal update callback for an attribute signal.
    ///
    /// Similar to `register_dynamic`, but for attribute-level signals that
    /// need to update DOM element attributes rather than entire subtrees.
    ///
    /// # Arguments
    ///
    /// - `usize` - The signal's inner address used as the registry key.
    /// - `Box<dyn FnMut()>` - The callback to invoke when the attribute needs updating.
    pub(crate) fn register_attr_listener(signal_key: usize, callback: Box<dyn FnMut()>) {
        let slot: Box<SignalUpdateSlot> =
            Box::new(SignalUpdateSlot::new(Some(callback), false, true));
        let entry: SignalUpdateEntry = Box::into_raw(slot);
        if let Some(old_entry) = Self::get_mut_update_registry().insert(signal_key, entry) {
            unsafe {
                let _: Box<SignalUpdateSlot> = Box::from_raw(old_entry);
            }
        }
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
        let registry_ref: &mut HandlerRegistryMap = Self::get_mut_handler_registry();
        let Some(event_map) = registry_ref.remove(&euv_id) else {
            return;
        };
        for (event_name, entry) in event_map {
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
    /// only mutated from the main thread; if a dispatch is in progress
    /// it is running in a separate microtask turn and cannot observe
    /// a stale `Some(entry)` here.
    ///
    /// OPT 6: also drops the id from `DIRTY_UPDATE_IDS` so the
    /// dispatcher does not re-discover a freed pointer on the next tick.
    ///
    /// # Arguments
    ///
    /// - `usize` - The dynamic node's unique ID.
    pub(crate) fn cleanup_dynamic_node(dynamic_id: usize) {
        Self::get_mut_dirty_update_ids().remove(&dynamic_id);
        if let Some(entry) = Self::get_mut_update_registry().remove(&dynamic_id) {
            unsafe {
                let _: Box<SignalUpdateSlot> = Box::from_raw(entry);
            }
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
        let map: &mut BindingCleanupsMap =
            unsafe { &mut *(*std::ptr::addr_of_mut!(BINDING_CLEANUPS)).get_0().get() };
        map.entry(euv_id).or_default().push(cleanup);
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
        let map: &mut BindingCleanupsMap =
            unsafe { &mut *(*std::ptr::addr_of_mut!(BINDING_CLEANUPS)).get_0().get() };
        map.remove(&euv_id)
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
        Self::get_delegated_events().contains(event_name)
    }

    /// Marks an event name as delegated in the global set.
    ///
    /// # Arguments
    ///
    /// - `&'static str` - The event name to mark as delegated.
    pub(crate) fn mark_delegated(event_name: &'static str) {
        Self::get_mut_delegated_events().insert(event_name);
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
    /// - `F: FnMut() + 'static` - The callback to invoke when the event fires.
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
        let registry: &mut WindowEventRegistryMap = Self::get_mut_window_registry();
        let is_new_event: bool = !registry.contains_key(event_name);
        registry
            .entry(event_name.to_string())
            .or_default()
            .push(entry);
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
        let registry: &mut WindowEventRegistryMap = Self::get_mut_window_registry();
        if let Some(handlers) = registry.get_mut(event_name) {
            handlers.retain(|(id, ptr): &WindowEventHandlerEntry| {
                if *id == handler_id {
                    unsafe {
                        let _: Box<Box<dyn FnMut()>> = Box::from_raw(*ptr);
                    }
                    false
                } else {
                    true
                }
            });
        }
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
            // OPT 15: in-place iterate the registered handler list instead
            // of `collect()`-ing the IDs into a Vec and then re-doing a
            // HashMap lookup per ID. `to_owned` clones the string once so
            // the closure does not borrow from the caller's `&str`.
            let event_name_for_iter: String = event_name_owned.clone();
            if let Some(handlers) = Self::get_window_registry().get(&event_name_for_iter) {
                for (_handler_id, callback_ptr) in handlers.iter() {
                    let callback: &mut Box<dyn FnMut() + 'static> = unsafe { &mut **callback_ptr };
                    callback();
                }
            }
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
        let registry: &mut NodeRefRegistryMap = Self::get_mut_noderef_registry();
        registry.entry(euv_id).or_default().push(entry);
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
        let registry: &mut NodeRefRegistryMap = Self::get_mut_noderef_registry();
        let Some(entries) = registry.remove(&euv_id) else {
            return;
        };
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
