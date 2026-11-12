use super::*;

/// A generic left-side mobile drawer with overlay, aligned with common
/// component libraries and the euv example's overlay-stack behaviour.
///
/// The drawer and overlay are always mounted and slide/fade via reactive
/// classes driven by the caller-owned `open` signal; clicking the overlay
/// closes the drawer through [`Router::overlay_stack_close`], so the browser
/// / system back gesture history entry pushed when the drawer opened is
/// consumed and stays in sync.
///
/// # Arguments
///
/// - `VirtualNode<EuvDrawerProps>` - The props node containing the open signal.
///
/// # Returns
///
/// - `VirtualNode` - The drawer virtual DOM tree.
#[component]
pub fn euv_drawer(node: VirtualNode<EuvDrawerProps>) -> VirtualNode {
    let EuvDrawerProps { open }: EuvDrawerProps = node.try_get_props().unwrap_or_default();
    let children: VirtualNode = node.get_children().into();
    html! {
        div {
            class: c_euv_drawer_overlay()
            class: if { open } {
                c_euv_drawer_overlay_open()
            } else {
                c_euv_drawer_overlay_closed()
            }
            onclick: close_drawer(open)
        }
        div {
            "data-slot": "drawer"
            class: c_euv_drawer()
            class: if { open } {
                c_euv_drawer_open()
            } else {
                c_euv_drawer_closed()
            }
            children
        }
    }
}

/// Closes the drawer, consuming its overlay history entry.
///
/// Mirrors the euv example: `Router::overlay_stack_close()` pops the overlay
/// stack and consumes the history entry pushed when the drawer opened (see
/// [`Router::use_overlay_history`]), then the signal is set.
///
/// # Arguments
///
/// - `Signal<bool>` - The drawer-open signal.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - The click handler.
fn close_drawer(open: Signal<bool>) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |_| {
        Router::overlay_stack_close();
        open.set(false);
    }))
}

/// Opens a drawer.
///
/// The trigger reports the state of the drawer through `aria-expanded`,
/// so a reader knows whether the panel is already on the page before
/// they press it.
///
/// # Arguments
///
/// - `VirtualNode<EuvDrawerTriggerProps>` - The props node carrying the
///   state of the drawer.
///
/// # Returns
///
/// - `VirtualNode` - The euv drawer trigger virtual DOM tree.
#[component]
pub fn euv_drawer_trigger(node: VirtualNode<EuvDrawerTriggerProps>) -> VirtualNode {
    let EuvDrawerTriggerProps { open }: EuvDrawerTriggerProps =
        node.try_get_props().unwrap_or_default();
    let children: VirtualNode = node.get_children().into();
    html! {
        button {
            "data-slot": "drawer-trigger"
            "aria-haspopup": "dialog"
            "aria-expanded": if { open.get() } {
                "true"
            } else {
                "false"
            }
            type: "button"
            class: c_euv_drawer_trigger()
            onclick: on_drawer_open(open)
            children
        }
    }
}

/// Holds the panel of a drawer.
///
/// The panel is the sheet that rises from the bottom edge. It reports
/// the state of the drawer and takes itself out of the flow while the
/// drawer is closed.
///
/// # Arguments
///
/// - `VirtualNode<EuvDrawerContentProps>` - The props node carrying the
///   state of the drawer.
///
/// # Returns
///
/// - `VirtualNode` - The euv drawer content virtual DOM tree.
#[component]
pub fn euv_drawer_content(node: VirtualNode<EuvDrawerContentProps>) -> VirtualNode {
    let EuvDrawerContentProps { open }: EuvDrawerContentProps =
        node.try_get_props().unwrap_or_default();
    let children: VirtualNode = node.get_children().into();
    html! {
        div {
            "data-slot": "drawer-content"
            "data-state": if { open.get() } {
                "open"
            } else {
                "closed"
            }
            role: "dialog"
            "aria-modal": "true"
            class: c_euv_drawer_content()
            class: if { open.get() } {
                c_euv_drawer_content_open()
            } else {
                c_euv_drawer_content_closed()
            }
            div {
                "data-slot": "drawer-handle"
                class: c_euv_drawer_handle()
            }
            children
        }
    }
}

/// Holds the heading of a drawer.
///
/// The header is centred on a narrow screen and left aligned on a wide
/// one, the way the reference lays it out.
///
/// # Arguments
///
/// - `VirtualNode<EuvDrawerHeaderProps>` - The props node carrying the
///   settings of the header.
///
/// # Returns
///
/// - `VirtualNode` - The euv drawer header virtual DOM tree.
#[component]
pub fn euv_drawer_header(node: VirtualNode<EuvDrawerHeaderProps>) -> VirtualNode {
    let EuvDrawerHeaderProps {}: EuvDrawerHeaderProps = node.try_get_props().unwrap_or_default();
    let children: VirtualNode = node.get_children().into();
    html! {
        div {
            "data-slot": "drawer-header"
            class: c_euv_drawer_header()
            children
        }
    }
}

