use super::*;

/// Obtains the [`UseReducedMotion`] registered against the current hook
/// context slot.
///
/// Behaves like `HookContext::use_hook` - the same handle is returned on
/// every render at the same hook index, so the listener is attached
/// once and the answer survives re-renders.
///
/// # Returns
///
/// - `UseReducedMotion` - The handle, with `get_reduced()` returning the
///   reactive `Signal<bool>`.
pub fn use_reduced_motion() -> UseReducedMotion {
    let handle: UseReducedMotion = HookContext::use_hook(|| {
        let handle: UseReducedMotion = UseReducedMotion::new();
        handle.attach();
        handle
    });
    App::use_cleanup({
        let handle: UseReducedMotion = handle.clone();
        move || handle.detach()
    });
    handle
}
