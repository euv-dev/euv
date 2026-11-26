use super::*;

/// Copy handle semantics for `LocalState<T>`.
impl<T> Copy for LocalState<T> where T: 'static {}

/// Clone handle semantics for `LocalState<T>`.
impl<T> Clone for LocalState<T>
where
    T: 'static,
{
    /// Copies the lightweight `LocalState<T>` handle.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Source handle.
    ///
    /// # Returns
    ///
    /// - `Self` - Copied handle pointing to the same local state slot.
    fn clone(&self) -> Self {
        *self
    }
}

/// Handle equality comparison for `LocalState<T>`.
impl<T> PartialEq for LocalState<T>
where
    T: 'static,
{
    /// Compares two `LocalState<T>` handles by underlying signal identity.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Left handle.
    /// - `&Self` - Right handle.
    ///
    /// # Returns
    ///
    /// - `bool` - `true` when both handles reference the same state slot.
    fn eq(&self, other: &Self) -> bool {
        let Self { signal: lhs } = *self;
        let Self { signal: rhs } = *other;
        lhs == rhs
    }
}

/// Full equality marker for `LocalState<T>`.
impl<T> Eq for LocalState<T> where T: 'static {}

/// Scoped local state creation, reading, writing, and change observation for `LocalState<T>`.
impl<T> LocalState<T>
where
    T: 'static,
{
    /// Creates a new reactive `LocalState<T>` owned by the active component or page `Scope`.
    ///
    /// # Arguments
    ///
    /// - `T` - Initial local state value.
    ///
    /// # Returns
    ///
    /// - `Self` - Lightweight handle to the local reactive state.
    #[must_use]
    pub fn new(initial: T) -> Self {
        Self {
            signal: Signal::new(initial),
        }
    }

    /// Returns the underlying `Signal<T>` handle for interoperability with `view!` and widgets.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Local state handle.
    ///
    /// # Returns
    ///
    /// - `Signal<T>` - Underlying fine-grained reactive signal.
    #[must_use]
    pub fn signal(&self) -> Signal<T> {
        let Self { signal } = *self;
        signal
    }

    /// Reads the current value reactively and registers dependency tracking on the active observer.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Local state handle.
    ///
    /// # Returns
    ///
    /// - `T` - Current cloned state value.
    #[must_use]
    pub fn get(&self) -> T
    where
        T: Clone + Default,
    {
        self.signal().get()
    }

    /// Reads the current value without tracking dependencies.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Local state handle.
    ///
    /// # Returns
    ///
    /// - `T` - Current cloned state value.
    #[must_use]
    pub fn get_untracked(&self) -> T
    where
        T: Clone + Default,
    {
        self.signal().get_untracked()
    }

    /// Replaces the stored local state value and notifies active subscribers.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Local state handle.
    /// - `T` - Replacement value.
    ///
    /// # Returns
    ///
    /// - `bool` - `true` when the state slot is still alive.
    pub fn set(&self, value: T) -> bool {
        self.signal().set(value)
    }

    /// Replaces the stored value only when `value` differs from the current state.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Local state handle.
    /// - `T` - Candidate replacement value.
    ///
    /// # Returns
    ///
    /// - `bool` - `true` when the value changed and subscribers were notified.
    pub fn set_if_changed(&self, value: T) -> bool
    where
        T: PartialEq,
    {
        self.signal().set_if_changed(value)
    }

    /// Mutates the stored local state in place via `updater`.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Local state handle.
    /// - `F` - In-place mutation closure.
    ///
    /// # Returns
    ///
    /// - `bool` - `true` when updated.
    pub fn update<F>(&self, updater: F) -> bool
    where
        T: Clone,
        F: FnOnce(&mut T),
    {
        self.signal().update(updater)
    }

    /// Subscribes a change watcher receiving `(&previous, &next)` whenever this local state mutates.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Local state handle.
    /// - `F` - Change observer invoked on subsequent mutations (skipped during `Scope::freeze()`, coalesced on `Scope::resume()`).
    ///
    /// # Returns
    ///
    /// - `Effect` - Scope-owned watcher effect handle.
    #[must_use]
    pub fn watch<F>(&self, mut handler: F) -> Effect
    where
        T: Clone,
        F: FnMut(&T, &T) + 'static,
    {
        let sig: Signal<T> = self.signal();
        let prev_cell: Rc<RefCell<Option<T>>> = Rc::new(RefCell::new(None));
        Effect::new(move || {
            if let Some(next) = sig.try_get() {
                let prev_opt: Option<T> = match prev_cell.try_borrow_mut() {
                    Ok(mut guard) => guard.replace(next.clone()),
                    Err(_) => None,
                };
                if let Some(prev) = prev_opt {
                    handler(&prev, &next);
                }
            }
        })
    }
}

