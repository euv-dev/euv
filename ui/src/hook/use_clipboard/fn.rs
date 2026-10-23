use super::*;

/// Obtains the [`UseClipboard`] registered against the current hook
/// context slot.
///
/// Behaves like `HookContext::use_hook` - the same handle is returned on
/// every render at the same hook index, so the status survives
/// re-renders.
///
/// # Returns
///
/// - `UseClipboard` - The handle, with `get_status()` returning the
///   reactive `Signal<UseClipboardStatus>`.
pub fn use_clipboard() -> UseClipboard {
    HookContext::use_hook(UseClipboard::new)
}
