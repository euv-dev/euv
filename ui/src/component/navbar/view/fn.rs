use super::*;

/// A generic fixed top navigation bar aligned with common site frameworks.
///
/// Renders the mobile hamburger button (when `drawer_open` is provided), the
/// brand block, the link list and a trailing actions area taken from the
/// children.
///
/// # Arguments
///
/// - `VirtualNode<EuvNavbarProps>` - The props node.
///
/// # Returns
///
/// - `VirtualNode` - The navbar virtual DOM tree.
#[component]
pub fn euv_navbar(node: VirtualNode<EuvNavbarProps>) -> VirtualNode {
    let EuvNavbarProps {
        route_signal,
        brand_logo,
        brand_title,
        brand_href,
        items,
        drawer_open,
    }: EuvNavbarProps = node.try_get_props().unwrap_or_default();
    let children: VirtualNode = node.get_children().into();
    let menu_button: VirtualNode = match drawer_open {
        Some(signal) => html! {
            button {
                class: if { signal } {
                    c_euv_navbar_menu_button_active()
                } else {
                    c_euv_navbar_menu_button()
                }
                onclick: UseEuvLayout::use_drawer_toggle(signal)
                "≡"
            }
        },
        None => html! {
            ""
        },
    };
    html! {
        nav {
            class: c_euv_navbar()
            menu_button
            a {
                class: c_euv_navbar_brand()
                // OPT 30: avoid `format!("#{...}")` per render; concatenate
                // the constant `#` prefix with `brand_href` once.
                href: {
                    let mut
                    href: String =
                    String::with_capacity(ROUTE_HASH_PREFIX.len() + brand_href.len());
                    href.push_str(ROUTE_HASH_PREFIX);
                    href.push_str(brand_href);
                    href
                }
                onclick: Router::link_handler(brand_href)
                span {
                    class: c_euv_navbar_logo()
                    {
                        brand_logo
                    }
                }
                span {
                    {
                        brand_title
                    }
                }
            }
            div {
                class: c_euv_navbar_links()
                for item in items.iter() {
                    euv_navbar_link {
                        route_signal
                        item: *item
                    }
                }
            }
            div {
                class: c_euv_navbar_actions()
                children
            }
        }
    }
}

/// Renders a single navbar link (internal hash route or external URL).
///
/// # Arguments
///
/// - `VirtualNode<EuvNavbarLinkProps>` - The props node.
///
/// # Returns
///
/// - `VirtualNode` - The link virtual DOM tree.
#[component]
pub fn euv_navbar_link(node: VirtualNode<EuvNavbarLinkProps>) -> VirtualNode {
    let EuvNavbarLinkProps { route_signal, item }: EuvNavbarLinkProps =
        node.try_get_props().unwrap_or_default();
    let external: bool = item.link.starts_with("http");
    let route: String = route_signal.get();
    let path: &str = strip_hash_anchor(&route);
    let active: bool = !external && item.link != "/" && path.starts_with(item.link);
    let link_class: fn() -> &'static Css = if active {
        c_euv_navbar_link_active
    } else {
        c_euv_navbar_link
    };
    if external {
        html! {
            a {
                class: c_euv_navbar_link()
                href: item.link
                target: "_blank"
                onclick: Router::external_link_handler(item.link)
                {
                    item.text
                }
            }
        }
    } else {
        html! {
            a {
                class: {
                    link_class()
                }
                // OPT 30: see comment in `euv_navbar` above — the same
                // `#`-prefix concatenation replaces `format!("#{}", ...)`
                // here.
                href: {
                    let mut href: String =
                        String::with_capacity(ROUTE_HASH_PREFIX.len() + item.link.len());
                    href.push_str(ROUTE_HASH_PREFIX);
                    href.push_str(item.link);
                    href
                }
                onclick: Router::link_handler(item.link)
                {
                    item.text
                }
            }
        }
    }
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

