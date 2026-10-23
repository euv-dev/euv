use super::*;

/// Obtains the [`UseWindowSize`] registered against the current hook
/// context slot.
///
/// Behaves like `HookContext::use_hook` - the same handle is returned on
/// every render at the same hook index, so the window listener is
/// attached once and the size survives re-renders.
///
/// # Returns
///
/// - `UseWindowSize` - The handle, with `get_width()` and `get_height()`
///   returning the reactive size signals.
pub fn use_window_size() -> UseWindowSize {
    let handle: UseWindowSize = HookContext::use_hook(|| {
        let handle: UseWindowSize = UseWindowSize::new();
        handle.attach();
        handle
    });
    App::use_cleanup({
        let handle: UseWindowSize = handle.clone();
        move || handle.detach()
    });
    handle
}

/// Reads one dimension of the viewport.
///
/// # Arguments
///
/// - `&Window` - The window to read.
/// - `bool` - `true` for the width, `false` for the height.
///
/// # Returns
///
/// - `i32` - The dimension in CSS pixels, or zero when it cannot be
///   read.
#[cfg(target_arch = "wasm32")]
pub(crate) fn read_dimension(window: &Window, width: bool) -> i32 {
    let value: Result<JsValue, JsValue> = if width {
        window.inner_width()
    } else {
        window.inner_height()
    };
    match value {
        Ok(value) => value.as_f64().unwrap_or(0.0) as i32,
        Err(_) => 0,
    }
}
