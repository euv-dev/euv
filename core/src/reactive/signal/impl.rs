use super::*;

/// Implementation of reactive signal operations.
impl<T> Signal<T>
where
    T: Clone + PartialEq + 'static,
{
    /// Returns a shared reference to the global typed signal slab.
    ///
    /// # Returns
    ///
    /// - `&'static SignalSlab` - A shared reference to the global signal slab.
    #[allow(static_mut_refs)]
    fn slab() -> &'static SignalSlab {
        unsafe { &*SIGNAL_SLAB.deref().get() }
    }

    /// Returns a mutable reference to the global typed signal slab.
    ///
    /// # Returns
    ///
    /// - `&'static mut SignalSlab` - A mutable reference to the global signal slab.
    #[allow(static_mut_refs)]
    fn slab_mut() -> &'static mut SignalSlab {
        unsafe { &mut *SIGNAL_SLAB.deref().get() }
    }

    /// Creates a new `Signal` with the given initial value.
    ///
    /// Stores the `SignalInner<T>` in the global slab ([`SIGNAL_SLAB`]) and
    /// returns a `Signal<T>` handle carrying the slot index. The slab is
    /// append-only: a slot always belongs to the `Signal` that created it,
    /// so stale handles can never observe a recycled slot of a different
    /// type.
    ///
    /// # Arguments
    ///
    /// - `T: Clone + PartialEq + 'static` - The initial value of the signal.
    ///
    /// # Returns
    ///
    /// - `Self` - A handle to the newly created reactive signal.
    pub fn create(value: T) -> Self {
        let inner: SignalInner<T> = SignalInner::new(value, Vec::new(), true);
        let idx: usize = Self::slab_mut().insert(inner);
        let mut signal: Self = Self::new(0, PhantomData);
        signal.set_inner(idx);
        signal
    }

    /// Returns the current value of the signal.
    ///
    /// Directly reads the value from the slot stored in the global slab.
    ///
    /// If the signal has been marked inactive (`alive == false`), returns the
    /// last stored value without registering tracking dependencies. This
    /// ensures that stale async callbacks (e.g., orphaned `setInterval`)
    /// holding a `Signal` copy can still call `.get()` safely without
    /// triggering side effects or panics.
    ///
    /// If a tracking context is active (i.e., a DynamicNode is being rendered),
    /// automatically registers the current dynamic node as a dependent of
    /// this signal for precise reactive updates.
    ///
    /// # Returns
    ///
    /// - `T: Clone + PartialEq + 'static` - The current value of the signal.
    pub fn get(&self) -> T {
        let idx: usize = self.get_inner();
        let Some(inner) = Self::slab_mut().get_mut::<T>(idx) else {
            // Out-of-bounds handle: the slot index was never issued by this
            // slab (a corrupted or foreign handle). Slots are never freed or
            // recycled, so this branch is unreachable for any handle produced
            // by `Signal::create`. Returning a zero-initialized `T` keeps the
            // defensive contract deterministic instead of panicking.
            return unsafe { std::mem::zeroed() };
        };
        if !inner.get_alive() {
            return inner.get_value().clone();
        }
        let tracking_id: usize = CURRENT_TRACKING_DYNAMIC_ID.load(Ordering::Relaxed);
        if tracking_id != usize::MAX {
            self.add_dependent(tracking_id);
        }
        inner.get_value().clone()
    }

    /// Read-only access to the signal value without cloning.
    ///
    /// OPT 17: callers that only need to inspect the value (e.g. format!, eq
    /// check, debug print, length) can borrow via `with(|v| ...)` and avoid
    /// one `T::clone` per call. The closure runs under the same tracking
    /// rules as `get` (still registers `CURRENT_TRACKING_DYNAMIC_ID` if a
    /// DynamicNode is rendering). The `T: Clone` bound stays on the impl
    /// because `get` is required by the existing public API; `with` is the
    /// zero-copy alternative for new code.
    ///
    /// # Arguments
    ///
    /// - `F: FnOnce(&T) -> R` - Closure receiving `&T`.
    ///
    /// # Returns
    ///
    /// - `R` - Whatever the closure returns.
    pub fn with<F, R>(&self, f: F) -> R
    where
        F: FnOnce(&T) -> R,
    {
        let idx: usize = self.get_inner();
        let Some(inner) = Self::slab_mut().get_mut::<T>(idx) else {
            // Out-of-bounds handle: unreachable for slab-issued handles (see
            // `get`). We avoid adding an `R: Default` bound to preserve the
            // public API (R is whatever the closure returns).
            return unsafe { std::mem::zeroed() };
        };
        if !inner.get_alive() {
            return f(inner.get_value());
        }
        let tracking_id: usize = CURRENT_TRACKING_DYNAMIC_ID.load(Ordering::Relaxed);
        if tracking_id != usize::MAX {
            self.add_dependent(tracking_id);
        }
        f(inner.get_value())
    }

    /// Subscribes a callback to be invoked when the signal changes.
    ///
    /// Returns the subscription id, which can later be passed to
    /// [`Signal::unsubscribe`] to detach exactly this listener. Framework
    /// DOM bindings use the id to tear down a binding when its element is
    /// removed; macro-generated `watch!` / `computed!` subscriptions keep
    /// the id unused because their lifetime is the enclosing hook context.
    ///
    /// # Arguments
    ///
    /// - `FnMut() + 'static` - The callback to invoke when the signal changes.
    ///
    /// # Returns
    ///
    /// - `u64` - The subscription id. `u64::MAX` when the handle is stale
    ///   (out-of-bounds slot); such an id is a safe no-op for `unsubscribe`.
    pub fn subscribe<F>(&self, callback: F) -> u64
    where
        F: FnMut() + 'static,
    {
        let Some(inner) = Self::slab_mut().get_mut::<T>(self.get_inner()) else {
            // Stale handle: no slot to register against — the subscription
            // is silently dropped, matching the previous no-op semantics.
            return u64::MAX;
        };
        let id: u64 = inner.get_next_listener_id();
        inner.set_next_listener_id(id.wrapping_add(1));
        inner.get_mut_listeners().push((id, Box::new(callback)));
        id
    }

    /// Detaches a single listener previously registered by [`Signal::subscribe`].
    ///
    /// When called while the signal is mid-notification (a listener callback
    /// triggered this call re-entrantly), the removal is deferred: the id is
    /// recorded and filtered out during `update`'s merge-back pass, so the
    /// detached listener cannot be resurrected into the live list.
    ///
    /// # Arguments
    ///
    /// - `u64` - The subscription id returned by `subscribe`.
    pub fn unsubscribe(&self, id: u64) {
        let Some(inner) = Self::slab_mut().get_mut::<T>(self.get_inner()) else {
            return;
        };
        if inner.get_notifying() {
            inner.get_mut_removed_listener_ids().push(id);
            return;
        }
        inner
            .get_mut_listeners()
            .retain(|(listener_id, _): &(u64, Box<dyn FnMut()>)| *listener_id != id);
    }

    /// Detaches this signal from the reactive system without freeing memory.
    ///
    /// Marks the signal inactive and clears its listeners and dependents, but
    /// intentionally keeps the slab slot alive.
    ///
    /// This is the only supported teardown path for a signal, and is used by
    /// the `use_signal` hook cleanup (when a component unmounts or a `match`
    /// arm switches). The slot is deliberately never freed or recycled because
    /// `Signal<T>` is `Copy` (just a `usize` slot index): async callbacks
    /// (`spawn_local` futures, `setTimeout` / `setInterval` closures, Promise
    /// continuations) may still hold copies of the signal, and recycling would
    /// turn their later `.get()` / `.set()` calls into reads of an unrelated
    /// signal. Deactivating instead makes those stale calls safe no-ops.
    pub(crate) fn deactivate(&self) {
        let idx: usize = self.get_inner();
        let Some(inner) = Self::slab_mut().get_mut::<T>(idx) else {
            // Out-of-bounds handle — treat as no-op. Mirrors the
            // "deactivate on already-deactivated signal is a safe no-op"
            // semantic.
            return;
        };
        inner.set_alive(false);
        inner.get_mut_listeners().clear();
        inner.get_mut_dependents().clear();
        inner.get_mut_removed_listener_ids().clear();
    }

    /// Core implementation of value update and listener notification.
    ///
    /// Returns `true` if the value was updated and listeners were notified.
    /// Returns `false` if the signal is inactive or the value is unchanged.
    ///
    /// Uses a swap-out pattern for listeners: moves all listeners into a local
    /// `Vec`, drops the mutable reference to inner state, then invokes each
    /// listener. After invocation, listeners are moved back. This prevents
    /// issues with re-entrant access during listener callbacks. Listeners
    /// detached via `unsubscribe` mid-notification are filtered out during
    /// the merge-back pass via `removed_listener_ids`.
    ///
    /// # Arguments
    ///
    /// - `T: Clone + PartialEq + 'static` - A generic type parameter.
    ///
    /// # Returns
    ///
    /// - `bool` - A boolean.
    fn update(&self, value: T) -> bool {
        let idx: usize = self.get_inner();
        let Some(inner) = Self::slab_mut().get_mut::<T>(idx) else {
            // Stale handle — treat as no-op.
            return false;
        };
        if !inner.get_alive() {
            return false;
        }
        if *inner.get_value() == value {
            return false;
        }
        inner.set_value(value);
        inner.set_notifying(true);
        let mut listeners: Vec<(u64, Box<dyn FnMut()>)> = Vec::new();
        swap(inner.get_mut_listeners(), &mut listeners);
        for (_id, listener) in listeners.iter_mut() {
            listener();
        }
        if !Self::is_alive(self.get_inner()) {
            // The signal was deactivated by a listener mid-notification.
            // Nothing should be merged back into a dead slot; clear the
            // notification state so a later `unsubscribe` cannot pile up
            // deferred removals that will never be drained.
            if let Some(inner) = Self::slab_mut().get_mut::<T>(idx) {
                inner.set_notifying(false);
                inner.get_mut_removed_listener_ids().clear();
            }
            return true;
        }
        if let Some(inner) = Self::slab_mut().get_mut::<T>(idx)
            && inner.get_alive()
        {
            let removed: Vec<u64> = take(inner.get_mut_removed_listener_ids());
            if !removed.is_empty() {
                listeners.retain(|(listener_id, _): &(u64, Box<dyn FnMut()>)| {
                    !removed.contains(listener_id)
                });
            }
            let new_listeners: &mut Vec<(u64, Box<dyn FnMut()>)> = inner.get_mut_listeners();
            if new_listeners.is_empty() {
                swap(new_listeners, &mut listeners);
            } else {
                listeners.append(new_listeners);
                swap(new_listeners, &mut listeners);
            }
            inner.set_notifying(false);
        }
        true
    }

    /// Registers a dynamic node ID as a dependent of this signal.
    ///
    /// When this signal changes, only its registered dependents will be
    /// marked dirty for re-rendering, enabling precise updates instead
    /// of broadcasting to all dynamic nodes.
    ///
    /// # Arguments
    ///
    /// - `usize` - The dynamic node ID to register as a dependent.
    ///
    /// OPT 9: the common rendering case is "this dependent was just added
    /// (last element of the list)". A `deps.last() == Some(&dynamic_id)`
    /// check short-circuits the `Vec::contains` linear scan, turning the
    /// typical append-into-existing-list call from O(N) to O(1). Only the
    /// rare cases (first add, or `dynamic_id` re-added after a previous
    /// unsubscription) fall back to the full scan + push.
    pub(crate) fn add_dependent(&self, dynamic_id: usize) {
        let Some(inner) = Self::slab_mut().get_mut::<T>(self.get_inner()) else {
            return;
        };
        let deps: &mut Vec<usize> = inner.get_mut_dependents();
        if let Some(last) = deps.last() {
            if *last == dynamic_id {
                return;
            }
            if !deps.contains(&dynamic_id) {
                deps.push(dynamic_id);
            }
        } else {
            deps.push(dynamic_id);
        }
    }

    /// Returns the list of dependent dynamic node IDs for this signal.
    ///
    /// # Returns
    ///
    /// - `Vec<usize>` - Clone of the dependents list.
    pub(crate) fn get_dependents(&self) -> Vec<usize> {
        Self::slab_mut()
            .get_mut::<T>(self.get_inner())
            .map(|inner: &mut SignalInner<T>| inner.get_dependents().clone())
            .unwrap_or_default()
    }

    /// Sets the value of the signal and notifies listeners.
    ///
    /// Uses precise dirty marking: only dynamic nodes that depend on
    /// this signal are marked dirty, avoiding full broadcast.
    ///
    /// When called inside `batch`, the dispatch is
    /// deferred (dirty slots are still marked precisely), and the
    /// outermost `set()` call outside the suppressed scope will
    /// trigger the actual dispatch cycle.
    ///
    /// # Arguments
    ///
    /// - `T: Clone + PartialEq + 'static` - The new value to assign to the signal.
    pub fn set(&self, value: T) {
        if self.update(value) {
            let dependents: Vec<usize> = self.get_dependents();
            App::schedule_update(&dependents);
        }
    }

    /// Returns whether the signal slot at `idx` is still alive
    /// (i.e. has not been deactivated).
    ///
    /// # Arguments
    ///
    /// - `usize` - Slab index to test.
    ///
    /// # Returns
    ///
    /// - `bool` - `true` when the slot refers to a live signal.
    pub(crate) fn is_alive(idx: usize) -> bool {
        Self::slab().is_alive(idx)
    }
}

