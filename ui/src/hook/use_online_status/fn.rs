use super::*;

/// Obtains the [`UseOnlineStatus`] registered against the current hook
/// context slot.
///
/// Behaves like `HookContext::use_hook` - the same handle is returned on
/// every render at the same hook index, so the window listeners are
/// attached once and the answer survives re-renders.
///
/// # Returns
///
/// - `UseOnlineStatus` - The handle, with `get_online()` returning the
///   reactive `Signal<bool>`.
pub fn use_online_status() -> UseOnlineStatus {
    let handle: UseOnlineStatus = HookContext::use_hook(|| {
        let handle: UseOnlineStatus = UseOnlineStatus::new();
        handle.attach();
        handle
    });
    App::use_cleanup({
        let handle: UseOnlineStatus = handle.clone();
        move || handle.detach()
    });
    handle
}
