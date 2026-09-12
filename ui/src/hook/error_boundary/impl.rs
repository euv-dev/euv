use super::*;

/// Inherent implementation of [`ErrorBoundary`].
impl ErrorBoundary {
    /// Creates a new `ErrorBoundary` in the `Healthy`
    /// phase.
    pub fn new() -> Self {
        Self {
            phase: Signal::create(ErrorBoundaryPhase::Healthy),
        }
    }

    /// Runs a closure and, if it panics, transitions
    /// the boundary to `Caught` and returns `Err`.
    ///
    /// On success, the closure's return value is
    /// returned wrapped in `Ok`. The closure is
    /// wrapped in `AssertUnwindSafe` so it does not
    /// have to satisfy `UnwindSafe`.
    ///
    /// # Arguments
    ///
    /// - `F: FnOnce() -> R + UnwindSafe` - A generic type parameter.
    ///
    /// # Returns
    ///
    /// - `Result<R, String>` - Result of the operation; an `Err` variant on failure.
    pub fn try_with<F, R>(&self, closure: F) -> Result<R, String>
    where
        F: FnOnce() -> R + UnwindSafe,
    {
        match catch_unwind(closure) {
            Ok(value) => Ok(value),
            Err(payload) => {
                let message: String = extract_message(&payload);
                let _ = catch_unwind(AssertUnwindSafe(|| {
                    self.get_phase()
                        .set(ErrorBoundaryPhase::Caught(message.clone()));
                }));
                Err(message)
            }
        }
    }

    /// Feeds an `Err` straight into the boundary
    /// without forcing the caller to `panic!`.
    ///
    /// `try_with` requires a real panic to transition
    /// the phase to `Caught`, which makes demonstrating
    /// the hook from outside `tests/` awkward. This
    /// helper lets demo / driver code report a failure
    /// message via the regular `Result` channel and
    /// still flip the boundary into `Caught`.
    ///
    /// # Arguments
    ///
    /// - `&str` - The error message to surface.
    ///
    /// # Returns
    ///
    /// - `String` - The same message that was passed in.
    pub fn report_error(&self, message: &str) -> String {
        let owned: String = String::from(message);
        let _ = catch_unwind(AssertUnwindSafe(|| {
            self.get_phase()
                .set(ErrorBoundaryPhase::Caught(owned.clone()));
        }));
        owned
    }

    /// Transitions the boundary back to `Healthy`.
    /// Useful when invalidating the cache (e.g.,
    /// after a retry).
    pub fn reset(&self) {
        let _ = catch_unwind(AssertUnwindSafe(|| {
            self.get_phase().set(ErrorBoundaryPhase::Healthy);
        }));
    }
}

/// Default-construction for [`ErrorBoundary`].
impl Default for ErrorBoundary {
    /// Constructs a default [`ErrorBoundary`] value.
    fn default() -> Self {
        Self::new()
    }
}

/// Formatting / debug-printing for [`ErrorBoundary`].
impl Display for ErrorBoundary {
    /// Formats the [`ErrorBoundary`] via the supplied formatter.
    ///
    /// # Arguments
    ///
    /// - `&mut Formatter<'_>` - The formatter receiving the formatted output.
    ///
    /// # Returns
    ///
    /// - `FmtResult` - Result of the formatting operation.
    fn fmt(&self, formatter: &mut Formatter<'_>) -> FmtResult {
        write!(formatter, "ErrorBoundary({:?})", self.get_phase().get())
    }
}

/// Equality comparison for [`ErrorBoundaryPhase`].
impl PartialEq for ErrorBoundaryPhase {
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
            (ErrorBoundaryPhase::Healthy, ErrorBoundaryPhase::Healthy) => true,
            (ErrorBoundaryPhase::Caught(a), ErrorBoundaryPhase::Caught(b)) => a == b,
            _ => false,
        }
    }
}
/// `ErrorBoundary` is `Copy` because `Signal<ErrorBoundaryPhase>`
/// is itself `Copy` — the signal registry hands out cheap
/// `usize` addresses.
impl Copy for ErrorBoundary {}
