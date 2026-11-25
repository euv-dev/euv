/// HTML tag name created when mounting a cross-platform canvas surface.
pub(crate) const WEB_CANVAS_TAG: &str = "canvas";

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
