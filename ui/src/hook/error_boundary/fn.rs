use super::*;

/// Extracts the panic message from a `catch_unwind` payload.
///
/// `catch_unwind` returns a `Box<dyn Any + Send>`. The boxed type
/// is whatever the panic site threw — most commonly a `String` /
/// `&str`, but Rust also supports throwing `&'static str` from
/// `std::panic!`. This helper tries each, in order, and falls
/// back to `"<unknown panic payload>"` so the boundary always
/// has a useful message to display.
///
/// # Arguments
///
/// - `&Box<dyn Any + Send>` - The boxed payload from
///   [`std::panic::catch_unwind`].
///
/// # Returns
///
/// - `String` - The recovered panic message.
pub(crate) fn extract_message(payload: &Box<dyn Any + Send>) -> String {
    if let Some(s) = payload.downcast_ref::<String>() {
        return s.clone();
    }
    if let Some(s) = payload.downcast_ref::<&'static str>() {
        return (*s).to_string();
    }
    String::from("<unknown panic payload>")
}

/// Obtains an `ErrorBoundary` registered against the current hook context slot.
///
/// Behaves like `HookContext::use_hook` - the same `ErrorBoundary` is
/// returned on every render at the same hook index, preserving the
/// `Idle` / `Caught(message)` phase across renders.
///
/// Use [`ErrorBoundary::try_with`] to run a closure under the
/// boundary; panics inside the closure are caught and the phase
/// transitions to `Caught`. The parent's render code reads
/// [`ErrorBoundary::phase`] (a `Signal<ErrorBoundaryPhase>`) to decide
/// whether to render the children or a fallback.
///
/// # Returns
///
/// - `ErrorBoundary` - The error boundary handle.
///   Returns the factory result directly when no hook context is
///   active (e.g. when called outside a render cycle).
pub fn use_error_boundary() -> ErrorBoundary {
    HookContext::use_hook(ErrorBoundary::default)
}