/// Holds the actions of a drawer.
///
/// The footer sits at the bottom of the panel, so the actions stay
/// reachable however long the content above them is.
///
/// # Arguments
///
/// - `VirtualNode<EuvDrawerFooterProps>` - The props node carrying the
///   settings of the footer.
///
/// # Returns
///
/// - `VirtualNode` - The euv drawer footer virtual DOM tree.
#[component]
pub fn euv_drawer_footer(node: VirtualNode<EuvDrawerFooterProps>) -> VirtualNode {
    let EuvDrawerFooterProps {}: EuvDrawerFooterProps = node.try_get_props().unwrap_or_default();
    let children: VirtualNode = node.get_children().into();
    html! {
        div {
            "data-slot": "drawer-footer"
            class: c_euv_drawer_footer()
            children
        }
    }
}

/// Holds the title of a drawer.
///
/// # Arguments
///
/// - `VirtualNode<EuvDrawerTitleProps>` - The props node carrying the
///   settings of the title.
///
/// # Returns
///
/// - `VirtualNode` - The euv drawer title virtual DOM tree.
#[component]
pub fn euv_drawer_title(node: VirtualNode<EuvDrawerTitleProps>) -> VirtualNode {
    let EuvDrawerTitleProps {}: EuvDrawerTitleProps = node.try_get_props().unwrap_or_default();
    let children: VirtualNode = node.get_children().into();
    html! {
        div {
            "data-slot": "drawer-title"
            class: c_euv_drawer_title()
            children
        }
    }
}

/// Holds the description of a drawer.
///
/// # Arguments
///
/// - `VirtualNode<EuvDrawerDescriptionProps>` - The props node carrying
///   the settings of the description.
///
/// # Returns
///
/// - `VirtualNode` - The euv drawer description virtual DOM tree.
#[component]
pub fn euv_drawer_description(node: VirtualNode<EuvDrawerDescriptionProps>) -> VirtualNode {
    let EuvDrawerDescriptionProps {}: EuvDrawerDescriptionProps =
        node.try_get_props().unwrap_or_default();
    let children: VirtualNode = node.get_children().into();
    html! {
        div {
            "data-slot": "drawer-description"
            class: c_euv_drawer_description()
            children
        }
    }
}

/// Closes a drawer.
///
/// The close button consumes the overlay history entry the drawer pushed
/// when it opened, so the browser back gesture does not reopen the
/// drawer behind the reader's back.
///
/// # Arguments
///
/// - `VirtualNode<EuvDrawerCloseProps>` - The props node carrying the
///   state of the drawer.
///
/// # Returns
///
/// - `VirtualNode` - The euv drawer close virtual DOM tree.
#[component]
pub fn euv_drawer_close(node: VirtualNode<EuvDrawerCloseProps>) -> VirtualNode {
    let EuvDrawerCloseProps { open }: EuvDrawerCloseProps =
        node.try_get_props().unwrap_or_default();
    let children: VirtualNode = node.get_children().into();
    html! {
        button {
            "data-slot": "drawer-close"
            type: "button"
            class: c_euv_drawer_close()
            onclick: close_drawer(open)
            children
        }
    }
}

/// Builds the handler that opens the drawer.
///
/// # Arguments
///
/// - `Signal<bool>` - The drawer-open signal.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - The press handler.
fn on_drawer_open(open: Signal<bool>) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |_: Event| {
        open.set(true);
    }))
}

/// Dims the page behind the drawer.
///
/// The overlay covers the page and takes the click, so a click outside the
/// panel of the drawer does not reach the page behind it.
///
/// # Arguments
///
/// - `VirtualNode<EuvDrawerOverlayProps>` - The props node carrying whether
///   the drawer is open.
///
/// # Returns
///
/// - `VirtualNode` - The overlay element.
#[component]
pub fn euv_drawer_overlay(node: VirtualNode<EuvDrawerOverlayProps>) -> VirtualNode {
    let EuvDrawerOverlayProps { open }: EuvDrawerOverlayProps =
        node.try_get_props().unwrap_or_default();
    html! {
        div {
            "data-slot": "drawer-overlay"
            "data-state": if { open.get() } {
                "open"
            } else {
                "closed"
            }
            class: c_euv_drawer_overlay()
            class: if { open.get() } {
                c_euv_drawer_overlay_open()
            } else {
                c_euv_drawer_overlay_closed()
            }
        }
    }
}
