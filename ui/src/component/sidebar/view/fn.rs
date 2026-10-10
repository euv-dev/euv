use super::*;

/// Returns the first navigable route inside a subtree of sidebar items.
///
/// A pure directory (`essay/2025`) carries no index page, so the group
/// itself is not a destination and the site would render an empty article
/// for it. The user's intent when clicking the group title is "show me
/// this section", and the section's first readable page is the only thing
/// in it that can satisfy that. The search therefore descends:
/// depth-first, in sidebar order, taking a group's own index page before
/// any of its children.
///
/// The descent repeats for nested groups, so `a/b/c` where only `c/d` has
/// content still resolves to the first readable page under `c`, following
/// the same rule at every level. A subtree with no readable page at all
/// yields `None`, and the caller keeps the current toggle-only behaviour
/// rather than navigating somewhere arbitrary.
///
/// # Arguments
///
/// - `&[EuvSidebarItem]` - The subtree to search, in sidebar order.
///
/// # Returns
///
/// - `Option<&'static str>` - The first readable route, or `None` when the
///   subtree holds none.
pub fn first_navigable_route(items: &[EuvSidebarItem]) -> Option<&'static str> {
    for item in items.iter() {
        if let Some(link) = item.link {
            return Some(link);
        }
        if let Some(found) = first_navigable_route(item.children) {
            return Some(found);
        }
    }
    None
}

/// A generic collapsible navigation tree aligned with common docs frameworks.
///
/// Renders leaf links and collapsible groups recursively. The collapse state
/// lives in the caller-owned `collapsed` signal so it survives re-renders and
/// can be shared between multiple sidebar instances. All groups start
/// expanded.
///
/// # Arguments
///
/// - `VirtualNode<EuvSidebarProps>` - The props node.
///
/// # Returns
///
/// - `VirtualNode` - The sidebar virtual DOM tree.
#[component]
pub fn euv_sidebar(node: VirtualNode<EuvSidebarProps>) -> VirtualNode {
    let EuvSidebarProps {
        route_signal,
        collapsed,
        items,
        prefix,
        on_navigate,
    }: EuvSidebarProps = node.try_get_props().unwrap_or_default();
    html! {
        div {
            for item in items.iter() {
                euv_sidebar_item {
                    route_signal
                    collapsed
                    item: *item
                    prefix: prefix.clone()
                    on_navigate: on_navigate.clone()
                }
            }
        }
    }
}

/// Renders one sidebar node: a leaf link or a collapsible group.
///
/// # Arguments
///
/// - `VirtualNode<EuvSidebarItemProps>` - The props node.
///
/// # Returns
///
/// - `VirtualNode` - The item virtual DOM tree.
#[component]
pub fn euv_sidebar_item(node: VirtualNode<EuvSidebarItemProps>) -> VirtualNode {
    let EuvSidebarItemProps {
        route_signal,
        collapsed,
        item,
        prefix,
        on_navigate,
    }: EuvSidebarItemProps = node.try_get_props().unwrap_or_default();
    let route: String = route_signal.get();
    let path: &str = strip_hash_anchor(&route);
    if item.children.is_empty() {
        let Some(link) = item.link else {
            return html! {
                ""
            };
        };
        let active: bool = path == link;
        // A leaf under a collapsed-in group is inset by `c_euv_sidebar_children`.
        // When it is the active page the accent fill has to reach back to the
        // sidebar edge, otherwise the highlight reads as a chip floating in a
        // gutter instead of a row of the nav. `prefix` is empty only for a
        // top-level leaf, which is already flush and keeps the plain class.
        let flush: bool = !prefix.is_empty();
        let link_class: fn() -> &'static Css = if active {
            if flush {
                c_euv_sidebar_link_active_flush
            } else {
                c_euv_sidebar_link_active
            }
        } else {
            c_euv_sidebar_link
        };
        return html! {
            a {
                class: link_class()
                // OPT 30: see comment in `euv_navbar` — replace
                // `format!("#{...}")` with a single `#`-prefix concat.
                href: {
                    let mut href: String =
                        String::with_capacity(ROUTE_HASH_PREFIX.len() + link.len());
                    href.push_str(ROUTE_HASH_PREFIX);
                    href.push_str(link);
                    href
                }
                onclick: navigate_link(route_signal, on_navigate.clone(), link)
                {
                    item.text
                }
            }
        };
    }
    // OPT 30: `format!("{prefix}/{}", item.text)` becomes a pre-sized
    // concatenation of the constant separator `/` between two borrowed
    // slices.
    let key: String = {
        let mut key: String = String::with_capacity(prefix.len() + 1 + item.text.len());
        key.push_str(&prefix);
        key.push('/');
        key.push_str(item.text);
        key
    };
    let open: bool = !collapsed.get().contains(&key);
    let active: bool = item.link.is_some_and(|link: &'static str| path == link);
    // The arrow span used to carry two `class:` attributes — the first
    // returned the base or open-arrow class, the second layered the active
    // class on top. On an active row that meant `c_euv_sidebar_group_arrow`'s
    // `:hover` inset bar in `foreground` (white in dark mode) was painted
    // straight over the accent fill, which is what showed up as a white edge
    // on hover. Pick exactly one class instead.
    let arrow_class: fn() -> &'static Css = if active {
        c_euv_sidebar_group_arrow_active
    } else if open {
        c_euv_sidebar_group_arrow_open
    } else {
        c_euv_sidebar_group_arrow
    };
    // A top-level group has no `c_euv_sidebar_children` ancestor to line up
    // with, so it uses the `_root` title variant — the same row with a
    // heavier, inset hover bar that stays clear of the sidebar edge.
    let is_root: bool = prefix.is_empty();
    let title_class: fn() -> &'static Css = if is_root {
        if active {
            c_euv_sidebar_group_title_root_active
        } else {
            c_euv_sidebar_group_title_root
        }
    } else if active {
        c_euv_sidebar_group_title_active
    } else {
        c_euv_sidebar_group_title
    };
    let title_node: VirtualNode = match item.link {
        Some(link) => html! {
            a {
                // OPT 30: same `#`-prefix concat as above.
                href: {
                    let mut
                    href: String =
                    String::with_capacity(ROUTE_HASH_PREFIX.len() + link.len());
                    href.push_str(ROUTE_HASH_PREFIX);
                    href.push_str(link);
                    href
                }
                onclick: toggle_navigate_group(collapsed, key.clone(), Some(link), item.children, on_navigate.clone(), active)
                {
                    item.text
                }
            }
        },
        None => html! {
            {
                item.text
            }
        },
    };
    html! {
    div {
        class: c_euv_sidebar_group()
        div {
            class: title_class()
            onclick: toggle_navigate_group(collapsed, key.clone(), item.link, item.children, on_navigate.clone(), active)
            span {
                title_node
            }
            span {
                class: arrow_class()
                "▸"
            }
        }
        if open {
            div {
                class: c_euv_sidebar_children()
                euv_sidebar {
                    route_signal
                    collapsed
                    items: item.children
                    prefix: key.clone()
                    on_navigate: on_navigate.clone()
                }
            }
        }
    }
    }
}

