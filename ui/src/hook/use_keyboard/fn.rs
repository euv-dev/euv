use super::*;

/// Obtains the [`UseKeyboard`] registered against the current hook
/// context slot.
///
/// Behaves like `HookContext::use_hook` - the same handle is returned on
/// every render at the same hook index, so the window listeners are
/// attached once and the pressed key survives re-renders.
///
/// # Returns
///
/// - `UseKeyboard` - The handle, with `get_pressed()` returning the
///   reactive `Signal<String>`.
pub fn use_keyboard() -> UseKeyboard {
    let handle: UseKeyboard = HookContext::use_hook(|| {
        let handle: UseKeyboard = UseKeyboard::new();
        handle.attach();
        handle
    });
    App::use_cleanup({
        let handle: UseKeyboard = handle.clone();
        move || handle.detach()
    });
    handle
}
