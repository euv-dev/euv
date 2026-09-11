use super::*;

/// A form field component with label, input, and validation error display.
///
/// Renders a labeled input wrapped in `c_euv_input_wrapper`. When the `error`
/// signal is provided and non-empty, the input switches to error styling
/// (`c_euv_input_error`) and the error text is displayed below.
///
/// # Arguments
///
/// - `VirtualNode<EuvFieldProps>` - The props node containing field configuration.
///
/// # Returns
///
/// - `VirtualNode` - A labeled form field element.
#[component]
pub fn euv_field(node: VirtualNode<EuvFieldProps>) -> VirtualNode {
    let EuvFieldProps {
        id,
        name,
        label: label_text,
        input_type,
        placeholder,
        autocomplete,
        value,
        error,
        oninput: custom_oninput,
    }: EuvFieldProps = node.try_get_props().unwrap_or_default();
    let handler: Option<Rc<dyn Fn(Event)>> =
        custom_oninput.or_else(|| UseEuvInput::on_input_value(value));
    let error_state: Signal<String> = error
        .as_ref()
        .map(|error_signal: &Signal<String>| *error_signal)
        .unwrap_or(App::use_signal(String::new));
    html! {
        div {
            class: c_euv_input_wrapper()
            label {
                for: id
                class: c_form_label()
                label_text
            }
            input {
                id: id
                name: name
                type: input_type
                placeholder: placeholder
                value: value
                autocomplete: autocomplete
                class: if { !error_state.get().is_empty() } {
                    c_euv_input_error()
                } else {
                    c_euv_input_no_transition()
                }
                oninput: handler
            }
            if { !error_state.get().is_empty() } {
                p {
                    class: c_field_error_text()
                    error_state.get()
                }
            }
        }
    }
}
