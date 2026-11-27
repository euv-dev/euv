/// Default window title for the native showcase binary.
pub const SHOWCASE_WINDOW_TITLE: &str = "Euv Studio";
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
/// DOM tag name for the hidden browser IME proxy input element.
#[cfg(target_arch = "wasm32")]
pub(crate) const WEB_IME_INPUT_TAG: &str = "input";
/// DOM id attribute for the hidden browser IME proxy input element.
#[cfg(target_arch = "wasm32")]
pub(crate) const WEB_IME_INPUT_ID: &str = "euv-ime-proxy";
/// DOM attribute name for `id`.
#[cfg(target_arch = "wasm32")]
pub(crate) const WEB_IME_ATTR_ID: &str = "id";
/// DOM attribute name for `type`.
#[cfg(target_arch = "wasm32")]
pub(crate) const WEB_IME_ATTR_TYPE: &str = "type";
/// DOM `<input type="text">` attribute value.
#[cfg(target_arch = "wasm32")]
pub(crate) const WEB_IME_ATTR_TEXT: &str = "text";
/// DOM attribute name for `autocomplete`.
#[cfg(target_arch = "wasm32")]
pub(crate) const WEB_IME_ATTR_AUTOCOMPLETE: &str = "autocomplete";
/// DOM attribute value disabling autocomplete/autocorrect/autocapitalize.
#[cfg(target_arch = "wasm32")]
pub(crate) const WEB_IME_ATTR_OFF: &str = "off";
/// DOM attribute name for `autocorrect`.
#[cfg(target_arch = "wasm32")]
pub(crate) const WEB_IME_ATTR_AUTOCORRECT: &str = "autocorrect";
/// DOM attribute name for `autocapitalize`.
#[cfg(target_arch = "wasm32")]
pub(crate) const WEB_IME_ATTR_AUTOCAPITALIZE: &str = "autocapitalize";
/// DOM attribute name for `spellcheck`.
#[cfg(target_arch = "wasm32")]
pub(crate) const WEB_IME_ATTR_SPELLCHECK: &str = "spellcheck";
/// DOM attribute value `"false"`.
#[cfg(target_arch = "wasm32")]
pub(crate) const WEB_IME_ATTR_FALSE: &str = "false";
/// DOM attribute name for `aria-label`.
#[cfg(target_arch = "wasm32")]
pub(crate) const WEB_IME_ATTR_ARIA_LABEL: &str = "aria-label";
/// Accessible label applied to the hidden IME proxy input.
#[cfg(target_arch = "wasm32")]
pub(crate) const WEB_IME_ARIA_LABEL: &str = "euv text input";
/// DOM attribute name for inline CSS `style`.
#[cfg(target_arch = "wasm32")]
pub(crate) const WEB_IME_ATTR_STYLE: &str = "style";
/// Inline CSS keeping the IME proxy input visually hidden without triggering iOS Safari auto-zoom.
#[cfg(target_arch = "wasm32")]
pub(crate) const WEB_IME_STYLE: &str = "position:fixed;left:0;top:0;width:1px;height:1px;opacity:0;pointer-events:none;border:0;padding:0;margin:0;z-index:-1;font-size:16px;";
/// DOM event name for text input changes.
#[cfg(target_arch = "wasm32")]
pub(crate) const WEB_EVENT_INPUT: &str = "input";
/// DOM event name for IME composition start.
#[cfg(target_arch = "wasm32")]
pub(crate) const WEB_EVENT_COMPOSITION_START: &str = "compositionstart";
/// DOM event name for IME composition end.
#[cfg(target_arch = "wasm32")]
pub(crate) const WEB_EVENT_COMPOSITION_END: &str = "compositionend";
/// DOM JS property name for an input element's string value.
#[cfg(target_arch = "wasm32")]
pub(crate) const WEB_PROP_VALUE: &str = "value";
/// Native action subcommand showing the browser soft keyboard / IME proxy.
#[cfg(target_arch = "wasm32")]
pub(crate) const WEB_IME_ACTION_SHOW: &str = "show";
/// Native action subcommand hiding the browser soft keyboard / IME proxy.
#[cfg(target_arch = "wasm32")]
pub(crate) const WEB_IME_ACTION_HIDE: &str = "hide";
/// Native action subcommand requesting browser fullscreen entry.
#[cfg(target_arch = "wasm32")]
pub(crate) const WEB_FS_ACTION_ON: &str = "on";
/// Native action subcommand requesting browser fullscreen exit.
#[cfg(target_arch = "wasm32")]
pub(crate) const WEB_FS_ACTION_OFF: &str = "off";
/// DOM Element method name entering browser fullscreen.
#[cfg(target_arch = "wasm32")]
pub(crate) const WEB_METHOD_REQUEST_FULLSCREEN: &str = "requestFullscreen";
/// DOM Document method name exiting browser fullscreen.
#[cfg(target_arch = "wasm32")]
pub(crate) const WEB_METHOD_EXIT_FULLSCREEN: &str = "exitFullscreen";
/// DOM Document property reporting the active fullscreen element.
#[cfg(target_arch = "wasm32")]
pub(crate) const WEB_PROP_FULLSCREEN_ELEMENT: &str = "fullscreenElement";
/// DOM event name for page visibility changes.
#[cfg(target_arch = "wasm32")]
pub(crate) const WEB_EVENT_VISIBILITY_CHANGE: &str = "visibilitychange";
/// DOM event name for window focus gain.
#[cfg(target_arch = "wasm32")]
pub(crate) const WEB_EVENT_FOCUS: &str = "focus";
/// DOM event name for page show lifecycle notifications.
#[cfg(target_arch = "wasm32")]
pub(crate) const WEB_EVENT_PAGE_SHOW: &str = "pageshow";
/// DOM event name for page hide lifecycle notifications.
#[cfg(target_arch = "wasm32")]
pub(crate) const WEB_EVENT_PAGE_HIDE: &str = "pagehide";
/// DOM event name for page unload termination warnings.
#[cfg(target_arch = "wasm32")]
pub(crate) const WEB_EVENT_BEFORE_UNLOAD: &str = "beforeunload";
/// DOM event name for network online transitions.
#[cfg(target_arch = "wasm32")]
pub(crate) const WEB_EVENT_ONLINE: &str = "online";
/// DOM event name for network offline transitions.
#[cfg(target_arch = "wasm32")]
pub(crate) const WEB_EVENT_OFFLINE: &str = "offline";
/// DOM event name for media-query preference changes.
#[cfg(target_arch = "wasm32")]
pub(crate) const WEB_EVENT_CHANGE: &str = "change";
/// DOM event name for URL fragment hash navigation changes.
#[cfg(target_arch = "wasm32")]
pub(crate) const WEB_EVENT_HASH_CHANGE: &str = "hashchange";
/// CSS media query detecting browser dark color scheme preference.
#[cfg(target_arch = "wasm32")]
pub(crate) const WEB_MEDIA_PREFERS_DARK: &str = "(prefers-color-scheme: dark)";
/// DOM window property exposing the mobile soft-keyboard `VisualViewport`.
#[cfg(target_arch = "wasm32")]
pub(crate) const WEB_PROP_VISUAL_VIEWPORT: &str = "visualViewport";
/// DOM `VisualViewport.height` property key.
#[cfg(target_arch = "wasm32")]
pub(crate) const WEB_PROP_HEIGHT: &str = "height";
/// Minimum logical pixel viewport contraction treated as an active browser soft keyboard.
#[cfg(target_arch = "wasm32")]
pub(crate) const WEB_KEYBOARD_MIN_DELTA_PX: f32 = 80.0;
