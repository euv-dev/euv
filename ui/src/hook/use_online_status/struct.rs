use super::*;

/// Reactive state for the browser's online status.
///
/// The handle carries the current answer and the teardown that removes
/// the `online` / `offline` listeners from the window.
///
/// The answer is a `Signal<bool>`: a render closure that reads it
/// re-renders when the connection state changes.
#[derive(Clone, CustomDebug, Data)]
pub struct UseOnlineStatus {
    /// Whether the browser believes it is online.
    #[get(type(copy))]
    pub online: Signal<bool>,
    /// The teardown that removes the window listeners.
    #[get(pub(crate))]
    #[debug(skip)]
    pub(crate) teardown: Teardown,
}