/// Global singleton state registration, access, mutation, and scope-aware change watching for `GlobalState`.
impl GlobalState {
    /// Registers or replaces a type-keyed root-lifetime global state value of type `T`.
    ///
    /// # Arguments
    ///
    /// - `T` - Initial or replacement global state value.
    ///
    /// # Returns
    ///
    /// - `Signal<T>` - Root-lifetime reactive signal bound to global type `T`.
    pub fn provide<T>(initial: T) -> Signal<T>
    where
        T: 'static,
    {
        let (id, should_flush): (SignalId, bool) = with_runtime(
            |rt: &mut ReactiveRuntime| rt.provide_global_state(initial),
            (SignalId::default(), false),
        );
        if should_flush {
            flush_effects();
        }
        Signal::from_raw_id(id)
    }

    /// Looks up a previously registered global state signal of type `T`.
    ///
    /// # Returns
    ///
    /// - `Option<Signal<T>>` - Registered global signal if present.
    #[must_use]
    pub fn use_state<T>() -> Option<Signal<T>>
    where
        T: 'static,
    {
        with_runtime(
            |rt: &mut ReactiveRuntime| rt.lookup_global_state::<T>(),
            None,
        )
        .map(Signal::from_raw_id)
    }

    /// Returns the registered global state signal of type `T`, lazily initializing it with `init` when absent.
    ///
    /// # Arguments
    ///
    /// - `F` - Lazy initializer invoked only when global state `T` has not yet been registered.
    ///
    /// # Returns
    ///
    /// - `Signal<T>` - Root-lifetime reactive signal for global state `T`.
    #[must_use]
    pub fn use_or_init<T, F>(init: F) -> Signal<T>
    where
        T: 'static,
        F: FnOnce() -> T,
    {
        let id: SignalId = with_runtime(
            |rt: &mut ReactiveRuntime| rt.get_or_init_global_state(init),
            SignalId::default(),
        );
        Signal::from_raw_id(id)
    }

    /// Reactively reads the current value of global state `T`, tracking dependency on the active observer.
    ///
    /// # Returns
    ///
    /// - `Option<T>` - Current global state value if registered.
    #[must_use]
    pub fn get<T>() -> Option<T>
    where
        T: Clone + 'static,
    {
        Self::use_state::<T>().and_then(|sig: Signal<T>| sig.try_get())
    }

    /// Reads the current value of global state `T` without tracking reactive dependencies.
    ///
    /// # Returns
    ///
    /// - `Option<T>` - Current global state value if registered.
    #[must_use]
    pub fn get_untracked<T>() -> Option<T>
    where
        T: Clone + 'static,
    {
        Self::use_state::<T>().and_then(|sig: Signal<T>| sig.try_get_untracked())
    }

    /// Replaces the registered global state `T` and notifies all active observers.
    ///
    /// # Arguments
    ///
    /// - `T` - Replacement global state value.
    ///
    /// # Returns
    ///
    /// - `bool` - `true` when global state `T` existed and was updated.
    pub fn set<T>(value: T) -> bool
    where
        T: 'static,
    {
        match Self::use_state::<T>() {
            Some(sig) => sig.set(value),
            None => false,
        }
    }

    /// Replaces global state `T` only when `value` differs from the stored value.
    ///
    /// # Arguments
    ///
    /// - `T` - Candidate replacement value.
    ///
    /// # Returns
    ///
    /// - `bool` - `true` when the state changed.
    pub fn set_if_changed<T>(value: T) -> bool
    where
        T: PartialEq + 'static,
    {
        match Self::use_state::<T>() {
            Some(sig) => sig.set_if_changed(value),
            None => false,
        }
    }

