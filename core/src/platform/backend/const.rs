/// Error prefix when a requested OS window handle does not exist.
pub(crate) const ERR_WINDOW_NOT_FOUND: &str = "window handle not found";

/// Error prefix when the native OS API fails to initialize a display or window.
pub(crate) const ERR_NATIVE_INIT_FAILED: &str = "native platform initialization failed";

/// Error prefix when a requested graphics surface operation is unsupported.
pub(crate) const ERR_SURFACE_UNAVAILABLE: &str = "native rendering surface unavailable";
