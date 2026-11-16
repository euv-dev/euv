use super::*;

/// Search interaction state shared by the input and its preview rows.
#[derive(Clone, Data, Debug, Default, PartialEq)]
pub(crate) struct HeaderSearchState {
    /// Whether the user has opened the preview panel.
    #[get(pub(crate), type(copy))]
    #[set(pub(crate))]
    pub(crate) open: bool,
    /// Explicitly highlighted result; typing never selects a result.
    #[get(pub(crate), type(copy))]
    #[set(pub(crate))]
    pub(crate) active: Option<usize>,
}
