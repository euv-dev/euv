use super::*;

/// A notification stack pinned to the top right of the viewport.
///
/// The component is purely presentational: it renders the notifications it
/// is given and reports the clicks through the item callbacks. It owns no
/// queue and starts no timers, so the auto-dismiss timing stays with the
/// caller — `use_timer` or `App::use_interval` — and the rendered tree
/// stays deterministic and testable. The stack container is always
/// mounted, even when `items` is empty, so a caller can push notifications
/// in without remounting the component.
///
/// # Arguments
///
/// - `VirtualNode<EuvNotificationProps>` - The props node containing the notifications.
///
/// # Returns
///
/// - `VirtualNode` - The notification stack virtual DOM tree.
#[component]
pub fn euv_notification(node: VirtualNode<EuvNotificationProps>) -> VirtualNode {
    let EuvNotificationProps { items }: EuvNotificationProps =
        node.try_get_props().unwrap_or_default();
    let notifications: Vec<VirtualNode> = items
        .into_iter()
        .enumerate()
        .map(
            |(index, item): (usize, notification::view::r#struct::EuvNotificationItem)| {
                euv_notification_item(index, item)
            },
        )
        .collect();
    html! {
        div {
            role: "status"
            aria-live: "polite"
            class: c_euv_notification_stack()
            notifications
        }
    }
}

/// Renders one notification: its body and the optional close button.
///
/// # Arguments
///
/// - `usize` - The index of the notification in the stack, used as the diff key.
/// - `EuvNotificationItem` - The notification to render, consumed.
///
/// # Returns
///
/// - `VirtualNode` - The notification virtual DOM tree.
fn euv_notification_item(index: usize, item: EuvNotificationItem) -> VirtualNode {
    let EuvNotificationItem {
        title,
        text,
        on_close,
        dismissible,
    }: EuvNotificationItem = item;
    let is_dismissible: bool = dismissible || on_close.is_some();
    let close_handler: Option<Rc<dyn Fn(Event)>> = notification_callback_handler(on_close);
    html! {
        div {
            key: index.to_string()
            class: c_euv_notification()
            div {
                class: c_euv_notification_body()
                div {
                    class: c_euv_notification_title()
                    title
                }
                div {
                    class: c_euv_notification_text()
                    text
                }
            }
            if is_dismissible {
                button {
                    type: "button"
                    class: c_euv_notification_close()
                    onclick: close_handler
                    "×"
                }
            }
        }
    }
}

/// Builds the click handler that fires a caller's notification callback.
///
/// The event type is a parameter so the DOM-free test suite can drive the
/// handler with a stand-in event; the component always instantiates it
/// with [`Event`].
///
/// # Arguments
///
/// - `Option<Rc<dyn Fn()>>` - The callback fired by the handler.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(E)>>` - The click handler, or `None` when there is no callback.
pub fn notification_callback_handler<E: 'static>(
    callback: Option<Rc<dyn Fn()>>,
) -> Option<Rc<dyn Fn(E)>> {
    callback.map(|handler: Rc<dyn Fn()>| -> Rc<dyn Fn(E)> {
        Rc::new(move |_: E| {
            handler();
        })
    })
}

/// A toast card.
///
/// The card carries `role="status"` and `aria-live="polite"`, so a toast
/// that arrives while the reader is somewhere else is announced without
/// interrupting what they are doing.
///
/// # Arguments
///
/// - `VirtualNode<EuvToastProps>` - The props node carrying the component settings.
///
/// # Returns
///
/// - `VirtualNode` - The euv toast virtual DOM tree.
#[component]
pub fn euv_toast(node: VirtualNode<EuvToastProps>) -> VirtualNode {
    let EuvToastProps {
        variant,
        title,
        description,
        action_label,
        dismissible,
        on_action,
        on_close,
    }: EuvToastProps = node.try_get_props().unwrap_or_default();
    let toast_class: fn() -> &'static Css = match variant {
        EuvToastVariant::Destructive => c_euv_toast_destructive,
        EuvToastVariant::Default => c_euv_toast,
    };
    let mut children: Vec<VirtualNode> = Vec::new();
    if !title.is_empty() {
        children.push(html! {
            div {
                class: c_euv_toast_title()
                {
                    title
                }
            }
        });
    }
    if !description.is_empty() {
        children.push(html! {
            div {
                class: c_euv_toast_description()
                {
                    description
                }
            }
        });
    }
    if !action_label.is_empty() {
        let on_click: Option<Rc<dyn Fn(Event)>> = Some(Rc::new(move |_: Event| {
            if let Some(callback) = &on_action {
                callback();
            }
        }));
        children.push(html! {
            button {
                type: "button"
                class: c_euv_toast_action()
                onclick: on_click
                {
                    action_label
                }
            }
        });
    }
    if dismissible || on_close.is_some() {
        let on_click: Option<Rc<dyn Fn(Event)>> = Some(Rc::new(move |_: Event| {
            if let Some(callback) = &on_close {
                callback();
            }
        }));
        children.push(html! {
            button {
                type: "button"
                class: c_euv_toast_close()
                "aria-label": "Dismiss"
                onclick: on_click
            }
        });
    }
    html! {
        div {
            role: "status"
            "aria-live": "polite"
            class: toast_class()
            children
        }
    }
}

/// The headline of a toast.
///
/// # Arguments
///
/// - `VirtualNode<EuvToastTitleProps>` - The props node carrying the component settings.
///
/// # Returns
///
/// - `VirtualNode` - The euv toast title virtual DOM tree.
#[component]
pub fn euv_toast_title(node: VirtualNode<EuvToastTitleProps>) -> VirtualNode {
    let EuvToastTitleProps { text }: EuvToastTitleProps = node.try_get_props().unwrap_or_default();
    html! {
        div {
            class: c_euv_toast_title()
            {
                text
            }
        }
    }
}

/// The body line of a toast.
///
/// # Arguments
///
/// - `VirtualNode<EuvToastDescriptionProps>` - The props node carrying the component settings.
///
/// # Returns
///
/// - `VirtualNode` - The euv toast description virtual DOM tree.
#[component]
pub fn euv_toast_description(node: VirtualNode<EuvToastDescriptionProps>) -> VirtualNode {
    let EuvToastDescriptionProps { text }: EuvToastDescriptionProps =
        node.try_get_props().unwrap_or_default();
    html! {
        div {
            class: c_euv_toast_description()
            {
                text
            }
        }
    }
}

/// The action button of a toast.
///
/// # Arguments
///
/// - `VirtualNode<EuvToastActionProps>` - The props node carrying the component settings.
///
/// # Returns
///
/// - `VirtualNode` - The euv toast action virtual DOM tree.
#[component]
pub fn euv_toast_action(node: VirtualNode<EuvToastActionProps>) -> VirtualNode {
    let EuvToastActionProps { label, on_click }: EuvToastActionProps =
        node.try_get_props().unwrap_or_default();
    let handler: Option<Rc<dyn Fn(Event)>> = Some(Rc::new(move |_: Event| {
        if let Some(callback) = &on_click {
            callback();
        }
    }));
    html! {
        button {
            type: "button"
            class: c_euv_toast_action()
            onclick: handler
            {
                label
            }
        }
    }
}
