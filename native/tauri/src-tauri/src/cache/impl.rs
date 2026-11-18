use super::*;

/// Implements `fmt::Display` for `CacheError`.
impl fmt::Display for CacheError {
    /// Formats the [`CacheError`] via the supplied formatter.
    ///
    /// # Arguments
    ///
    /// - `&mut fmt::Formatter<'_>` - The formatter receiving the formatted output.
    ///
    /// # Returns
    ///
    /// - `fmt::Result` - A `fmt::Result` value.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CacheError::Fetch(message) => write!(f, "Fetch: {message}"),
            CacheError::Write(message) => write!(f, "Write: {message}"),
            CacheError::Read(message) => write!(f, "Read: {message}"),
        }
    }
}

/// Implements `error::Error` for `CacheError`.
impl error::Error for CacheError {}

/// Inherent implementation of [`CacheUpdateStatus`].
impl CacheUpdateStatus {
    /// String tag sent over the bridge.
    ///
    /// Maps the enum to the lowercase literal that the JS side compares
    /// against — see `UPDATE_RESULT_SUCCESS` / `UPDATE_RESULT_FAILED` in
    /// `super::const`. Centralising the mapping here keeps the wire format
    /// a single grep target.
    ///
    /// # Returns
    ///
    /// - `&'static str` - The lowercase status tag for this variant.
    fn as_tag(self) -> &'static str {
        match self {
            CacheUpdateStatus::Success => UPDATE_RESULT_SUCCESS,
            CacheUpdateStatus::Failed => UPDATE_RESULT_FAILED,
        }
    }
}

/// Implements `Serialize` for `CacheUpdateStatus`.
///
/// Emits the lowercase tag from `as_tag` so the JS side can compare with
/// `===` directly — see the matching `UPDATE_RESULT_*` constants on the
/// webview side.
impl Serialize for CacheUpdateStatus {
    /// Implements the `serialize` operation on [`CacheUpdateStatus`].
    ///
    /// # Arguments
    ///
    /// - `S` - A `S` parameter.
    ///
    /// # Returns
    ///
    /// - `Result<S::Ok, S::Error>` - Result of the operation; an `Err` variant on failure.
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.as_tag())
    }
}
