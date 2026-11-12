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
                        "data-slot": "dialog-close"
                        "aria-label": DIALOG_CLOSE_ARIA_LABEL
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
    let dialog_ref: NodeRef<HtmlElement> = App::use_node_ref();
    let previous_focus: Rc<RefCell<Option<HtmlElement>>> =
        HookContext::use_hook(|| Rc::new(RefCell::new(None)));
    let _open_subscription: usize = HookContext::use_hook({
        let watched_open: Signal<bool> = open;
        let watched_dialog_ref: NodeRef<HtmlElement> = dialog_ref.clone();
        let watched_previous_focus: Rc<RefCell<Option<HtmlElement>>> = previous_focus.clone();
        move || {
            let callback_open: Signal<bool> = watched_open;
            let callback_dialog_ref: NodeRef<HtmlElement> = watched_dialog_ref;
            let callback_previous_focus: Rc<RefCell<Option<HtmlElement>>> = watched_previous_focus;
            let subscription_id: usize = callback_open.subscribe(move || {
                if callback_open.get() {
                    if let Some(active_element) = window()
                        .and_then(|window: Window| window.document())
                        .and_then(|document: Document| document.active_element())
                        .and_then(|element: Element| element.dyn_into::<HtmlElement>().ok())
                    {
                        *callback_previous_focus.borrow_mut() = Some(active_element);
                    }
                    let focus_ref: NodeRef<HtmlElement> = callback_dialog_ref.clone();
                    let focus_callback: Function = Closure::once_into_js(move || {
                        if let Some(dialog) = focus_ref.get_cloned() {
                            let _: Result<(), JsValue> = dialog.focus();
                        }
                    })
                    .unchecked_into();
                    if let Some(window) = window() {
                        let _: Result<i32, JsValue> =
                            window.request_animation_frame(&focus_callback);
                    }
                } else if let Some(previous_element) = callback_previous_focus.borrow_mut().take() {
                    let _: Result<(), JsValue> = previous_element.focus();
                }
            });
            let cleanup_open: Signal<bool> = callback_open;
            App::use_cleanup(move || cleanup_open.unsubscribe(subscription_id));
            subscription_id
        }
    });
    let close_handler: Rc<dyn Fn(Event)> = Rc::new(move |_: Event| {
        open.set(false);
    });
    let dialog_keydown_ref: NodeRef<HtmlElement> = dialog_ref.clone();
    let escape_handler: Option<Rc<dyn Fn(Event)>> = Some(Rc::new(move |event: Event| {
        let Ok(keyboard_event) = event.dyn_into::<KeyboardEvent>() else {
            return;
        };
        match keyboard_event.key().as_str() {
            "Escape" => {
                keyboard_event.prevent_default();
                open.set(false);
            }
            "Tab" => {
                let Some(dialog) = dialog_keydown_ref.get_cloned() else {
                    return;
                };
                let Ok(focusable_nodes) = dialog.query_selector_all(DIALOG_FOCUSABLE_SELECTOR)
                else {
                    return;
                };
                let Some(first) = focusable_nodes
                    .item(0)
                    .and_then(|node: Node| node.dyn_into::<HtmlElement>().ok())
                else {
                    keyboard_event.prevent_default();
                    let _: Result<(), JsValue> = dialog.focus();
                    return;
                };
                let Some(last) = focusable_nodes
                    .item(focusable_nodes.length() - 1)
                    .and_then(|node: Node| node.dyn_into::<HtmlElement>().ok())
                else {
                    return;
                };
                let Some(active) = window()
                    .and_then(|window: Window| window.document())
                    .and_then(|document: Document| document.active_element())
                    .and_then(|element: Element| element.dyn_into::<HtmlElement>().ok())
                else {
                    return;
                };
                let active_node: Node = active.into();
                let dialog_node: Node = dialog.clone().into();
                let first_node: Node = first.clone().into();
                let last_node: Node = last.clone().into();
                let active_is_dialog: bool = active_node.is_same_node(Some(&dialog_node));
                let active_is_first: bool = active_node.is_same_node(Some(&first_node));
                let active_is_last: bool = active_node.is_same_node(Some(&last_node));
                if keyboard_event.shift_key() && (active_is_first || active_is_dialog) {
                    keyboard_event.prevent_default();
                    let _: Result<(), JsValue> = last.focus();
                } else if !keyboard_event.shift_key() && (active_is_last || active_is_dialog) {
                    keyboard_event.prevent_default();
                    let _: Result<(), JsValue> = first.focus();
                }
            }
            _ => {}
        }
    }));
    html! {
        div {
            "data-slot": "dialog-overlay"
            class: c_euv_dialog_overlay()
            onclick: Some(Rc::clone(&close_handler))
            class: if { open.get() } {
                c_euv_dialog_overlay_open()
            } else {
                c_euv_dialog_overlay_closed()
            }
        }
        div {
            role: "dialog"
            "data-slot": "dialog"
            "aria-modal": "true"
            "aria-label": title
            tabindex: "-1"
            ref: dialog_ref
            onkeydown: escape_handler
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
                label: DIALOG_CLOSE_ARIA_LABEL
                onclick: Some(Rc::clone(&close_handler))
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
            "data-slot": "dialog-header"
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
            "data-slot": "dialog-title"
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
            "data-slot": "dialog-description"
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
            "data-slot": "dialog-footer"
            class: c_euv_dialog_footer()
            children
        }
    }
}

