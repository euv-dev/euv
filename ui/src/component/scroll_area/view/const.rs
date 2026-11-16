/// Stable root ownership boundary.
#[cfg(target_arch = "wasm32")]
pub(crate) const SCROLL_AREA_ROOT: &str = "[data-slot=scroll-area]";
/// Public viewport marker.
#[cfg(target_arch = "wasm32")]
pub(crate) const SCROLL_AREA_VIEWPORT: &str = "[data-slot=scroll-area-viewport]";
/// Public bar marker retained for existing consumers.
#[cfg(target_arch = "wasm32")]
pub(crate) const SCROLL_AREA_BAR: &str = "[data-slot=scroll-bar]";
/// Stable thumb marker.
#[cfg(target_arch = "wasm32")]
pub(crate) const SCROLL_AREA_THUMB: &str = "[data-slot=scroll-bar-thumb]";
/// Official minimum vertical thumb size.
#[cfg(any(target_arch = "wasm32", test))]
pub(crate) const SCROLL_AREA_MIN_THUMB: f64 = 16.0;
/// Native event names registered and removed together.
#[cfg(target_arch = "wasm32")]
pub(crate) const SCROLL_AREA_EVENTS: [&str; 7] = [
    "scroll",
    "pointerdown",
    "pointermove",
    "pointerup",
    "pointercancel",
    "lostpointercapture",
    "keydown",
];
/// CSS properties read by the vertical track geometry.
#[cfg(target_arch = "wasm32")]
pub(crate) const SCROLL_AREA_PADDING: [&str; 2] = ["padding-top", "padding-bottom"];
/// Browser mutation observer constructor.
#[cfg(target_arch = "wasm32")]
pub(crate) const SCROLL_AREA_OBSERVER: &str = "MutationObserver";
/// Mutation observer configuration keys.
#[cfg(target_arch = "wasm32")]
pub(crate) const SCROLL_AREA_OBSERVER_OPTIONS: [&str; 4] =
    ["childList", "subtree", "characterData", "attributes"];
/// Mutation record target field.
#[cfg(target_arch = "wasm32")]
pub(crate) const SCROLL_AREA_TARGET: &str = "target";
/// Pointer identity field, read without adding a web-sys feature.
#[cfg(target_arch = "wasm32")]
pub(crate) const SCROLL_AREA_POINTER_ID: &str = "pointerId";
/// Mutation observer methods.
#[cfg(target_arch = "wasm32")]
pub(crate) const SCROLL_AREA_OBSERVE: &str = "observe";
/// Mutation observer teardown method.
#[cfg(target_arch = "wasm32")]
pub(crate) const SCROLL_AREA_DISCONNECT: &str = "disconnect";
/// Runtime-only thumb geometry properties.
#[cfg(target_arch = "wasm32")]
pub(crate) const SCROLL_AREA_HEIGHT: &str = "height";
/// Runtime-only thumb offset property.
#[cfg(target_arch = "wasm32")]
pub(crate) const SCROLL_AREA_TRANSFORM: &str = "transform";
/// Runtime bar visibility property.
#[cfg(target_arch = "wasm32")]
pub(crate) const SCROLL_AREA_DISPLAY: &str = "display";
/// No-overflow bar visibility value.
#[cfg(target_arch = "wasm32")]
pub(crate) const SCROLL_AREA_NONE: &str = "none";
/// Native keyboard commands supported by a focused viewport.
#[cfg(any(target_arch = "wasm32", test))]
pub(crate) const SCROLL_AREA_KEYS: [&str; 6] =
    ["Home", "End", "PageUp", "PageDown", "ArrowUp", "ArrowDown"];
