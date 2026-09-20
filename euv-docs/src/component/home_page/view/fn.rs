use super::*;

/// Hash routing prefix for `Router::link_handler`. Inlined here because
/// `euv_ui::Router` exposes `ROUTE_HASH_PREFIX` as `pub(crate)`.
const ROUTE_HASH_PREFIX: &str = "#";

/// Renders the home page: `euv_hero` for the title block + a custom
/// feature grid that wraps each card in an `<a>` (when a `link` is set).
///
/// The custom grid is rendered inline (instead of delegating to
/// `euv_feature_grid`) because that component does not accept a link
/// on `EuvFeature`. The `icon` field is hidden when empty or equal to
/// the placeholder string `"blog"` so it does not leak as literal text.
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
            docs_stats_row {
                stats: page.stats
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

/// Renders the home-page stat tiles (icon + value + label) in one row,
/// mirroring the euv example home stats section. Empty stats render
/// nothing.
#[component]
pub(crate) fn docs_stats_row(node: VirtualNode<DocsStatsRowProps>) -> VirtualNode {
    let DocsStatsRowProps { stats }: DocsStatsRowProps = node.try_get_props().unwrap_or_default();
    if stats.is_empty() {
        return html! {
            ""
        };
    }
    html! {
        div {
            class: c_home_stats()
            for stat in stats.iter() {
                div {
                    class: c_home_stat_card()
                    key: stat.label
                    div {
                        class: c_home_stat_icon()
                        {
                            stat.icon
                        }
                    }
                    div {
                        class: c_home_stat_value()
                        {
                            stat.value
                        }
                    }
                    div {
                        class: c_home_stat_label()
                        {
                            stat.label
                        }
                    }
                }
            }
        }
    }
}

/// Renders one feature card. When the feature has a `link`, the entire
/// card is wrapped in an `<a>` so the whole tile is clickable.
#[component]
pub(crate) fn docs_feature_card(node: VirtualNode<DocsFeatureProps>) -> VirtualNode {
    let DocsFeatureProps { feature }: DocsFeatureProps = node.try_get_props().unwrap_or_default();
    let show_icon: bool = !feature.icon.is_empty() && feature.icon != "blog";
    let has_icon: fn() -> bool = if show_icon { || true } else { || false };
    let inner: VirtualNode = html! {
        div {
            class: "c_docs_feature_card_inner"
            if { has_icon() } {
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

/// Renders a grid of feature cards. Empty grid renders nothing.
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
