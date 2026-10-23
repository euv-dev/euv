/// The document event fired while the pointer moves during a drag.
#[cfg(target_arch = "wasm32")]
pub(crate) const EVENT_MOUSEMOVE: &str = "mousemove";

/// The document event fired when the pointer button comes back up.
#[cfg(target_arch = "wasm32")]
pub(crate) const EVENT_MOUSEUP: &str = "mouseup";