/// Provides a safe default for `Signal<T>` by creating a valid signal
/// initialized with `T::default()`.
///
/// This prevents the creation of invalid signals with `inner = 0` (null
/// pointer), which would cause a panic when `.get()` is called.
///
/// # Returns
///
/// - `Self` - A valid signal initialized with `T::default()`.
impl<T> Default for Signal<T>
where
    T: Clone + Default + PartialEq + 'static,
{
    /// Constructs a default [`Signal`] value.
    fn default() -> Self {
        Self::create(T::default())
    }
}

/// Clones the signal, sharing the same inner state.
///
/// Since `Signal` is `Copy`, this simply returns `*self`.
///
/// # Returns
///
/// - `Self` - A copy of the signal handle sharing the same inner state.
impl<T> Clone for Signal<T>
where
    T: Clone + PartialEq + 'static,
{
    /// Clones the [`Signal`] by reusing shared, cheap-to-clone state where possible.
    fn clone(&self) -> Self {
        *self
    }
}

/// Copies the signal, sharing the same inner state.
///
/// Safe because only the inner address (a `usize`) is copied;
/// the actual heap allocation is owned by the global signal registry.
impl<T> Copy for Signal<T> where T: Clone + PartialEq + 'static {}

/// Marks `SignalCell` as `Sync` for single-threaded WASM contexts.
///
/// SAFETY: `SignalCell` is only used in single-threaded WASM contexts.
/// Concurrent access from multiple threads would be undefined behavior.
unsafe impl<T> Sync for SignalCell<T> where T: Clone + PartialEq + 'static {}

