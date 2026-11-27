use super::*;

/// Registers a platform-host native OS capability bridge (haptics, notifications, permissions, fullscreen, Dynamic Island).
///
/// # Arguments
///
/// - `NativeActionBridge` - Host callback handling `(kind, arg1, arg2)` and returning whether native OS execution succeeded.
pub fn register_native_action_bridge(bridge: NativeActionBridge) {
    EXTERNAL_NATIVE_ACTION_BRIDGE.store(bridge as *mut (), Ordering::Release);
}

/// Dispatches a native OS capability action `(kind, arg1, arg2)` to the registered host bridge, if any.
///
/// # Arguments
///
/// - `&str` - Action category (`"haptic"`, `"notify"`, `"island"`, `"permission"`, `"fullscreen"`, `"protocol"`).
/// - `&str` - Primary payload argument (e.g. title or permission key).
/// - `&str` - Secondary payload argument (e.g. body or rationale).
///
/// # Returns
///
/// - `bool` - `true` when handled by an active native OS host bridge.
pub fn dispatch_native_action(kind: &str, arg1: &str, arg2: &str) -> bool {
    let raw: *mut () = EXTERNAL_NATIVE_ACTION_BRIDGE.load(Ordering::Acquire);
    if raw.is_null() {
        return false;
    }
    // SAFETY: `raw` is stored exclusively via `register_native_action_bridge` from a `NativeActionBridge` fn pointer.
    let func: NativeActionBridge = unsafe { transmute::<*mut (), NativeActionBridge>(raw) };
    func(kind, arg1, arg2)
}

/// Copies `text` to the active OS or browser clipboard via direct platform hooks and any registered native action bridge.
///
/// # Arguments
///
/// - `&str` - UTF-8 text string to place on the system clipboard.
///
/// # Returns
///
/// - `bool` - `true` when at least one native or host clipboard sink accepted `text`.
pub fn write_system_clipboard(text: &str) -> bool {
    let bridge_ok: bool =
        dispatch_native_action(NATIVE_ACTION_CLIPBOARD, CLIPBOARD_ACTION_WRITE, text);
    #[cfg(target_os = "macos")]
    {
        macos_write_pasteboard(text) || bridge_ok
    }
    #[cfg(target_arch = "wasm32")]
    {
        web_write_clipboard(text) || bridge_ok
    }
    #[cfg(not(any(target_os = "macos", target_arch = "wasm32")))]
    {
        bridge_ok
    }
}
