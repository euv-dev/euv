use super::*;

/// A caption that names a form control.
///
/// The element is a `<label>` carrying the `for` attribute, so a click on
/// the text moves the focus to the control it names and a screen reader
/// announces the control under that name.
///
/// A disabled label takes the muted treatment, mirroring the way the
/// reference dims a label whose control is disabled.
///
/// # Arguments
///
/// - `VirtualNode<EuvLabelProps>` - The props node carrying the target
///   id, the text and the disabled state.
///
/// # Returns
///
/// - `VirtualNode` - The label virtual DOM tree.
#[component]
pub fn euv_label(node: VirtualNode<EuvLabelProps>) -> VirtualNode {
    let EuvLabelProps {
        html_for,
        text,
        disabled,
        class: custom_class,
    }: EuvLabelProps = node.try_get_props().unwrap_or_default();
    let children: VirtualNode = node.get_children().into();
    let has_children: bool = !matches!(&children, VirtualNode::Empty);
    html! {
        label {
            for: html_for
            "data-slot": "label"
            class: if { disabled.get() } {
                c_euv_label_disabled()
            } else {
                c_euv_label()
            }
            class: custom_class
            if has_children {
                children
            } else {
                text
            }
        }
    }
}
