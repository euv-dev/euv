use super::*;

/// A toast stack pinned to the top centre of the viewport.
///
/// The component is purely presentational: it renders the messages it is
/// given and reports the clicks through the item callbacks. It owns no
/// queue and starts no timers, so the auto-dismiss timing stays with the
/// caller — `use_timer` or `App::use_interval` — and the rendered tree
/// stays deterministic and testable. The stack container is always
/// mounted, even when `items` is empty, so a caller can push messages in
/// without remounting the component.
///
/// # Arguments
///
/// - `VirtualNode<EuvMessageProps>` - The props node containing the messages.
///
/// # Returns
///
/// - `VirtualNode` - The message stack virtual DOM tree.
#[component]
pub fn euv_message(node: VirtualNode<EuvMessageProps>) -> VirtualNode {
    let EuvMessageProps { items }: EuvMessageProps = node.try_get_props().unwrap_or_default();
    let messages: Vec<VirtualNode> = items
        .into_iter()
        .enumerate()
        .map(
            |(index, item): (usize, message::view::r#struct::EuvMessageItem)| {
                euv_message_item(index, item)
            },
        )
        .collect();
    html! {
        div {
            role: "status"
            aria-live: "polite"
            "data-slot": "message"
            class: c_euv_message_stack()
            messages
        }
    }
}

/// Renders one message: its text, the optional action and the optional close button.
///
/// # Arguments
///
/// - `usize` - The index of the message in the stack, used as the diff key.
/// - `EuvMessageItem` - The message to render, consumed.
///
/// # Returns
///
/// - `VirtualNode` - The message virtual DOM tree.
fn euv_message_item(index: usize, item: EuvMessageItem) -> VirtualNode {
    let EuvMessageItem {
        text,
        action_label,
        on_action,
        on_close,
        dismissible,
    }: EuvMessageItem = item;
    let has_action: bool = !action_label.is_empty();
    let is_dismissible: bool = dismissible || on_close.is_some();
    let action_handler: Option<Rc<dyn Fn(Event)>> = message_callback_handler(on_action);
    let close_handler: Option<Rc<dyn Fn(Event)>> = message_callback_handler(on_close);
    html! {
        div {
            key: index.to_string()
            class: c_euv_message()
            div {
                class: c_euv_message_text()
                text
            }
            if has_action {
                button {
                    type: "button"
                    class: c_euv_message_action()
                    onclick: action_handler
                    action_label
                }
            }
            if is_dismissible {
                button {
                    type: "button"
                    class: c_euv_message_close()
                    onclick: close_handler
                    "×"
                }
            }
        }
    }
}

/// Builds the click handler that fires a caller's message callback.
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
pub fn message_callback_handler<E: 'static>(
    callback: Option<Rc<dyn Fn()>>,
) -> Option<Rc<dyn Fn(E)>> {
    callback.map(|handler: Rc<dyn Fn()>| -> Rc<dyn Fn(E)> {
        Rc::new(move |_: E| {
            handler();
        })
    })
}

/// Holds a cluster of messages.
///
/// The group stacks the messages one under the other and keeps them
/// from spilling out of the row they sit in.
///
/// # Arguments
///
/// - `VirtualNode<EuvMessageGroupProps>` - The props node carrying the
///   settings of the group.
///
/// # Returns
///
/// - `VirtualNode` - The euv message group virtual DOM tree.
#[component]
pub fn euv_message_group(node: VirtualNode<EuvMessageGroupProps>) -> VirtualNode {
    let EuvMessageGroupProps {}: EuvMessageGroupProps = node.try_get_props().unwrap_or_default();
    let children: VirtualNode = node.get_children().into();
    html! {
        div {
            "data-slot": "message-group"
            class: c_euv_message_group()
            children
        }
    }
}

