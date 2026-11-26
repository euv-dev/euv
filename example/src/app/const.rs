/// Default window title for the native showcase binary.
pub const SHOWCASE_WINDOW_TITLE: &str = "euv Cross-Platform Native Showcase";
/// Default logical window width in pixels.
pub const SHOWCASE_WINDOW_WIDTH: u32 = 1200;
/// Default logical window height in pixels.
pub const SHOWCASE_WINDOW_HEIGHT: u32 = 800;
/// Command-line flag enabling deterministic headless execution.
pub const FLAG_HEADLESS: &str = "--headless";
/// Command-line flag selecting the software rasterizer.
pub const FLAG_SOFTWARE: &str = "--software";
/// Command-line flag selecting the hardware GPU pipeline.
pub const FLAG_HARDWARE: &str = "--hardware";
/// Command-line flag selecting the hybrid compositor pipeline.
pub const FLAG_HYBRID: &str = "--hybrid";
/// Command-line flag keeping the native window open in an interactive event loop.
pub const FLAG_INTERACTIVE: &str = "--interactive";
/// Command-line flag capturing raw RGBA screenshots of all showcase routes into the following directory.
pub const FLAG_CAPTURE_DIR: &str = "--capture-dir";
/// Dev-only CLI flag printing the live AI Schema v1 runtime & layout snapshot JSON.
#[cfg(all(debug_assertions, not(target_arch = "wasm32")))]
pub const FLAG_AI_SCHEMA: &str = "--ai-schema";
/// Dev-only CLI flag executing a single AI Schema v1 JSON request against the headless showcase.
#[cfg(all(debug_assertions, not(target_arch = "wasm32")))]
pub const FLAG_AI_QUERY: &str = "--ai-query";
/// DOM KeyboardEvent key name for Enter.
#[cfg(target_arch = "wasm32")]
pub(crate) const WEB_KEY_ENTER: &str = "Enter";
/// DOM KeyboardEvent key name for Backspace.
#[cfg(target_arch = "wasm32")]
pub(crate) const WEB_KEY_BACKSPACE: &str = "Backspace";
/// DOM KeyboardEvent key name for Delete.
#[cfg(target_arch = "wasm32")]
pub(crate) const WEB_KEY_DELETE: &str = "Delete";
/// DOM KeyboardEvent key name for Escape.
#[cfg(target_arch = "wasm32")]
pub(crate) const WEB_KEY_ESCAPE: &str = "Escape";
/// DOM KeyboardEvent key name for Tab.
#[cfg(target_arch = "wasm32")]
pub(crate) const WEB_KEY_TAB: &str = "Tab";
/// DOM KeyboardEvent key name for Spacebar.
#[cfg(target_arch = "wasm32")]
pub(crate) const WEB_KEY_SPACE: &str = " ";
/// DOM KeyboardEvent key name for ArrowLeft.
#[cfg(target_arch = "wasm32")]
pub(crate) const WEB_KEY_ARROW_LEFT: &str = "ArrowLeft";
/// DOM KeyboardEvent key name for ArrowRight.
#[cfg(target_arch = "wasm32")]
pub(crate) const WEB_KEY_ARROW_RIGHT: &str = "ArrowRight";
/// DOM KeyboardEvent key name for ArrowUp.
#[cfg(target_arch = "wasm32")]
pub(crate) const WEB_KEY_ARROW_UP: &str = "ArrowUp";
/// DOM KeyboardEvent key name for ArrowDown.
#[cfg(target_arch = "wasm32")]
pub(crate) const WEB_KEY_ARROW_DOWN: &str = "ArrowDown";
/// DOM event name for pointer movement.
#[cfg(target_arch = "wasm32")]
pub(crate) const WEB_EVENT_POINTER_MOVE: &str = "pointermove";
/// DOM event name for pointer press.
#[cfg(target_arch = "wasm32")]
pub(crate) const WEB_EVENT_POINTER_DOWN: &str = "pointerdown";
/// DOM event name for pointer release.
#[cfg(target_arch = "wasm32")]
pub(crate) const WEB_EVENT_POINTER_UP: &str = "pointerup";
/// DOM event name for pointer cancel.
#[cfg(target_arch = "wasm32")]
pub(crate) const WEB_EVENT_POINTER_CANCEL: &str = "pointercancel";
/// DOM event name for lost pointer capture.
#[cfg(target_arch = "wasm32")]
pub(crate) const WEB_EVENT_LOST_POINTER_CAPTURE: &str = "lostpointercapture";
/// DOM event name for window blur.
#[cfg(target_arch = "wasm32")]
pub(crate) const WEB_EVENT_BLUR: &str = "blur";
/// DOM event name for mouse/trackpad wheel scroll.
#[cfg(target_arch = "wasm32")]
pub(crate) const WEB_EVENT_WHEEL: &str = "wheel";
/// DOM event name for key down.
#[cfg(target_arch = "wasm32")]
pub(crate) const WEB_EVENT_KEY_DOWN: &str = "keydown";
/// DOM event name for key up.
#[cfg(target_arch = "wasm32")]
pub(crate) const WEB_EVENT_KEY_UP: &str = "keyup";
/// DOM event name for window resize.
#[cfg(target_arch = "wasm32")]
pub(crate) const WEB_EVENT_RESIZE: &str = "resize";
/// DOM `PointerEvent.pointerType` value for touch contacts.
#[cfg(target_arch = "wasm32")]
pub(crate) const WEB_PTR_TYPE_TOUCH: &str = "touch";
/// DOM `PointerEvent.pointerType` value for pen/stylus contacts.
#[cfg(target_arch = "wasm32")]
pub(crate) const WEB_PTR_TYPE_PEN: &str = "pen";
/// DOM CSS selector for the mounted canvas element.
#[cfg(target_arch = "wasm32")]
pub(crate) const WEB_SELECTOR_CANVAS: &str = "canvas";
