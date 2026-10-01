use super::*;

/// An IntersectionObserver demo page showcasing viewport intersection detection.
///
/// Demonstrates how to observe a container element for viewport intersection
/// changes, logging intersection ratio and child item visibility.
///
/// # Returns
///
/// - `VirtualNode` - The observer demo page virtual DOM tree.
///
/// # Arguments
///
/// - `VirtualNode<PageObserverProps>` - The `node` argument.
///
#[component]
pub(crate) fn page_observer(node: VirtualNode<PageObserverProps>) -> VirtualNode {
    let PageObserverProps: PageObserverProps = node.try_get_props().unwrap_or_default();
    use_intersection_observer("[data-observer-container]");
    html! {
        div {
            class: c_page_container()
            euv_header {
                icon: "👁️"
                title: "Observer"
                subtitle: "Detect when elements enter or leave the viewport using the IntersectionObserver API. Open the browser console to see intersection events."
            }
            euv_card {
                title: "Intersection Observer"
                p {
                    class: c_demo_text()
                    "The container below is observed for viewport intersection changes."
                }
                p {
                    class: c_demo_text_muted()
                    "Open the browser console to see intersection events logged as you scroll."
                }
                ul {
                    class: c_list_ul()
                    data-observer-container: "true"
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
