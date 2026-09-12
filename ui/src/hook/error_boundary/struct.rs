use super::*;

/// A handle that tracks whether a wrapped subtree has
/// thrown.
///
/// The boundary's phase is exposed via `phase()` as a
/// `Signal<ErrorBoundaryPhase>`. The parent component
/// reads that signal to decide whether to render the
/// child or a fallback.
///
/// `try_with(closure)` runs the closure and, if it
/// panics, transitions the boundary to `Caught`. The
/// closure's return value (if it didn't panic) is
/// returned to the caller; if it panicked, an `Err`
/// is returned containing the panic message.
#[derive(Clone, Data, Debug)]
pub struct ErrorBoundary {
    /// The phase signal.
    pub(crate) phase: Signal<ErrorBoundaryPhase>,
}
