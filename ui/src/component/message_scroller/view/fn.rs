use super::*;

/// Holds a list of messages that scrolls.
///
/// The scroller is the frame the list is drawn in: the viewport inside
/// it scrolls, the content is the column of messages, and the items are
/// the messages themselves.
///
/// # Arguments
///
/// - `VirtualNode<EuvMessageScrollerProps>` - The props node carrying the
///   settings of the scroller.
///
/// # Returns
///
/// - `VirtualNode` - The euv message scroller virtual DOM tree.
#[component]
pub fn euv_message_scroller(node: VirtualNode<EuvMessageScrollerProps>) -> VirtualNode {
    let EuvMessageScrollerProps {}: EuvMessageScrollerProps =
        node.try_get_props().unwrap_or_default();
    let children: VirtualNode = node.get_children().into();
    html! {
        div {
            "data-slot": "message-scroller"
            class: c_euv_message_scroller()
            children
        }
    }
}

/// Holds the scrolling area of a message list.
///
/// The viewport keeps the scroll to itself, so a page that is scrolled
/// past the list does not drag the list with it.
///
/// # Arguments
///
/// - `VirtualNode<EuvMessageScrollerViewportProps>` - The props node
///   carrying the settings of the viewport.
///
/// # Returns
///
/// - `VirtualNode` - The euv message scroller viewport virtual DOM tree.
#[component]
pub fn euv_message_scroller_viewport(
    node: VirtualNode<EuvMessageScrollerViewportProps>,
) -> VirtualNode {
    let EuvMessageScrollerViewportProps {}: EuvMessageScrollerViewportProps =
        node.try_get_props().unwrap_or_default();
    let children: VirtualNode = node.get_children().into();
    html! {
        div {
            "data-slot": "message-scroller-viewport"
            class: c_euv_message_scroller_viewport()
            children
        }
    }
}

/// Holds the column of messages inside the viewport.
///
/// # Arguments
///
/// - `VirtualNode<EuvMessageScrollerContentProps>` - The props node
///   carrying the settings of the content.
///
/// # Returns
///
/// - `VirtualNode` - The euv message scroller content virtual DOM tree.
#[component]
pub fn euv_message_scroller_content(
    node: VirtualNode<EuvMessageScrollerContentProps>,
) -> VirtualNode {
    let EuvMessageScrollerContentProps {}: EuvMessageScrollerContentProps =
        node.try_get_props().unwrap_or_default();
    let children: VirtualNode = node.get_children().into();
    html! {
        div {
            "data-slot": "message-scroller-content"
            class: c_euv_message_scroller_content()
            children
        }
    }
}

/// Holds one message inside the list.
///
/// The item never shrinks, so a long message keeps its own height
/// instead of being squeezed into the row.
///
/// # Arguments
///
/// - `VirtualNode<EuvMessageScrollerItemProps>` - The props node carrying
///   the anchor flag of the item.
///
/// # Returns
///
/// - `VirtualNode` - The euv message scroller item virtual DOM tree.
#[component]
pub fn euv_message_scroller_item(node: VirtualNode<EuvMessageScrollerItemProps>) -> VirtualNode {
    let EuvMessageScrollerItemProps { scroll_anchor }: EuvMessageScrollerItemProps =
        node.try_get_props().unwrap_or_default();
    let children: VirtualNode = node.get_children().into();
    html! {
        div {
            "data-slot": "message-scroller-item"
            "data-scroll-anchor": if scroll_anchor {
                "true"
            } else {
                "false"
            }
            class: c_euv_message_scroller_item()
            children
        }
    }
}

/// Jumps the list to its start or its end.
///
/// The button floats over the list and sits at the end of the viewport
/// when it jumps to the end, and at the start when it jumps to the
/// start.
///
/// # Arguments
///
/// - `VirtualNode<EuvMessageScrollerButtonProps>` - The props node
///   carrying the direction the button jumps to.
///
/// # Returns
///
/// - `VirtualNode` - The euv message scroller button virtual DOM tree.
#[component]
pub fn euv_message_scroller_button(
    node: VirtualNode<EuvMessageScrollerButtonProps>,
) -> VirtualNode {
    let EuvMessageScrollerButtonProps { direction }: EuvMessageScrollerButtonProps =
        node.try_get_props().unwrap_or_default();
    let direction_attr: &str = match direction {
        EuvMessageScrollerDirection::Start => "start",
        EuvMessageScrollerDirection::End => "end",
    };
    let direction_class: fn() -> &'static Css = match direction {
        EuvMessageScrollerDirection::Start => c_euv_message_scroller_button_start,
        EuvMessageScrollerDirection::End => c_euv_message_scroller_button_end,
    };
    let children: VirtualNode = node.get_children().into();
    html! {
        button {
            "data-slot": "message-scroller-button"
            "data-direction": direction_attr
            type: "button"
            class: c_euv_message_scroller_button()
            class: direction_class()
            children
        }
    }
}