/// Implementation of SignalCell construction and access.
impl<T> SignalCell<T>
where
    T: Clone + PartialEq + 'static,
{
    /// Creates a new `SignalCell` with no signal stored.
    ///
    /// # Returns
    ///
    /// - `Self` - An empty `SignalCell` with `None` stored in the inner `UnsafeCell`.
    pub const fn none() -> Self {
        Self {
            inner: UnsafeCell::new(None),
        }
    }

    /// Stores a signal into the cell.
    ///
    /// First write wins: if a signal has already been stored, the new
    /// signal is dropped and the existing one is kept.
    ///
    /// # Arguments
    ///
    /// - `Signal<T>` - The signal to store.
    pub fn set(&self, signal: Signal<T>) {
        unsafe {
            let ptr: &mut Option<Signal<T>> = &mut *self.get_inner().get();
            if ptr.is_none() {
                *ptr = Some(signal);
            }
        }
    }

    /// Returns the signal stored in the cell, if any.
    ///
    /// # Returns
    ///
    /// - `Option<Signal<T>>` - The stored signal, or `None` when no signal
    ///   has been stored via `set` yet.
    pub fn loaded(&self) -> Option<Signal<T>> {
        unsafe {
            let ptr: &Option<Signal<T>> = &*self.get_inner().get();
            *ptr
        }
    }
}

