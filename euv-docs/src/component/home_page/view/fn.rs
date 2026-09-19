use super::*;

/// Renders the home page out of euv-ui components: `euv_hero`,
/// `euv_feature_grid`, an optional `euv_markdown` body and the footer.
///
/// # Arguments
///
/// - `DocsPageProps` - The typed props containing the route signal.
///
/// # Returns
///
/// - `VirtualNode` - The home page virtual DOM tree.
#[component]
pub(crate) fn docs_home_page(node: VirtualNode<DocsPageProps>) -> VirtualNode {
    let DocsPageProps { route_signal }: DocsPageProps = node.try_get_props().unwrap_or_default();
    let (path, _anchor) = parse_route(&route_signal.get());
    let locale: &DocsLocale = locale_of(&path);
    let site: &DocsSite = &crate::generated::SITE;
    let Some(page) = find_page(&path) else {
        return html! {
            ""
        };
    };

    let hero_title: &str = if page.hero_text.is_empty() {
        if locale.title.is_empty() {
            site.title
        } else {
            locale.title
        }
    } else {
        page.hero_text
    };
    let footer_text: &str = if page.footer.is_empty() {
        locale.footer
    } else {
        page.footer
    };

    html! {
        div {
            class: c_page_container()
            euv_hero {
                title: hero_title
                subtitle: page.tagline
                actions: page.actions
            }
            euv_feature_grid {
                features: page.features
            }
            if { !page.blocks.is_empty() } {
                euv_markdown {
                    blocks: page.blocks
                }
            }
            if { !footer_text.is_empty() } {
                footer {
                    class: c_euv_footer()
                    {
                        footer_text
                    }
                }
            }
        }
    }
}
