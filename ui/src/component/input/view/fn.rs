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
        aria_label,
        readonly,
        disabled,
        wrapper_class,
        required,
        max_length,
        step,
        aria_describedby,
    }: EuvInputProps = node.try_get_props().unwrap_or_default();
    let input_id: UseId = use_id("input");
    let input_id_value: String = if id.is_empty() {
        input_id.get_value().get()
    } else {
        String::from(id)
    };
    let input_label_for: String = input_id_value.clone();
    let effective_name: String = if !name.is_empty() {
        String::from(name)
    } else if !id.is_empty() {
        String::from(id)
    } else {
        String::new()
    };
    let initial_value: String = value.get();
    let reset_value_signal: Signal<String> = value;
    let reset_input_id: String = input_id_value.clone();
    let reset_callback: Rc<dyn Fn()> = Rc::new(move || {
        let reset_value: String = initial_value.clone();
        reset_value_signal.set(reset_value.clone());
        let input: Option<HtmlInputElement> = window()
            .and_then(|window: Window| window.document())
            .and_then(|document: Document| document.get_element_by_id(&reset_input_id))
            .and_then(|element: Element| element.dyn_into::<HtmlInputElement>().ok());
        if let Some(input) = input {
            input.set_value(&reset_value);
        }
    });
    use_form_reset(input_id_value.clone(), "", reset_callback);
    let effective_type: &'static str = if input_type.is_empty() {
        "text"
    } else {
        input_type
    };
    let effective_aria_label: &'static str = if aria_label.is_empty() {
        label_text
    } else {
        aria_label
    };
    let effective_class: Css = if class.get_name().is_empty() {
        c_euv_input().clone()
    } else {
        class
    };
    let shadcn_class: Css = { c_euv_input().clone() };
    html! {
        div {
            class: c_euv_input_wrapper()
            class: wrapper_class
            if { !label_text.is_empty() } {
                label {
                    r#for: input_label_for.clone()
                    class: c_form_label()
                    label_text
                }
            }
            input {
                "data-slot": "input"
                id: input_id_value
                name: effective_name
                type: effective_type
                placeholder: placeholder
                value: value
                autocomplete: autocomplete
                "aria-label": effective_aria_label
                readonly: readonly
                disabled: disabled
                required: required
                maxlength: if max_length > 0 {
                    max_length.to_string()
                } else {
                    String::new()
                }
                step: step
                "aria-describedby": aria_describedby
                class: effective_class
                class: shadcn_class
                oninput: oninput
                onfocus: UseEuvInput::on_focus_scroll_into_view()
                onblur: UseEuvInput::on_blur_restore_height()
            }
        }
    }
}