/// Builds the leaf-link click handler: the interceptor when set, otherwise
/// the default hash-router navigation. When the clicked leaf matches the
/// current route the handler is a no-op — re-triggering `hashchange` for the
/// same path would re-run the route subscriber and cause unnecessary work.
///
/// # Arguments
///
/// - `Signal<String>` - The current route signal (drives the no-op check).
/// - `Option<Rc<dyn Fn(&'static str)>>` - The navigation interceptor.
/// - `&'static str` - The target link.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - The click handler.
fn navigate_link(
    route_signal: Signal<String>,
    on_navigate: Option<Rc<dyn Fn(&'static str)>>,
    link: &'static str,
) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |event: Event| {
        event.prevent_default();
        if strip_hash_anchor(&route_signal.get()) == link {
            return;
        }
        match &on_navigate {
            Some(interceptor) => interceptor(link),
            None => Router::navigate(link),
        }
    }))
}

/// Group title click handler. When the group owns an index route (`link`),
/// the action taken depends on whether the index route is the current route:
///
/// - **Index is inactive** (`link != current route`): navigate to the
///   index page and leave the collapsed state alone. The whole group is
///   the user's way of saying "take me to this section", and changing the
///   expand/collapse state when the user only wants to navigate is a
///   surprising side effect.
/// - **Index is active** (`link == current route`): the user is already on
///   the index page, so skip navigation and just toggle collapsed. This is
///   the only state where flipping the expand arrow makes sense.
///
/// When the group has no index page, the click only toggles collapsed.
///
/// The matched `active` flag is captured up-front so the closure does not
/// re-read the route signal on every dispatch (avoids an unnecessary
/// signal subscription).
///
/// # Arguments
///
/// - `Signal<Vec<String>>` - The collapsed-keys signal.
/// - `String` - The group key.
/// - `Option<&'static str>` - The group's index route when it exists.
/// - `&'static [EuvSidebarItem]` - The group's children, searched for the
///   first readable page when the group has no index of its own.
/// - `Option<Rc<dyn Fn(&'static str)>>` - The navigation interceptor.
/// - `bool` - Whether the group's index route is the current route.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - The click handler.
fn toggle_navigate_group(
    collapsed: Signal<Vec<String>>,
    key: String,
    link: Option<&'static str>,
    children: &'static [EuvSidebarItem],
    on_navigate: Option<Rc<dyn Fn(&'static str)>>,
    active: bool,
) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |event: Event| {
        event.prevent_default();
        match link {
            // Group with index page.
            Some(link) if !active => {
                // Inactive index: just navigate, leave collapsed state alone.
                match &on_navigate {
                    Some(interceptor) => interceptor(link),
                    None => Router::navigate(link),
                }
            }
            Some(_) => {
                // Active index: user is already here — toggle collapsed only.
                let mut keys: Vec<String> = collapsed.get();
                if let Some(index) = keys.iter().position(|k: &String| k == &key) {
                    keys.remove(index);
                } else {
                    keys.push(key.clone());
                }
                collapsed.set(keys);
            }
            // Pure folder with no index page. It is not a destination of
            // its own — the site renders an empty article for it — so the
            // click means "take me into this section": navigate to the first
            // readable page below it, descending through nested directories
            // until one is found. Only a subtree with nothing readable at all
            // falls back to folding.
            None => match first_navigable_route(children) {
                Some(target) => match &on_navigate {
                    Some(interceptor) => interceptor(target),
                    None => Router::navigate(target),
                },
                None => {
                    let mut keys: Vec<String> = collapsed.get();
                    if let Some(index) = keys.iter().position(|k: &String| k == &key) {
                        keys.remove(index);
                    } else {
                        keys.push(key.clone());
                    }
                    collapsed.set(keys);
                }
            },
        }
    }))
}

/// Strips the `#anchor` suffix from a hash route.
///
/// # Arguments
///
/// - `&str` - The raw route string.
///
/// # Returns
///
/// - `&str` - The route path without the anchor.
fn strip_hash_anchor(route: &str) -> &str {
    match route.split_once('#') {
        Some((path, _)) => path,
        None => route,
    }
}
