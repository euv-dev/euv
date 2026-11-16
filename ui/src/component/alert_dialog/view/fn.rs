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
    let EuvAlertDialogProps { open }: EuvAlertDialogProps =
        node.try_get_props().unwrap_or_default();
    let children: VirtualNode = node.get_children().into();
    html! {
        div {
            "data-slot": "alert-dialog"
            "data-state": if { open.get() } {
                "open"
            } else {
                "closed"
            }
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
    let EuvAlertDialogTriggerProps {
        open,
        content_id,
        label,
    }: EuvAlertDialogTriggerProps = node.try_get_props().unwrap_or_default();
    let on_click: Option<Rc<dyn Fn(Event)>> = Some(Rc::new(move |event: Event| {
        let root: Option<Element> = alert_dialog_root_from_event(&event);
        open.set(true);
        if let Some(root) = root {
            alert_dialog_schedule_focus(root, false);
        }
    }));

    html! {
            button {
                type: "button"
                "aria-expanded": if { open.get() } {
                    "true"
                } else {
                    "false"
                }
                "aria-haspopup": ALERT_DIALOG_HASPOPUP_DIALOG
                "aria-controls": content_id
                class: c_euv_alert_dialog_trigger()

                "data-slot": "alert-dialog-trigger"
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
            "data-slot": "alert-dialog-overlay"
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
    let EuvAlertDialogContentProps {
        open,
        id,
        title_id,
        description_id,
    }: EuvAlertDialogContentProps = node.try_get_props().unwrap_or_default();
    let children: VirtualNode = node.get_children().into();
    let on_keydown: Option<Rc<dyn Fn(Event)>> = alert_dialog_content_keydown(open);
    let aria_hidden: &str = if open.get() { "false" } else { "true" };
    html! {
        div {
            id: id
            role: "alertdialog"
            "aria-modal": "true"
            "aria-hidden": aria_hidden
            "aria-labelledby": title_id
            "aria-describedby": description_id
            tabindex: "-1"
            "data-slot": "alert-dialog-content"
            "data-size": ALERT_DIALOG_DATA_SIZE_DEFAULT
            class: c_euv_alert_dialog_content()
            class: if { open } {
                c_euv_alert_dialog_content_open()
            } else {
                c_euv_alert_dialog_content_closed()
            }
            onkeydown: on_keydown
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
            "data-slot": "alert-dialog-header"
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
            "data-slot": "alert-dialog-footer"
            class: c_euv_alert_dialog_footer()
            children
        }
    }
}

/// The glyph the dialog leads with.
///
/// # Arguments
///
/// - `VirtualNode<EuvAlertDialogMediaProps>` - The props node carrying the
///   component settings.
///
/// # Returns
///
/// - `VirtualNode` - The euv alert dialog media virtual DOM tree.
#[component]
pub fn euv_alert_dialog_media(node: VirtualNode<EuvAlertDialogMediaProps>) -> VirtualNode {
    let EuvAlertDialogMediaProps { glyph }: EuvAlertDialogMediaProps =
        node.try_get_props().unwrap_or_default();
    html! {
        span {
            "data-slot": "alert-dialog-media"
            "aria-hidden": "true"
            class: c_euv_alert_dialog_media()
            {
                glyph
            }
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
    let EuvAlertDialogTitleProps { id, text }: EuvAlertDialogTitleProps =
        node.try_get_props().unwrap_or_default();
    html! {
        h2 {
            id: id
            "data-slot": "alert-dialog-title"
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
    let EuvAlertDialogDescriptionProps { id, text }: EuvAlertDialogDescriptionProps =
        node.try_get_props().unwrap_or_default();
    html! {
        p {
            id: id
            "data-slot": "alert-dialog-description"
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
    let EuvAlertDialogActionProps {
        label,
        variant,
        open,
        onclick,
    }: EuvAlertDialogActionProps = node.try_get_props().unwrap_or_default();
    let on_click: Option<Rc<dyn Fn(Event)>> = Some(Rc::new(move |event: Event| {
        let root: Option<Element> = alert_dialog_root_from_event(&event);
        if let Some(callback) = &onclick {
            callback(event);
        }
        open.set(false);
        if let Some(root) = root {
            alert_dialog_schedule_focus(root, true);
        }
    }));
    html! {
        euv_button {
            label: label
            variant: variant.unwrap_or(EuvButtonVariant::Primary)
            size: EuvButtonSize::Default
            data_slot: Some(ALERT_DIALOG_ACTION_SLOT)
            onclick: on_click
        }
    }
}

/// Finds the alert-dialog root that contains an event target.
///
/// # Arguments
///
/// - `&Event` - The event whose target is inside the dialog.
///
/// # Returns
///
/// - `Option<Element>` - The containing dialog root, when found.
fn alert_dialog_root_from_event(event: &Event) -> Option<Element> {
    event
        .target()
        .and_then(|target: EventTarget| target.dyn_into::<Element>().ok())
        .and_then(|target: Element| target.closest(ALERT_DIALOG_ROOT_SELECTOR).ok().flatten())
}

/// Schedules focus on the dialog's first control or its trigger.
///
/// # Arguments
///
/// - `Element` - The containing alert-dialog root.
/// - `bool` - Whether focus should return to the trigger.
fn alert_dialog_schedule_focus(root: Element, focus_trigger: bool) {
    let callback: Function = Closure::once_into_js(move || {
        let target: Option<Element> = if focus_trigger {
            root.query_selector(ALERT_DIALOG_TRIGGER_SELECTOR)
                .ok()
                .flatten()
        } else {
            root.query_selector(ALERT_DIALOG_CANCEL_SELECTOR)
                .ok()
                .flatten()
                .or_else(|| {
                    root.query_selector(ALERT_DIALOG_ACTION_SELECTOR)
                        .ok()
                        .flatten()
                })
                .or_else(|| {
                    root.query_selector(ALERT_DIALOG_CONTENT_SELECTOR)
                        .ok()
                        .flatten()
                })
        };
        if let Some(target) =
            target.and_then(|target: Element| target.dyn_into::<HtmlElement>().ok())
        {
            let _: Result<(), JsValue> = target.focus();
        }
    })
    .unchecked_into();
    if let Some(window) = web_sys::window() {
        let _: Result<i32, JsValue> = window.request_animation_frame(&callback);
    }
}

/// Traps focus in the alert dialog and closes it on Escape.
///
/// # Arguments
///
/// - `Signal<bool>` - The shared dialog open state.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - The keyboard handler for the dialog panel.
fn alert_dialog_content_keydown(open: Signal<bool>) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |event: Event| {
        let root: Option<Element> = alert_dialog_root_from_event(&event);
        let Ok(keyboard_event) = event.dyn_into::<KeyboardEvent>() else {
            return;
        };
        let key: String = keyboard_event.key();
        if key == ALERT_DIALOG_KEY_ESCAPE && open.get() {
            keyboard_event.prevent_default();
            open.set(false);
            if let Some(root) = root {
                alert_dialog_schedule_focus(root, true);
            }
            return;
        }
        if key != ALERT_DIALOG_KEY_TAB || !open.get() {
            return;
        }
        let Some(root) = root else {
            return;
        };
        let Ok(Some(content)) = root.query_selector(ALERT_DIALOG_CONTENT_SELECTOR) else {
            return;
        };
        let Ok(nodes) = content.query_selector_all(ALERT_DIALOG_FOCUSABLE_SELECTOR) else {
            return;
        };
        let focusables: Vec<HtmlElement> = (0..nodes.length())
            .filter_map(|index: u32| {
                nodes
                    .item(index)
                    .and_then(|node: Node| node.dyn_into::<HtmlElement>().ok())
            })
            .collect();
        if focusables.is_empty() {
            keyboard_event.prevent_default();
            if let Ok(content) = content.dyn_into::<HtmlElement>() {
                let _: Result<(), JsValue> = content.focus();
            }
            return;
        }
        let active: Option<HtmlElement> = web_sys::window()
            .and_then(|window: web_sys::Window| window.document())
            .and_then(|document: web_sys::Document| document.active_element())
            .and_then(|element: Element| element.dyn_into::<HtmlElement>().ok());
        let active_index: Option<usize> = focusables.iter().position(|item: &HtmlElement| {
            active
                .as_ref()
                .is_some_and(|active_item: &HtmlElement| active_item == item)
        });
        let should_wrap: bool = if keyboard_event.shift_key() {
            active_index.is_none() || active_index == Some(0)
        } else {
            active_index.is_none() || active_index == Some(focusables.len() - 1)
        };
        if should_wrap {
            keyboard_event.prevent_default();
            let index: usize = if keyboard_event.shift_key() {
                focusables.len() - 1
            } else {
                0
            };
            if let Some(target) = focusables.get(index) {
                let _: Result<(), JsValue> = target.focus();
            }
        }
    }))
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
    let EuvAlertDialogCancelProps {
        label,
        open,
        onclick,
    }: EuvAlertDialogCancelProps = node.try_get_props().unwrap_or_default();
    let on_click: Option<Rc<dyn Fn(Event)>> = Some(Rc::new(move |event: Event| {
        let root: Option<Element> = alert_dialog_root_from_event(&event);
        if let Some(callback) = &onclick {
            callback(event);
        }
        open.set(false);
        if let Some(root) = root {
            alert_dialog_schedule_focus(root, true);
        }
    }));
    html! {
        euv_button {
            label: label
            variant: EuvButtonVariant::Outline
            size: EuvButtonSize::Default
            data_slot: Some(ALERT_DIALOG_CANCEL_SLOT)
            onclick: on_click
        }
    }
}
