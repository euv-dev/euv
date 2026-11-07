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
///
/// # Returns
///
/// - `VirtualNode` - The header virtual DOM tree.
pub(crate) fn docs_header(
    locale: &'static DocsLocale,
    route_signal: Signal<String>,
    theme_signal: Signal<String>,
    search_query: Signal<String>,
) -> VirtualNode {
    html! {
        header {
            class: CLASS_DOCS_HEADER
            docs_header_brand(locale)
            nav {
                class: CLASS_DOCS_HEADER_LINKS
                for item in locale.navbar.iter() {
                    docs_header_link(item, route_signal)
                }
            }
            div {
                class: CLASS_DOCS_HEADER_ACTIONS
                docs_header_search(route_signal, search_query)
                div {
                    class: CLASS_DOCS_HEADER_SEPARATOR
                }
                button {
                    class: CLASS_DOCS_HEADER_ICON_BUTTON
                    title: THEME_TOGGLE_TITLE
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
/// - `Signal<String>` - The current route signal.
///
/// # Returns
///
/// - `VirtualNode` - The link virtual DOM tree.
fn docs_header_link(item: &EuvNavbarItem, route_signal: Signal<String>) -> VirtualNode {
    let text: &str = item.text;
    let link: &str = item.link;
    if link.starts_with(URL_SCHEME_HTTP_PREFIX) {
        return html! {
            a {
                class: CLASS_DOCS_HEADER_LINK
                href: link
                target: LINK_TARGET_BLANK
                rel: LINK_REL_NOOPENER
                {
                    text
                }
            }
        };
    }
    let active: bool = header_link_is_active(link, &route_signal.get());
    html! {
        a {
            class: if active {
                CLASS_DOCS_HEADER_LINK_ACTIVE
            } else {
                CLASS_DOCS_HEADER_LINK
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
/// The home link owns the site root, so it is active only on the root
/// itself; every other link owns its section, so it stays active for the
/// routes underneath it.
///
/// # Arguments
///
/// - `&str` - The navigation link.
/// - `&str` - The current route.
///
/// # Returns
///
/// - `bool` - Whether the link covers the current route.
fn header_link_is_active(link: &str, route: &str) -> bool {
    let (path, _anchor) = parse_route(route);
    if link == path {
        return true;
    }
    if link.is_empty() || link == "/" {
        return path == "/" || path.is_empty();
    }
    path.starts_with(&format!("{link}/"))
}

/// Renders the search field.
///
/// The field filters the site's pages by title and route; pressing Enter
/// opens the best match.
///
/// # Arguments
///
/// - `Signal<String>` - The current route signal.
/// - `Signal<String>` - The search query signal.
///
/// # Returns
///
/// - `VirtualNode` - The search field virtual DOM tree.
fn docs_header_search(route_signal: Signal<String>, query: Signal<String>) -> VirtualNode {
    html! {
        div {
            class: CLASS_DOCS_HEADER_SEARCH
            input {
                class: CLASS_DOCS_HEADER_SEARCH_INPUT
                type: "search"
                placeholder: HEADER_SEARCH_PLACEHOLDER
                value: query
                oninput: UseEuvInput::on_input_value(query)
                onkeydown: header_search_keydown(route_signal, query)
            }
        }
    }
}

/// Opens the best match for the typed query when Enter is pressed.
///
/// A title match wins over a route match, and the first match wins over
/// the rest, so the field always opens one page instead of a list.
///
/// # Arguments
///
/// - `Signal<String>` - The current route signal.
/// - `Signal<String>` - The search query signal.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - The key handler.
fn header_search_keydown(
    route_signal: Signal<String>,
    query: Signal<String>,
) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |event: Event| {
        let Some(keyboard) = event.dyn_ref::<KeyboardEvent>() else {
            return;
        };
        if keyboard.key() != KEY_ENTER {
            return;
        }
        event.prevent_default();
        let needle: String = query.get().trim().to_lowercase();
        if needle.is_empty() {
            return;
        }
        let Some(route) = find_page_route(&needle) else {
            return;
        };
        route_signal.set(route.clone());
        Router::navigate(route);
    }))
}

/// Finds the page whose title or route contains the query.
///
/// # Arguments
///
/// - `&str` - The lowercased query.
///
/// # Returns
///
/// - `Option<String>` - The route of the best match.
fn find_page_route(needle: &str) -> Option<String> {
    let site: &DocsSite = &crate::generated::SITE;
    for page in site.pages.iter() {
        if page.title.to_lowercase().contains(needle) {
            return Some(page.route.to_string());
        }
    }
    for page in site.pages.iter() {
        if page.route.to_lowercase().contains(needle) {
            return Some(page.route.to_string());
        }
    }
    None
}
