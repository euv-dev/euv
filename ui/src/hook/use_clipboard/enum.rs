use super::*;

/// The outcome of the last clipboard write.
///
/// The clipboard is asynchronous, so the status is what a render
/// closure reads: the write is *started* synchronously and the status
/// settles once the browser has answered.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UseClipboardStatus {
    /// No write has been attempted yet.
    Idle,
    /// The last write succeeded.
    Copied,
    /// The last write was rejected by the browser.
    Failed,
    /// The host has no usable clipboard, or refused to grant access.
    Unavailable,
}
