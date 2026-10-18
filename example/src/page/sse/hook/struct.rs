use super::*;

/// Reactive state for the Server-Sent Events (SSE) demo page.
#[derive(Clone, Copy, Data, Debug, Default, Eq, Hash, New, Ord, PartialEq, PartialOrd)]
pub(crate) struct UseSse {
    /// The SSE endpoint URL input.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) url: Signal<String>,
    /// Whether the SSE connection is currently active.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) connected: Signal<bool>,
    /// Whether the SSE connection is currently being established.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) connecting: Signal<bool>,
    /// The collected SSE event messages.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) messages: Signal<Vec<String>>,
    /// The error message, empty if no error.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) error: Signal<String>,
}
