/// HTML tag name created when mounting a cross-platform canvas surface.
pub(crate) const WEB_CANVAS_TAG: &str = "canvas";

/// Inline style HTML attribute name.
pub(crate) const WEB_ATTR_STYLE: &str = "style";

/// Tabindex HTML attribute name enabling keyboard focus on canvas.
pub(crate) const WEB_ATTR_TABINDEX: &str = "tabindex";

/// Default full-viewport body inline CSS eliminating scrollbars and borders.
pub(crate) const WEB_BODY_STYLE: &str =
    "margin:0;padding:0;width:100vw;height:100vh;overflow:hidden;background:#09090b;";

/// 2D rendering context identifier passed to `HTMLCanvasElement.getContext`.
pub(crate) const WEB_CONTEXT_2D: &str = "2d";

/// Media query string used to detect browser dark mode preference.
pub(crate) const WEB_DARK_MEDIA_QUERY: &str = "(prefers-color-scheme: dark)";

/// Error message when browser `window` or `document` is unavailable.
pub(crate) const ERR_WEB_WINDOW_MISSING: &str = "browser window or document unavailable";

/// Error message when `HTMLCanvasElement` creation or casting fails.
pub(crate) const ERR_WEB_CANVAS_CREATE: &str = "failed to create HTMLCanvasElement";

/// Error message when `CanvasRenderingContext2d` cannot be acquired.
pub(crate) const ERR_WEB_CTX2D_MISSING: &str = "failed to acquire 2d canvas rendering context";

/// Error message when `ImageData` creation or `putImageData` fails.
pub(crate) const ERR_WEB_PRESENT_FAILED: &str = "failed to blit ImageData to canvas";

/// CSS cursor keyword for default arrow.
pub(crate) const WEB_CURSOR_DEFAULT: &str = "default";

/// CSS cursor keyword for clickable hand pointer.
pub(crate) const WEB_CURSOR_POINTER: &str = "pointer";

/// CSS cursor keyword for text selection I-beam.
pub(crate) const WEB_CURSOR_TEXT: &str = "text";

/// CSS cursor keyword for precision crosshair.
pub(crate) const WEB_CURSOR_CROSSHAIR: &str = "crosshair";

/// CSS cursor keyword for disabled control.
pub(crate) const WEB_CURSOR_NOT_ALLOWED: &str = "not-allowed";

/// CSS cursor keyword for open grab hand.
pub(crate) const WEB_CURSOR_GRAB: &str = "grab";

/// CSS cursor keyword for active grabbing hand.
pub(crate) const WEB_CURSOR_GRABBING: &str = "grabbing";

/// CSS cursor keyword for omnidirectional move.
pub(crate) const WEB_CURSOR_MOVE: &str = "move";

/// CSS cursor keyword for horizontal resize.
pub(crate) const WEB_CURSOR_EW_RESIZE: &str = "ew-resize";

/// CSS cursor keyword for vertical resize.
pub(crate) const WEB_CURSOR_NS_RESIZE: &str = "ns-resize";
