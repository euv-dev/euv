use super::*;

/// Obtains the previous-value tracker registered against the current
/// hook context slot.
///
/// Behaves like `HookContext::use_hook` - the same `Previous` is
/// returned on every render at the same hook index, so the captured
/// `previous` signal survives across renders without losing state.
///
/// # Returns
///
/// - `Previous<T>` - The previous-value tracker handle.
///   Returns the factory result directly when no hook context is
///   active (e.g. when called outside a render cycle).
pub fn use_previous<T>() -> Previous<T>
where
    T: Clone + PartialEq + Debug + 'static,
{
    HookContext::use_hook(Previous::<T>::new)
}

/// Records `current` against the supplied tracker and returns the
/// snapshot of what was previously recorded.
///
/// Convenience wrapper used by component-level consumers that want
/// the "compute previous" + "record new current" steps glued together.
/// Returns `None` on the first call (no prior value exists yet).
///
/// # Arguments
///
/// - `Previous<T>` - The tracker obtained from `use_previous()`.
/// - `T` - The current value to record.
///
/// # Returns
///
/// - `Option<T>` - The value that was recorded on the previous call,
///   or `None` if no prior value exists.
pub fn previous_step<T>(previous: Previous<T>, current: T) -> Option<T>
where
    T: Clone + PartialEq + Debug + 'static,
{
    let snapshot: Option<T> = previous.get_previous_snapshot();
    previous.record(current);
    snapshot
}
