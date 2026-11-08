use super::*;

/// A modal dialog component with overlay, title, and close handler.
///
/// # Arguments
///
/// - `VirtualNode<EuvModalProps>` - The props node containing title and onclick.
///
/// # Returns
///
/// - `VirtualNode` - A modal overlay element.
#[component]
pub fn euv_modal(node: VirtualNode<EuvModalProps>) -> VirtualNode {
    let EuvModalProps { title, onclick }: EuvModalProps = node.try_get_props().unwrap_or_default();
    let children: VirtualNode = node.get_children().into();
    let on_modal_content_click: Box<dyn FnMut(Event)> = Box::new(move |_: Event| {});
    html! {
        div {
            class: c_modal_overlay()
            onclick: onclick.clone()
            div {
                role: "dialog"
                aria-modal: "true"
                class: c_modal_content()
                onclick: on_modal_content_click
                div {
                    class: c_modal_header()
                    h3 {
                        class: c_modal_title()
                        title
                    }
                    button {
                        type: "button"
                        class: c_modal_close_button()
                        onclick: onclick
                        "×"
                    }
                }
                div {
                    class: c_modal_body()
                    children
                }
            }
        }
    }
}

/// The panel of a dialog.
///
/// The panel is a dialog: it carries the role and the modal flag, so the
/// page behind it is announced as unreachable. The title and the
/// description are drawn by the panel itself, which is what the
/// reference does with its header.
///
/// # Arguments
///
/// - `VirtualNode<EuvDialogProps>` - The props node carrying the component settings.
///
/// # Returns
///
/// - `VirtualNode` - The euv dialog virtual DOM tree.
#[component]
pub fn euv_dialog(node: VirtualNode<EuvDialogProps>) -> VirtualNode {
    let EuvDialogProps {
        open,
        title,
        description,
    }: EuvDialogProps = node.try_get_props().unwrap_or_default();
    let children: VirtualNode = node.get_children().into();
    html! {
        div {
            class: c_euv_dialog_overlay()
            class: if { open.get() } {
                c_euv_dialog_overlay_open()
            } else {
                c_euv_dialog_overlay_closed()
            }
        }
        div {
            role: "dialog"
            "aria-modal": "true"
            class: c_euv_dialog()
            class: if { open.get() } {
                c_euv_dialog_open()
            } else {
                c_euv_dialog_closed()
            }
            euv_dialog_header {
                label: title
                euv_dialog_title {
                    text: title
                }
                if !description.is_empty() {
                    euv_dialog_description {
                        text: description
                    }
                }
            }
            children
            euv_dialog_close {
                label: "Close"
            }
        }
    }
}

/// The head of the dialog: its title and its description.
///
/// # Arguments
///
/// - `VirtualNode<EuvDialogHeaderProps>` - The props node carrying the component settings.
///
/// # Returns
///
/// - `VirtualNode` - The euv dialog header virtual DOM tree.
#[component]
pub fn euv_dialog_header(node: VirtualNode<EuvDialogHeaderProps>) -> VirtualNode {
    let EuvDialogHeaderProps { label }: EuvDialogHeaderProps =
        node.try_get_props().unwrap_or_default();
    let children: VirtualNode = node.get_children().into();
    html! {
        div {
            "aria-label": label
            class: c_euv_dialog_header()
            children
        }
    }
}

/// The title of the dialog.
///
/// # Arguments
///
/// - `VirtualNode<EuvDialogTitleProps>` - The props node carrying the component settings.
///
/// # Returns
///
/// - `VirtualNode` - The euv dialog title virtual DOM tree.
#[component]
pub fn euv_dialog_title(node: VirtualNode<EuvDialogTitleProps>) -> VirtualNode {
    let EuvDialogTitleProps { text }: EuvDialogTitleProps =
        node.try_get_props().unwrap_or_default();
    html! {
        h2 {
            class: c_euv_dialog_title()
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
/// - `VirtualNode<EuvDialogDescriptionProps>` - The props node carrying the component settings.
///
/// # Returns
///
/// - `VirtualNode` - The euv dialog description virtual DOM tree.
#[component]
pub fn euv_dialog_description(node: VirtualNode<EuvDialogDescriptionProps>) -> VirtualNode {
    let EuvDialogDescriptionProps { text }: EuvDialogDescriptionProps =
        node.try_get_props().unwrap_or_default();
    html! {
        p {
            class: c_euv_dialog_description()
            {
                text
            }
        }
    }
}

/// The foot of the dialog: the actions that end the interaction.
///
/// # Arguments
///
/// - `VirtualNode<EuvDialogFooterProps>` - The props node carrying the component settings.
///
/// # Returns
///
/// - `VirtualNode` - The euv dialog footer virtual DOM tree.
#[component]
pub fn euv_dialog_footer(node: VirtualNode<EuvDialogFooterProps>) -> VirtualNode {
    let EuvDialogFooterProps { label }: EuvDialogFooterProps =
        node.try_get_props().unwrap_or_default();
    let children: VirtualNode = node.get_children().into();
    html! {
        div {
            "aria-label": label
            class: c_euv_dialog_footer()
            children
        }
    }
}

/// The button that closes the dialog.
///
/// # Arguments
///
/// - `VirtualNode<EuvDialogCloseProps>` - The props node carrying the component settings.
///
/// # Returns
///
/// - `VirtualNode` - The euv dialog close virtual DOM tree.
#[component]
pub fn euv_dialog_close(node: VirtualNode<EuvDialogCloseProps>) -> VirtualNode {
    let EuvDialogCloseProps { label }: EuvDialogCloseProps =
        node.try_get_props().unwrap_or_default();
    html! {
        button {
            type: "button"
            "aria-label": label
            class: c_euv_dialog_close()
        }
    }
}
