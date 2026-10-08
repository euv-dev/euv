use super::*;

/// An IntersectionObserver demo page showcasing viewport intersection detection.
///
/// Demonstrates how to observe a container element for viewport intersection
/// changes, logging intersection ratio and child item visibility.
///
/// # Arguments
///
/// - `VirtualNode<PageObserverProps>` - The page component node carrying the
///   page props.
///
/// # Returns
///
/// - `VirtualNode` - The observer demo page virtual DOM tree.
#[component]
pub(crate) fn page_observer(node: VirtualNode<PageObserverProps>) -> VirtualNode {
    let PageObserverProps: PageObserverProps = node.try_get_props().unwrap_or_default();
    use_intersection_observer(OBSERVER_CONTAINER_SELECTOR);
    html! {
        div {
            class: c_page_container()
            euv_header {
                icon: "👁️"
                title: OBSERVER_HEADER_TITLE
                subtitle: OBSERVER_HEADER_SUBTITLE
            }
            euv_card {
                title: OBSERVER_INTERSECTION_CARD_TITLE
                p {
                    class: c_demo_text()
                    OBSERVER_CONTAINER_DESC
                }
                p {
                    class: c_demo_text_muted()
                    OBSERVER_CONSOLE_HINT
                }
                ul {
                    class: c_list_ul()
                    data-observer-container: OBSERVER_CONTAINER_ATTR_VALUE
                    for index in 0..100 {
                        li {
                            key: index.to_string()
                            class: c_list_item()
                            data_index: index.to_string()
                            span {
                                class: c_list_item_text()
                                format!("Observed Item {}", index + 1)
                            }
                        }
                    }
                }
            }
        }
    }
}
