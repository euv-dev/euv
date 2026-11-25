use super::*;

/// Cross-platform reactive navigation router with history stack shared across Web, Mobile, Tablet, and Desktop.
#[derive(Clone, Data, Debug)]
pub struct Router {
    /// Reactive current route path signal.
    #[get(type(copy))]
    pub(crate) current: Signal<String>,
    /// Ordered navigation history stack.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) history: Vec<String>,
    /// Current cursor index inside `history`.
    #[get(type(copy))]
    pub(crate) cursor: usize,
}
