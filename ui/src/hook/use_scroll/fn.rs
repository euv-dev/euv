use super::*;

/// Obtains the [`UseScroll`] registered against the current hook context
/// slot.
///
/// Behaves like `HookContext::use_hook` - the same handle is returned on
/// every render at the same hook index, so the window listener is
/// attached once and the offsets survive re-renders.
///
/// # Returns
///
/// - `UseScroll` - The handle, with `get_x()` and `get_y()` returning
///   the reactive scroll offset signals.
pub fn use_scroll() -> UseScroll {
    let handle: UseScroll = HookContext::use_hook(|| {
        let handle: UseScroll = UseScroll::new();
        handle.attach();
        handle
    });
    App::use_cleanup({
        let handle: UseScroll = handle.clone();
        move || handle.detach()
    });
    handle
}
