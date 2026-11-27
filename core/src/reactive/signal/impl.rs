use super::*;

/// Manual `Clone` implementation so `Signal<T>` is always `Copy` regardless of `T`.
impl<T> Clone for Signal<T>
where
    T: 'static,
{
    /// Copies the lightweight 8-byte signal handle.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Signal handle to copy.
    ///
    /// # Returns
    ///
    /// - `Self` - Identical signal handle.
    fn clone(&self) -> Self {
        *self
    }
}

/// Marker `Copy` implementation for `Signal<T>`.
impl<T> Copy for Signal<T> where T: 'static {}

/// Zero-allocation dangling `Default` handle for `Signal<T>`, used when initializing component prop descriptors.
impl<T> Default for Signal<T>
where
    T: 'static,
{
    /// Constructs an unallocated placeholder `Signal<T>` handle with `SignalId::default()`.
    ///
    /// # Returns
    ///
    /// - `Self` - Zero-cost placeholder signal handle.
    fn default() -> Self {
        Self {
            id: SignalId::default(),
            marker: PhantomData,
        }
    }
}

/// Equality comparison by underlying `SignalId`.
impl<T> PartialEq for Signal<T>
where
    T: 'static,
{
    /// Compares two signal handles by arena identifier.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Left-hand signal handle.
    /// - `&Self` - Right-hand signal handle.
    ///
    /// # Returns
    ///
    /// - `bool` - True if both handles reference the same arena slot and generation.
    fn eq(&self, other: &Self) -> bool {
        self.id() == other.id()
    }
}

/// Reflexive equality for `Signal<T>`.
impl<T> Eq for Signal<T> where T: 'static {}

/// Hash implementation based on underlying `SignalId`.
impl<T> Hash for Signal<T>
where
    T: 'static,
{
    /// Hashes the underlying arena signal identifier.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Signal handle to hash.
    /// - `&mut H` - Hasher state.
    fn hash<H>(&self, state: &mut H)
    where
        H: Hasher,
    {
        self.id().hash(state);
    }
}

