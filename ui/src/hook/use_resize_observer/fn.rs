use super::*;

/// Obtains the [`UseResizeObserver`] registered against the current hook
/// context slot.
///
/// Behaves like `HookContext::use_hook` - the same handle is returned on
/// every render at the same hook index, so the observer is created once
/// and the size survives re-renders.
///
/// Attach `get_target()` to the caller's markup with the `ref:`
/// attribute, then read `get_width()` / `get_height()` inside a render
/// closure to react to a resize.
///
/// # Returns
///
/// - `UseResizeObserver` - The handle, with `get_target()` returning the
///   [`NodeRef`] and `get_width()` / `get_height()` the reactive size
///   signals.
pub fn use_resize_observer() -> UseResizeObserver {
    let handle: UseResizeObserver = HookContext::use_hook(|| {
        let handle: UseResizeObserver = UseResizeObserver::new();
        handle.attach();
        handle
    });
    App::use_cleanup({
        let handle: UseResizeObserver = handle.clone();
        move || handle.detach()
    });
    handle
}
