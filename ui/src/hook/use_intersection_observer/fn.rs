use super::*;

/// Obtains the [`UseIntersectionObserver`] registered against the
/// current hook context slot.
///
/// Behaves like `HookContext::use_hook` - the same handle is returned on
/// every render at the same hook index, so the observer is created once
/// and the visibility survives re-renders.
///
/// Attach `get_target()` to the caller's markup with the `ref:`
/// attribute, then read `get_visible()` inside a render closure to
/// react to the element entering or leaving the viewport.
///
/// # Returns
///
/// - `UseIntersectionObserver` - The handle, with `get_target()`
///   returning the [`NodeRef`] and `get_visible()` the reactive
///   `Signal<bool>`.
pub fn use_intersection_observer() -> UseIntersectionObserver {
    let handle: UseIntersectionObserver = HookContext::use_hook(|| {
        let handle: UseIntersectionObserver = UseIntersectionObserver::new();
        handle.attach();
        handle
    });
    App::use_cleanup({
        let handle: UseIntersectionObserver = handle.clone();
        move || handle.detach()
    });
    handle
}
