use super::*;

/// A virtual list demo page showcasing high-performance rendering of 10,000 items.
///
/// Uses a windowed rendering strategy that only creates DOM nodes for the
/// currently visible items plus a small overscan buffer, keeping the DOM
/// node count constant regardless of total list size.
/// # Arguments
///
/// - `VirtualNode<PageVirtualListProps>` - The component props node.
///
/// # Returns
///
/// - `VirtualNode` - The virtual list demo page virtual DOM tree.
///
/// # Arguments
///
/// - `VirtualNode<PageVirtualListProps>` - The `node` argument.
///
#[component]
pub(crate) fn page_virtual_list(node: VirtualNode<PageVirtualListProps>) -> VirtualNode {
    let PageVirtualListProps: PageVirtualListProps = node.try_get_props().unwrap_or_default();
    let visible_range: Signal<(usize, usize)> = App::use_signal(|| (0, 0));
    let item_renderer: VirtualListItemRenderer = Rc::new(|index: usize| {
        html! {
            div {
                class: c_virtual_list_row()
                span {
                    class: c_virtual_list_row_index()
                    index.to_string()
                }
                span {
                    class: c_virtual_list_row_label()
                    format!("Item #{index}")
                }
                span {
                    class: c_virtual_list_row_description()
                    format!("Description for item {index}")
                }
            }
        }
    });
    let on_visible_range_change: Option<VirtualListRangeHandler> = {
        let visible_range: Signal<(usize, usize)> = visible_range;
        Some(Rc::new(move |range: (usize, usize)| {
            visible_range.set(range);
        }))
    };
    let (visible_start, visible_end): (usize, usize) = visible_range.get();
    let visible_count: usize = visible_end.saturating_sub(visible_start);
    let virtual_list_config: VirtualListConfig = VirtualListConfig::new(
        String::from(VIRTUAL_LIST_DEMO_ID),
        VIRTUAL_LIST_DEMO_TOTAL_COUNT,
        VIRTUAL_LIST_DEMO_ITEM_HEIGHT,
        VIRTUAL_LIST_DEMO_OVERSCAN_COUNT,
    );
    html! {
        div {
            class: c_page_container()
            euv_header {
                icon: "📊"
                title: "Virtual List"
                subtitle: "High-performance windowed list rendering 10000 items with minimal DOM nodes. Only visible rows are rendered."
            }
            div {
                class: c_virtual_list_card()
                h3 {
                    class: c_card_title()
                    format!("{VIRTUAL_LIST_DEMO_TOTAL_COUNT} Items Virtual Scroll")
                }
                div {
                    class: c_virtual_list_status()
                    span {
                        class: c_virtual_list_status_item()
                        "Total: "
                        span {
                            class: c_virtual_list_status_value()
                            VIRTUAL_LIST_DEMO_TOTAL_COUNT.to_string()
                        }
                    }
                    span {
                        class: c_virtual_list_status_item()
                        "Visible: "
                        span {
                            class: c_virtual_list_status_value()
                            visible_count.to_string()
                        }
                    }
                    span {
                        class: c_virtual_list_status_item()
                        "Range: "
                        span {
                            class: c_virtual_list_status_value()
                            format!("{visible_start}-{}", visible_end.saturating_sub(1))
                        }
                    }
                }
                euv_virtual_list {
                    config: virtual_list_config
                    item_renderer: item_renderer
                    on_visible_range_change: on_visible_range_change
                }
            }
        }
    }
}