/// Core reactive operations for `Signal<T>`.
impl<T> Signal<T>
where
    T: 'static,
{
    /// Allocates a new reactive signal initialized with `initial_value`.
    ///
    /// # Arguments
    ///
    /// - `T` - Initial signal value.
    ///
    /// # Returns
    ///
    /// - `Self` - Copyable handle to the created signal.
    #[must_use]
    pub fn new(initial_value: T) -> Self {
        let id: SignalId = with_runtime(
            |rt: &mut ReactiveRuntime| rt.create_signal(initial_value),
            SignalId::default(),
        );
        Self {
            id,
            marker: PhantomData,
        }
    }

    /// Allocates a new reactive signal detached from any enclosing transient `Scope` so hub/store state survives UI rebuilds.
    ///
    /// # Arguments
    ///
    /// - `T` - Initial signal value.
    ///
    /// # Returns
    ///
    /// - `Self` - Copyable handle to the created detached signal.
    #[must_use]
    pub fn new_detached(initial_value: T) -> Self {
        let id: SignalId = with_runtime(
            |rt: &mut ReactiveRuntime| rt.create_detached_signal(initial_value),
            SignalId::default(),
        );
        Self {
            id,
            marker: PhantomData,
        }
    }

    /// Reconstructs a typed `Signal<T>` handle from an arena `SignalId`.
    ///
    /// # Arguments
    ///
    /// - `SignalId` - Arena signal identifier.
    ///
    /// # Returns
    ///
    /// - `Self` - Lightweight copyable signal handle.
    #[must_use]
    pub(crate) fn from_raw_id(id: SignalId) -> Self {
        Self {
            id,
            marker: PhantomData,
        }
    }

    /// Returns the underlying `SignalId`.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Signal handle.
    ///
    /// # Returns
    ///
    /// - `SignalId` - Underlying arena identifier.
    #[must_use]
    pub fn id(&self) -> SignalId {
        let Self { id, .. } = *self;
        id
    }

    /// Returns whether this signal currently has at least one active downstream subscriber.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Signal handle to inspect.
    ///
    /// # Returns
    ///
    /// - `bool` - True when the signal is alive and observed by at least one memo or effect.
    #[must_use]
    pub fn has_subscribers(&self) -> bool {
        let id: SignalId = self.id();
        with_runtime(
            |rt: &mut ReactiveRuntime| rt.signal_has_subscribers(id),
            false,
        )
    }

    /// Records a dependency on this signal in the active observer without cloning its value.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Signal handle to track.
    pub fn track(&self) {
        let id: SignalId = self.id();
        with_runtime(
            |rt: &mut ReactiveRuntime| {
                rt.track_dependency(DependencyId::Signal(id));
            },
            (),
        );
    }

    /// Reads the current value by cloning and tracks the active observer dependency.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Signal handle to read.
    ///
    /// # Returns
    ///
    /// - `Option<T>` - Current value if the signal is still alive.
    #[must_use]
    pub fn try_get(&self) -> Option<T>
    where
        T: Clone,
    {
        let id: SignalId = self.id();
        with_runtime(
            |rt: &mut ReactiveRuntime| rt.read_signal_cloned::<T>(id),
            None,
        )
    }

    /// Reads the current value, returning `T::default()` if the signal was disposed.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Signal handle to read.
    ///
    /// # Returns
    ///
    /// - `T` - Current value or default fallback.
    #[must_use]
    pub fn get(&self) -> T
    where
        T: Clone + Default,
    {
        self.try_get().unwrap_or_default()
    }

    /// Reads the current value without tracking any reactive dependency.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Signal handle to inspect.
    ///
    /// # Returns
    ///
    /// - `Option<T>` - Current value if alive.
    #[must_use]
    pub fn try_get_untracked(&self) -> Option<T>
    where
        T: Clone,
    {
        let id: SignalId = self.id();
        with_runtime(
            |rt: &mut ReactiveRuntime| rt.read_signal_untracked::<T>(id),
            None,
        )
    }

    /// Reads the current value untracked, returning `T::default()` if disposed.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Signal handle to inspect.
    ///
    /// # Returns
    ///
    /// - `T` - Current value or default fallback.
    #[must_use]
    pub fn get_untracked(&self) -> T
    where
        T: Clone + Default,
    {
        self.try_get_untracked().unwrap_or_default()
    }

    /// Applies `reader` to a cloned snapshot outside any `RefCell` borrow so nested reads remain safe.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Signal handle to read.
    /// - `F` - Reader closure receiving `&T`.
    ///
    /// # Returns
    ///
    /// - `Option<R>` - Closure return value if the signal is alive.
    pub fn with<F, R>(&self, reader: F) -> Option<R>
    where
        T: Clone,
        F: FnOnce(&T) -> R,
    {
        let snapshot: T = self.try_get()?;
        Some(reader(&snapshot))
    }

    /// Replaces the stored value and flushes downstream reactive effects.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Signal handle to write.
    /// - `T` - Replacement value.
    ///
    /// # Returns
    ///
    /// - `bool` - True when the signal was alive and updated.
    pub fn set(&self, value: T) -> bool {
        let id: SignalId = self.id();
        let (updated, should_flush): (bool, bool) = with_runtime(
            |rt: &mut ReactiveRuntime| rt.write_signal(id, value),
            (false, false),
        );
        if should_flush {
            flush_effects();
        }
        updated
    }

    /// Replaces the stored value only when `value` differs from the current stored value.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Signal handle to write.
    /// - `T` - Candidate value.
    ///
    /// # Returns
    ///
    /// - `bool` - True when the value changed and downstream effects were scheduled.
    pub fn set_if_changed(&self, value: T) -> bool
    where
        T: PartialEq,
    {
        let id: SignalId = self.id();
        let (updated, should_flush): (bool, bool) = with_runtime(
            |rt: &mut ReactiveRuntime| rt.write_signal_if_changed(id, value),
            (false, false),
        );
        if should_flush {
            flush_effects();
        }
        updated
    }

    /// Mutates the signal value using `updater` outside the runtime borrow and flushes effects.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Signal handle to update.
    /// - `F` - Mutation closure receiving `&mut T`.
    ///
    /// # Returns
    ///
    /// - `bool` - True when the signal was alive and updated.
    pub fn update<F>(&self, updater: F) -> bool
    where
        T: Clone,
        F: FnOnce(&mut T),
    {
        let Some(mut current) = self.try_get_untracked() else {
            return false;
        };
        updater(&mut current);
        self.set(current)
    }

    /// Disposes this signal and reclaims its arena slot.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Signal handle to dispose.
    pub fn dispose(&self) {
        let id: SignalId = self.id();
        with_runtime(
            |rt: &mut ReactiveRuntime| {
                rt.dispose_signal(id);
            },
            (),
        );
    }
}
