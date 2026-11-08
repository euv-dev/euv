use super::*;

/// The frame that holds an alert dialog.
///
/// The frame owns the open state, so the trigger and the panel always
/// agree on whether the dialog is showing.
///
/// # Arguments
///
/// - `VirtualNode<EuvAlertDialogProps>` - The props node carrying the component settings.
///
/// # Returns
///
/// - `VirtualNode` - The euv alert dialog virtual DOM tree.
#[component]
pub fn euv_alert_dialog(node: VirtualNode<EuvAlertDialogProps>) -> VirtualNode {
    let EuvAlertDialogProps { open: _open }: EuvAlertDialogProps =
        node.try_get_props().unwrap_or_default();
    let children: VirtualNode = node.get_children().into();
    html! {
        div {
            class: c_euv_alert_dialog()
            children
        }
    }
}

/// The button that opens the dialog.
///
/// # Arguments
///
/// - `VirtualNode<EuvAlertDialogTriggerProps>` - The props node carrying the component settings.
///
/// # Returns
///
/// - `VirtualNode` - The euv alert dialog trigger virtual DOM tree.
#[component]
pub fn euv_alert_dialog_trigger(node: VirtualNode<EuvAlertDialogTriggerProps>) -> VirtualNode {
    let EuvAlertDialogTriggerProps { open, label }: EuvAlertDialogTriggerProps =
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
            class: c_euv_alert_dialog_trigger()
            onclick: on_click
            {
                label
            }
        }
    }
}

/// The veil that covers the page while the dialog is open.
///
/// # Arguments
///
/// - `VirtualNode<EuvAlertDialogOverlayProps>` - The props node carrying the component settings.
///
/// # Returns
///
/// - `VirtualNode` - The euv alert dialog overlay virtual DOM tree.
#[component]
pub fn euv_alert_dialog_overlay(node: VirtualNode<EuvAlertDialogOverlayProps>) -> VirtualNode {
    let EuvAlertDialogOverlayProps { open }: EuvAlertDialogOverlayProps =
        node.try_get_props().unwrap_or_default();
    html! {
        div {
            class: c_euv_alert_dialog_overlay()
            class: if { open } {
                c_euv_alert_dialog_overlay_open()
            } else {
                c_euv_alert_dialog_overlay_closed()
            }
        }
    }
}

/// The panel of the dialog.
///
/// The panel is an alert dialog: the role tells the screen reader that
/// it interrupts the reader with something they have to answer.
///
/// # Arguments
///
/// - `VirtualNode<EuvAlertDialogContentProps>` - The props node carrying the component settings.
///
/// # Returns
///
/// - `VirtualNode` - The euv alert dialog content virtual DOM tree.
#[component]
pub fn euv_alert_dialog_content(node: VirtualNode<EuvAlertDialogContentProps>) -> VirtualNode {
    let EuvAlertDialogContentProps { open }: EuvAlertDialogContentProps =
        node.try_get_props().unwrap_or_default();
    let children: VirtualNode = node.get_children().into();
    html! {
        div {
            role: "alertdialog"
            "aria-modal": "true"
            class: c_euv_alert_dialog_content()
            class: if { open } {
                c_euv_alert_dialog_content_open()
            } else {
                c_euv_alert_dialog_content_closed()
            }
            children
        }
    }
}

/// The head of the dialog: its title and its description.
///
/// # Arguments
///
/// - `VirtualNode<EuvAlertDialogHeaderProps>` - The props node carrying the component settings.
///
/// # Returns
///
/// - `VirtualNode` - The euv alert dialog header virtual DOM tree.
#[component]
pub fn euv_alert_dialog_header(node: VirtualNode<EuvAlertDialogHeaderProps>) -> VirtualNode {
    let EuvAlertDialogHeaderProps { label }: EuvAlertDialogHeaderProps =
        node.try_get_props().unwrap_or_default();
    let children: VirtualNode = node.get_children().into();
    html! {
        div {
            "aria-label": label
            class: c_euv_alert_dialog_header()
            children
        }
    }
}

/// The foot of the dialog: the two buttons that end it.
///
/// # Arguments
///
/// - `VirtualNode<EuvAlertDialogFooterProps>` - The props node carrying the component settings.
///
/// # Returns
///
/// - `VirtualNode` - The euv alert dialog footer virtual DOM tree.
#[component]
pub fn euv_alert_dialog_footer(node: VirtualNode<EuvAlertDialogFooterProps>) -> VirtualNode {
    let EuvAlertDialogFooterProps { label }: EuvAlertDialogFooterProps =
        node.try_get_props().unwrap_or_default();
    let children: VirtualNode = node.get_children().into();
    html! {
        div {
            "aria-label": label
            class: c_euv_alert_dialog_footer()
            children
        }
    }
}

/// The title of the dialog.
///
/// # Arguments
///
/// - `VirtualNode<EuvAlertDialogTitleProps>` - The props node carrying the component settings.
///
/// # Returns
///
/// - `VirtualNode` - The euv alert dialog title virtual DOM tree.
#[component]
pub fn euv_alert_dialog_title(node: VirtualNode<EuvAlertDialogTitleProps>) -> VirtualNode {
    let EuvAlertDialogTitleProps { text }: EuvAlertDialogTitleProps =
        node.try_get_props().unwrap_or_default();
    html! {
        h2 {
            class: c_euv_alert_dialog_title()
            {
                text
            }
        }
    }
}

/// The description of the dialog.
///
/// # Arguments
///
///
/// # Returns
///
/// - `VirtualNode` - The euv alert dialog description virtual DOM tree.
#[component]
pub fn euv_alert_dialog_description(
    node: VirtualNode<EuvAlertDialogDescriptionProps>,
) -> VirtualNode {
    let EuvAlertDialogDescriptionProps { text }: EuvAlertDialogDescriptionProps =
        node.try_get_props().unwrap_or_default();
    html! {
        p {
            class: c_euv_alert_dialog_description()
            {
                text
            }
        }
    }
}

/// The button that confirms the dialog.
///
/// # Arguments
///
/// - `VirtualNode<EuvAlertDialogActionProps>` - The props node carrying the component settings.
///
/// # Returns
///
/// - `VirtualNode` - The euv alert dialog action virtual DOM tree.
#[component]
pub fn euv_alert_dialog_action(node: VirtualNode<EuvAlertDialogActionProps>) -> VirtualNode {
    let EuvAlertDialogActionProps { label }: EuvAlertDialogActionProps =
        node.try_get_props().unwrap_or_default();
    html! {
        button {
            type: "button"
            class: c_euv_alert_dialog_action()
            {
                label
            }
        }
    }
}

/// The button that dismisses the dialog.
///
/// # Arguments
///
/// - `VirtualNode<EuvAlertDialogCancelProps>` - The props node carrying the component settings.
///
/// # Returns
///
/// - `VirtualNode` - The euv alert dialog cancel virtual DOM tree.
#[component]
pub fn euv_alert_dialog_cancel(node: VirtualNode<EuvAlertDialogCancelProps>) -> VirtualNode {
    let EuvAlertDialogCancelProps { label }: EuvAlertDialogCancelProps =
        node.try_get_props().unwrap_or_default();
    html! {
        button {
            type: "button"
            class: c_euv_alert_dialog_cancel()
            {
                label
            }
        }
    }
}
