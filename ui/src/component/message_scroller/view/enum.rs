use super::*;

/// The direction a message scroller jumps to.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum EuvMessageScrollerDirection {
    /// The scroller jumps to the start of the list.
    Start,
    /// The scroller jumps to the end of the list.
    #[default]
    End,
}
