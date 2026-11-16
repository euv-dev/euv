use super::*;

/// Holds the sections of a navigation menu.
///
/// The menu is the frame the sections live in. A section opens under
/// its trigger, and the state of that section is carried by the signal
/// the caller owns.
///
/// # Arguments
///
/// - `VirtualNode<EuvNavigationMenuProps>` - The props node carrying the
///   state of the menu.
///
/// # Returns
///
/// - `VirtualNode` - The euv navigation menu virtual DOM tree.
#[component]
pub fn euv_navigation_menu(node: VirtualNode<EuvNavigationMenuProps>) -> VirtualNode {
    let EuvNavigationMenuProps { open }: EuvNavigationMenuProps =
        node.try_get_props().unwrap_or_default();
    let children: VirtualNode = node.get_children().into();
    html! {
        div {
            "data-slot": "navigation-menu"
            "data-state": if { open.get() } {
                "open"
            } else {
                "closed"
            }
            class: c_euv_navigation_menu()
            children
        }
    }
}

/// Holds the sections of a navigation menu in a row.
///
/// # Arguments
///
/// - `VirtualNode<EuvNavigationMenuListProps>` - The props node carrying
///   the settings of the list.
///
/// # Returns
///
/// - `VirtualNode` - The euv navigation menu list virtual DOM tree.
#[component]
pub fn euv_navigation_menu_list(node: VirtualNode<EuvNavigationMenuListProps>) -> VirtualNode {
    let EuvNavigationMenuListProps {}: EuvNavigationMenuListProps =
        node.try_get_props().unwrap_or_default();
    let children: VirtualNode = node.get_children().into();
    html! {
        ul {
            "data-slot": "navigation-menu-list"
            class: c_euv_navigation_menu_list()
            children
        }
    }
}

/// Holds one section of a navigation menu.
///
/// # Arguments
///
/// - `VirtualNode<EuvNavigationMenuItemProps>` - The props node carrying
///   the settings of the item.
///
/// # Returns
///
/// - `VirtualNode` - The euv navigation menu item virtual DOM tree.
#[component]
pub fn euv_navigation_menu_item(node: VirtualNode<EuvNavigationMenuItemProps>) -> VirtualNode {
    let EuvNavigationMenuItemProps {}: EuvNavigationMenuItemProps =
        node.try_get_props().unwrap_or_default();
    let children: VirtualNode = node.get_children().into();
    html! {
        li {
            "data-slot": "navigation-menu-item"
            class: c_euv_navigation_menu_item()
            children
        }
    }
}

/// Opens the section of a navigation menu.
///
/// The trigger reports the state of the section through
/// `aria-expanded`, so a reader knows whether the panel below it is
/// there before they press it.
///
/// # Arguments
///
/// - `VirtualNode<EuvNavigationMenuTriggerProps>` - The props node
///   carrying the state of the section.
///
/// # Returns
///
/// - `VirtualNode` - The euv navigation menu trigger virtual DOM tree.
#[component]
pub fn euv_navigation_menu_trigger(
    node: VirtualNode<EuvNavigationMenuTriggerProps>,
) -> VirtualNode {
    let EuvNavigationMenuTriggerProps { open }: EuvNavigationMenuTriggerProps =
        node.try_get_props().unwrap_or_default();
    let children: VirtualNode = node.get_children().into();
    html! {
        button {
            "data-slot": "navigation-menu-trigger"
            "aria-expanded": if { open.get() } {
                "true"
            } else {
                "false"
            }
            type: "button"
            class: c_euv_navigation_menu_trigger()
            class: if { open.get() } {
                c_euv_navigation_menu_trigger_open()
            } else {
                c_euv_navigation_menu_trigger_closed()
            }
            onclick: on_navigation_menu_toggle(open)
            children
        }
    }
}

