/// Distinguishes network response failures from local snapshot write failures.
#[derive(Debug)]
pub(crate) enum ResourceError {
    /// The remote request, redirect, timeout, or response body failed.
    Fetch(String),
    /// A temporary or final resource file could not be written.
    Write(String),
}
