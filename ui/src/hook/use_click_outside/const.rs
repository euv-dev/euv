/// The document event fired when a pointer button goes down.
#[cfg(target_arch = "wasm32")]
pub(crate) const EVENT_MOUSEDOWN: &str = "mousedown";

/// The document event fired when a touch contact starts.
#[cfg(target_arch = "wasm32")]
pub(crate) const EVENT_TOUCHSTART: &str = "touchstart";

/// Captures presses before a descendant can stop event propagation.
#[cfg(target_arch = "wasm32")]
pub(crate) const USE_CLICK_OUTSIDE_CAPTURE: bool = true;
