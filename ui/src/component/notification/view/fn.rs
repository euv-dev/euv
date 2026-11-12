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
                "data-slot": "toast-title"
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
                "data-slot": "toast-description"
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
                "data-slot": "toast-action"
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
                "data-slot": "toast-close"
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
            "data-slot": "toast"
            class: toast_class()
            children
        }
    }
}

/// The shell a toast is built from.
///
/// The reference lets the caller compose a toast out of a headline, a
/// body line, an action and a close button. This component is the box
/// those parts live in: it carries the `toast` slot and the card
/// styling, and it keeps whatever the caller puts inside it.
///
/// # Arguments
///
/// - `VirtualNode<EuvToastCardProps>` - The props node carrying the component settings.
///
/// # Returns
///
/// - `VirtualNode` - The euv toast card virtual DOM tree.
#[component]
pub fn euv_toast_card(node: VirtualNode<EuvToastCardProps>) -> VirtualNode {
    let EuvToastCardProps {}: EuvToastCardProps = node.try_get_props().unwrap_or_default();
    let children: VirtualNode = node.get_children().into();
    html! {
        div {
            role: "status"
            "aria-live": "polite"
            "data-slot": "toast"
            class: c_euv_toast()
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
            "data-slot": "toast-title"
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
            "data-slot": "toast-description"
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
            "data-slot": "toast-action"
            class: c_euv_toast_action()
            onclick: handler
            {
                label
            }
        }
    }
}

/// The provider that owns the queue of the toasts.
///
/// The reference hands the queue to a provider that sits above the
/// viewport and hands every toast to it. euv has no context, so the
/// provider renders a wrapper that keeps its children in the flow of the
/// page and leaves the queue to the caller.
///
/// # Arguments
///
/// - `VirtualNode<EuvToastProviderProps>` - The props node carrying the component settings.
///
/// # Returns
///
/// - `VirtualNode` - The euv toast provider virtual DOM tree.
#[component]
pub fn euv_toast_provider(node: VirtualNode<EuvToastProviderProps>) -> VirtualNode {
    let EuvToastProviderProps {}: EuvToastProviderProps = node.try_get_props().unwrap_or_default();
    let children: VirtualNode = node.get_children().into();
    html! {
        div {
            "data-slot": "toast-provider"
            class: c_euv_toast_provider()
            children
        }
    }
}

/// The fixed box the toasts stack inside.
///
/// The viewport is the anchor the toasts are placed in: it is pinned to
/// the edge of the viewport and the toasts inside it stack downwards.
///
/// # Arguments
///
/// - `VirtualNode<EuvToastViewportProps>` - The props node carrying the component settings.
///
/// # Returns
///
/// - `VirtualNode` - The euv toast viewport virtual DOM tree.
#[component]
pub fn euv_toast_viewport(node: VirtualNode<EuvToastViewportProps>) -> VirtualNode {
    let EuvToastViewportProps {}: EuvToastViewportProps = node.try_get_props().unwrap_or_default();
    let children: VirtualNode = node.get_children().into();
    html! {
        div {
            "data-slot": "toast-viewport"
            class: c_euv_toast_viewport()
            children
        }
    }
}

/// The button that dismisses one toast.
///
/// The close button reports the click to the caller, which owns the
/// queue and decides what dismissing a toast means.
///
/// # Arguments
///
/// - `VirtualNode<EuvToastCloseProps>` - The props node carrying the component settings.
///
/// # Returns
///
/// - `VirtualNode` - The euv toast close virtual DOM tree.
#[component]
pub fn euv_toast_close(node: VirtualNode<EuvToastCloseProps>) -> VirtualNode {
    let EuvToastCloseProps { on_click }: EuvToastCloseProps =
        node.try_get_props().unwrap_or_default();
    let handler: Option<Rc<dyn Fn(Event)>> = Some(Rc::new(move |_: Event| {
        if let Some(callback) = &on_click {
            callback();
        }
    }));
    let children: VirtualNode = node.get_children().into();
    html! {
        button {
            type: "button"
            "data-slot": "toast-close"
            "aria-label": "Dismiss"
            class: c_euv_toast_close()
            onclick: handler
            children
        }
    }
}
