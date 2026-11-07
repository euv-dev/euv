use super::*;

/// One highlighted segment of a code block.
///
/// The segment text is owned rather than borrowed: the tokenizer cuts
/// the source at arbitrary boundaries, and a block's code is a `&'static
/// str` that the renderer copies into the DOM anyway.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct CodeToken {
    /// The kind of the segment.
    pub(crate) kind: CodeTokenKind,
    /// The segment text.
    pub(crate) text: String,
}
