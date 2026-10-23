use super::*;

/// One column of a grid row.
///
/// Carries the column's share of the row and the space it is pushed to the
/// right by, both counted in the row's twenty-four columns.
#[derive(Clone, Copy, CustomDebug, Data, Default, New)]
pub struct EuvGridCol {
    /// How many of the row's twenty-four columns this column takes.
    #[get(type(copy))]
    pub span: u32,
    /// How many columns to push this one to the right by.
    #[get(type(copy))]
    pub offset: u32,
}

/// Props for the [`euv_grid`] component.
///
/// Defines the strongly-typed interface for a grid row: how its columns are
/// laid out and whether the row spaces them with a gap.
#[derive(Clone, CustomDebug, Default)]
pub struct EuvGridProps {
    /// The columns, in order.
    ///
    /// The Nth entry lays out the Nth child of the grid; a child with no
    /// entry here falls back to the default column.
    pub cols: Vec<EuvGridCol>,
    /// Whether the row spaces its columns with a gap.
    pub gutter: bool,
}
