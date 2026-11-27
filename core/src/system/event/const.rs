/// Minimum viewport dimension or scale delta required before emitting a new normalized `Resize` event.
pub(crate) const RESIZE_DEDUP_EPSILON: f32 = 0.01;

/// Minimum virtual keyboard height change in logical pixels required to emit a new `KeyboardVisibilityChanged` event.
pub(crate) const KEYBOARD_HEIGHT_EPSILON: f32 = 0.5;

/// Default BCP-47 locale tag reported before an OS locale change arrives.
pub(crate) const DEFAULT_SYSTEM_LOCALE: &str = "en-US";
