use super::*;

/// A labeled checkbox component with two-way binding via a signal.
///
/// Renders a checkbox input paired with a label, using `c_form_checkbox_row`,
/// `c_form_checkbox`, and `c_form_checkbox_label` styling.
/// The checked state is bound to the provided signal.
///
/// # Arguments
///
/// - `VirtualNode<EuvCheckboxProps>` - The props node containing checkbox configuration.
///
/// # Returns
///
/// - `VirtualNode` - A styled labeled checkbox element.
#[component]
pub fn euv_checkbox(node: VirtualNode<EuvCheckboxProps>) -> VirtualNode {
    let EuvCheckboxProps {
        id,
        name,
        autocomplete,
        checked,
        disabled,
        label: label_text,
    }: EuvCheckboxProps = node.try_get_props().unwrap_or_default();
    let checkbox_id: UseId = use_id("checkbox");
    let checkbox_id_value: String = if id.is_empty() {
        checkbox_id.get_value().get()
    } else {
        String::from(id)
    };
    let initial_checked: bool = checked.get();
    let reset_checked_signal: Signal<bool> = checked;
    let reset_checkbox_id: String = checkbox_id_value.clone();
    let reset_callback: Rc<dyn Fn()> = Rc::new(move || {
        reset_checked_signal.set(initial_checked);
        let input: Option<HtmlInputElement> = window()
            .and_then(|window: Window| window.document())
            .and_then(|document: Document| document.get_element_by_id(&reset_checkbox_id))
            .and_then(|element: Element| element.dyn_into::<HtmlInputElement>().ok());
        if let Some(input) = input {
            input.set_checked(initial_checked);
        }
    });
    use_form_reset(checkbox_id_value.clone(), "", reset_callback);
    let base_row_class: &Css = c_form_checkbox_row();
    let base_checkbox_class: &Css = c_form_checkbox();
    html! {
        div {
            class: base_row_class
            input {
                id: checkbox_id_value.clone()
                name: name
                type: "checkbox"
                autocomplete: autocomplete
                checked: checked
                disabled: disabled
                "data-slot": "checkbox"
                class: base_checkbox_class
                onchange: UseEuvInput::on_change_checked(checked)
            }
            span {
                aria-hidden: "true"
                "data-slot": "checkbox-indicator"
                class: c_form_checkbox_indicator()
                class: if { checked.get() } {
                    c_form_checkbox_indicator_checked()
                } else {
                    c_form_checkbox_indicator()
                }
            }
            label {
                r#for: checkbox_id_value
                class: c_form_checkbox_label()
                label_text
            }
        }
    }
}
