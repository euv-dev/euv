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
    let base_row_class: &Css = c_form_checkbox_row();
    let shadcn_row_class: &Css = if cfg!(feature = "theme-shadcn") {
        c_form_checkbox_row_shadcn()
    } else {
        c_form_checkbox_row()
    };
    let base_checkbox_class: &Css = c_form_checkbox();
    let shadcn_checkbox_class: &Css = if cfg!(feature = "theme-shadcn") {
        c_form_checkbox_shadcn()
    } else {
        c_form_checkbox()
    };
    html! {
        div {
            class: base_row_class
            class: shadcn_row_class
            input {
                id: id
                name: name
                type: "checkbox"
                autocomplete: autocomplete
                checked: checked
                disabled: disabled
                "data-slot": "checkbox"
                class: base_checkbox_class
                class: shadcn_checkbox_class
                onchange: UseEuvInput::on_change_checked(checked)
            }
            if { cfg!(feature = "theme-shadcn") } {
                span {
                    aria-hidden: "true"
                    "data-slot": "checkbox-indicator"
                    class: c_form_checkbox_indicator_shadcn()
                    class: if { checked.get() } {
                        c_form_checkbox_indicator_checked_shadcn()
                    } else {
                        c_form_checkbox_indicator_shadcn()
                    }
                }
            }
            label {
                for: id
                class: c_form_checkbox_label()
                label_text
            }
        }
    }
}
