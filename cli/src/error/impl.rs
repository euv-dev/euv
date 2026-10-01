use super::*;

/// Implements `Display` for `EuvError` to provide human-readable error messages.
impl Display for EuvError {
    /// Formats the error message based on the variant.
    ///
    /// # Arguments
    ///
    /// - `&mut Formatter<'_>` - The formatter.
    ///
    /// # Returns
    ///
    /// - `std::fmt::Result` - The result of the formatting operation.
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            EuvError::Io { message, error } => write!(formatter, "{message}: {error}"),
            EuvError::IoPath {
                message,
                path,
                error,
            } => {
                write!(formatter, "{message} '{}': {error}", path.display())
            }
            EuvError::Utf8 { message, error } => write!(formatter, "{message}: {error}"),
            EuvError::Server(message) => write!(formatter, "{message}"),
            EuvError::Message(message) => write!(formatter, "{message}"),
            EuvError::Watch(error) => write!(formatter, "{error}"),
        }
    }
}

/// Implements `Error` for `EuvError` so it integrates with the Rust error ecosystem.
impl Error for EuvError {
    /// Returns the lower-level source of this error, if any.
    ///
    /// # Returns
    ///
    /// - `Option<&(dyn Error + 'static)>` - The underlying error source.
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            EuvError::Io { error, .. } => Some(error),
            EuvError::IoPath { error, .. } => Some(error),
            EuvError::Utf8 { error, .. } => Some(error),
            EuvError::Watch(error) => Some(error),
            EuvError::Server(_) | EuvError::Message(_) => None,
        }
    }
}

/// Implements `From<notify::Error>` for `EuvError` to enable `?` operator conversion.
impl From<notify::Error> for EuvError {
    /// Converts a `notify::Error` into `EuvError::Watch`.
    ///
    /// # Arguments
    ///
    /// - `notify::Error` - The file watcher error to convert.
    ///
    /// # Returns
    ///
    /// - `EuvError` - The converted `Watch` variant.
    fn from(error: notify::Error) -> Self {
        EuvError::Watch(error)
    }
}
