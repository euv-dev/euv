use super::*;

/// A vertical event log with a continuous rail connecting each entry.
///
/// Renders a `c_euv_timeline` column holding one `c_euv_timeline_item` per
/// entry. Each item is anchored by a `c_euv_timeline_marker` dot and a
/// `c_euv_timeline_line` rail drawn down to the next entry; the rail is
/// omitted on the final item so the column ends on the last marker
/// instead of trailing a dangling stub. The `c_euv_timeline_content`
/// column holds the title, the optional description, and the optional
/// `c_euv_timeline_time` timestamp; the two optional fields are each
/// skipped when empty.
///
/// # Arguments
///
/// - `VirtualNode<EuvTimelineProps>` - The props node containing the event items.
///
/// # Returns
///
/// - `VirtualNode` - The timeline virtual DOM tree.
#[component]
pub fn euv_timeline(node: VirtualNode<EuvTimelineProps>) -> VirtualNode {
    let EuvTimelineProps { items }: EuvTimelineProps = node.try_get_props().unwrap_or_default();
    let total: usize = items.len();
    let mut entries: Vec<VirtualNode> = Vec::with_capacity(total);
    for (index, item) in items.into_iter().enumerate() {
        entries.push(euv_timeline_item(item, index + 1 < total));
    }
    html! {
        div {
            class: c_euv_timeline()
            entries
        }
    }
}

/// Renders one timeline entry: marker, connecting rail, and content column.
///
/// # Arguments
///
/// - `EuvTimelineItem` - The entry to render, consumed.
/// - `bool` - Whether a rail should be drawn below this entry.
///
/// # Returns
///
/// - `VirtualNode` - The timeline entry virtual DOM tree.
fn euv_timeline_item(item: EuvTimelineItem, has_line: bool) -> VirtualNode {
    html! {
        div {
            key: item.title
            class: c_euv_timeline_item()
            div {
                class: c_euv_timeline_marker()
            }
            // `& true` keeps the condition a non-path expression so the `html!`
            // auto-unwrap heuristic skips it. A bare `has_line` would be
            // rewritten to `has_line.get()` and fail to compile, because
            // `has_line` is a plain `bool`.
            if { has_line & true } {
                div {
                    class: c_euv_timeline_line()
                }
            }
            div {
                class: c_euv_timeline_content()
                div {
                    class: c_euv_timeline_title()
                    item.title
                }
                if { !item.description.is_empty() } {
                    div {
                        class: c_euv_timeline_desc()
                        item.description
                    }
                }
                if { !item.time.is_empty() } {
                    div {
                        class: c_euv_timeline_time()
                        item.time
                    }
                }
            }
        }
    }
}
