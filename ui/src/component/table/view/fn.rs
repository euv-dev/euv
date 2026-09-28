use super::*;

/// A bordered monochrome data table with a header row and optional caption.
///
/// Renders a real `<table>` carrying `c_euv_table`, split into a
/// `c_euv_table_head` `<thead>` and a `c_euv_table_body` `<tbody>`. Every
/// body cell is a `<td>` carrying `c_euv_table_cell` plus the alignment
/// modifier of its column; with `zebra` enabled every other row swaps to
/// `c_euv_table_row_zebra` so long tables stay scannable. An empty
/// `caption` is skipped.
///
/// # Arguments
///
/// - `VirtualNode<EuvTableProps>` - The props node containing columns, rows, caption and zebra.
///
/// # Returns
///
/// - `VirtualNode` - The table virtual DOM tree.
#[component]
pub fn euv_table(node: VirtualNode<EuvTableProps>) -> VirtualNode {
    let EuvTableProps {
        columns,
        rows,
        caption,
        zebra,
    }: EuvTableProps = node.try_get_props().unwrap_or_default();
    let head: VirtualNode = table_head(columns.as_slice());
    let body: VirtualNode = table_body(columns.as_slice(), rows, zebra);
    html! {
        table {
            class: c_euv_table()
            if !caption.is_empty() {
                caption {
                    class: c_euv_table_caption()
                    {
                        caption
                    }
                }
            }
            head
            body
        }
    }
}

/// Builds the `<thead>` holding one `<th>` per column.
///
/// # Arguments
///
/// - `&[EuvTableColumn]` - The column definitions, in display order.
///
/// # Returns
///
/// - `VirtualNode` - The table head virtual DOM tree.
fn table_head(columns: &[EuvTableColumn]) -> VirtualNode {
    let header_cells: Vec<VirtualNode> = columns
        .iter()
        .map(|column: &EuvTableColumn| {
            let align_class: &'static str = table_cell_align_class(column.align);
            html! {
                th {
                    key: column.key
                    class: c_euv_table_header_cell()
                    class: align_class
                    column.label
                }
            }
        })
        .collect();
    html! {
        thead {
            class: c_euv_table_head()
            tr {
                header_cells
            }
        }
    }
}

/// Builds the `<tbody>` by mapping every row through [`table_row`].
///
/// # Arguments
///
/// - `&[EuvTableColumn]` - The column definitions driving each row's cells.
/// - `Vec<EuvTableRow>` - The body rows, consumed in order.
/// - `bool` - Whether every other row is zebra striped.
///
/// # Returns
///
/// - `VirtualNode` - The table body virtual DOM tree.
fn table_body(columns: &[EuvTableColumn], rows: Vec<EuvTableRow>, zebra: bool) -> VirtualNode {
    let row_nodes: Vec<VirtualNode> = rows
        .into_iter()
        .enumerate()
        .map(|(index, row): (usize, EuvTableRow)| table_row(columns, row, zebra && index % 2 == 0))
        .collect();
    html! {
        tbody {
            class: c_euv_table_body()
            row_nodes
        }
    }
}

/// Builds one `<tr>` of the table body.
///
/// Cells are positional against `columns`: a short row renders fewer
/// cells and a long one is truncated to the column count, so the body can
/// never drift out of alignment with the header.
///
/// # Arguments
///
/// - `&[EuvTableColumn]` - The column definitions driving each cell's class.
/// - `EuvTableRow` - The row to render, consumed.
/// - `bool` - Whether this row gets the zebra fill.
///
/// # Returns
///
/// - `VirtualNode` - The table row virtual DOM tree.
fn table_row(columns: &[EuvTableColumn], row: EuvTableRow, zebra: bool) -> VirtualNode {
    let row_class: fn() -> &'static Css = if zebra {
        c_euv_table_row_zebra
    } else {
        c_euv_table_row
    };
    let cells: Vec<VirtualNode> = columns
        .iter()
        .enumerate()
        .map(|(index, column): (usize, &EuvTableColumn)| {
            let align_class: &'static str = table_cell_align_class(column.align);
            let text: &'static str = row.cells.get(index).copied().unwrap_or_default();
            html! {
                td {
                    class: c_euv_table_cell()
                    class: align_class
                    text
                }
            }
        })
        .collect();
    html! {
        tr {
            key: row.key
            class: row_class()
            cells
        }
    }
}

/// Returns the class modifier for a column's alignment.
///
/// `Left` yields the empty string so the base `c_euv_table_header_cell` /
/// `c_euv_table_cell` class stands alone; the class merge drops empty
/// segments, which keeps the rendered attribute free of padding spaces.
///
/// # Arguments
///
/// - `EuvTableAlign` - The alignment of the column.
///
/// # Returns
///
/// - `&'static str` - The alignment class name, or the empty string for `Left`.
fn table_cell_align_class(align: EuvTableAlign) -> &'static str {
    match align {
        EuvTableAlign::Left => "",
        EuvTableAlign::Center => c_euv_table_cell_center().get_name(),
        EuvTableAlign::Right => c_euv_table_cell_right().get_name(),
    }
}
