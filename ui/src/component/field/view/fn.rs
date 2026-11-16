use super::*;

/// A form field component with label, input, and validation error display.
///
/// Renders a labeled input wrapped in `c_euv_input_wrapper`. When the `error`
/// signal is provided and non-empty, the input switches to error styling
/// (`c_euv_input_error`) and the error text is displayed below. The input
/// also binds `onfocus` / `onblur` so mobile virtual keyboards never
/// obscure the field — the handler reads `--euv-keyboard-height` and
/// scrolls the input above the IME with a 12px gap.
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
    let initial_value: String = value.get();
    let reset_value_signal: Signal<String> = value;
    let reset_input_id: String = String::from(id);
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
    use_form_reset(String::from(id), "", reset_callback);
    let handler: Option<Rc<dyn Fn(Event)>> =
        custom_oninput.or_else(|| UseEuvInput::on_input_value(value));
    let error_state: Signal<String> = error
        .as_ref()
        .map(|error_signal: &Signal<String>| *error_signal)
        .unwrap_or(App::use_signal(String::new));
    let base_input_class: &Css = if error_state.get().is_empty() {
        c_euv_input_no_transition()
    } else {
        c_euv_input_error()
    };
    let shadcn_input_class: Css = { c_euv_input().clone() };
    html! {
        div {
            "data-slot": "field"
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
                "data-slot": "input"
                class: base_input_class
                class: shadcn_input_class
                oninput: handler
                onfocus: UseEuvInput::on_focus_scroll_into_view()
                onblur: UseEuvInput::on_blur_restore_height()
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

/// Holds a set of fields that belong together.
///
/// The set is a `fieldset`, so a browser groups the controls inside it
/// and a screen reader announces them as one group.
///
/// # Arguments
///
/// - `VirtualNode<EuvFieldSetProps>` - The props node carrying the
///   settings of the set.
///
/// # Returns
///
/// - `VirtualNode` - The euv field set virtual DOM tree.
#[component]
pub fn euv_field_set(node: VirtualNode<EuvFieldSetProps>) -> VirtualNode {
    let EuvFieldSetProps {}: EuvFieldSetProps = node.try_get_props().unwrap_or_default();
    let children: VirtualNode = node.get_children().into();
    html! {
        fieldset {
            "data-slot": "field-set"
            class: c_euv_field_set()
            children
        }
    }
}

/// Names a set of fields.
///
/// The legend is written at the size of the legend of a set of fields,
/// unless the caller asks for the size of the caption of one control.
///
/// # Arguments
///
/// - `VirtualNode<EuvFieldLegendProps>` - The props node carrying the
///   size the legend is written at.
///
/// # Returns
///
/// - `VirtualNode` - The euv field legend virtual DOM tree.
#[component]
pub fn euv_field_legend(node: VirtualNode<EuvFieldLegendProps>) -> VirtualNode {
    let EuvFieldLegendProps { variant }: EuvFieldLegendProps =
        node.try_get_props().unwrap_or_default();
    let variant_attr: &str = match variant {
        EuvFieldLegendVariant::Legend => FIELD_LEGEND_VARIANT_LEGEND,
        EuvFieldLegendVariant::Label => FIELD_LEGEND_VARIANT_LABEL,
    };
    let size_class: fn() -> &'static Css = match variant {
        EuvFieldLegendVariant::Legend => c_euv_field_legend_legend,
        EuvFieldLegendVariant::Label => c_euv_field_legend_label,
    };
    let children: VirtualNode = node.get_children().into();
    html! {
        legend {
            "data-slot": "field-legend"
            "data-variant": variant_attr
            class: c_euv_field_legend()
            class: size_class()
            children
        }
    }
}

/// Holds the rows of a group of fields.
///
/// The group says how its rows are laid out: stacked one under the
/// other, side by side, or stacked until the group is wide enough to
/// hold them side by side. A group whose fields are in the invalid
/// state says so, so the label, the description and the message inside
/// it all read the same fact.
///
/// # Arguments
///
/// - `VirtualNode<EuvFieldGroupProps>` - The props node carrying the
///   layout and the invalid state of the group.
///
/// # Returns
///
/// - `VirtualNode` - The euv field group virtual DOM tree.
#[component]
pub fn euv_field_group(node: VirtualNode<EuvFieldGroupProps>) -> VirtualNode {
    let EuvFieldGroupProps {
        orientation,
        invalid,
    }: EuvFieldGroupProps = node.try_get_props().unwrap_or_default();
    let orientation_attr: &str = match orientation {
        EuvFieldOrientation::Vertical => FIELD_ORIENTATION_VERTICAL,
        EuvFieldOrientation::Horizontal => FIELD_ORIENTATION_HORIZONTAL,
        EuvFieldOrientation::Responsive => FIELD_ORIENTATION_RESPONSIVE,
    };
    let orientation_class: fn() -> &'static Css = match orientation {
        EuvFieldOrientation::Vertical => c_euv_field_group_vertical,
        EuvFieldOrientation::Horizontal => c_euv_field_group_horizontal,
        EuvFieldOrientation::Responsive => c_euv_field_group_responsive,
    };
    let children: VirtualNode = node.get_children().into();
    html! {
        div {
            role: "group"
            "data-slot": "field-group"
            "data-orientation": orientation_attr
            "data-invalid": if invalid {
                "true"
            } else {
                "false"
            }
            class: c_euv_field_group()
            class: orientation_class()
            children
        }
    }
}

