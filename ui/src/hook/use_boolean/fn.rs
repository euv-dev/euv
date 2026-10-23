use super::*;

/// Obtains the [`UseBoolean`] registered against the current hook context slot.
///
/// Behaves like `HookContext::use_hook` - the same `UseBoolean` is
/// returned on every render at the same hook index, so the value
/// survives re-renders.
///
/// The initial value is applied once, on the first render. Later
/// renders never reset the value back to the initial one - use
/// `set_value` to change it.
///
/// # Arguments
///
/// - `bool` - The value the toggle starts at.
///
/// # Returns
///
/// - `UseBoolean` - The toggle handle, with `get_value()` returning the
///   reactive `Signal<bool>`.
pub fn use_boolean(initial: bool) -> UseBoolean {
    HookContext::use_hook(|| {
        let handle: UseBoolean = UseBoolean::new();
        handle.get_value().set(initial);
        handle
    })
}