    /// Mutates global state `T` in place via `updater`.
    ///
    /// # Arguments
    ///
    /// - `F` - Mutation closure.
    ///
    /// # Returns
    ///
    /// - `bool` - `true` when global state `T` existed and was mutated.
    pub fn update<T, F>(updater: F) -> bool
    where
        T: Clone + 'static,
        F: FnOnce(&mut T),
    {
        match Self::use_state::<T>() {
            Some(sig) => sig.update(updater),
            None => false,
        }
    }

    /// Subscribes a scope-owned change watcher receiving `(&previous, &next)` whenever global state `T` updates.
    ///
    /// # Arguments
    ///
    /// - `F` - Change callback (automatically suspended when owning page is frozen and coalesced on resume).
    ///
    /// # Returns
    ///
    /// - `Option<Effect>` - Watcher effect handle if global state `T` is registered.
    #[must_use]
    pub fn watch<T, F>(mut handler: F) -> Option<Effect>
    where
        T: Clone + 'static,
        F: FnMut(&T, &T) + 'static,
    {
        let sig: Signal<T> = Self::use_state::<T>()?;
        let prev_cell: Rc<RefCell<Option<T>>> = Rc::new(RefCell::new(None));
        Some(Effect::new(move || {
            if let Some(next) = sig.try_get() {
                let prev_opt: Option<T> = match prev_cell.try_borrow_mut() {
                    Ok(mut guard) => guard.replace(next.clone()),
                    Err(_) => None,
                };
                if let Some(prev) = prev_opt {
                    handler(&prev, &next);
                }
            }
        }))
    }

    /// Registers or replaces a string-keyed global state channel storing `T`.
    ///
    /// # Arguments
    ///
    /// - `&str` - Channel name.
    /// - `T` - Initial or replacement value.
    ///
    /// # Returns
    ///
    /// - `Signal<T>` - Root-lifetime signal handle.
    pub fn provide_named<T>(name: &str, initial: T) -> Signal<T>
    where
        T: 'static,
    {
        let (id, should_flush): (SignalId, bool) = with_runtime(
            |rt: &mut ReactiveRuntime| rt.provide_named_global_state(name, initial),
            (SignalId::default(), false),
        );
        if should_flush {
            flush_effects();
        }
        Signal::from_raw_id(id)
    }

    /// Looks up a string-keyed global state channel storing `T`.
    ///
    /// # Arguments
    ///
    /// - `&str` - Channel name.
    ///
    /// # Returns
    ///
    /// - `Option<Signal<T>>` - Registered signal handle if present.
    #[must_use]
    pub fn use_named<T>(name: &str) -> Option<Signal<T>>
    where
        T: 'static,
    {
        with_runtime(
            |rt: &mut ReactiveRuntime| rt.lookup_named_global_state(name),
            None,
        )
        .map(Signal::from_raw_id)
    }

    /// Subscribes a scope-owned watcher receiving `(&previous, &next)` for a string-keyed global state channel.
    ///
    /// # Arguments
    ///
    /// - `&str` - Channel name.
    /// - `F` - Change callback.
    ///
    /// # Returns
    ///
    /// - `Option<Effect>` - Watcher effect handle if the named channel exists.
    #[must_use]
    pub fn watch_named<T, F>(name: &str, mut handler: F) -> Option<Effect>
    where
        T: Clone + 'static,
        F: FnMut(&T, &T) + 'static,
    {
        let sig: Signal<T> = Self::use_named::<T>(name)?;
        let prev_cell: Rc<RefCell<Option<T>>> = Rc::new(RefCell::new(None));
        Some(Effect::new(move || {
            if let Some(next) = sig.try_get() {
                let prev_opt: Option<T> = match prev_cell.try_borrow_mut() {
                    Ok(mut guard) => guard.replace(next.clone()),
                    Err(_) => None,
                };
                if let Some(prev) = prev_opt {
                    handler(&prev, &next);
                }
            }
        }))
    }
}
