use super::*;

/// Display formatting for `PlatformError`.
impl fmt::Display for PlatformError {
    /// Formats `PlatformError` into a human-readable diagnostic message.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Error value.
    /// - `&mut Formatter<'_>` - Output formatter.
    ///
    /// # Returns
    ///
    /// - `fmt::Result` - Formatting status.
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::WindowNotFound(id) => write!(f, "{ERR_WINDOW_NOT_FOUND}: {id}"),
            Self::InitializationFailed(msg) => write!(f, "{ERR_NATIVE_INIT_FAILED}: {msg}"),
            Self::SurfaceError(msg) => write!(f, "{ERR_SURFACE_UNAVAILABLE}: {msg}"),
            Self::Unsupported(msg) => write!(f, "{msg}"),
        }
    }
}

/// Standard `Error` marker implementation for `PlatformError`.
impl Error for PlatformError {}
