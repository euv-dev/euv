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
                class: c_modal_content()
                onclick: on_modal_content_click
                div {
                    class: c_modal_header()
                    h3 {
                        class: c_modal_title()
                        title
                    }
                    button {
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
