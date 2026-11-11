use super::*;

/// A frame around a scrolling viewport.
///
/// The frame hides what overflows it, the viewport inside it scrolls,
/// and the bar drawn along the edge is the reader's handle on that
/// scroll. The three are one component, so a caller never has to wire
/// them together by hand.
///
/// The frame is the `scroll-area` part of the reference and the viewport
/// inside it is the `scroll-area-viewport` part, so the two can be
/// addressed from the DOM the way the reference addresses them.
///
/// # Arguments
///
/// - `VirtualNode<EuvScrollAreaProps>` - The props node carrying the component settings.
///
/// # Returns
///
/// - `VirtualNode` - The euv scroll area virtual DOM tree.
#[component]
pub fn euv_scroll_area(node: VirtualNode<EuvScrollAreaProps>) -> VirtualNode {
    let EuvScrollAreaProps { label }: EuvScrollAreaProps = node.try_get_props().unwrap_or_default();
    let children: VirtualNode = node.get_children().into();
    html! {
        div {
            role: "region"
            "aria-label": label
            "data-slot": "scroll-area"
            class: c_euv_scroll_area()
            euv_scroll_area_viewport {
                children
            }
            div {
                class: c_euv_scroll_bar()
                div {
                    class: c_euv_scroll_bar_thumb()
                }
            }
        }
    }
}

/// The part of a scroll area that scrolls.
///
/// The viewport is the `scroll-area-viewport` part of the reference: it
/// holds the content and takes the scroll, while the frame around it
/// keeps the content inside its own bounds.
///
/// # Arguments
///
/// - `VirtualNode<EuvScrollAreaProps>` - The props node carrying the component settings.
///
/// # Returns
///
/// - `VirtualNode` - The euv scroll area viewport virtual DOM tree.
#[component]
pub fn euv_scroll_area_viewport(node: VirtualNode<EuvScrollAreaProps>) -> VirtualNode {
    let children: VirtualNode = node.get_children().into();
    html! {
        div {
            "data-slot": "scroll-area-viewport"
            class: c_euv_scroll_area_viewport()
            children
        }
    }
}

/// The handle the reader drags to scroll the viewport.
///
/// # Arguments
///
/// - `VirtualNode<EuvScrollAreaProps>` - The props node carrying the component settings.
///
/// # Returns
///
/// - `VirtualNode` - The euv scroll bar virtual DOM tree.
#[component]
pub fn euv_scroll_bar(node: VirtualNode<EuvScrollAreaProps>) -> VirtualNode {
    let children: VirtualNode = node.get_children().into();
    html! {
        div {
            "data-slot": "scroll-bar"
            class: c_euv_scroll_bar()
            div {
                class: c_euv_scroll_bar_thumb()
            }
            children
        }
    }
}
