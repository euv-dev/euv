/// The kind of a coordinate scale.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub enum ScaleKind {
    /// A linear scale: equal steps in the data are equal steps on screen.
    #[default]
    Linear,
    /// A logarithmic scale.
    Log,
    /// A time scale.
    Time,
    /// An ordinal scale: the data are discrete categories.
    Category,
}

/// How an axis is positioned relative to its grid.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub enum AxisSide {
    /// The axis sits on the bottom edge of the grid.
    #[default]
    Bottom,
    /// The axis sits on the top edge of the grid.
    Top,
    /// The axis sits on the left edge of the grid.
    Left,
    /// The axis sits on the right edge of the grid.
    Right,
}