/// Holds the panel of a section of a navigation menu.
///
/// The panel is hidden while the section is closed, so the links it
/// holds are not read out before the reader asks for them.
///
/// # Arguments
///
/// - `VirtualNode<EuvNavigationMenuContentProps>` - The props node
///   carrying the state of the section.
///
/// # Returns
///
/// - `VirtualNode` - The euv navigation menu content virtual DOM tree.
#[component]
pub fn euv_navigation_menu_content(
    node: VirtualNode<EuvNavigationMenuContentProps>,
) -> VirtualNode {
    let EuvNavigationMenuContentProps { open }: EuvNavigationMenuContentProps =
        node.try_get_props().unwrap_or_default();
    let children: VirtualNode = node.get_children().into();
    html! {
        div {
            "data-slot": "navigation-menu-content"
            "data-state": if { open.get() } {
                "open"
            } else {
                "closed"
            }
            class: c_euv_navigation_menu_content()
            class: if { open.get() } {
                c_euv_navigation_menu_content_open()
            } else {
                c_euv_navigation_menu_content_closed()
            }
            children
        }
    }
}

/// Holds one link inside the panel of a navigation menu.
///
/// # Arguments
///
/// - `VirtualNode<EuvNavigationMenuLinkProps>` - The props node carrying
///   the settings of the link.
///
/// # Returns
///
/// - `VirtualNode` - The euv navigation menu link virtual DOM tree.
#[component]
pub fn euv_navigation_menu_link(node: VirtualNode<EuvNavigationMenuLinkProps>) -> VirtualNode {
    let EuvNavigationMenuLinkProps {}: EuvNavigationMenuLinkProps =
        node.try_get_props().unwrap_or_default();
    let children: VirtualNode = node.get_children().into();
    let base_class: &Css = c_euv_navigation_menu_link();

    html! {
            a {
                "data-slot": "navigation-menu-link"
                class: base_class

                children
            }
    }
}

/// Holds the panel of an open section of a navigation menu.
///
/// The viewport is the frame the open panel is drawn in, so the panel
/// does not shift the rest of the menu while it is open.
///
/// # Arguments
///
/// - `VirtualNode<EuvNavigationMenuViewportProps>` - The props node
///   carrying the settings of the viewport.
///
/// # Returns
///
/// - `VirtualNode` - The euv navigation menu viewport virtual DOM tree.
#[component]
pub fn euv_navigation_menu_viewport(
    node: VirtualNode<EuvNavigationMenuViewportProps>,
) -> VirtualNode {
    let EuvNavigationMenuViewportProps {}: EuvNavigationMenuViewportProps =
        node.try_get_props().unwrap_or_default();
    let children: VirtualNode = node.get_children().into();
    html! {
            div {
                "data-slot": "navigation-menu-viewport"
                class: c_euv_navigation_menu_viewport()
                children
            }
    }
}

/// Holds the marker that shows which section of a navigation menu is
/// open.
///
/// # Arguments
///
/// - `VirtualNode<EuvNavigationMenuIndicatorProps>` - The props node
///   carrying the settings of the indicator.
///
/// # Returns
///
/// - `VirtualNode` - The euv navigation menu indicator virtual DOM tree.
#[component]
pub fn euv_navigation_menu_indicator(
    node: VirtualNode<EuvNavigationMenuIndicatorProps>,
) -> VirtualNode {
    let EuvNavigationMenuIndicatorProps {}: EuvNavigationMenuIndicatorProps =
        node.try_get_props().unwrap_or_default();
    let children: VirtualNode = node.get_children().into();
    html! {
        div {
            "data-slot": "navigation-menu-indicator"
            "aria-hidden": "true"
            class: c_euv_navigation_menu_indicator()
            children
        }
    }
}

/// Builds the handler that opens a section of a navigation menu or
/// closes it.
///
/// # Arguments
///
/// - `Signal<bool>` - The state of the section.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - The press handler.
fn on_navigation_menu_toggle(open: Signal<bool>) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |_: Event| {
        let next: bool = !open.get();
        open.set(next);
    }))
}
