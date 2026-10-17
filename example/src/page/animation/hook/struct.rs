use super::*;

/// Reactive state for a progress bar feature.
#[derive(Clone, Copy, Data, Debug, Default, Eq, Hash, New, Ord, PartialEq, PartialOrd)]
pub(crate) struct UseProgress {
    /// Whether the progress bar animation is currently running.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) running: Signal<bool>,
}
