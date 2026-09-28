use super::*;

/// The horizontal alignment of a table column and of every cell in it.
///
/// Drives the `c_euv_table_cell_center` / `c_euv_table_cell_right`
/// modifier stacked on top of the shared `c_euv_table_cell` base, so a
/// column's alignment is expressed in one place instead of being repeated
/// per cell.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum EuvTableAlign {
    /// Flush-left content; renders the base `c_euv_table_cell` alone.
    #[default]
    Left,
    /// Horizontally centred content.
    Center,
    /// Flush-right content, the usual home for numeric columns.
    Right,
}