/// Holds the head of one message.
///
/// The header holds the picture of the sender and the name beside it.
///
/// # Arguments
///
/// - `VirtualNode<EuvMessageHeaderProps>` - The props node carrying the
///   settings of the header.
///
/// # Returns
///
/// - `VirtualNode` - The euv message header virtual DOM tree.
#[component]
pub fn euv_message_header(node: VirtualNode<EuvMessageHeaderProps>) -> VirtualNode {
    let EuvMessageHeaderProps {}: EuvMessageHeaderProps = node.try_get_props().unwrap_or_default();
    let children: VirtualNode = node.get_children().into();
    html! {
        div {
            "data-slot": "message-header"
            class: c_euv_message_header()
            children
        }
    }
}

/// Holds the picture of the sender of one message.
///
/// The avatar is a round frame the picture is cut to, so the messages
/// of one conversation line up with one another.
///
/// # Arguments
///
/// - `VirtualNode<EuvMessageAvatarProps>` - The props node carrying the
///   settings of the avatar.
///
/// # Returns
///
/// - `VirtualNode` - The euv message avatar virtual DOM tree.
#[component]
pub fn euv_message_avatar(node: VirtualNode<EuvMessageAvatarProps>) -> VirtualNode {
    let EuvMessageAvatarProps {}: EuvMessageAvatarProps = node.try_get_props().unwrap_or_default();
    let children: VirtualNode = node.get_children().into();
    html! {
        div {
            "data-slot": "message-avatar"
            class: c_euv_message_avatar()
            children
        }
    }
}

/// Holds the body of one message.
///
/// The content holds the words of the message and takes the width the
/// row gives it, so a long message wraps instead of pushing the row
/// wider. The words sit in the bubble, which is the surface the
/// reference draws them on.
///
/// # Arguments
///
/// - `VirtualNode<EuvMessageContentProps>` - The props node carrying the
///   settings of the content.
///
/// # Returns
///
/// - `VirtualNode` - The euv message content virtual DOM tree.
#[component]
pub fn euv_message_content(node: VirtualNode<EuvMessageContentProps>) -> VirtualNode {
    let EuvMessageContentProps {}: EuvMessageContentProps =
        node.try_get_props().unwrap_or_default();
    let children: VirtualNode = node.get_children().into();
    html! {
        div {
            "data-slot": "message-content"
            class: c_euv_message_content()
            euv_message_bubble {
                children
            }
        }
    }
}

/// The surface the words of one message sit on.
///
/// The bubble is the `bubble` part of the reference: it wraps the words
/// of the message so they read as one block of speech.
///
/// # Arguments
///
/// - `VirtualNode<EuvMessageBubbleProps>` - The props node carrying the
///   settings of the bubble.
///
/// # Returns
///
/// - `VirtualNode` - The euv message bubble virtual DOM tree.
#[component]
pub fn euv_message_bubble(node: VirtualNode<EuvMessageBubbleProps>) -> VirtualNode {
    let EuvMessageBubbleProps {}: EuvMessageBubbleProps = node.try_get_props().unwrap_or_default();
    let children: VirtualNode = node.get_children().into();
    html! {
        div {
            "data-slot": "bubble"
            class: c_euv_message_bubble()
            children
        }
    }
}

/// Holds the trailing actions of one message.
///
/// The footer holds what the reader can do with the message, so the
/// actions sit below the words they belong to.
///
/// # Arguments
///
/// - `VirtualNode<EuvMessageFooterProps>` - The props node carrying the
///   settings of the footer.
///
/// # Returns
///
/// - `VirtualNode` - The euv message footer virtual DOM tree.
#[component]
pub fn euv_message_footer(node: VirtualNode<EuvMessageFooterProps>) -> VirtualNode {
    let EuvMessageFooterProps {}: EuvMessageFooterProps = node.try_get_props().unwrap_or_default();
    let children: VirtualNode = node.get_children().into();
    html! {
        div {
            "data-slot": "message-footer"
            class: c_euv_message_footer()
            children
        }
    }
}
