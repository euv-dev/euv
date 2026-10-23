use super::*;

/// Obtains the [`UseHover`] registered against the current hook context
/// slot.
///
/// Behaves like `HookContext::use_hook` - the same handle is returned on
/// every render at the same hook index, so the document listeners are
/// attached once and the hover state survives re-renders.
///
/// Attach `get_target()` to the caller's markup with the `ref:`
/// attribute, then read `get_hovered()` inside a render closure to
/// react to the pointer entering or leaving.
///
/// # Returns
///
/// - `UseHover` - The handle, with `get_target()` returning the
///   [`NodeRef`] and `get_hovered()` the reactive `Signal<bool>`.
pub fn use_hover() -> UseHover {
    let handle: UseHover = HookContext::use_hook(|| {
        let handle: UseHover = UseHover::new();
        handle.attach();
        handle
    });
    App::use_cleanup({
        let handle: UseHover = handle.clone();
        move || handle.detach()
    });
    handle
}

/// Reports whether a pointer event landed inside the watched element.
///
/// # Arguments
///
/// - `&NodeRef<Element>` - The element being watched.
/// - `&Event` - The pointer event.
///
/// # Returns
///
/// - `bool` - `true` when the event's node is inside the element.
#[cfg(target_arch = "wasm32")]
pub(crate) fn pointer_is_inside(target: &NodeRef<Element>, event: &Event) -> bool {
    let Some(node) = event
        .target()
        .and_then(|target: EventTarget| target.dyn_into::<Node>().ok())
    else {
        return false;
    };
    let Some(element) = target.get_cloned() else {
        return false;
    };
    node_contains(&element, &node)
}
