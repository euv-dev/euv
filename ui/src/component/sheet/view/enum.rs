use super::*;

/// The edge a sheet slides in from.
///
/// The reference publishes four, one per edge of the viewport.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum EuvSheetSide {
    /// The sheet slides in from the top edge.
    Top,
    /// The sheet slides in from the bottom edge.
    Bottom,
    /// The sheet slides in from the left edge.
    Left,
    /// The sheet slides in from the right edge.
    #[default]
    Right,
}
