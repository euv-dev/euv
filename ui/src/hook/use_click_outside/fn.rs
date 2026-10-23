use super::*;

/// Obtains the [`UseClickOutside`] registered against the current hook
/// context slot.
///
/// Behaves like `HookContext::use_hook` - the same handle is returned on
/// every render at the same hook index, so the document listener is
/// attached once and the trigger survives re-renders.
///
/// Attach `get_target()` to the caller's markup with the `ref:`
/// attribute, then read `get_triggered()` inside a render closure to
/// react to a press outside.
///
/// # Returns
///
/// - `UseClickOutside` - The handle, with `get_target()` returning the
///   [`NodeRef`] and `get_triggered()` the reactive `Signal<bool>`.
pub fn use_click_outside() -> UseClickOutside {
    let handle: UseClickOutside = HookContext::use_hook(|| {
        let handle: UseClickOutside = UseClickOutside::new();
        handle.attach();
        handle
    });
    App::use_cleanup({
        let handle: UseClickOutside = handle.clone();
        move || handle.detach()
    });
    handle
}

/// Reports whether a pointer press landed outside the watched element.
///
/// A press on the watched element itself is *not* outside it, and a
/// press that arrives before the element has been mounted cannot be
/// classified at all — the element ref is still empty then, and an
/// unclassified press must not raise the trigger.
///
/// # Arguments
///
/// - `&NodeRef<Element>` - The element being watched.
/// - `&Event` - The press event.
///
/// # Returns
///
/// - `bool` - `true` when the press landed outside the element.
#[cfg(target_arch = "wasm32")]
pub(crate) fn press_is_outside(target: &NodeRef<Element>, event: &Event) -> bool {
    let Some(node) = event
        .target()
        .and_then(|target: EventTarget| target.dyn_into::<Node>().ok())
    else {
        return false;
    };
    let Some(element) = target.get_cloned() else {
        return false;
    };
    !node_contains(&element, &node)
}
