use super::*;

/// Resolves localized footer props without rendering any document content.
///
/// # Arguments
///
/// - `&DocsLocale` - The current locale.
/// - `Option<&DocsPage>` - The matched page, if any.
/// - `&DocsSite` - The site carrying the footer link and notices.
///
/// # Returns
///
/// - `EuvFooterProps` - The page override or locale fallback plus site notices.
pub(crate) fn docs_footer_props(
    locale: &DocsLocale,
    page: Option<&DocsPage>,
    site: &DocsSite,
) -> EuvFooterProps {
    let text: &'static str = page
        .map(|page: &DocsPage| page.footer)
        .filter(|footer: &&str| !footer.is_empty())
        .unwrap_or(locale.footer);
    EuvFooterProps {
        text,
        link: site.footer_link,
        items: site.footer_items,
    }
}

/// Renders a localized footer in the consuming page's normal content flow.
///
/// # Arguments
///
/// - `VirtualNode<DocsSiteFooterProps>` - The props carrying the current route.
///
/// # Returns
///
/// - `VirtualNode` - The shared footer with the site's notices and links.
#[component]
pub(crate) fn docs_site_footer(node: VirtualNode<DocsSiteFooterProps>) -> VirtualNode {
    let DocsSiteFooterProps { route }: DocsSiteFooterProps =
        node.try_get_props().unwrap_or_default();
    let (path, _anchor) = parse_route(&route);
    let locale: &DocsLocale = locale_of(&path);
    let site: &DocsSite = &crate::generated::SITE;
    let footer: EuvFooterProps = docs_footer_props(locale, find_page(&path), site);
    html! {
        euv_footer {
            text: footer.text
            link: footer.link
            items: footer.items
        }
    }
}
