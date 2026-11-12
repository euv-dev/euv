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

/// The frame that holds the state the sidebar and its groups share.
///
/// # Arguments
///
/// - `VirtualNode<EuvSidebarProviderProps>` - The props node carrying the
///   collapsed groups.
///
/// # Returns
///
/// - `VirtualNode` - The euv sidebar provider virtual DOM tree.
#[component]
pub fn euv_sidebar_provider(node: VirtualNode<EuvSidebarProviderProps>) -> VirtualNode {
    let EuvSidebarProviderProps { collapsed }: EuvSidebarProviderProps =
        node.try_get_props().unwrap_or_default();
    let children: VirtualNode = node.get_children().into();
    html! {
        div {
            "data-slot": "sidebar-provider"
            "data-collapsed": if { collapsed.get().is_empty() } {
                "false"
            } else {
                "true"
            }
            class: c_euv_sidebar_provider()
            children
        }
    }
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
            "data-slot": "sidebar"
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
                "data-slot": "sidebar-menu-button"
                class: link_class()
                // OPT 30: see comment in `euv_navbar` — replace
                // `format!("#{...}")` with a single `#`-prefix concat.
                href: {
                    let mut
                    href: String =
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
    //
    // That "exactly one" is also why the arrow stopped tracking the toggle:
    // `active` and `open` were alternatives, so an active group always took
    // the active class and never the rotation, whether it was expanded or
    // collapsed. The two concerns are orthogonal — colour follows the active
    // fill, rotation follows `collapsed` — so all four combinations exist.
    let arrow_class: fn() -> &'static Css = match (active, open) {
        (true, true) => c_euv_sidebar_group_arrow_active_open,
        (true, false) => c_euv_sidebar_group_arrow_active,
        (false, true) => c_euv_sidebar_group_arrow_open,
        (false, false) => c_euv_sidebar_group_arrow,
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
        "data-slot": "sidebar-group"
        class: c_euv_sidebar_group()
        div {
            class: title_class()
            onclick: toggle_navigate_group(collapsed, key.clone(), item.link, item.children, on_navigate.clone(), active)
            span {
                title_node
            }
            // The triangle is its own control, not part of the title row's
            // click target. It used to sit inside the title `<div>` with no
            // handler of its own, so a click on the glyph bubbled up to
            // `toggle_navigate_group` and navigated to the group index (or
            // jumped to the first child) instead of folding the group —
            // the arrow has to mean "fold/unfold", nothing else. Give it its
            // own `stop_propagation` handler so it only ever toggles.
            span {
                class: arrow_class()
                onclick: toggle_group_only(collapsed, key.clone())
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

/// Builds the group title's toggle-only click handler, used by the arrow.
///
/// Unlike [`toggle_navigate_group`] this never navigates: the arrow is a
/// disclosure control, so its only job is to add or remove `key` from the
/// collapsed set. The event is stopped so it does not bubble into the
/// enclosing title row, whose handler would otherwise treat the click as a
/// navigation request.
///
/// # Arguments
///
/// - `Signal<Vec<String>>` - The collapsed-keys signal.
/// - `String` - The group key.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - The click handler.
fn toggle_group_only(collapsed: Signal<Vec<String>>, key: String) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |event: Event| {
        event.prevent_default();
        event.stop_propagation();
        let mut keys: Vec<String> = collapsed.get();
        if let Some(index) = keys.iter().position(|k: &String| k == &key) {
            keys.remove(index);
        } else {
            keys.push(key.clone());
        }
        collapsed.set(keys);
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

/// The band that holds the title of the sidebar.
///
/// The header sits above the scrollable middle of the sidebar and holds
/// whatever the caller puts in it: a wordmark, a search field, a sign-in
/// control.
///
/// # Arguments
///
/// - `VirtualNode<EuvSidebarHeaderProps>` - The props node carrying the component settings.
///
/// # Returns
///
/// - `VirtualNode` - The euv sidebar header virtual DOM tree.
#[component]
pub fn euv_sidebar_header(node: VirtualNode<EuvSidebarHeaderProps>) -> VirtualNode {
    let EuvSidebarHeaderProps {}: EuvSidebarHeaderProps = node.try_get_props().unwrap_or_default();
    let children: VirtualNode = node.get_children().into();
    html! {
        div {
            "data-slot": "sidebar-header"
            class: c_euv_sidebar_header()
            children
        }
    }
}

/// The scrollable middle of the sidebar.
///
/// The content holds the groups of the sidebar and takes the room that is
/// left between the header and the footer, so it is the only band of the
/// sidebar that scrolls.
///
/// # Arguments
///
/// - `VirtualNode<EuvSidebarContentProps>` - The props node carrying the component settings.
///
/// # Returns
///
/// - `VirtualNode` - The euv sidebar content virtual DOM tree.
#[component]
pub fn euv_sidebar_content(node: VirtualNode<EuvSidebarContentProps>) -> VirtualNode {
    let EuvSidebarContentProps {}: EuvSidebarContentProps =
        node.try_get_props().unwrap_or_default();
    let children: VirtualNode = node.get_children().into();
    html! {
        div {
            "data-slot": "sidebar-content"
            class: c_euv_sidebar_content()
            children
        }
    }
}

/// The band that holds the footer of the sidebar.
///
/// The footer sits below the scrollable middle of the sidebar and holds
/// whatever the caller puts in it: an account row, a version line, a
/// theme switch.
///
/// # Arguments
///
/// - `VirtualNode<EuvSidebarFooterProps>` - The props node carrying the component settings.
///
/// # Returns
///
/// - `VirtualNode` - The euv sidebar footer virtual DOM tree.
#[component]
pub fn euv_sidebar_footer(node: VirtualNode<EuvSidebarFooterProps>) -> VirtualNode {
    let EuvSidebarFooterProps {}: EuvSidebarFooterProps = node.try_get_props().unwrap_or_default();
    let children: VirtualNode = node.get_children().into();
    html! {
        div {
            "data-slot": "sidebar-footer"
            class: c_euv_sidebar_footer()
            children
        }
    }
}

/// One group of menu rows.
///
/// A group holds a caption and its rows. The caption names the group and
/// the rows are what the reader clicks, so the two are separate parts
/// that the caller arranges inside this one.
///
/// # Arguments
///
/// - `VirtualNode<EuvSidebarGroupProps>` - The props node carrying the component settings.
///
/// # Returns
///
/// - `VirtualNode` - The euv sidebar group virtual DOM tree.
#[component]
pub fn euv_sidebar_group(node: VirtualNode<EuvSidebarGroupProps>) -> VirtualNode {
    let EuvSidebarGroupProps {}: EuvSidebarGroupProps = node.try_get_props().unwrap_or_default();
    let children: VirtualNode = node.get_children().into();
    html! {
        div {
            "data-slot": "sidebar-group"
            class: c_euv_sidebar_group_panel()
            children
        }
    }
}

/// The caption that names a group of menu rows.
///
/// The caption holds the name of the group and takes no settings of its
/// own.
///
/// # Arguments
///
/// - `VirtualNode<EuvSidebarGroupLabelProps>` - The props node carrying the component settings.
///
/// # Returns
///
/// - `VirtualNode` - The euv sidebar group label virtual DOM tree.
#[component]
pub fn euv_sidebar_group_label(node: VirtualNode<EuvSidebarGroupLabelProps>) -> VirtualNode {
    let EuvSidebarGroupLabelProps {}: EuvSidebarGroupLabelProps =
        node.try_get_props().unwrap_or_default();
    let children: VirtualNode = node.get_children().into();
    html! {
        div {
            "data-slot": "sidebar-group-label"
            class: c_euv_sidebar_group_label()
            children
        }
    }
}

/// The control at the end of a group caption.
///
/// The reference puts a small action there — an add button, a settings
/// button. It is a button of its own so the caption stays a caption, and
/// what it does is left to the caller.
///
/// # Arguments
///
/// - `VirtualNode<EuvSidebarGroupActionProps>` - The props node carrying the component settings.
///
/// # Returns
///
/// - `VirtualNode` - The euv sidebar group action virtual DOM tree.
#[component]
pub fn euv_sidebar_group_action(node: VirtualNode<EuvSidebarGroupActionProps>) -> VirtualNode {
    let EuvSidebarGroupActionProps {}: EuvSidebarGroupActionProps =
        node.try_get_props().unwrap_or_default();
    let children: VirtualNode = node.get_children().into();
    html! {
        button {
            type: "button"
            "data-slot": "sidebar-group-action"
            class: c_euv_sidebar_group_action()
            children
        }
    }
}

/// The rows of a group, below its caption.
///
/// The reference separates the caption from the rows so the rows can be
/// collapsed on their own; this part is that separation.
///
/// # Arguments
///
/// - `VirtualNode<EuvSidebarGroupContentProps>` - The props node carrying the component settings.
///
/// # Returns
///
/// - `VirtualNode` - The euv sidebar group content virtual DOM tree.
#[component]
pub fn euv_sidebar_group_content(node: VirtualNode<EuvSidebarGroupContentProps>) -> VirtualNode {
    let EuvSidebarGroupContentProps {}: EuvSidebarGroupContentProps =
        node.try_get_props().unwrap_or_default();
    let children: VirtualNode = node.get_children().into();
    html! {
        div {
            "data-slot": "sidebar-group-content"
            class: c_euv_sidebar_group_content()
            children
        }
    }
}

/// The list of menu rows.
///
/// The menu is the stack of rows the reader clicks through, and it is the
/// part that a group holds.
///
/// # Arguments
///
/// - `VirtualNode<EuvSidebarMenuProps>` - The props node carrying the component settings.
///
/// # Returns
///
/// - `VirtualNode` - The euv sidebar menu virtual DOM tree.
#[component]
pub fn euv_sidebar_menu(node: VirtualNode<EuvSidebarMenuProps>) -> VirtualNode {
    let EuvSidebarMenuProps {}: EuvSidebarMenuProps = node.try_get_props().unwrap_or_default();
    let children: VirtualNode = node.get_children().into();
    html! {
        div {
            "data-slot": "sidebar-menu"
            class: c_euv_sidebar_menu()
            children
        }
    }
}

/// One row of the menu.
///
/// A row holds its button and whatever else belongs to it — a count, an
/// action at its end — so the row can carry them without the button
/// having to know about them.
///
/// # Arguments
///
/// - `VirtualNode<EuvSidebarMenuItemProps>` - The props node carrying the component settings.
///
/// # Returns
///
/// - `VirtualNode` - The euv sidebar menu item virtual DOM tree.
#[component]
pub fn euv_sidebar_menu_item(node: VirtualNode<EuvSidebarMenuItemProps>) -> VirtualNode {
    let EuvSidebarMenuItemProps {}: EuvSidebarMenuItemProps =
        node.try_get_props().unwrap_or_default();
    let children: VirtualNode = node.get_children().into();
    html! {
        div {
            "data-slot": "sidebar-menu-item"
            class: c_euv_sidebar_menu_item()
            children
        }
    }
}

/// The clickable row of a menu item.
///
/// The reference renders a link when the row leads somewhere and a button
/// when it only acts, so the target route decides which of the two this
/// row is. The active state travels in the `data-active` attribute, which
/// keeps the styling of the row drivable from the markup alone.
///
/// # Arguments
///
/// - `VirtualNode<EuvSidebarMenuButtonProps>` - The props node carrying the component settings.
///
/// # Returns
///
/// - `VirtualNode` - The euv sidebar menu button virtual DOM tree.
#[component]
pub fn euv_sidebar_menu_button(node: VirtualNode<EuvSidebarMenuButtonProps>) -> VirtualNode {
    let EuvSidebarMenuButtonProps { is_active, href }: EuvSidebarMenuButtonProps =
        node.try_get_props().unwrap_or_default();
    let children: VirtualNode = node.get_children().into();
    let active_value: &str = if is_active {
        DATA_ACTIVE_TRUE
    } else {
        DATA_ACTIVE_FALSE
    };
    let button_class: fn() -> &'static Css = if is_active {
        c_euv_sidebar_menu_button_active
    } else {
        c_euv_sidebar_menu_button
    };
    if href.is_empty() {
        return html! {
            button {
                type: "button"
                "data-slot": "sidebar-menu-button"
                "data-active": active_value
                class: button_class()
                children
            }
        };
    }
    html! {
        a {
            href: href
            "data-slot": "sidebar-menu-button"
            "data-active": active_value
            class: button_class()
            children
        }
    }
}

/// The control at the end of a menu row.
///
/// The reference puts a small action at the end of a row — a remove
/// button, a shortcut hint. It is a button of its own so the row stays a
/// row, and what it does is left to the caller.
///
/// # Arguments
///
/// - `VirtualNode<EuvSidebarMenuActionProps>` - The props node carrying the component settings.
///
/// # Returns
///
/// - `VirtualNode` - The euv sidebar menu action virtual DOM tree.
#[component]
pub fn euv_sidebar_menu_action(node: VirtualNode<EuvSidebarMenuActionProps>) -> VirtualNode {
    let EuvSidebarMenuActionProps {}: EuvSidebarMenuActionProps =
        node.try_get_props().unwrap_or_default();
    let children: VirtualNode = node.get_children().into();
    html! {
        button {
            type: "button"
            "data-slot": "sidebar-menu-action"
            class: c_euv_sidebar_menu_action()
            children
        }
    }
}

/// The count at the end of a menu row.
///
/// The badge holds the count itself — the number of unread messages, the
/// number of items in a folder — and is the last thing in its row.
///
/// # Arguments
///
/// - `VirtualNode<EuvSidebarMenuBadgeProps>` - The props node carrying the component settings.
///
/// # Returns
///
/// - `VirtualNode` - The euv sidebar menu badge virtual DOM tree.
#[component]
pub fn euv_sidebar_menu_badge(node: VirtualNode<EuvSidebarMenuBadgeProps>) -> VirtualNode {
    let EuvSidebarMenuBadgeProps {}: EuvSidebarMenuBadgeProps =
        node.try_get_props().unwrap_or_default();
    let children: VirtualNode = node.get_children().into();
    html! {
        div {
            "data-slot": "sidebar-menu-badge"
            class: c_euv_sidebar_menu_badge()
            children
        }
    }
}

/// The list of the rows nested under a menu row.
///
/// The nested list hangs off the guide that runs down the side of the
/// rows, so the rows under a row read as belonging to it.
///
/// # Arguments
///
/// - `VirtualNode<EuvSidebarMenuSubProps>` - The props node carrying the component settings.
///
/// # Returns
///
/// - `VirtualNode` - The euv sidebar menu sub virtual DOM tree.
#[component]
pub fn euv_sidebar_menu_sub(node: VirtualNode<EuvSidebarMenuSubProps>) -> VirtualNode {
    let EuvSidebarMenuSubProps {}: EuvSidebarMenuSubProps =
        node.try_get_props().unwrap_or_default();
    let children: VirtualNode = node.get_children().into();
    html! {
        ul {
            "data-slot": "sidebar-menu-sub"
            class: c_euv_sidebar_menu_sub()
            children
        }
    }
}

/// One row of a nested menu.
///
/// # Arguments
///
/// - `VirtualNode<EuvSidebarMenuSubItemProps>` - The props node carrying the component settings.
///
/// # Returns
///
/// - `VirtualNode` - The euv sidebar menu sub item virtual DOM tree.
#[component]
pub fn euv_sidebar_menu_sub_item(node: VirtualNode<EuvSidebarMenuSubItemProps>) -> VirtualNode {
    let EuvSidebarMenuSubItemProps {}: EuvSidebarMenuSubItemProps =
        node.try_get_props().unwrap_or_default();
    let children: VirtualNode = node.get_children().into();
    html! {
        li {
            "data-slot": "sidebar-menu-sub-item"
            class: c_euv_sidebar_menu_sub_item()
            children
        }
    }
}

/// The clickable row of a nested menu item.
///
/// A nested row behaves like a row of the menu itself: it leads to a
/// route when it has one, and it carries the active state of the page it
/// leads to.
///
/// # Arguments
///
/// - `VirtualNode<EuvSidebarMenuSubButtonProps>` - The props node carrying the component settings.
///
/// # Returns
///
/// - `VirtualNode` - The euv sidebar menu sub button virtual DOM tree.
#[component]
pub fn euv_sidebar_menu_sub_button(node: VirtualNode<EuvSidebarMenuSubButtonProps>) -> VirtualNode {
    let EuvSidebarMenuSubButtonProps { is_active, href }: EuvSidebarMenuSubButtonProps =
        node.try_get_props().unwrap_or_default();
    let children: VirtualNode = node.get_children().into();
    let active_value: &str = if is_active {
        DATA_ACTIVE_TRUE
    } else {
        DATA_ACTIVE_FALSE
    };
    let button_class: fn() -> &'static Css = if is_active {
        c_euv_sidebar_menu_sub_button_active
    } else {
        c_euv_sidebar_menu_sub_button
    };
    if href.is_empty() {
        return html! {
            button {
                type: "button"
                "data-slot": "sidebar-menu-sub-button"
                "data-active": active_value
                class: button_class()
                children
            }
        };
    }
    html! {
        a {
            href: href
            "data-slot": "sidebar-menu-sub-button"
            "data-active": active_value
            class: button_class()
            children
        }
    }
}

/// The placeholder that stands in for a row while it loads.
///
/// The skeleton holds the shape of the row it stands in for, so the
/// sidebar does not jump when the rows arrive. The icon is optional: the
/// reference shows it only for the rows that carry one.
///
/// # Arguments
///
/// - `VirtualNode<EuvSidebarMenuSkeletonProps>` - The props node carrying the component settings.
///
/// # Returns
///
/// - `VirtualNode` - The euv sidebar menu skeleton virtual DOM tree.
#[component]
pub fn euv_sidebar_menu_skeleton(node: VirtualNode<EuvSidebarMenuSkeletonProps>) -> VirtualNode {
    let EuvSidebarMenuSkeletonProps { show_icon }: EuvSidebarMenuSkeletonProps =
        node.try_get_props().unwrap_or_default();
    let children: VirtualNode = node.get_children().into();
    let icon: VirtualNode = if show_icon {
        html! {
            span {
                "data-slot": "sidebar-menu-skeleton-icon"
                "aria-hidden": "true"
                class: c_euv_sidebar_menu_skeleton_icon()
            }
        }
    } else {
        html! {
            ""
        }
    };
    html! {
        div {
            "data-slot": "sidebar-menu-skeleton"
            class: c_euv_sidebar_menu_skeleton()
            icon
            span {
                "data-slot": "sidebar-menu-skeleton-text"
                "aria-hidden": "true"
                class: c_euv_sidebar_menu_skeleton_text()
            }
            children
        }
    }
}

/// The search field at the top of the sidebar.
///
/// The field sits in the header of the sidebar and narrows the menu to
/// the rows that match what the reader types.
///
/// # Arguments
///
/// - `VirtualNode<EuvSidebarInputProps>` - The props node carrying the component settings.
///
/// # Returns
///
/// - `VirtualNode` - The euv sidebar input virtual DOM tree.
#[component]
pub fn euv_sidebar_input(node: VirtualNode<EuvSidebarInputProps>) -> VirtualNode {
    let EuvSidebarInputProps { placeholder }: EuvSidebarInputProps =
        node.try_get_props().unwrap_or_default();
    html! {
        input {
            type: "text"
            "data-slot": "sidebar-input"
            placeholder: placeholder
            class: c_euv_sidebar_input()
        }
    }
}

/// The main area beside the sidebar.
///
/// The inset is the page itself: the sidebar sits beside it, and the
/// inset takes the room that is left.
///
/// # Arguments
///
/// - `VirtualNode<EuvSidebarInsetProps>` - The props node carrying the component settings.
///
/// # Returns
///
/// - `VirtualNode` - The euv sidebar inset virtual DOM tree.
#[component]
pub fn euv_sidebar_inset(node: VirtualNode<EuvSidebarInsetProps>) -> VirtualNode {
    let EuvSidebarInsetProps {}: EuvSidebarInsetProps = node.try_get_props().unwrap_or_default();
    let children: VirtualNode = node.get_children().into();
    html! {
        main {
            "data-slot": "sidebar-inset"
            class: c_euv_sidebar_inset()
            children
        }
    }
}

/// The thin edge that opens and closes the sidebar.
///
/// The rail is the strip along the edge of the sidebar that the reader
/// drags or clicks to fold the sidebar away. It carries no state of its
/// own: the sidebar decides what folding means.
///
/// # Arguments
///
/// - `VirtualNode<EuvSidebarRailProps>` - The props node carrying the component settings.
///
/// # Returns
///
/// - `VirtualNode` - The euv sidebar rail virtual DOM tree.
#[component]
pub fn euv_sidebar_rail(node: VirtualNode<EuvSidebarRailProps>) -> VirtualNode {
    let EuvSidebarRailProps {}: EuvSidebarRailProps = node.try_get_props().unwrap_or_default();
    html! {
        button {
            type: "button"
            "data-slot": "sidebar-rail"
            "aria-label": "Toggle Sidebar"
            class: c_euv_sidebar_rail()
        }
    }
}

/// The rule that divides one band of the sidebar from the next.
///
/// The separator is a hairline in the border colour. It is a decorative
/// rule, so it carries no role.
///
/// # Arguments
///
/// - `VirtualNode<EuvSidebarSeparatorProps>` - The props node carrying the component settings.
///
/// # Returns
///
/// - `VirtualNode` - The euv sidebar separator virtual DOM tree.
#[component]
pub fn euv_sidebar_separator(node: VirtualNode<EuvSidebarSeparatorProps>) -> VirtualNode {
    let EuvSidebarSeparatorProps {}: EuvSidebarSeparatorProps =
        node.try_get_props().unwrap_or_default();
    html! {
        div {
            role: "separator"
            "data-slot": "sidebar-separator"
            class: c_euv_sidebar_separator()
        }
    }
}

/// The button that opens and closes the sidebar.
///
/// The trigger reports the state the sidebar is in, so the reader can
/// tell an open sidebar from a closed one and the styling can follow the
/// state without a second source of truth.
///
/// # Arguments
///
/// - `VirtualNode<EuvSidebarTriggerProps>` - The props node carrying the component settings.
///
/// # Returns
///
/// - `VirtualNode` - The euv sidebar trigger virtual DOM tree.
#[component]
pub fn euv_sidebar_trigger(node: VirtualNode<EuvSidebarTriggerProps>) -> VirtualNode {
    let EuvSidebarTriggerProps { expanded }: EuvSidebarTriggerProps =
        node.try_get_props().unwrap_or_default();
    let expanded_value: &str = if expanded {
        ARIA_EXPANDED_TRUE
    } else {
        ARIA_EXPANDED_FALSE
    };
    let state_value: &str = if expanded {
        DATA_STATE_EXPANDED
    } else {
        DATA_STATE_COLLAPSED
    };
    let children: VirtualNode = node.get_children().into();
    html! {
        button {
            type: "button"
            "data-slot": "sidebar-trigger"
            "aria-expanded": expanded_value
            "data-state": state_value
            class: c_euv_sidebar_trigger()
            children
        }
    }
}