/// Provides a default empty `SignalCell`.
///
/// Creates a `SignalCell` with `None` stored in the inner `UnsafeCell`.
///
/// # Returns
///
/// - `Self` - An empty `SignalCell` with no signal stored.
impl<T> Default for SignalCell<T>
where
    T: Clone + PartialEq + 'static,
{
    /// Constructs a default [`SignalCell`] value.
    fn default() -> Self {
        Self::new(UnsafeCell::new(None))
    }
}

/// Implementation of `FireHandle` construction, invocation, and conversions.
impl FireHandle {
    /// Leaks the given closure and returns a handle pointing to its heap address.
    ///
    /// The closure is double-boxed (`Box<Box<dyn FnMut()>>`) and leaked so the
    /// inner box's address remains stable for the lifetime of the program.
    /// The address is captured as a `usize` and wrapped in a `FireHandle`.
    ///
    /// # Arguments
    ///
    /// - `F: FnMut() + 'static` - The fire closure to leak.
    ///
    /// # Returns
    ///
    /// - `Self` - A handle holding the leaked closure's address.
    pub fn new<F>(fire: F) -> Self
    where
        F: FnMut() + 'static,
    {
        let leaked: &'static mut Box<dyn FnMut()> =
            Box::leak(Box::new(Box::new(fire) as Box<dyn FnMut()>));
        let addr: usize = leaked as *mut Box<dyn FnMut()> as usize;
        let mut handle: Self = Self { inner: 0 };
        handle.set_inner(addr);
        handle
    }

