/// Unified error type returned by cross-platform OS window and rendering operations.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum PlatformError {
    /// Referenced window handle does not exist or was closed.
    WindowNotFound(u64),
    /// Native OS API initialization or resource creation failed.
    InitializationFailed(String),
    /// Surface presentation or GPU submission failed.
    SurfaceError(String),
    /// Requested operation is unsupported on the active platform.
    Unsupported(String),
}
