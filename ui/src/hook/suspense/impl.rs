use super::*;

/// Inherent implementation of [`SuspenseHandle`].
impl<T: Clone + PartialEq + 'static> SuspenseHandle<T> {
    /// Creates a new `SuspenseHandle` in the `Pending`
    /// phase.
    pub fn new() -> Self {
        Self {
            phase: Signal::create(SuspensePhase::Pending),
        }
    }

    /// Transitions the phase to `Resolved(value)`. Works
    /// on every target.
    ///
    /// # Arguments
    ///
    /// - `T: Clone + PartialEq + 'static` - A generic type parameter.
    pub fn resolve_sync(&self, value: T) {
        self.get_phase().set(SuspensePhase::Resolved(value));
    }

    /// Transitions the phase to `Failed(message)`. Works
    /// on every target.
    ///
    /// # Arguments
    ///
    /// - `String` - A `String` parameter.
    pub fn fail(&self, message: String) {
        self.get_phase().set(SuspensePhase::Failed(message));
    }

    /// Transitions the phase back to `Pending`. Useful
    /// when invalidating the cache (e.g., after a
    /// mutation that requires refetching).
    pub fn reset(&self) {
        self.get_phase().set(SuspensePhase::Pending);
    }
}

/// Default-construction for [`SuspenseHandle`].
impl<T: Clone + PartialEq + 'static> Default for SuspenseHandle<T> {
    /// Constructs a default [`SuspenseHandle`] value.
    fn default() -> Self {
        Self::new()
    }
}

/// Debug formatting for [`SuspenseHandle`].
impl<T: Clone + PartialEq + Debug + 'static> Display for SuspenseHandle<T> {
    /// Formats the [`SuspenseHandle`] via the supplied formatter.
    ///
    /// # Arguments
    ///
    /// - `&mut Formatter<'_>` - The formatter receiving the formatted output.
    ///
    /// # Returns
    ///
    /// - `fmt::Result` - Result of the formatting operation.
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        write!(formatter, "SuspenseHandle({:?})", self.get_phase().get())
    }
}

/// Equality for [`SuspensePhase`].
impl<T: PartialEq> PartialEq for SuspensePhase<T> {
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
            (SuspensePhase::Pending, SuspensePhase::Pending) => true,
            (SuspensePhase::Resolved(a), SuspensePhase::Resolved(b)) => a == b,
            (SuspensePhase::Failed(a), SuspensePhase::Failed(b)) => a == b,
            _ => false,
        }
    }
}
