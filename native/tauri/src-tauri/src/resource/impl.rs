use super::*;

/// Implements `impl fmt::Display for ResourceError`.
impl fmt::Display for ResourceError {
    /// Formats a resource failure with its network or filesystem category.
    ///
    /// # Arguments
    /// - `&mut fmt::Formatter<'_>` - The destination formatter.
    ///
    /// # Returns
    /// - `fmt::Result` - The formatting result.
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ResourceError::Fetch(message) => write!(formatter, "Fetch: {message}"),
            ResourceError::Write(message) => write!(formatter, "Write: {message}"),
        }
    }
}

/// Implements `impl error::Error for ResourceError {}`.
impl error::Error for ResourceError {}
