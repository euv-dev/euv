use super::*;

/// Inherent implementation of [`Previous`].
impl<T: Clone + PartialEq + 'static> Previous<T> {
    /// Creates a new `Previous` with no recorded value.
    /// The `previous` signal starts at `None`.
    pub fn new() -> Self {
        Self {
            previous: Signal::create(None),
        }
    }

    /// Records `current` as the new previous value. The
    /// next call to `get_previous_snapshot()` will return
    /// `Some(current)`.
    ///
    /// This is typically called at the top of a render
    /// closure so the signal stores the value just seen.
    ///
    /// # Arguments
    ///
    /// - `T: Clone + PartialEq + 'static` - A generic type parameter.
    pub fn record(&self, current: T) {
        self.get_previous().set(Some(current));
    }

    /// Returns a snapshot of the previously recorded
    /// value, or `None` if no value has been recorded yet.
    ///
    /// # Returns
    ///
    /// - `Option<T>` - The previous captured value, or `None`.
    pub fn get_previous_snapshot(&self) -> Option<T> {
        self.get_previous().get()
    }

    /// Clears the recorded previous value, returning the
    /// tracker to the `None` state.
    pub fn clear(&self) {
        self.get_previous().set(None);
    }
}

/// Debug formatting for [`Previous`].
impl<T: Clone + PartialEq + Debug + 'static> Display for Previous<T> {
    /// Formats the [`Previous`] via the supplied formatter.
    ///
    /// # Arguments
    ///
    /// - `&mut Formatter<'_>` - The formatter receiving the formatted output.
    ///
    /// # Returns
    ///
    /// - `fmt::Result` - Result of the formatting operation.
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        match self.get_previous().get() {
            Some(value) => write!(formatter, "Previous(Some({value:?}))"),
            None => write!(formatter, "Previous(None)"),
        }
    }
}

/// Default-construction for [`Previous`].
impl<T: Clone + PartialEq + 'static> Default for Previous<T> {
    /// Constructs a default [`Previous`] value.
    fn default() -> Self {
        Self::new()
    }
}
