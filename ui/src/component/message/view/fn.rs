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
                    class: c_euv_message_action()
                    onclick: action_handler
                    action_label
                }
            }
            if is_dismissible {
                button {
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
