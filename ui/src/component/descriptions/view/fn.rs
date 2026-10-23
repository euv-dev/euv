use super::*;

/// A key/value detail grid laid out in columns.
///
/// Renders a `c_euv_descriptions` grid holding one `c_euv_descriptions_item`
/// cell per entry, each split into a `c_euv_descriptions_label` line and a
/// `c_euv_descriptions_value` line. The column count is not a class but a
/// CSS custom property: the grid rule reads
/// `repeat(var(--descriptions-columns, 1), minmax(0, 1fr))`, so the
/// component writes `--descriptions-columns` onto the grid element through
/// the inline `style:` attribute. A column count of zero falls back to a
/// single column, since a zero repeat count is not a valid grid track.
///
/// # Arguments
///
/// - `VirtualNode<EuvDescriptionsProps>` - The props node containing the entries and the column count.
///
/// # Returns
///
/// - `VirtualNode` - The descriptions grid virtual DOM tree.
#[component]
pub fn euv_descriptions(node: VirtualNode<EuvDescriptionsProps>) -> VirtualNode {
    let EuvDescriptionsProps { items, columns }: EuvDescriptionsProps =
        node.try_get_props().unwrap_or_default();
    let column_count: usize = if columns == 0 { 1 } else { columns };
    let columns_style: String = format!("--descriptions-columns: {column_count}");
    let columns_style_ref: &str = columns_style.as_str();
    let cells: Vec<VirtualNode> = items.into_iter().map(descriptions_item).collect();
    html! {
        div {
            class: c_euv_descriptions()
            style: columns_style_ref
            cells
        }
    }
}

/// Renders one grid cell: the label line over the value line.
///
/// # Arguments
///
/// - `EuvDescriptionsItem` - The entry to render, consumed.
///
/// # Returns
///
/// - `VirtualNode` - The grid cell virtual DOM tree.
fn descriptions_item(item: EuvDescriptionsItem) -> VirtualNode {
    html! {
        div {
            key: item.label
            class: c_euv_descriptions_item()
            div {
                class: c_euv_descriptions_label()
                item.label
            }
            div {
                class: c_euv_descriptions_value()
                item.value
            }
        }
    }
}
