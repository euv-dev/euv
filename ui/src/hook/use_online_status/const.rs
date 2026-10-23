/// The window event fired when the browser comes back online.
#[cfg(target_arch = "wasm32")]
pub(crate) const EVENT_ONLINE: &str = "online";

/// The window event fired when the browser loses its connection.
#[cfg(target_arch = "wasm32")]
pub(crate) const EVENT_OFFLINE: &str = "offline";
