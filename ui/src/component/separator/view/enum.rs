use super::*;

/// The direction a separator runs in.
///
/// The reference publishes two: the rule that divides a column of text
/// and the rule that divides a row.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum EuvSeparatorOrientation {
    /// The rule runs from the left edge to the right edge.
    #[default]
    Horizontal,
    /// The rule runs from the top edge to the bottom edge.
    Vertical,
}
