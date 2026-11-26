use super::*;

/// Mounted modal dialog overlay toggled reactively via `open` without unmounting its children.
#[derive(Clone, Data, Debug, New)]
pub struct EuvModal {
    /// Reactive visibility signal.
    #[get(type(copy))]
    pub(crate) open: Signal<bool>,
    /// Dialog heading.
    pub(crate) title: String,
    /// Mounted body node rendered inside the modal dialog card.
    #[get(type(copy))]
    pub(crate) body: NodeId,
    /// Active theme tokens.
    #[get(type(copy))]
    pub(crate) tokens: ThemeTokens,
}

/// Floating toast notification banner rendered at `Z_INDEX_TOAST`.
#[derive(Clone, Data, Debug, New)]
pub struct EuvToast {
    /// Reactive visibility signal.
    #[get(type(copy))]
    pub(crate) visible: Signal<bool>,
    /// Notification message text.
    pub(crate) message: String,
    /// Semantic tone.
    #[get(type(copy))]
    pub(crate) tone: EuvTone,
    /// Active theme tokens.
    #[get(type(copy))]
    pub(crate) tokens: ThemeTokens,
}
