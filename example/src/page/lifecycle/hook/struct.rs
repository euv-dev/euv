use super::*;

/// Reactive state for a lifecycle demo feature.
#[derive(Clone, Copy, Data, Debug, Default, Eq, Hash, New, Ord, PartialEq, PartialOrd)]
pub(crate) struct UseLifecycle {
    /// The render count.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) render_count: Signal<i32>,
    /// The event log entries.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) logs: Signal<Vec<String>>,
}
