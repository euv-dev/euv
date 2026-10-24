use super::*;

/// The site footer: one line of text plus a row of links.
///
/// The link row is where a filing notice lives — the number as plain,
/// selectable text, the official badge as an image, and the destination
/// pointing at the authority's own query page.
///
/// The two rows are always rendered and the empty one is hidden by its
/// class, so the footer never reserves the space of a row it does not
/// use.
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
    let EuvFooterProps { text, links }: EuvFooterProps = node.try_get_props().unwrap_or_default();
    html! {
        footer {
            class: c_euv_footer()
            div {
                class: c_euv_footer_text()
                {
                    text
                }
            }
            div {
                class: if links.is_empty() {
                    c_euv_footer_links_hidden()
                } else {
                    c_euv_footer_links()
                }
                for link in links.iter() {
                    a {
                        class: c_euv_footer_link()
                        href: link.link
                        target: LINK_TARGET_BLANK
                        rel: LINK_REL_NOOPENER
                        if { !link.icon.is_empty() } {
                            img {
                                class: c_euv_footer_link_icon()
                                src: link.icon
                                alt: link.alt
                            }
                        }
                        {
                            link.text
                        }
                    }
                }
            }
        }
    }
}
