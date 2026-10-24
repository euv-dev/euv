use super::*;

/// The site footer: one row of text and links.
///
/// Every part of the content comes from the caller: the text with its
/// own destination — a license line that opens the project's repository
/// — and the items, each with its own text, its own destination and its
/// own optional icon.
///
/// The row is a single line: an item that does not fit is cut with an
/// ellipsis instead of wrapping — a filing number broken across two
/// lines is a number nobody can read or copy.
///
/// # Arguments
///
/// - `VirtualNode<EuvFooterProps>` - The props node.
///
/// # Returns
///
/// - `VirtualNode` - The footer virtual DOM tree.
#[component]
pub fn euv_footer(node: VirtualNode<EuvFooterProps>) -> VirtualNode {
    let EuvFooterProps { text, link, items }: EuvFooterProps =
        node.try_get_props().unwrap_or_default();
    html! {
        footer {
            class: c_euv_footer()
            if { !text.is_empty() } {
                a {
                    class: c_euv_footer_item_link()
                    href: link
                    target: LINK_TARGET_BLANK
                    rel: LINK_REL_NOOPENER
                    {
                        text
                    }
                }
            }
            for item in items.iter() {
                a {
                    class: c_euv_footer_item_link()
                    href: item.link
                    target: LINK_TARGET_BLANK
                    rel: LINK_REL_NOOPENER
                    footer_item_icon(item)
                    {
                        item.text
                    }
                }
            }
        }
    }
}

/// Renders the optional icon of a footer item.
///
/// An item without an icon renders nothing, so the caller does not
/// have to branch on the icon's presence.
///
/// # Arguments
///
/// - `&EuvFooterItem` - The item to read the icon from.
///
/// # Returns
///
/// - `VirtualNode` - The icon element, or an empty node.
fn footer_item_icon(item: &EuvFooterItem) -> VirtualNode {
    if item.icon.is_empty() {
        return html! {
            ""
        };
    }
    html! {
        img {
            class: c_euv_footer_item_icon()
            src: item.icon
            alt: item.alt
        }
    }
}
