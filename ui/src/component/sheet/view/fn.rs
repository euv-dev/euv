use super::*;

/// The frame that holds a sheet.
///
/// The frame owns the open state, so the trigger and the panel always
/// agree on whether the sheet is showing.
///
/// # Arguments
///
/// - `VirtualNode<EuvSheetProps>` - The props node carrying the component settings.
///
/// # Returns
///
/// - `VirtualNode` - The euv sheet virtual DOM tree.
#[component]
pub fn euv_sheet(node: VirtualNode<EuvSheetProps>) -> VirtualNode {
    let EuvSheetProps {
        open: _open,
        side: _side,
    }: EuvSheetProps = node.try_get_props().unwrap_or_default();
    let children: VirtualNode = node.get_children().into();
    html! {
        div {
            class: c_euv_sheet()
            children
        }
    }
}

/// The button that opens the sheet.
///
/// # Arguments
///
/// - `VirtualNode<EuvSheetTriggerProps>` - The props node carrying the component settings.
///
/// # Returns
///
/// - `VirtualNode` - The euv sheet trigger virtual DOM tree.
#[component]
pub fn euv_sheet_trigger(node: VirtualNode<EuvSheetTriggerProps>) -> VirtualNode {
    let EuvSheetTriggerProps { open, label }: EuvSheetTriggerProps =
        node.try_get_props().unwrap_or_default();
    let on_click: Option<Rc<dyn Fn(Event)>> = Some(Rc::new(move |_: Event| {
        open.set(true);
    }));
    html! {
        button {
            type: "button"
            "aria-expanded": if { open.get() } {
                "true"
            } else {
                "false"
            }
            class: c_euv_sheet_trigger()
            onclick: on_click
            {
                label
            }
        }
    }
}

/// The veil that covers the page while the sheet is open.
///
/// # Arguments
///
/// - `VirtualNode<EuvSheetOverlayProps>` - The props node carrying the component settings.
///
/// # Returns
///
/// - `VirtualNode` - The euv sheet overlay virtual DOM tree.
#[component]
pub fn euv_sheet_overlay(node: VirtualNode<EuvSheetOverlayProps>) -> VirtualNode {
    let EuvSheetOverlayProps { open }: EuvSheetOverlayProps =
        node.try_get_props().unwrap_or_default();
    html! {
        div {
            class: c_euv_sheet_overlay()
            class: if { open } {
                c_euv_sheet_overlay_open()
            } else {
                c_euv_sheet_overlay_closed()
            }
        }
    }
}

/// The panel of the sheet.
///
/// The panel is a dialog: it carries the role and the modal flag, so the
/// page behind it is announced as unreachable.
///
/// # Arguments
///
/// - `VirtualNode<EuvSheetContentProps>` - The props node carrying the component settings.
///
/// # Returns
///
/// - `VirtualNode` - The euv sheet content virtual DOM tree.
#[component]
pub fn euv_sheet_content(node: VirtualNode<EuvSheetContentProps>) -> VirtualNode {
    let EuvSheetContentProps { open, side }: EuvSheetContentProps =
        node.try_get_props().unwrap_or_default();
    let side_class: fn() -> &'static Css = match side {
        EuvSheetSide::Top => c_euv_sheet_content_top,
        EuvSheetSide::Bottom => c_euv_sheet_content_bottom,
        EuvSheetSide::Left => c_euv_sheet_content_left,
        EuvSheetSide::Right => c_euv_sheet_content_right,
    };
    let children: VirtualNode = node.get_children().into();
    html! {
        div {
            role: "dialog"
            "aria-modal": "true"
            class: c_euv_sheet_content()
            class: side_class()
            class: if { open } {
                c_euv_sheet_content_open()
            } else {
                c_euv_sheet_content_closed()
            }
            children
        }
    }
}

/// The head of the sheet: its title and its description.
///
/// # Arguments
///
/// - `VirtualNode<EuvSheetHeaderProps>` - The props node carrying the component settings.
///
/// # Returns
///
/// - `VirtualNode` - The euv sheet header virtual DOM tree.
#[component]
pub fn euv_sheet_header(node: VirtualNode<EuvSheetHeaderProps>) -> VirtualNode {
    let EuvSheetHeaderProps { label }: EuvSheetHeaderProps =
        node.try_get_props().unwrap_or_default();
    let children: VirtualNode = node.get_children().into();
    html! {
        div {
            "aria-label": label
            class: c_euv_sheet_header()
            children
        }
    }
}

/// The foot of the sheet: the actions that end the interaction.
///
/// # Arguments
///
/// - `VirtualNode<EuvSheetFooterProps>` - The props node carrying the component settings.
///
/// # Returns
///
/// - `VirtualNode` - The euv sheet footer virtual DOM tree.
#[component]
pub fn euv_sheet_footer(node: VirtualNode<EuvSheetFooterProps>) -> VirtualNode {
    let EuvSheetFooterProps { label }: EuvSheetFooterProps =
        node.try_get_props().unwrap_or_default();
    let children: VirtualNode = node.get_children().into();
    html! {
        div {
            "aria-label": label
            class: c_euv_sheet_footer()
            children
        }
    }
}

/// The title of the sheet.
///
/// # Arguments
///
/// - `VirtualNode<EuvSheetTitleProps>` - The props node carrying the component settings.
///
/// # Returns
///
/// - `VirtualNode` - The euv sheet title virtual DOM tree.
#[component]
pub fn euv_sheet_title(node: VirtualNode<EuvSheetTitleProps>) -> VirtualNode {
    let EuvSheetTitleProps { text }: EuvSheetTitleProps = node.try_get_props().unwrap_or_default();
    html! {
        h2 {
            class: c_euv_sheet_title()
            {
                text
            }
        }
    }
}

/// The description of the sheet.
///
/// # Arguments
///
/// - `VirtualNode<EuvSheetDescriptionProps>` - The props node carrying the component settings.
///
/// # Returns
///
/// - `VirtualNode` - The euv sheet description virtual DOM tree.
#[component]
pub fn euv_sheet_description(node: VirtualNode<EuvSheetDescriptionProps>) -> VirtualNode {
    let EuvSheetDescriptionProps { text }: EuvSheetDescriptionProps =
        node.try_get_props().unwrap_or_default();
    html! {
        p {
            class: c_euv_sheet_description()
            {
                text
            }
        }
    }
}

/// The button that closes the sheet.
///
/// # Arguments
///
/// - `VirtualNode<EuvSheetCloseProps>` - The props node carrying the component settings.
///
/// # Returns
///
/// - `VirtualNode` - The euv sheet close virtual DOM tree.
#[component]
pub fn euv_sheet_close(node: VirtualNode<EuvSheetCloseProps>) -> VirtualNode {
    let EuvSheetCloseProps { label }: EuvSheetCloseProps = node.try_get_props().unwrap_or_default();
    html! {
        button {
            type: "button"
            "aria-label": label
            class: c_euv_sheet_close()
        }
    }
}
