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
/// Every element carries the `data-slot` name the reference publishes, so
/// the parts of the table can be addressed from the DOM the same way the
/// reference addresses them.
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
        class: custom_class,
    }: EuvTableProps = node.try_get_props().unwrap_or_default();
    let children: VirtualNode = node.get_children().into();
    let has_custom_content: bool = !matches!(&children, VirtualNode::Empty);
    let header_row: VirtualNode = table_header_row(columns.as_slice());
    let body_rows: Vec<VirtualNode> = table_body_rows(columns.as_slice(), rows, zebra);
    let base_class: &Css = c_euv_table();
    html! {
        div {
            "data-slot": "table-container"
            class: c_euv_table_container()
            table {
                "data-slot": "table"
                class: base_class
                class: custom_class
                if has_custom_content {
                    children
                } else {
                    if !caption.is_empty() {
                        euv_table_caption {
                            text: caption
                        }
                    }
                    euv_table_header {
                        header_row
                    }
                    euv_table_body {
                        body_rows
                    }
                }
            }
        }
    }
}

/// Builds the header row holding one heading cell per column.
///
/// The row is a `table-row` part and the cells in it are `table-head`
/// parts, so the header reads as a part of the table rather than as an
/// anonymous block of headings.
///
/// # Arguments
///
/// - `&[EuvTableColumn]` - The column definitions, in display order.
///
/// # Returns
///
/// - `VirtualNode` - The header row virtual DOM tree.
fn table_header_row(columns: &[EuvTableColumn]) -> VirtualNode {
    let header_cells: Vec<VirtualNode> = columns
        .iter()
        .map(|column: &EuvTableColumn| {
            let base_class: &Css = c_euv_table_header_cell();
            let align_class: &'static str = table_cell_align_class(column.align);
            let align_shadcn_class: &Css = {
                match column.align {
                    EuvTableAlign::Left => &EMPTY_CLASS,
                    EuvTableAlign::Center => c_euv_table_header_cell_center(),
                    EuvTableAlign::Right => c_euv_table_header_cell_right(),
                }
            };
            // The tree may be written again, so the label is copied out of
            // the column rather than borrowed from it.
            let column_key: String = column.key.to_string();
            let column_label: ::euv::CowText = column.label.clone();
            html! {
                th {
                    key: column_key.clone()
                    "data-slot": "table-head"
                    class: base_class
                    class: align_class
                    class: align_shadcn_class
                    {column_label.clone()}
                }
            }
        })
        .collect();
    html! {
        tr {
            "data-slot": "table-row"
            header_cells
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
    let row_base_class: &Css = if zebra {
        c_euv_table_row_zebra()
    } else {
        c_euv_table_row()
    };
    let row_shadcn_class: &Css = { c_euv_table_row() };
    let zebra_shadcn_class: &Css = if zebra {
        c_euv_table_row_zebra()
    } else {
        &EMPTY_CLASS
    };
    let cells: Vec<VirtualNode> = columns
        .iter()
        .enumerate()
        .map(|(index, column): (usize, &EuvTableColumn)| {
            let base_class: &Css = c_euv_table_cell();
            let align_class: &'static str = table_cell_align_class(column.align);
            let align_shadcn_class: &Css = {
                match column.align {
                    EuvTableAlign::Left => &EMPTY_CLASS,
                    EuvTableAlign::Center => c_euv_table_cell_center(),
                    EuvTableAlign::Right => c_euv_table_cell_right(),
                }
            };
            let text: ::euv::CowText = row
                .cells
                .get(index)
                .cloned()
                .unwrap_or_else(|| ::euv::CowText::from_text(""));
            html! {
                td {
                    "data-slot": "table-cell"
                    class: base_class
                    class: align_class
                    class: align_shadcn_class
                    {text.clone()}
                }
            }
        })
        .collect();
    html! {
        tr {
            key: row.key.clone().as_str()
            "data-slot": "table-row"
            class: row_base_class
            class: row_shadcn_class
            class: zebra_shadcn_class
            cells
        }
    }
}

/// Builds the body rows of the table.
///
/// The rows are the `table-row` parts of the body and the cells inside
/// them are the `table-cell` parts.
///
/// # Arguments
///
/// - `&[EuvTableColumn]` - The column definitions driving each row's cells.
/// - `Vec<EuvTableRow>` - The body rows, consumed in order.
/// - `bool` - Whether every other row is zebra striped.
///
/// # Returns
///
/// - `Vec<VirtualNode>` - The body rows, in display order.
fn table_body_rows(
    columns: &[EuvTableColumn],
    rows: Vec<EuvTableRow>,
    zebra: bool,
) -> Vec<VirtualNode> {
    rows.into_iter()
        .enumerate()
        .map(|(index, row): (usize, EuvTableRow)| table_row(columns, row, zebra && index % 2 == 0))
        .collect()
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

/// The section of a table that holds its heading cells.
///
/// The section is the `table-header` part of the reference: it introduces
/// the columns of the table and it carries the heading row the reference
/// puts inside it.
///
/// # Arguments
///
/// - `VirtualNode<EuvTableHeaderProps>` - The props node carrying the
///   component settings.
///
/// # Returns
///
/// - `VirtualNode` - The euv table header virtual DOM tree.
#[component]
pub fn euv_table_header(node: VirtualNode<EuvTableHeaderProps>) -> VirtualNode {
    let EuvTableHeaderProps {}: EuvTableHeaderProps = node.try_get_props().unwrap_or_default();
    let children: VirtualNode = node.get_children().into();
    let base_class: &Css = c_euv_table_head();
    html! {
        thead {
            "data-slot": "table-header"
            class: base_class
            children
        }
    }
}

/// The section of a table that holds its body rows.
///
/// The section is the `table-body` part of the reference: it carries the
/// rows of the table and the cells inside them.
///
/// # Arguments
///
/// - `VirtualNode<EuvTableBodyProps>` - The props node carrying the
///   component settings.
///
/// # Returns
///
/// - `VirtualNode` - The euv table body virtual DOM tree.
#[component]
pub fn euv_table_body(node: VirtualNode<EuvTableBodyProps>) -> VirtualNode {
    let EuvTableBodyProps {}: EuvTableBodyProps = node.try_get_props().unwrap_or_default();
    let children: VirtualNode = node.get_children().into();
    let base_class: &Css = c_euv_table_body();
    html! {
        tbody {
            "data-slot": "table-body"
            class: base_class
            children
        }
    }
}

/// The section of a table that holds its summary rows.
///
/// The footer is a `table-footer` part of the reference: it closes the
/// table with the rows that summarise it, and it carries the same row and
/// cell parts as the body.
///
/// # Arguments
///
/// - `VirtualNode<EuvTableFooterProps>` - The props node carrying the
///   component settings.
///
/// # Returns
///
/// - `VirtualNode` - The euv table footer virtual DOM tree.
#[component]
pub fn euv_table_footer(node: VirtualNode<EuvTableFooterProps>) -> VirtualNode {
    let EuvTableFooterProps {}: EuvTableFooterProps = node.try_get_props().unwrap_or_default();
    let children: VirtualNode = node.get_children().into();
    let base_class: &Css = c_euv_table_footer();
    html! {
        tfoot {
            "data-slot": "table-footer"
            class: base_class
            children
        }
    }
}

/// The caption of a table.
///
/// The caption names what the table shows, so it is read before the rows
/// and it stays attached to the table it describes.
///
/// # Arguments
///
/// - `VirtualNode<EuvTableCaptionProps>` - The props node carrying the
///   component settings.
///
/// # Returns
///
/// - `VirtualNode` - The euv table caption virtual DOM tree.
#[component]
pub fn euv_table_caption(node: VirtualNode<EuvTableCaptionProps>) -> VirtualNode {
    let EuvTableCaptionProps { text }: EuvTableCaptionProps =
        node.try_get_props().unwrap_or_default();
    let base_class: &Css = c_euv_table_caption();
    html! {
        caption {
            "data-slot": "table-caption"
            class: base_class
            text
        }
    }
}

/// One row of a table.
///
/// The row is a `table-row` part: it owns the cells of one line of the
/// table and it holds the heading or body cells of that line.
///
/// # Arguments
///
/// - `VirtualNode<EuvTableRowProps>` - The props node carrying the
///   component settings.
///
/// # Returns
///
/// - `VirtualNode` - The euv table row virtual DOM tree.
#[component]
pub fn euv_table_row(node: VirtualNode<EuvTableRowProps>) -> VirtualNode {
    let EuvTableRowProps {}: EuvTableRowProps = node.try_get_props().unwrap_or_default();
    let children: VirtualNode = node.get_children().into();
    html! {
        tr {
            "data-slot": "table-row"
            class: c_euv_table_row()
            children
        }
    }
}

/// One heading cell of a table.
///
/// The heading names the column it sits in, so it is written as a heading
/// and the column it names is the one the body cells under it belong to.
///
/// # Arguments
///
/// - `VirtualNode<EuvTableHeadProps>` - The props node carrying the
///   component settings.
///
/// # Returns
///
/// - `VirtualNode` - The euv table head virtual DOM tree.
#[component]
pub fn euv_table_head(node: VirtualNode<EuvTableHeadProps>) -> VirtualNode {
    let EuvTableHeadProps {}: EuvTableHeadProps = node.try_get_props().unwrap_or_default();
    let children: VirtualNode = node.get_children().into();
    let base_class: &Css = c_euv_table_header_cell();
    html! {
        th {
            "data-slot": "table-head"
            class: base_class
            children
        }
    }
}

/// One body cell of a table.
///
/// The cell holds the value of one column in one row, so it takes the
/// width the column gives it and wraps rather than pushing the row wider.
///
/// # Arguments
///
/// - `VirtualNode<EuvTableCellProps>` - The props node carrying the
///   component settings.
///
/// # Returns
///
/// - `VirtualNode` - The euv table cell virtual DOM tree.
#[component]
pub fn euv_table_cell(node: VirtualNode<EuvTableCellProps>) -> VirtualNode {
    let EuvTableCellProps {}: EuvTableCellProps = node.try_get_props().unwrap_or_default();
    let children: VirtualNode = node.get_children().into();
    let base_class: &Css = c_euv_table_cell();
    html! {
        td {
            "data-slot": "table-cell"
            class: base_class
            children
        }
    }
}
