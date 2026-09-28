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

/// One column of the [`euv_table`] component.
///
/// `key` is the stable identity used for the column's own diffing key; it
/// is not rendered, so a caller can pass a short slug (`"qty"`) while
/// `label` carries the visible uppercase heading.
#[derive(Clone, Copy, Debug, Default)]
pub struct EuvTableColumn {
    /// The stable column identity, used as the diffing key only.
    pub key: &'static str,
    /// The visible column heading text.
    pub label: &'static str,
    /// The horizontal alignment applied to this column's cells.
    pub align: EuvTableAlign,
}

/// One body row of the [`euv_table`] component.
///
/// `cells` is positional: cell `i` is rendered under `columns[i]`, and a
/// row shorter than the column list simply renders fewer cells. Extra
/// cells beyond the column list are dropped, so a ragged row can never
/// escape the header.
#[derive(Clone, Debug, Default)]
pub struct EuvTableRow {
    /// The cell texts, positionally matched to [`EuvTableProps::columns`].
    pub cells: Vec<&'static str>,
    /// The stable row identity, used as the diffing key only.
    pub key: &'static str,
}

/// Props for the [`euv_table`] component.
///
/// Defines the strongly-typed interface for a bordered data table with a
/// header, a zebra mode, and an optional caption.
#[derive(Clone, CustomDebug, Default)]
pub struct EuvTableProps {
    /// The column definitions driving the header and each cell's alignment.
    pub columns: Vec<EuvTableColumn>,
    /// The body rows; an empty list renders an empty `<tbody>`.
    pub rows: Vec<EuvTableRow>,
    /// The caption rendered under the table (skipped when empty).
    pub caption: &'static str,
    /// Whether every other row gets the `c_euv_table_row_zebra` fill.
    pub zebra: bool,
}