/// The bar that holds the menus of an application.
///
/// The bar carries `role="menubar"`, which is what tells the screen
/// reader that the triggers inside it open menus rather than navigate.
///
/// # Arguments
///
/// - `VirtualNode<EuvMenubarProps>` - The props node carrying the component settings.
///
/// # Returns
///
/// - `VirtualNode` - The euv menubar virtual DOM tree.
#[component]
pub fn euv_menubar(node: VirtualNode<EuvMenubarProps>) -> VirtualNode {
    let EuvMenubarProps { label }: EuvMenubarProps = node.try_get_props().unwrap_or_default();
    let children: VirtualNode = node.get_children().into();
    html! {
        div {
            role: "menubar"
            "aria-label": label
            class: c_euv_menubar()
            children
        }
    }
}

/// One menu of the bar: its trigger and its content.
///
/// # Arguments
///
/// - `VirtualNode<EuvMenubarMenuProps>` - The props node carrying the component settings.
///
/// # Returns
///
/// - `VirtualNode` - The euv menubar menu virtual DOM tree.
#[component]
pub fn euv_menubar_menu(node: VirtualNode<EuvMenubarMenuProps>) -> VirtualNode {
    let EuvMenubarMenuProps { open }: EuvMenubarMenuProps =
        node.try_get_props().unwrap_or_default();
    let children: VirtualNode = node.get_children().into();
    html! {
        div {
            class: c_euv_menubar_menu()
            class: if { open } {
                c_euv_menubar_menu_open()
            } else {
                c_euv_menubar_menu_closed()
            }
            children
        }
    }
}

/// The button that opens one menu of the bar.
///
/// # Arguments
///
/// - `VirtualNode<EuvMenubarTriggerProps>` - The props node carrying the component settings.
///
/// # Returns
///
/// - `VirtualNode` - The euv menubar trigger virtual DOM tree.
#[component]
pub fn euv_menubar_trigger(node: VirtualNode<EuvMenubarTriggerProps>) -> VirtualNode {
    let EuvMenubarTriggerProps { label, open }: EuvMenubarTriggerProps =
        node.try_get_props().unwrap_or_default();
    let trigger_class: fn() -> &'static Css = if open.get() {
        c_euv_menubar_trigger_open
    } else {
        c_euv_menubar_trigger
    };
    let on_click: Option<Rc<dyn Fn(Event)>> = Some(Rc::new(move |_: Event| {
        open.set(!open.get());
    }));
    html! {
        button {
            type: "button"
            role: "menuitem"
            "aria-haspopup": "menu"
            "aria-expanded": if open.get() {
                "true"
            } else {
                "false"
            }
            class: trigger_class()
            onclick: on_click
            {
                label
            }
        }
    }
}

/// The panel that holds the rows of one menu.
///
/// # Arguments
///
/// - `VirtualNode<EuvMenubarContentProps>` - The props node carrying the component settings.
///
/// # Returns
///
/// - `VirtualNode` - The euv menubar content virtual DOM tree.
#[component]
pub fn euv_menubar_content(node: VirtualNode<EuvMenubarContentProps>) -> VirtualNode {
    let EuvMenubarContentProps { open }: EuvMenubarContentProps =
        node.try_get_props().unwrap_or_default();
    let children: VirtualNode = node.get_children().into();
    html! {
        div {
            role: "menu"
            class: c_euv_menubar_content()
            class: if { open } {
                c_euv_menubar_content_open()
            } else {
                c_euv_menubar_content_closed()
            }
            children
        }
    }
}

/// One row of a menu panel.
///
/// # Arguments
///
/// - `VirtualNode<EuvMenubarItemProps>` - The props node carrying the component settings.
///
/// # Returns
///
/// - `VirtualNode` - The euv menubar item virtual DOM tree.
#[component]
pub fn euv_menubar_item(node: VirtualNode<EuvMenubarItemProps>) -> VirtualNode {
    let EuvMenubarItemProps {
        label,
        value,
        on_select,
    }: EuvMenubarItemProps = node.try_get_props().unwrap_or_default();
    let on_click: Option<Rc<dyn Fn(Event)>> = Some(Rc::new(move |_: Event| {
        if let Some(callback) = &on_select {
            callback(value);
        }
    }));
    html! {
        button {
            type: "button"
            role: "menuitem"
            class: c_euv_menubar_item()
            onclick: on_click
            {
                label
            }
        }
    }
}
