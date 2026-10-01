use super::*;

/// Renders a navigation item link with active state styling.
///
/// # Arguments
///
/// - `VirtualNode<NavItemProps>` - The typed props containing route signal, icon, label, and target.
///
/// # Returns
///
/// - `VirtualNode` - The navigation item virtual DOM tree.
#[component]
pub(crate) fn nav_item(node: VirtualNode<NavItemProps>) -> VirtualNode {
    let NavItemProps {
        route_signal,
        icon,
        label,
        target,
    }: NavItemProps = node.try_get_props().unwrap_or_default();
    html! {
        euv_nav_item {
            route_signal: route_signal
            icon: icon
            label: label
            target: target
        }
    }
}

/// Renders a mobile navigation item link that closes the drawer on navigation.
///
/// # Arguments
///
/// - `VirtualNode<MobileNavItemProps>` - The typed props containing route signal, drawer open signal, icon, label, and target.
///
/// # Returns
///
/// - `VirtualNode` - The mobile navigation item virtual DOM tree.
#[component]
pub(crate) fn mobile_nav_item(node: VirtualNode<MobileNavItemProps>) -> VirtualNode {
    let MobileNavItemProps {
        route_signal,
        drawer_open,
        icon,
        label,
        target,
    }: MobileNavItemProps = node.try_get_props().unwrap_or_default();
    html! {
        euv_mobile_nav_item {
            route_signal: route_signal
            drawer_open: drawer_open
            icon: icon
            label: label
            target: target
        }
    }
}

/// Builds the navigation items list for the desktop sidebar.
///
/// # Arguments
///
/// - `VirtualNode<BuildDesktopNavItemsProps>` - The typed props containing the route signal.
///
/// # Returns
///
/// - `VirtualNode` - The scrollable nav items container virtual DOM tree.
#[component]
pub(crate) fn build_desktop_nav_items(node: VirtualNode<BuildDesktopNavItemsProps>) -> VirtualNode {
    let BuildDesktopNavItemsProps { route_signal }: BuildDesktopNavItemsProps =
        node.try_get_props().unwrap_or_default();
    html! {
        div {
            class: c_nav_items_scroll()
            NAV_ITEMS.iter().map(|&(icon, label, target): &(&str, &str, &str)| {
                html! {
                    nav_item {
                        route_signal: route_signal
                        icon: icon
                        label: label
                        target: target
                    }
                }
            }).collect::<Vec<_>>()
        }
    }
}

/// Builds the navigation items list for the mobile drawer.
///
/// Navigation items close the drawer on click.
///
/// # Arguments
///
/// - `VirtualNode<BuildMobileNavItemsProps>` - The typed props containing route signal and drawer open signal.
///
/// # Returns
///
/// - `VirtualNode` - The scrollable nav items container virtual DOM tree.
#[component]
pub(crate) fn build_mobile_nav_items(node: VirtualNode<BuildMobileNavItemsProps>) -> VirtualNode {
    let BuildMobileNavItemsProps {
        route_signal,
        drawer_open,
    }: BuildMobileNavItemsProps = node.try_get_props().unwrap_or_default();
    html! {
        div {
            class: c_nav_items_scroll()
            NAV_ITEMS.iter().map(|&(icon, label, target): &(&str, &str, &str)| {
                html! {
                    mobile_nav_item {
                        route_signal: route_signal
                        drawer_open: drawer_open
                        icon: icon
                        label: label
                        target: target
                    }
                }
            }).collect::<Vec<_>>()
        }
    }
}
