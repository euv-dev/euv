use super::*;

/// A back-to-top button.
///
/// Renders a fixed button that is on screen only while the `visible`
/// signal says so: it carries `c_euv_back_top_visible` when it is
/// showing and `c_euv_back_top_hidden` when it is not. The button stays
/// mounted either way — it is hidden with a class, not removed, so it
/// cannot lose its place in the stacking order.
///
/// # Arguments
///
/// - `VirtualNode<EuvBackTopProps>` - The props node containing the
///   button configuration.
///
/// # Returns
///
/// - `VirtualNode` - The back-to-top virtual DOM tree.
#[component]
pub fn euv_back_top(node: VirtualNode<EuvBackTopProps>) -> VirtualNode {
    let EuvBackTopProps {
        visible,
        label,
        on_click,
    }: EuvBackTopProps = node.try_get_props().unwrap_or_default();
    let is_visible: bool = visible.get();
    html! {
        button {
            class: if is_visible {
                c_euv_back_top_visible()
            } else {
                c_euv_back_top_hidden()
            }
            onclick: on_click
            label
        }
    }
}
