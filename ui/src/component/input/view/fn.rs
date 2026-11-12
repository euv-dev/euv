use super::*;

/// A custom input component with label and event handling.
///
/// Internally binds `onfocus` and `onblur` so that mobile virtual keyboards
/// never obscure the field. The handler reads `--euv-keyboard-height`
/// (provided by the Tauri host / page-level bridge) and scrolls the input
/// above the IME with a 12px gap. The label is optional — leave it empty to
/// render an input without a label.
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
    let shadcn_class: Css = if cfg!(feature = "theme-shadcn") {
        c_euv_input_shadcn().clone()
    } else {
        Css::default()
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
                "data-slot": "input"
                id: id
                name: effective_name
                type: effective_type
                placeholder: placeholder
                value: value
                autocomplete: autocomplete
                class: effective_class
                class: shadcn_class
                oninput: oninput
                onfocus: UseEuvInput::on_focus_scroll_into_view()
                onblur: UseEuvInput::on_blur_restore_height()
            }
        }
    }
}