    /// Invokes the closure pointed to by this handle.
    ///
    /// Takes `self` by value because `FireHandle: Copy` — repeated invocations
    /// on a single captured handle each copy the address and operate on the
    /// same underlying closure.
    ///
    /// # Safety
    ///
    /// The handle must come from `FireHandle::new` (or `From`) and the
    /// underlying boxed closure must still be live.
    pub unsafe fn fire(self) {
        unsafe { Self::fire_at(self.get_inner()) };
    }

    /// Invokes the closure stored at the given address.
    ///
    /// This is the static counterpart of `fire` for call sites that have
    /// only the raw `usize` address (e.g., macro-generated code that
    /// captures the address by `move` into a subscribe closure).
    ///
    /// # Arguments
    ///
    /// - `usize` - The address of a leaked `Box<dyn FnMut()>`.
    ///
    /// # Safety
    ///
    /// `addr` must come from a valid `FireHandle` produced by `new` (or
    /// `From`) and the underlying boxed closure must still be live.
    pub unsafe fn fire_at(addr: usize) {
        let ptr: *mut Box<dyn FnMut()> = addr as *mut Box<dyn FnMut()>;
        unsafe { (&mut *ptr)() };
    }
}

/// Leaks a fire closure into a `FireHandle`.
///
/// This is the canonical `Into` path used by `watch!`/`computed!` macros
/// and the virtual list component to obtain a `FireHandle` from a closure.
impl<F> From<F> for FireHandle
where
    F: FnMut() + 'static,
{
    /// Leaks this closure and stores its address in the returned handle.
    ///
    /// # Returns
    ///
    /// - `FireHandle` - A handle holding the leaked closure's address.
    ///
    /// # Arguments
    ///
    /// - `F` - Input value to convert from.
    fn from(fire: F) -> Self {
        Self::new(fire)
    }
}

/// Extracts the raw address from a `FireHandle`.
///
/// This is used by macro-generated code that needs to capture the address
/// (a `Copy` type) into `FnMut() + 'static` subscribe closures.
impl From<FireHandle> for usize {
    /// Returns the leaked closure's heap address.
    ///
    /// # Returns
    ///
    /// - `usize` - The address held by this handle.
    ///
    /// # Arguments
    ///
    /// - `FireHandle` - Input value to convert from.
    fn from(handle: FireHandle) -> Self {
        handle.get_inner()
    }
}

/// Implementation of the typed signal slab allocator.
impl SignalSlab {
    /// Creates an empty slab.
    pub(crate) fn new() -> Self {
        Self {
            entries: Vec::new(),
        }
    }

    /// Inserts a new typed `SignalInner<T>` and returns its slot index.
    ///
    /// Append-only: the slot index issued here is never reused for another
    /// signal, which is what makes stale-handle reads sound (they always
    /// resolve to this slot's original, possibly deactivated, inner state).
    pub(crate) fn insert<T>(&mut self, inner: SignalInner<T>) -> usize
    where
        T: Clone + PartialEq + 'static,
    {
        let boxed: Box<dyn AnySignalInner> = Box::new(inner);
        let idx: usize = self.entries.len();
        self.entries.push(boxed);
        idx
    }

    /// Returns a typed `&mut SignalInner<T>` view of the slot at `idx`.
    ///
    /// Returns `None` when the index is out of bounds or was issued for a
    /// different concrete `T` (defensive TypeId check). Slots are never
    /// freed, so `None` means the caller is holding a corrupted handle —
    /// surfaced as `None` rather than panicking so that stale handles
    /// degrade into safe no-ops (matching the `alive == false` semantics).
    pub(crate) fn get_mut<T>(&mut self, idx: usize) -> Option<&mut SignalInner<T>>
    where
        T: Clone + PartialEq + 'static,
    {
        self.entries
            .get_mut(idx)?
            .as_any_mut()
            .downcast_mut::<SignalInner<T>>()
    }

    /// Returns `true` when the slot at `idx` exists AND its inner signal is
    /// still marked `alive`. Used by `Signal::is_alive`.
    pub(crate) fn is_alive(&self, idx: usize) -> bool {
        match self.entries.get(idx) {
            Some(inner) => inner.alive(),
            None => false,
        }
    }
}
