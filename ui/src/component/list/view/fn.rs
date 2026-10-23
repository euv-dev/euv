use super::*;

/// A list of titled rows with optional descriptions and trailing meta text.
///
/// Renders a `c_euv_list` column holding one `c_euv_list_item` per entry.
/// Each row holds a `c_euv_list_item_body` column with the
/// `c_euv_list_item_title` and — when the description is not empty — the
/// `c_euv_list_item_desc` line underneath it; a non-empty meta string
/// renders as a `c_euv_list_item_meta` at the trailing edge of the row.
///
/// The rows are built before the `html!` block rather than with a `for`
/// loop inside it: the per-row class values are plain strings resolved
/// once per row, and the borrowed row list cannot satisfy the `'static`
/// bound the reactive attribute closure needs.
///
/// # Arguments
///
/// - `VirtualNode<EuvListProps>` - The props node containing the rows and the empty-state text.
///
/// # Returns
///
/// - `VirtualNode` - The list virtual DOM tree.
#[component]
pub fn euv_list(node: VirtualNode<EuvListProps>) -> VirtualNode {
    let EuvListProps { items, empty_text }: EuvListProps = node.try_get_props().unwrap_or_default();
    let mut rows: Vec<VirtualNode> = items.into_iter().map(list_item).collect();
    if rows.is_empty() && !empty_text.is_empty() {
        rows.push(html! {
            div {
                class: c_euv_list_empty()
                empty_text
            }
        });
    }
    html! {
        div {
            class: c_euv_list()
            rows
        }
    }
}

/// Renders one list row: the body column plus the optional trailing meta.
///
/// The description line is skipped when the description is empty, and the
/// meta block is skipped when the meta text is empty, so a one-line row
/// leaves no empty gap in the body column and no dangling block at the
/// trailing edge.
///
/// # Arguments
///
/// - `EuvListItem` - The row to render, consumed.
///
/// # Returns
///
/// - `VirtualNode` - The list row virtual DOM tree.
fn list_item(item: EuvListItem) -> VirtualNode {
    html! {
        div {
            key: item.title
            class: c_euv_list_item()
            div {
                class: c_euv_list_item_body()
                div {
                    class: c_euv_list_item_title()
                    item.title
                }
                if !item.desc.is_empty() {
                    div {
                        class: c_euv_list_item_desc()
                        item.desc
                    }
                }
            }
            if !item.meta.is_empty() {
                div {
                    class: c_euv_list_item_meta()
                    item.meta
                }
            }
        }
    }
}
