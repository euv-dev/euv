use super::*;

/// The desktop site header.
///
/// The reference documentation site opens with one full-width sticky row
/// above the sidebar and the content column: the brand on the left, the
/// top-level navigation links next to it, and the search field and theme
/// toggle on the right. The header sits outside the scrolling column, so
/// it stays put while the article scrolls.
///
/// # Arguments
///
/// - `&'static DocsLocale` - The current locale.
/// - `Signal<String>` - The current route signal.
/// - `Signal<String>` - The current theme name signal.
/// - `Signal<String>` - The search query signal.
/// - `Signal<bool>` - The responsive layout mode.
///
/// # Returns
///
/// - `VirtualNode` - The header virtual DOM tree.
pub(crate) fn docs_header(
    locale: &'static DocsLocale,
    route_signal: Signal<String>,
    theme_signal: Signal<String>,
    search_query: Signal<String>,
    mobile_signal: Signal<bool>,
) -> VirtualNode {
    html! {
        header {
            class: CLASS_DOCS_HEADER
            docs_header_brand(locale)
            nav {
                class: CLASS_DOCS_HEADER_LINKS
                for item in locale.navbar.iter() {
                    docs_header_link(item, locale.prefix, route_signal)
                }
            }
            div {
                class: CLASS_DOCS_HEADER_ACTIONS
                docs_header_search(route_signal, search_query, mobile_signal)
                button {
                    class: CLASS_DOCS_HEADER_ICON_BUTTON
                    title: locale_text(&route_signal.get(), DOCS_KEY_THEME_TOGGLE)
                    "aria-label": locale_text(&route_signal.get(), DOCS_KEY_THEME_TOGGLE)
                    onclick: ThemeState::toggle(theme_signal)
                    theme_icon_node(theme_signal)
                }
            }
        }
    }
}

/// Renders the brand block: the logo and the site title, linking home.
///
/// # Arguments
///
/// - `&'static DocsLocale` - The current locale.
///
/// # Returns
///
/// - `VirtualNode` - The brand virtual DOM tree.
fn docs_header_brand(locale: &'static DocsLocale) -> VirtualNode {
    let site: &DocsSite = &crate::generated::SITE;
    let title: &str = if locale.title.is_empty() {
        site.title
    } else {
        locale.title
    };
    html! {
        a {
            class: CLASS_DOCS_HEADER_BRAND
            href: format!("#{}", locale.prefix)
            onclick: Router::link_handler(locale.prefix)
            euv_logo {
                variant: LogoButtonVariant::Nav
            }
            span {
                class: CLASS_DOCS_HEADER_BRAND_TITLE
                {
                    title
                }
            }
        }
    }
}

/// Renders one top-level navigation link.
///
/// An external link leaves the site and opens in a new tab; an internal one
/// is routed in place and is marked active while the reader is inside its
/// section.
///
/// # Arguments
///
/// - `&EuvNavbarItem` - The navigation item.
/// - `&str` - The locale homepage route.
/// - `Signal<String>` - The current route signal.
///
/// # Returns
///
/// - `VirtualNode` - The link virtual DOM tree.
fn docs_header_link(
    item: &EuvNavbarItem,
    locale_root: &str,
    route_signal: Signal<String>,
) -> VirtualNode {
    let text: &str = item.text;
    let link: &str = item.link;
    if link.starts_with(URL_SCHEME_HTTP_PREFIX) {
        return html! {
            a {
                class: c_docs_header_link()
                href: link
                target: LINK_TARGET_BLANK
                rel: LINK_REL_NOOPENER
                {
                    text
                }
            }
        };
    }
    let active: bool = header_link_is_active(link, locale_root, &route_signal.get());
    html! {
        a {
            class: if active {
                c_docs_header_link_active().clone()
            } else {
                c_docs_header_link().clone()
            }
            href: format!("#{link}")
            onclick: Router::link_handler(link)
            {
                text
            }
        }
    }
}

/// Reports whether a navigation link covers the route the reader is on.
///
/// Site and locale home links match only their roots. Section links ignore
/// trailing slashes and anchors, but require a slash boundary for descendants.
///
/// # Arguments
///
/// - `&str` - The navigation link.
/// - `&str` - The locale homepage route.
/// - `&str` - The current route.
///
/// # Returns
///
/// - `bool` - Whether the link covers the current route.
fn header_link_is_active(link: &str, locale_root: &str, route: &str) -> bool {
    let (link_path, _link_anchor): (String, Option<String>) = parse_route(link);
    let (route_path, _route_anchor): (String, Option<String>) = parse_route(route);
    let link: &str = link_path.trim_end_matches('/');
    let path: &str = route_path.trim_end_matches('/');
    if link == path {
        return true;
    }
    if link.is_empty() || link == locale_root.trim_end_matches('/') {
        return false;
    }
    path.strip_prefix(link)
        .is_some_and(|suffix: &str| suffix.starts_with('/'))
}

/// Renders an independently scoped live-preview search.
///
/// # Arguments
///
/// - `Signal<String>` - Current route; changed only by explicit selection.
/// - `Signal<String>` - Caller-owned search text.
/// - `Signal<bool>` - The responsive layout mode.
///
/// # Returns
///
/// - `VirtualNode` - Hook-scoped search input and preview panel.
fn docs_header_search(
    route_signal: Signal<String>,
    query: Signal<String>,
    mobile: Signal<bool>,
) -> VirtualNode {
    VirtualNode::create_dynamic(move |context: &mut HookContext| {
        use_docs_responsive_scope(context.clone(), mobile, false);
        let state: Signal<HeaderSearchState> = App::use_signal(HeaderSearchState::default);
        let id: UseId = use_id(HEADER_SEARCH_ID);
        let input_id: String = id.get_value().get();
        let outside: UseClickOutside = use_click_outside();
        let outside_triggered: Signal<bool> = outside.get_triggered();
        watch!(outside_triggered, |triggered: bool| {
            if triggered {
                state.set(HeaderSearchState::default());
                outside_triggered.set(false);
            }
        });
        header_search_preview(
            route_signal,
            query,
            state,
            input_id,
            outside.get_target().clone(),
        )
    })
}
