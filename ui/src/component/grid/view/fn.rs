use super::*;

/// A responsive column grid.
///
/// Renders a `c_euv_grid` row — or a `c_euv_grid_gutter` row when the row
/// spaces its columns with a gap — holding one `c_euv_grid_col` per child.
/// A column that is pushed to the right carries `c_euv_grid_col_offset`
/// instead of the plain column class.
///
/// A row is twenty-four columns wide, so a column's share of the row is
/// written inline as `flex: 0 0 {span*100/24}%;`, and its offset is written
/// the same way, as `margin-left: {offset*100/24}%;`. Both are computed with
/// integer arithmetic, so a column never carries a fractional width.
///
/// The columns are the caller's children: the first child is the first
/// column, and so on. The Nth entry of `cols` lays out the Nth child; a
/// child with no entry falls back to the default column, whose span is
/// clamped to a single column of the row so the row never collapses.
///
/// # Arguments
///
/// - `VirtualNode<EuvGridProps>` - The props node containing the column
///   layout and the gutter flag.
///
/// # Returns
///
/// - `VirtualNode` - The grid virtual DOM tree.
#[component]
pub fn euv_grid(node: VirtualNode<EuvGridProps>) -> VirtualNode {
    let EuvGridProps { cols, gutter }: EuvGridProps = node.try_get_props().unwrap_or_default();
    let children: Vec<VirtualNode> = node.get_children().to_vec();
    let columns: Vec<VirtualNode> = children
        .into_iter()
        .enumerate()
        .map(|(index, content): (usize, euv::VirtualNode)| {
            let col: EuvGridCol = cols.get(index).copied().unwrap_or_default();
            grid_column(col, content, gutter)
        })
        .collect();
    html! {
        div {
            class: if gutter {
                c_euv_grid_gutter()
            } else {
                c_euv_grid()
            }
            columns
        }
    }
}

/// Renders one grid column: the caller's content in a column of the row.
///
/// The column's share of the row and its offset are both written into the
/// column's inline style, because the class registry carries no per-column
/// sizes.
///
/// # Arguments
///
/// - `EuvGridCol` - The column's share of the row and its offset.
/// - `VirtualNode` - The column's content, consumed.
/// - `bool` - Whether the row spaces its columns.
///
/// # Returns
///
/// - `VirtualNode` - The column virtual DOM tree.
fn grid_column(col: EuvGridCol, content: VirtualNode, gutter: bool) -> VirtualNode {
    let span: u32 = col.span.clamp(1, 24);
    let width: u64 = u64::from(span) * 100 / 24;
    let offset: u64 = u64::from(col.offset) * 100 / 24;
    let style: String = if col.offset > 0 {
        format!("flex: 0 0 {width}%; margin-left: {offset}%;")
    } else {
        format!("flex: 0 0 {width}%;")
    };
    let style_ref: &str = style.as_str();
    let offset: bool = col.offset > 0;
    html! {
        div {
            class: match (gutter, offset) {
                (true, true) => c_euv_grid_gutter_col_offset(),
                (true, false) => c_euv_grid_gutter_col(),
                (false, true) => c_euv_grid_col_offset(),
                (false, false) => c_euv_grid_col(),
            }
            style: style_ref
            content
        }
    }
}
