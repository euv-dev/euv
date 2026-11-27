use super::*;

/// Active character-range text selection targeting a live selectable `NodeKind::Text` node.
#[derive(Clone, Copy, Data, Debug, Eq, New, PartialEq)]
pub struct SelectionState {
    /// Scene node owning the selected text run.
    #[get(type(copy))]
    pub(crate) node_id: NodeId,
    /// Inclusive character index where selection started.
    #[get(type(copy))]
    pub(crate) anchor_char: usize,
    /// Active character index where selection currently ends.
    #[get(type(copy))]
    pub(crate) focus_char: usize,
}
