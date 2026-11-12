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
    let base_class: &Css = c_euv_kbd();
    let shadcn_class: &Css = if cfg!(feature = "theme-shadcn") {
        c_euv_kbd_shadcn()
    } else {
        &EMPTY_CLASS
    };
    html! {
            kbd {
                "data-slot": "kbd"
                class: base_class
                class: shadcn_class
                text
            }
    }
}

/// Holds a group of keys.
///
/// The group holds the keys of one shortcut side by side, so the reader
/// sees the whole shortcut rather than one key at a time.
///
/// # Arguments
///
/// - `VirtualNode<EuvKbdGroupProps>` - The props node.
///
/// # Returns
///
/// - `VirtualNode` - The key group virtual DOM tree.
#[component]
pub fn euv_kbd_group(node: VirtualNode<EuvKbdGroupProps>) -> VirtualNode {
    let EuvKbdGroupProps {}: EuvKbdGroupProps = node.try_get_props().unwrap_or_default();
    let children: VirtualNode = node.get_children().into();
    html! {
            kbd {
                "data-slot": "kbd-group"
                class: c_euv_kbd_group()
                children
            }
    }
}
