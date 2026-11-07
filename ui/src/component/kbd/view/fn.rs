use super::*;

/// A keyboard key.
///
/// The element is a `<kbd>`, so a browser and a screen reader both read
/// it as the key it stands for rather than as plain text.
///
/// # Arguments
///
/// - `VirtualNode<EuvKbdProps>` - The props node.
///
/// # Returns
///
/// - `VirtualNode` - The key virtual DOM tree.
#[component]
pub fn euv_kbd(node: VirtualNode<EuvKbdProps>) -> VirtualNode {
    let EuvKbdProps { text }: EuvKbdProps = node.try_get_props().unwrap_or_default();
    html! {
        kbd {
            class: c_euv_kbd()
            text
        }
    }
}
