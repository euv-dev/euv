/// HTML tag name created when mounting a cross-platform canvas surface.
pub(crate) const WEB_CANVAS_TAG: &str = "canvas";

/// HTML tag name used for zero-cost hidden safe-area CSS `env()` measurement probes.
pub(crate) const WEB_DIV_TAG: &str = "div";

/// Hidden probe style measuring CSS `env(safe-area-inset-left)` as width and `env(safe-area-inset-top)` as height.
pub(crate) const WEB_SAFE_AREA_TOP_LEFT_STYLE: &str = "position:fixed;top:0;left:0;width:env(safe-area-inset-left,0px);height:env(safe-area-inset-top,0px);pointer-events:none;visibility:hidden;";

/// Hidden probe style measuring CSS `env(safe-area-inset-right)` as width and `env(safe-area-inset-bottom)` as height.
pub(crate) const WEB_SAFE_AREA_BOTTOM_RIGHT_STYLE: &str = "position:fixed;top:0;left:0;width:env(safe-area-inset-right,0px);height:env(safe-area-inset-bottom,0px);pointer-events:none;visibility:hidden;";

/// Inline style HTML attribute name.
pub(crate) const WEB_ATTR_STYLE: &str = "style";

/// Tabindex HTML attribute name enabling keyboard focus on canvas.
pub(crate) const WEB_ATTR_TABINDEX: &str = "tabindex";

/// Default full-viewport body inline CSS eliminating scrollbars and borders.
pub(crate) const WEB_BODY_STYLE: &str =
    "margin:0;padding:0;width:100vw;height:100vh;overflow:hidden;background:#09090b;";

/// Inline CSS prefix for the full-viewport 1:1 physical canvas surface.
pub(crate) const WEB_CANVAS_STYLE_PREFIX: &str =
    "position:fixed;inset:0;width:100%;height:100%;display:block;outline:none;cursor:";

/// Inline CSS suffix disabling browser touch panning and text selection on the canvas surface.
pub(crate) const WEB_CANVAS_STYLE_SUFFIX: &str =
    ";margin:0;touch-action:none;user-select:none;-webkit-user-select:none;";

/// Default canvas tabindex value enabling keyboard focus.
pub(crate) const WEB_ATTR_TABINDEX_ZERO: &str = "0";

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

/// DOM Element method name entering browser fullscreen.
pub(crate) const WEB_METHOD_REQUEST_FULLSCREEN: &str = "requestFullscreen";

/// DOM Document method name exiting browser fullscreen.
pub(crate) const WEB_METHOD_EXIT_FULLSCREEN: &str = "exitFullscreen";
