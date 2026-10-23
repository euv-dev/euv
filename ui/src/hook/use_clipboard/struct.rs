use super::*;

/// Reactive state for the clipboard.
///
/// The handle carries the outcome of the last write. The write itself
/// is asynchronous — the browser answers a promise — so the status is
/// the reactive part.
#[derive(Clone, CustomDebug, Data)]
pub struct UseClipboard {
    /// The outcome of the last write.
    #[get(type(copy))]
    pub status: Signal<UseClipboardStatus>,
}
