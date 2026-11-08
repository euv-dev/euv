use super::*;

/// The way a marker separates one block of content from the next.
///
/// The reference publishes three: the plain marker, the separator that
/// sits between two blocks with a line through it, and the border that
/// draws the line above the content instead.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum EuvMarkerVariant {
    /// The marker is a plain row that holds an icon and its text.
    #[default]
    Default,
    /// The marker is a centered rule between two blocks.
    Separator,
    /// The marker draws a rule above the content it introduces.
    Border,
}