/// Holds the body of one field.
///
/// The content takes the description, the error and everything else
/// that belongs to the field below its label.
///
/// # Arguments
///
/// - `VirtualNode<EuvFieldContentProps>` - The props node carrying the
///   settings of the content.
///
/// # Returns
///
/// - `VirtualNode` - The euv field content virtual DOM tree.
#[component]
pub fn euv_field_content(node: VirtualNode<EuvFieldContentProps>) -> VirtualNode {
    let EuvFieldContentProps {}: EuvFieldContentProps = node.try_get_props().unwrap_or_default();
    let children: VirtualNode = node.get_children().into();
    html! {
        div {
            "data-slot": "field-content"
            class: c_euv_field_content()
            children
        }
    }
}

/// Names the control of one field.
///
/// The label is the caption of the control, so it names the control it
/// belongs to and the reader can click it to reach that control.
///
/// # Arguments
///
/// - `VirtualNode<EuvFieldLabelProps>` - The props node carrying the
///   id of the control the label names.
///
/// # Returns
///
/// - `VirtualNode` - The euv field label virtual DOM tree.
#[component]
pub fn euv_field_label(node: VirtualNode<EuvFieldLabelProps>) -> VirtualNode {
    let EuvFieldLabelProps { html_for }: EuvFieldLabelProps =
        node.try_get_props().unwrap_or_default();
    let children: VirtualNode = node.get_children().into();
    html! {
        label {
            for: html_for
            "data-slot": "field-label"
            class: c_euv_field_label()
            children
        }
    }
}

/// Names one field in the title of a group of fields.
///
/// The title is the caption of a field that carries a row of its own,
/// such as a checkbox with a row of text beside it.
///
/// # Arguments
///
/// - `VirtualNode<EuvFieldTitleProps>` - The props node carrying the
///   settings of the title.
///
/// # Returns
///
/// - `VirtualNode` - The euv field title virtual DOM tree.
#[component]
pub fn euv_field_title(node: VirtualNode<EuvFieldTitleProps>) -> VirtualNode {
    let EuvFieldTitleProps {}: EuvFieldTitleProps = node.try_get_props().unwrap_or_default();
    let children: VirtualNode = node.get_children().into();
    html! {
        div {
            "data-slot": "field-label"
            class: c_euv_field_title()
            children
        }
    }
}

/// Explains one field.
///
/// The description says what the field is for, in the quiet voice the
/// reference uses for text that is read rather than acted on.
///
/// # Arguments
///
/// - `VirtualNode<EuvFieldDescriptionProps>` - The props node carrying
///   the settings of the description.
///
/// # Returns
///
/// - `VirtualNode` - The euv field description virtual DOM tree.
#[component]
pub fn euv_field_description(node: VirtualNode<EuvFieldDescriptionProps>) -> VirtualNode {
    let EuvFieldDescriptionProps { id }: EuvFieldDescriptionProps =
        node.try_get_props().unwrap_or_default();
    let children: VirtualNode = node.get_children().into();
    html! {
        p {
            id: id
            "data-slot": "field-description"
            class: c_euv_field_description()
            children
        }
    }
}

/// Reports what is wrong with one field.
///
/// The error is announced the moment it appears, so a screen reader
/// reads the failure before the reader moves on.
///
/// # Arguments
///
/// - `VirtualNode<EuvFieldErrorProps>` - The props node carrying the
///   message that tells the reader what is wrong.
///
/// # Returns
///
/// - `VirtualNode` - The euv field error virtual DOM tree.
#[component]
pub fn euv_field_error(node: VirtualNode<EuvFieldErrorProps>) -> VirtualNode {
    let EuvFieldErrorProps { message, id }: EuvFieldErrorProps =
        node.try_get_props().unwrap_or_default();
    let children: VirtualNode = node.get_children().into();
    html! {
        div {
            id: id
            role: "alert"
            "data-slot": "field-error"
            "data-invalid": "true"
            class: c_euv_field_error()
            if !message.is_empty() {
                message
            } else {
                children
            }
        }
    }
}

/// Divides one part of a group of fields from the next.
///
/// The rule runs behind the text that sits in the middle of it, so
/// the text stays readable against the line.
///
/// # Arguments
///
/// - `VirtualNode<EuvFieldSeparatorProps>` - The props node carrying
///   the settings of the separator.
///
/// # Returns
///
/// - `VirtualNode` - The euv field separator virtual DOM tree.
#[component]
pub fn euv_field_separator(node: VirtualNode<EuvFieldSeparatorProps>) -> VirtualNode {
    let EuvFieldSeparatorProps {}: EuvFieldSeparatorProps =
        node.try_get_props().unwrap_or_default();
    let children: VirtualNode = node.get_children().into();
    html! {
        div {
            "data-slot": "field-separator"
            "data-content": if node.has_children() {
                "true"
            } else {
                "false"
            }
            class: c_euv_field_separator()
            div {
                class: c_euv_field_separator_line()
            }
            if node.has_children() {
                span {
                    "data-slot": "field-separator-content"
                    class: c_euv_field_separator_content()
                    children
                }
            }
        }
    }
}
