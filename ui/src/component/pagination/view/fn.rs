use super::*;

/// A generic prev/next pagination footer aligned with common docs frameworks.
///
/// Renders two bordered link cards; a missing side collapses into a spacer so
/// the remaining side keeps its alignment.
///
/// # Arguments
///
/// - `VirtualNode<EuvPaginationProps>` - The props node.
///
/// # Returns
///
/// - `VirtualNode` - The pagination virtual DOM tree.
#[component]
pub fn euv_pagination(node: VirtualNode<EuvPaginationProps>) -> VirtualNode {
    let EuvPaginationProps {
        prev_label,
        next_label,
        prev,
        next,
    }: EuvPaginationProps = node.try_get_props().unwrap_or_default();
    let prev_node: VirtualNode = pagination_side(prev, prev_label, false);
    let next_node: VirtualNode = pagination_side(next, next_label, true);
    html! {
        div {
            role: "navigation"
            aria-label: "pagination"
            class: c_euv_pagination()
            prev_node
            next_node
        }
    }
}

/// Renders one side of the pagination footer.
///
/// # Arguments
///
/// - `Option<EuvPaginationItem>` - The entry; `None` renders a spacer.
/// - `&'static str` - The small uppercase label above the entry text.
/// - `bool` - Whether this is the right-aligned next side.
///
/// # Returns
///
/// - `VirtualNode` - The side virtual DOM tree.
fn pagination_side(
    item: Option<EuvPaginationItem>,
    label: &'static str,
    is_next: bool,
) -> VirtualNode {
    let Some(item) = item else {
        return html! {
            div {
                class: c_euv_pagination_spacer()
            }
        };
    };
    if is_next {
        return html! {
            a {
                class: c_euv_pagination_link()
                class: c_euv_pagination_next()
                href: {
                    let mut
                    href: String =
                    String::with_capacity(ROUTE_HASH_PREFIX.len() + item.link.len());
                    href.push_str(ROUTE_HASH_PREFIX);
                    href.push_str(item.link);
                    href
                }
                onclick: Router::link_handler(item.link)
                span {
                    class: c_euv_pagination_label()
                    {
                        label
                    }
                }
                span {
                    class: c_euv_pagination_text()
                    {
                        item.text
                    }
                }
            }
        };
    }
    html! {
        a {
            class: c_euv_pagination_link()
            href: {
                let mut
                href: String =
                String::with_capacity(ROUTE_HASH_PREFIX.len() + item.link.len());
                href.push_str(ROUTE_HASH_PREFIX);
                href.push_str(item.link);
                href
            }
            onclick: Router::link_handler(item.link)
            span {
                class: c_euv_pagination_label()
                {
                    label
                }
            }
            span {
                class: c_euv_pagination_text()
                {
                    item.text
                }
            }
        }
    }
}
