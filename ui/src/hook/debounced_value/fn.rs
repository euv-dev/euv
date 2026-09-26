use super::*;

/// Obtains the debounced value registered against the current hook context slot.
///
/// Behaves like `HookContext::use_hook` - the same `DebouncedValue` is
/// returned on every render at the same hook index, so the in-flight
/// throttle / pending slot survives across renders without losing state.
///
/// The factory uses the supplied `delay_ms` to seed the slot. To change
/// the delay at runtime, call [`DebouncedValue::set`] with a `delay`
/// value of your choice (the field is `pub(crate)`, but `set` plus
/// `tick` is the supported public surface).
///
/// # Arguments
///
/// - `u32` - The quiet period in milliseconds. After this many
///   milliseconds without a fresh `set`, any pending value is committed.
///
/// # Returns
///
/// - `DebouncedValue<T>` - The debounced value handle.
///   Returns the factory result directly when no hook context is
///   active (e.g. when called outside a render cycle).
pub fn use_debounced_value<T>(delay_ms: u32) -> DebouncedValue<T>
where
    T: Clone + PartialEq + Debug + Default + 'static,
{
    HookContext::use_hook(|| DebouncedValue::<T>::new(delay_ms))
}
