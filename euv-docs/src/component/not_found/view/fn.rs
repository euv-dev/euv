use super::*;

/// Renders the 404 page with the euv-ui `euv_result` component.
///
/// # Arguments
///
/// - `DocsPageProps` - The typed props containing the route signal.
///
/// # Returns
///
/// - `VirtualNode` - The 404 virtual DOM tree.
#[component]
pub(crate) fn docs_not_found(node: VirtualNode<DocsPageProps>) -> VirtualNode {
    let DocsPageProps { route_signal }: DocsPageProps = node.try_get_props().unwrap_or_default();
    let (path, _anchor) = parse_route(&route_signal.get());
    let locale: &DocsLocale = locale_of(&path);
    html! {
        div {
            class: c_page_container()
            euv_result {
                code: "404"
                description: "Page not found"
                a {
                    class: c_home_btn_primary()
                    href: format!("#{}", locale.prefix)
                    onclick: Router::link_handler(locale.prefix)
                    "Home"
                }
            }
        }
    }
}
