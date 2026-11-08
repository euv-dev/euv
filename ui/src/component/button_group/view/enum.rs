use super::*;

/// The direction a button group is laid out in.
///
/// The reference publishes two: a row and a column.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum EuvButtonGroupOrientation {
    /// A row of joined buttons.
    #[default]
    Horizontal,
    /// A column of joined buttons.
    Vertical,
}

/// The direction a button-group separator runs in.
///
/// The reference publishes two: a vertical rule between the buttons of a
/// row and a horizontal rule between the buttons of a column.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum EuvButtonGroupSeparatorOrientation {
    /// A vertical rule between the buttons of a row.
    #[default]
    Vertical,
    /// A horizontal rule between the buttons of a column.
    Horizontal,
}
