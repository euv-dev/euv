use super::*;

/// A frame around a scrolling viewport.
///
/// The frame hides what overflows it, the viewport inside it scrolls,
/// and the bar drawn along the edge is the reader's handle on that
/// scroll. The three are one component, so a caller never has to wire
/// them together by hand.
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
            class: c_euv_scroll_area()
            div {
                class: c_euv_scroll_area_viewport()
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
            class: c_euv_scroll_bar()
            div {
                class: c_euv_scroll_bar_thumb()
            }
            children
        }
    }
}
