use super::*;

/// A 404 not found page component.
///
/// # Arguments
///
/// - `VirtualNode<PageNotFoundProps>` - The page component node carrying the
///   page props.
///
/// # Returns
///
/// - `VirtualNode` - The 404 page virtual DOM tree.
#[component]
pub(crate) fn page_not_found(node: VirtualNode<PageNotFoundProps>) -> VirtualNode {
    let PageNotFoundProps: PageNotFoundProps = node.try_get_props().unwrap_or_default();
    html! {
        div {
            class: c_page_container()
            euv_header {
                icon: "🔍"
                title: NOT_FOUND_HEADER_TITLE
                subtitle: NOT_FOUND_HEADER_SUBTITLE
            }
            euv_card {
                title: NOT_FOUND_NAV_CARD_TITLE
                div {
                    class: c_button_controls()
                    euv_button {
                        variant: EuvButtonVariant::Primary
                        label: NOT_FOUND_BACK_HOME_LABEL
                        onclick: not_found_on_go_home()
                    }
                }
            }
        }
    }
}
