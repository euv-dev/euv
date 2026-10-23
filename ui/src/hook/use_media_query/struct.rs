use super::*;

/// Reactive state for one CSS media query.
///
/// The handle carries the query it watches, the current match result,
/// and the teardown that removes the `change` listener from the
/// browser's `MediaQueryList`.
///
/// The match result is a `Signal<bool>`: a render closure that reads it
/// re-renders when the query starts or stops matching, and nothing
/// else in the tree is disturbed.
#[derive(Clone, CustomDebug, Data)]
pub struct UseMediaQuery {
    /// Whether the query currently matches.
    #[get(type(copy))]
    pub matches: Signal<bool>,
    /// The media query text this handle watches.
    #[get(type(copy))]
    pub query: &'static str,
    /// The teardown that removes the `change` listener.
    ///
    /// `None` until the browser listener is attached, and `None` again
    /// after the teardown has run.
    #[get(pub(crate))]
    #[debug(skip)]
    pub(crate) teardown: Teardown,
}
