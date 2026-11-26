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
