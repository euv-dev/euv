/// Represents errors that can occur during cache operations.
///
/// Each variant carries a human-readable description string.
#[derive(Debug)]
pub(crate) enum CacheError {
    /// An error occurred while fetching a remote resource.
    Fetch(String),
    /// An error occurred while writing to the local filesystem.
    Write(String),
    /// An error occurred while reading from the local filesystem.
    Read(String),
}

/// Outcome of a `update_cache` invocation, surfaced to the webview.
///
/// Serialized as a lowercase string (`"success"` / `"failed"`) so the JS
/// side can compare it directly without importing any custom decoders. Kept
/// deliberately small — once a cache snapshot is staged or we have decided
/// not to stage one, every retry decision and UI hint hangs off this enum.
///
/// Inherent methods (`as_tag`) and the `Serialize` impl live in
/// `super::impl` (see `impl.rs`).
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum CacheUpdateStatus {
    /// The cache snapshot was fetched, written, and staged for next launch.
    Success,
    /// The cache update failed before producing a usable snapshot.
    Failed,
}
