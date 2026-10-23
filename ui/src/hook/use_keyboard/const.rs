/// The window event fired when a key goes down.
#[cfg(target_arch = "wasm32")]
pub(crate) const EVENT_KEYDOWN: &str = "keydown";

/// The window event fired when a key comes back up.
#[cfg(target_arch = "wasm32")]
pub(crate) const EVENT_KEYUP: &str = "keyup";
