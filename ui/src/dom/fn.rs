// The browser handles and the wasm-bindgen glue this module reaches
// for are only used on the wasm target, so on a host build the glob
// import resolves to nothing.
#[cfg(target_arch = "wasm32")]
use super::*;

/// Schedules `callback` to run after the current render has been
/// committed to the DOM.
///
/// The element-bound hooks — the resize observer, the intersection
/// observer, the hover tracker — need the live DOM element before they
/// can subscribe to it, and the element does not exist while the first
/// render is still being built. A zero-delay timeout is used rather
/// than a microtask on purpose: the renderer commits a signal-driven
/// update in a microtask, so a microtask scheduled from inside a render
/// could run *before* that commit and would see a stale DOM. A timeout
/// is a macrotask, so it is always queued behind every pending
/// microtask and the DOM is settled by the time it runs.
///
/// On a host without a browser the callback is dropped: there is no DOM
/// to wait for, and the hooks that use this are inert there anyway.
///
/// # Arguments
///
/// - `F` - The callback to run after the DOM is committed.
pub(crate) fn after_mount<F>(callback: F)
where
    F: FnOnce() + 'static,
{
    schedule_after(0, callback);
}

/// Schedules `callback` to run `delay_ms` milliseconds from now.
///
/// The callback runs as a macrotask, so it is always queued behind the
/// pending microtasks the renderer uses to commit updates.
///
/// On a host without a browser the callback is dropped.
///
/// # Arguments
///
/// - `u32` - The delay in milliseconds.
/// - `F` - The callback to run.
pub(crate) fn schedule_after<F>(delay_ms: u32, callback: F)
where
    F: FnOnce() + 'static,
{
    #[cfg(target_arch = "wasm32")]
    {
        let Some(window) = window() else {
            return;
        };
        let function: JsValue = Closure::once_into_js(move || {
            callback();
        });
        let Ok(function) = function.dyn_into::<Function>() else {
            return;
        };
        let _: Result<i32, JsValue> = window.set_timeout_with_callback_and_timeout_and_arguments_0(
            function.as_ref(),
            delay_ms as i32,
        );
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        let _: u32 = delay_ms;
        let _: F = callback;
    }
}

/// Returns the browser window, or `None` on a host without one.
///
/// # Returns
///
/// - `Option<Window>` - The window, when the host has one.
#[cfg(target_arch = "wasm32")]
pub(crate) fn browser_window() -> Option<Window> {
    window()
}

/// Returns the browser document, or `None` on a host without one.
///
/// # Returns
///
/// - `Option<Document>` - The document, when the host has one.
#[cfg(target_arch = "wasm32")]
pub(crate) fn browser_document() -> Option<Document> {
    browser_window().and_then(|window: Window| window.document())
}

/// Reports whether the node `candidate` sits inside the node `container`.
///
/// A node counts as inside itself, which is what a click-outside check
/// needs: a click on the watched element is not "outside" it.
///
/// # Arguments
///
/// - `&Node` - The node that was clicked.
///
/// # Returns
///
/// - `bool` - `true` when `candidate` is inside `container`.
#[cfg(target_arch = "wasm32")]
pub(crate) fn node_contains(container: &Node, candidate: &Node) -> bool {
    container.contains(Some(candidate))
}
