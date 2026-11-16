use super::*;

/// One column of the [`euv_table`] component.
///
/// `key` is the stable identity used for the column's own diffing key; it
/// is not rendered, so a caller can pass a short slug (`"qty"`) while
/// `label` carries the visible uppercase heading.
#[derive(Clone, Debug, Default)]
pub struct EuvTableColumn {
    /// The stable column identity, used as the diffing key only.
    pub key: &'static str,
    /// The visible column heading text.
    pub label: ::euv::CowText,
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
    pub cells: Vec<::euv::CowText>,
    /// The stable row identity, used as the diffing key only.
    pub key: ::euv::CowText,
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
    /// An optional application class composed with the design-system table classes.
    pub class: Css,
}

/// Props for the [`euv_table_header`] component.
///
/// The header takes no settings of its own.
#[derive(Clone, CustomDebug, Default)]
pub struct EuvTableHeaderProps {}

/// Props for the [`euv_table_body`] component.
///
/// The body takes no settings of its own.
#[derive(Clone, CustomDebug, Default)]
pub struct EuvTableBodyProps {}

/// Props for the [`euv_table_footer`] component.
///
/// The footer takes no settings of its own.
#[derive(Clone, CustomDebug, Default)]
pub struct EuvTableFooterProps {}

/// Props for the [`euv_table_caption`] component.
///
/// The caption carries the text that names what the table shows.
#[derive(Clone, CustomDebug, Default)]
pub struct EuvTableCaptionProps {
    /// The text of the caption.
    pub text: &'static str,
}

/// Props for the [`euv_table_row`] component.
///
/// The row takes no settings of its own.
#[derive(Clone, CustomDebug, Default)]
pub struct EuvTableRowProps {}

/// Props for the [`euv_table_head`] component.
///
/// The heading takes no settings of its own.
#[derive(Clone, CustomDebug, Default)]
pub struct EuvTableHeadProps {}

/// Props for the [`euv_table_cell`] component.
///
/// The cell takes no settings of its own.
#[derive(Clone, CustomDebug, Default)]
pub struct EuvTableCellProps {}
