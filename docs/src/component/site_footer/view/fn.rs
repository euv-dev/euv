use super::*;

/// Renders the site-wide footer bar.
///
/// The bar is pinned to the bottom of the viewport and spans the full
/// width, the way the reference site carries its own footer. It holds
/// the license line — the locale's footer text, or the page's own
/// override — and the notices the site declares, each a link of its
/// own.
///
/// The content is resolved from the route, so the bar follows the
/// locale and the page without the shell having to thread the
/// resolved values into it.
///
/// # Arguments
///
/// - `VirtualNode<DocsSiteFooterProps>` - The props node carrying the
///   route signal.
///
/// # Returns
///
/// - `VirtualNode` - The footer bar virtual DOM tree.
#[component]
pub(crate) fn docs_site_footer(node: VirtualNode<DocsSiteFooterProps>) -> VirtualNode {
    let DocsSiteFooterProps { route_signal }: DocsSiteFooterProps =
        node.try_get_props().unwrap_or_default();
    let (path, _anchor) = parse_route(&route_signal.get());
    let locale: &DocsLocale = locale_of(&path);
    let site: &DocsSite = &crate::generated::SITE;
    let page_footer: &str = find_page(&path)
        .map(|page: &DocsPage| page.footer)
        .unwrap_or("");
    let text: &str = if page_footer.is_empty() {
        locale.footer
    } else {
        page_footer
    };
    html! {
        footer {
            class: c_docs_site_footer()
            if { !text.is_empty() } {
                a {
                    class: c_docs_site_footer_link()
                    href: site.footer_link
                    target: LINK_TARGET_BLANK
                    rel: LINK_REL_NOOPENER
                    {
                        text
                    }
                }
            }
            for item in site.footer_items.iter() {
                a {
                    class: c_docs_site_footer_link()
                    href: item.link
                    target: LINK_TARGET_BLANK
                    rel: LINK_REL_NOOPENER
                    if { !item.icon.is_empty() } {
                        img {
                            class: c_docs_site_footer_icon()
                            src: item.icon
                            alt: item.alt
                        }
                    }
                    {
                        item.text
                    }
                }
            }
        }
    }
}
