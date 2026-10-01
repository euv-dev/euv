use super::*;

/// A 404 not found page component.
///
/// # Returns
///
/// - `VirtualNode` - The 404 page virtual DOM tree.
///
/// # Arguments
///
/// - `VirtualNode<PageNotFoundProps>` - The `node` argument.
///
#[component]
pub(crate) fn page_not_found(node: VirtualNode<PageNotFoundProps>) -> VirtualNode {
    let PageNotFoundProps: PageNotFoundProps = node.try_get_props().unwrap_or_default();
    html! {
        div {
            class: c_page_container()
            euv_header {
                icon: "🔍"
                title: "404 Not Found"
                subtitle: "The page you are looking for does not exist or has been moved."
            }
            euv_card {
                title: "Navigation"
                div {
                    class: c_button_controls()
                    euv_button {
                        variant: EuvButtonVariant::Primary
                        label: "Back to Home"
                        onclick: not_found_on_go_home()
                    }
                }
            }
        }
    }
}
