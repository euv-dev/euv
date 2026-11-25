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
