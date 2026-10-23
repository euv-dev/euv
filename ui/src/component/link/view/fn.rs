use super::*;

/// A text link.
///
/// Renders an anchor when a destination is given and a plain span when
/// it is not, so a link that only triggers an action does not pretend
/// to be navigable. A disabled link carries `c_euv_link_disabled` and
/// takes no click.
///
/// # Arguments
///
/// - `VirtualNode<EuvLinkProps>` - The props node containing the link
///   configuration.
///
/// # Returns
///
/// - `VirtualNode` - The link virtual DOM tree.
#[component]
pub fn euv_link(node: VirtualNode<EuvLinkProps>) -> VirtualNode {
    let EuvLinkProps {
        text,
        href,
        disabled,
        on_click,
    }: EuvLinkProps = node.try_get_props().unwrap_or_default();
    let navigable: bool = !href.is_empty() && !disabled;
    if navigable {
        return html! {
            a {
                class: c_euv_link()
                href: href
                onclick: on_click
                text
            }
        };
    }
    html! {
        span {
            class: if disabled {
                c_euv_link_disabled()
            } else {
                c_euv_link()
            }
            text
        }
    }
}
