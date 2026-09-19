use super::*;

/// Hash routing prefix used by `Router::link_handler`. Inlined here
/// because `euv_ui::Router` keeps `ROUTE_HASH_PREFIX` `pub(crate)`,
/// which is invisible from the docs-site crate.
const ROUTE_HASH_PREFIX: &str = "#";

/// Renders the home page out of euv-ui components: `euv_hero` for the
/// title block and a custom feature grid that wraps each feature card
/// in an `<a>` (when a `link` is configured) so clicks navigate.
///
/// The custom grid is rendered inline (instead of delegating to
/// `euv_feature_grid`) because that component does not accept a link
/// on `EuvFeature` — the docs site needs each card to be clickable.
///
/// The `icon` field is hidden when empty or equal to the placeholder
/// string `"blog"` so the original VuePress-driven home (which used
/// `icon: blog` on every card to mean "no icon") does not leak the
/// literal text into the rendered card.
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
            docs_feature_grid {
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

/// Renders one feature card. When the feature has a `link`, the entire
/// card is wrapped in an `<a>` so the whole tile is clickable.
///
/// The wrapper classes (`c_docs_feature_card`, `c_docs_feature_card_inner`,
/// etc.) are defined as raw CSS rules in the site's local override
/// block (`euv_docs::lib`) and emitted via `Css::inject_css`.
///
/// # Arguments
///
/// - `VirtualNode<DocsFeatureProps>` - The props carrying the feature.
///
/// # Returns
///
/// - `VirtualNode` - The rendered card virtual DOM tree.
#[component]
pub(crate) fn docs_feature_card(node: VirtualNode<DocsFeatureProps>) -> VirtualNode {
    let DocsFeatureProps { feature }: DocsFeatureProps = node.try_get_props().unwrap_or_default();
    let show_icon: bool = !feature.icon.is_empty() && feature.icon != "blog";
    let inner: VirtualNode = html! {
        div {
            class: "c_docs_feature_card_inner"
            if { show_icon == true } {
                div {
                    class: "c_docs_feature_card_icon"
                    {
                        feature.icon
                    }
                }
            }
            div {
                class: "c_docs_feature_card_title"
                {
                    feature.title
                }
            }
            div {
                class: "c_docs_feature_card_details"
                {
                    feature.details
                }
            }
        }
    };
    if feature.link.is_empty() {
        html! {
            div {
                class: "c_docs_feature_card"
                key: feature.title
                inner
            }
        }
    } else if feature.link.starts_with("http") {
        html! {
            a {
                class: "c_docs_feature_card"
                key: feature.title
                href: feature.link
                target: "_blank"
                rel: "noopener noreferrer"
                onclick: Router::external_link_handler(feature.link)
                inner
            }
        }
    } else {
        html! {
            a {
                class: "c_docs_feature_card"
                key: feature.title
                href: {
                    let mut
                    href: String = String::with_capacity(
                    ROUTE_HASH_PREFIX.len() + feature.link.len(),
                    );
                    href.push_str(ROUTE_HASH_PREFIX);
                    href.push_str(feature.link);
                    href
                }
                onclick: Router::link_handler(feature.link)
                inner
            }
        }
    }
}

/// Renders a grid of feature cards. When `features` is empty, renders
/// nothing.
///
/// # Arguments
///
/// - `VirtualNode<DocsFeatureGridProps>` - The props carrying the
///   feature list.
///
/// # Returns
///
/// - `VirtualNode` - The grid virtual DOM tree.
#[component]
pub(crate) fn docs_feature_grid(node: VirtualNode<DocsFeatureGridProps>) -> VirtualNode {
    let DocsFeatureGridProps { features }: DocsFeatureGridProps =
        node.try_get_props().unwrap_or_default();
    if features.is_empty() {
        return html! {
            ""
        };
    }
    html! {
        div {
            class: "c_docs_feature_grid"
            for feature in features.iter() {
                docs_feature_card {
                    feature: *feature
                }
            }
        }
    }
}