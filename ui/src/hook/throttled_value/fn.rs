use super::*;

/// Obtains the throttled value registered against the current hook context slot.
///
/// Behaves like `HookContext::use_hook` - the same `ThrottledValue` is
/// returned on every render at the same hook index, preserving the
/// emitted value, the pending slot, and the cooldown state across
/// renders.
///
/// `interval_ms = 0` collapses to "every `set` is immediately committed";
/// see [`ThrottledValue::set`] for the full behaviour.
///
/// # Arguments
///
/// - `u32` - The throttle window in milliseconds. The most-recent
///   input is committed at most once per window.
///
/// # Returns
///
/// - `ThrottledValue<T>` - The throttled value handle.
///   Returns the factory result directly when no hook context is
///   active (e.g. when called outside a render cycle).
pub fn use_throttled_value<T>(interval_ms: u32) -> ThrottledValue<T>
where
    T: Clone + PartialEq + Debug + Default + 'static,
{
    HookContext::use_hook(|| ThrottledValue::<T>::new(interval_ms))
}