/// The button that opens the dialog.
///
/// The trigger carries the open state the panel reads, so the two always
/// agree on whether the dialog is showing.
///
/// # Arguments
///
/// - `VirtualNode<EuvDialogTriggerProps>` - The props node carrying the
///   open state.
///
/// # Returns
///
/// - `VirtualNode` - The euv dialog trigger virtual DOM tree.
#[component]
pub fn euv_dialog_trigger(node: VirtualNode<EuvDialogTriggerProps>) -> VirtualNode {
    let EuvDialogTriggerProps { open }: EuvDialogTriggerProps =
        node.try_get_props().unwrap_or_default();
    let children: VirtualNode = node.get_children().into();
    html! {
        button {
            type: "button"
            "data-slot": "dialog-trigger"
            "aria-haspopup": "dialog"
            "aria-expanded": if { open.get() } {
                "true"
            } else {
                "false"
            }
            class: c_euv_alert_dialog_trigger()
            onclick: dialog_open(open)
            children
        }
    }
}

/// Builds the handler that opens the dialog.
///
/// # Arguments
///
/// - `Signal<bool>` - The open state of the dialog.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - The open click handler.
fn dialog_open(open: Signal<bool>) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |_: Event| {
        open.set(true);
    }))
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
    let EuvDialogCloseProps { label, onclick }: EuvDialogCloseProps =
        node.try_get_props().unwrap_or_default();
    html! {
        button {
            type: "button"
            "data-slot": "dialog-close"
            "aria-label": label
            class: c_euv_dialog_close()
            onclick: onclick
        }
    }
}

/// Dims the page behind a dialog.
///
/// The overlay covers the page and takes the click, so a click outside the
/// panel of the dialog does not reach the page behind it.
///
/// # Arguments
///
/// - `VirtualNode<EuvDialogOverlayProps>` - The props node carrying whether
///   the dialog is open.
///
/// # Returns
///
/// - `VirtualNode` - The overlay element.
#[component]
pub fn euv_dialog_overlay(node: VirtualNode<EuvDialogOverlayProps>) -> VirtualNode {
    let EuvDialogOverlayProps { open }: EuvDialogOverlayProps =
        node.try_get_props().unwrap_or_default();
    html! {
        div {
            "data-slot": "dialog-overlay"
            "data-state": if { open.get() } {
                "open"
            } else {
                "closed"
            }
            class: c_euv_dialog_overlay()
            class: if { open.get() } {
                c_euv_dialog_overlay_open()
            } else {
                c_euv_dialog_overlay_closed()
            }
        }
    }
}

/// Holds the panel of a dialog.
///
/// The panel is what the reader reads while the dialog is open. It tells
/// the screen reader that it interrupts the reader with something they have
/// to answer.
///
/// # Arguments
///
/// - `VirtualNode<EuvDialogContentProps>` - The props node carrying whether
///   the dialog is open.
///
/// # Returns
///
/// - `VirtualNode` - The panel element.
#[component]
pub fn euv_dialog_content(node: VirtualNode<EuvDialogContentProps>) -> VirtualNode {
    let EuvDialogContentProps { open }: EuvDialogContentProps =
        node.try_get_props().unwrap_or_default();
    let children: VirtualNode = node.get_children().into();
    html! {
        div {
            "data-slot": "dialog-content"
            "data-state": if { open.get() } {
                "open"
            } else {
                "closed"
            }
            role: "dialog"
            "aria-modal": "true"
            class: c_euv_dialog_content()
            class: if { open.get() } {
                c_euv_dialog_content_open()
            } else {
                c_euv_dialog_content_closed()
            }
            children
        }
    }
}
