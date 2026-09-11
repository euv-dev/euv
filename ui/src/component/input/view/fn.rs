use super::*;

/// A custom input component with label and event handling.
///
/// The label is optional — leave it empty to render an input without a label.
///
/// # Arguments
///
/// - `VirtualNode<EuvInputProps>` - The props node containing id, label, placeholder, value, autocomplete, etc.
///
/// # Returns
///
/// - `VirtualNode` - A labeled input element.
#[component]
pub fn euv_input(node: VirtualNode<EuvInputProps>) -> VirtualNode {
    let EuvInputProps {
        id,
        name,
        label: label_text,
        input_type,
        placeholder,
        value,
        autocomplete,
        oninput,
        class,
    }: EuvInputProps = node.try_get_props().unwrap_or_default();
    let effective_name: &'static str = if name.is_empty() { id } else { name };
    let effective_type: &'static str = if input_type.is_empty() {
        "text"
    } else {
        input_type
    };
    let effective_class: Css = if class.get_name().is_empty() {
        c_euv_input().clone()
    } else {
        class
    };
    html! {
        div {
            class: c_euv_input_wrapper()
            if { !label_text.is_empty() } {
                label {
                    for: id
                    class: c_form_label()
                    label_text
                }
            }
            input {
                id: id
                name: effective_name
                type: effective_type
                placeholder: placeholder
                value: value
                autocomplete: autocomplete
                class: effective_class
                oninput: oninput
            }
        }
    }
}
