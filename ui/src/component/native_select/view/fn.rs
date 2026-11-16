use super::*;

/// Renders a select that uses the platform control.
///
/// The reference keeps the native control and dresses it: a wrapper holds
/// the control and the chevron that stands in for the browser's own
/// arrow. The native control is kept because it brings the platform's
/// picker, its keyboard handling and its screen reader behaviour, which
/// a listbox rebuilt from scratch cannot match.
///
/// # Arguments
///
/// - `VirtualNode<EuvNativeSelectProps>` - The props node carrying the
///   size and the state of the control. The options are passed as
///   children.
///
/// # Returns
///
/// - `VirtualNode` - The euv native select virtual DOM tree.
#[component]
pub fn euv_native_select(node: VirtualNode<EuvNativeSelectProps>) -> VirtualNode {
    let EuvNativeSelectProps {
        size,
        id,
        name,
        aria_label,
        disabled,
        required,
        invalid,
        value,
        onchange,
    }: EuvNativeSelectProps = node.try_get_props().unwrap_or_default();
    let select_id: UseId = use_id("native-select");
    let select_id_value: String = if id.is_empty() {
        select_id.get_value().get()
    } else {
        String::from(id)
    };
    let initial_value: String = value.get();
    let reset_value_signal: Signal<String> = value;
    let reset_select_id: String = select_id_value.clone();
    let reset_callback: Rc<dyn Fn()> = Rc::new(move || {
        let reset_value: String = initial_value.clone();
        reset_value_signal.set(reset_value.clone());
        let select: Option<HtmlSelectElement> = window()
            .and_then(|window: Window| window.document())
            .and_then(|document: Document| document.get_element_by_id(&reset_select_id))
            .and_then(|element: Element| element.dyn_into::<HtmlSelectElement>().ok());
        if let Some(select) = select {
            select.set_value(&reset_value);
        }
    });
    use_form_reset(select_id_value.clone(), "", reset_callback);
    let aria_invalid: &str = if invalid {
        NATIVE_SELECT_ATTRIBUTE_ON
    } else {
        NATIVE_SELECT_ATTRIBUTE_OFF
    };
    let disabled_attribute: &'static str = native_select_boolean_attribute(disabled);
    let required_attribute: &'static str = native_select_boolean_attribute(required);
    let invalid_class: Css = if invalid {
        c_euv_native_select_invalid().clone()
    } else {
        Css::default()
    };
    let wrapper_disabled_class: Css = if disabled {
        c_euv_native_select_wrapper_disabled().clone()
    } else {
        Css::default()
    };
    let size_attr: &str = match size {
        EuvNativeSelectSize::Default => "default",
        EuvNativeSelectSize::Sm => "sm",
    };
    let base_class: &Css = c_euv_native_select();
    let size_class: &Css = match size {
        EuvNativeSelectSize::Default => c_euv_native_select_size_default(),
        EuvNativeSelectSize::Sm => c_euv_native_select_size_sm(),
    };

    let children: VirtualNode = node.get_children().into();
    html! {
        div {
            "data-slot": "native-select-wrapper"
            "data-size": size_attr
            class: c_euv_native_select_wrapper()
            class: wrapper_disabled_class
            select {
                "data-slot": "native-select"
                "data-size": size_attr
                id: select_id_value.as_str()
                name: name
                "aria-label": aria_label.clone()
                "aria-invalid": aria_invalid
                disabled: disabled_attribute
                required: required_attribute
                value: value.get()
                onchange: onchange
                class: base_class
                class: size_class


                class: invalid_class
                children
            }
            span {
                "data-slot": "native-select-icon"
                "aria-hidden": "true"
                class: c_euv_native_select_icon()
                inner_html: crate::component::icon::ICON_CHEVRON_DOWN_MARKUP
            }
        }
    }
}

/// Renders one option of a native select.
///
/// The option carries the label a reader sees and the value the form
/// submits under.
///
/// # Arguments
///
/// - `VirtualNode<EuvNativeSelectOptionProps>` - The props node carrying
///   the value and the label of the option.
///
/// # Returns
///
/// - `VirtualNode` - The euv native select option virtual DOM tree.
#[component]
pub fn euv_native_select_option(node: VirtualNode<EuvNativeSelectOptionProps>) -> VirtualNode {
    let EuvNativeSelectOptionProps {
        value,
        label,
        disabled,
    }: EuvNativeSelectOptionProps = node.try_get_props().unwrap_or_default();
    let disabled_attribute: &'static str = native_select_boolean_attribute(disabled);
    html! {
        option {
            "data-slot": "native-select-option"
            value: value
            disabled: disabled_attribute
            class: c_euv_native_select_option()
            label
        }
    }
}

/// Renders a group of options inside a native select.
///
/// The group holds a label for the options it wraps.
///
/// # Arguments
///
/// - `VirtualNode<EuvNativeSelectOptGroupProps>` - The props node carrying
///   the label of the group. The options are passed as children.
///
/// # Returns
///
/// - `VirtualNode` - The euv native select option group virtual DOM tree.
#[component]
pub fn euv_native_select_opt_group(node: VirtualNode<EuvNativeSelectOptGroupProps>) -> VirtualNode {
    let EuvNativeSelectOptGroupProps { label, disabled }: EuvNativeSelectOptGroupProps =
        node.try_get_props().unwrap_or_default();
    let disabled_attribute: &'static str = native_select_boolean_attribute(disabled);
    let children: VirtualNode = node.get_children().into();
    html! {
        optgroup {
            "data-slot": "native-select-optgroup"
            label: label
            disabled: disabled_attribute
            class: c_euv_native_select_optgroup()
            children
        }
    }
}

/// Maps a native HTML boolean state to the value expected by the renderer.
///
/// # Arguments
///
/// - `bool` - Whether the native boolean attribute is enabled.
///
/// # Returns
///
/// - `&'static str` - `"true"` when enabled and an empty value otherwise.
fn native_select_boolean_attribute(flag: bool) -> &'static str {
    if flag {
        NATIVE_SELECT_ATTRIBUTE_ON
    } else {
        NATIVE_SELECT_ATTRIBUTE_EMPTY
    }
}
