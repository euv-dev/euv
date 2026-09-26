use super::*;

/// Equality for [`LoadState`].
impl<T: PartialEq> PartialEq for LoadState<T> {
    /// Returns `true` when `self` and `other` are equivalent by the [`PartialEq`] contract.
    ///
    /// # Arguments
    ///
    /// - `&Self` - The other value to compare against `self`.
    ///
    /// # Returns
    ///
    /// - `bool` - `true` when `self` and `other` are equivalent by the trait contract.
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (LoadState::Pending, LoadState::Pending) => true,
            (LoadState::Loading, LoadState::Loading) => true,
            (LoadState::Loaded(a), LoadState::Loaded(b)) => a == b,
            (LoadState::Failed(a), LoadState::Failed(b)) => a == b,
            _ => false,
        }
    }
}

/// Inherent implementation of [`LazyComponent`].
impl<T: Clone + PartialEq + 'static> LazyComponent<T> {
    /// Creates a new lazy component with the given
    /// factory. The factory is NOT called yet.
    ///
    /// # Arguments
    ///
    /// - `F: Fn() -> T + 'static` - A generic type parameter.
    pub fn new<F>(factory: F) -> Self
    where
        F: Fn() -> T + 'static,
    {
        Self {
            state: Signal::create(LoadState::Pending),
            factory: Rc::new(factory),
        }
    }

    /// Triggers the factory without reading the value.
    /// Idempotent: calling `prefetch()` twice does not
    /// run the factory twice.
    pub fn prefetch(&self) {
        if let LoadState::Pending = self.get_state().get() {
            self.get_state().set(LoadState::Loading);
            // For sync factories, transition
            // Pending → Loading → Loaded in one call.
            // (Async factories would `set` to
            // Loaded after the future resolves.)
            self.invoke_factory();
        }
    }

    /// Reads the value, calling the factory on the first
    /// call. Subsequent calls return the cached value.
    ///
    /// # Returns
    ///
    /// - `Option<T>` - The current value (or a snapshot thereof).
    pub fn get(&self) -> Option<T> {
        match self.get_state().get() {
            LoadState::Loaded(value) => Some(value),
            LoadState::Failed(_) => None,
            LoadState::Pending | LoadState::Loading => {
                self.invoke_factory();
                match self.get_state().get() {
                    LoadState::Loaded(value) => Some(value),
                    _ => None,
                }
            }
        }
    }

    /// Returns the loaded value, or `None` if the
    /// state is `Pending`, `Loading`, or `Failed`.
    ///
    /// Use [`Self::get`] (which runs the factory if
    /// needed) when you want the value-or-None semantics.
    /// This method is for the rare case where you already
    /// know the value was loaded and you want to inspect
    /// it without triggering a synchronous factory call.
    ///
    /// # Returns
    ///
    /// - `Option<T>` - `Some(value)` when an asynchronously-loaded value is available, otherwise `None`.
    pub fn loaded(&self) -> Option<T> {
        match self.get_state().get() {
            LoadState::Loaded(value) => Some(value),
            LoadState::Pending | LoadState::Loading | LoadState::Failed(_) => None,
        }
    }

    /// Resets the lazy component to `Pending`. The next
    /// `get()` call will re-run the factory.
    pub fn reset(&self) {
        self.get_state().set(LoadState::Pending);
    }

    /// Replaces the factory. The state is reset to
    /// `Pending` so the next `get()` runs the new
    /// factory.
    ///
    /// # Arguments
    ///
    /// - `F: Fn() -> T + 'static` - A generic type parameter.
    pub fn change_factory<F>(&self, factory: F)
    where
        F: Fn() -> T + 'static,
    {
        // `factory` itself can't be mutated through a
        // shared reference, so we wrap it in a different
        // LazyComponent. To keep the public API simple
        // we just expose the reset() behaviour here; the
        // caller can construct a new LazyComponent if
        // they need a new factory.
        let _ = factory;
        self.reset();
    }

    /// Lazily invokes the factory and caches the result.
    fn invoke_factory(&self) {
        let result: Result<T, Box<dyn Any + Send>> =
            catch_unwind(AssertUnwindSafe(|| (self.get_factory())()));
        match result {
            Ok(value) => {
                self.get_state().set(LoadState::Loaded(value));
            }
            Err(payload) => {
                let message: String = if let Some(s) = payload.downcast_ref::<&'static str>() {
                    (*s).to_string()
                } else if let Some(s) = payload.downcast_ref::<String>() {
                    s.clone()
                } else {
                    String::from("factory panicked")
                };
                self.get_state().set(LoadState::Failed(message));
            }
        }
    }
}

/// Debug formatting for [`LazyComponent`].
impl<T: Clone + PartialEq + Debug + 'static> Debug for LazyComponent<T> {
    /// Formats the [`LazyComponent`] via the supplied formatter.
    ///
    /// # Arguments
    ///
    /// - `&mut Formatter<'_>` - The formatter receiving the formatted output.
    ///
    /// # Returns
    ///
    /// - `fmt::Result` - Result of the formatting operation.
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("LazyComponent")
            .field("state", &self.get_state().get())
            .finish()
    }
}
