use super::*;

/// Obtains a `SuspenseHandle` registered against the current hook context slot.
///
/// Behaves like `HookContext::use_hook` - the same `SuspenseHandle` is
/// returned on every render at the same hook index, preserving the
/// `Pending` / `Resolved(value)` / `Failed(message)` phase across renders.
///
/// Pair the returned handle with [`SuspenseHandle::resolve_sync`] /
/// [`SuspenseHandle::fail`] to transition the phase; the parent
/// component reads [`SuspenseHandle::state`] (or the underlying
/// `phase` signal) to decide whether to render the children or a
/// fallback.
///
/// # Returns
///
/// - `SuspenseHandle<T>` - The suspense handle.
///   Returns the factory result directly when no hook context is
///   active (e.g. when called outside a render cycle).
pub fn use_suspense<T>() -> SuspenseHandle<T>
where
    T: Clone + PartialEq + Debug + 'static,
{
    HookContext::use_hook(SuspenseHandle::<T>::new)
}
