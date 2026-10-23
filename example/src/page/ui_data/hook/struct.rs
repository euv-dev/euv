use super::*;

/// The reactive state the data demo page drives.
///
/// The page owns the tree's selection and how many toasts and
/// notifications the stacks are showing. The stacks are driven by a
/// count rather than by a list of items: a toast carries a callback,
/// and a callback cannot be compared, so a `Signal<Vec<…>>` of them
/// would not type-check — the count is the part that changes.
#[derive(Clone, Debug)]
pub(crate) struct DataDemoState {
    /// The value of the tree node that is currently selected.
    pub(crate) selected: Signal<String>,
    /// How many toasts the toast stack is showing.
    pub(crate) message_count: Signal<usize>,
    /// How many notifications the notification stack is showing.
    pub(crate) notification_count: Signal<usize>,
}
