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
        name,
        disabled,
        value,
        onchange,
    }: EuvNativeSelectProps = node.try_get_props().unwrap_or_default();
    let size_attr: &str = match size {
        EuvNativeSelectSize::Default => "default",
        EuvNativeSelectSize::Sm => "sm",
    };
    let base_class: &Css = c_euv_native_select();
    let size_class: &Css = match size {
        EuvNativeSelectSize::Default => c_euv_native_select_size_default(),
        EuvNativeSelectSize::Sm => c_euv_native_select_size_sm(),
    };
    let shadcn_class: Css = if cfg!(feature = "theme-shadcn") {
        c_euv_native_select_shadcn().clone()
    } else {
        Css::default()
    };
    let shadcn_size_class: Css = if cfg!(feature = "theme-shadcn") {
        match size {
            EuvNativeSelectSize::Default => c_euv_native_select_size_default_shadcn().clone(),
            EuvNativeSelectSize::Sm => c_euv_native_select_size_sm_shadcn().clone(),
        }
    } else {
        Css::default()
    };
    let icon_class: fn() -> &'static Css = match size {
        EuvNativeSelectSize::Default => c_euv_native_select_icon,
        EuvNativeSelectSize::Sm => c_euv_native_select_icon_sm,
    };
    let children: VirtualNode = node.get_children().into();
    html! {
        div {
            "data-slot": "native-select-wrapper"
            "data-size": size_attr
            class: c_euv_native_select_wrapper()
            select {
                "data-slot": "native-select"
                "data-size": size_attr
                name: name
                disabled: disabled
                value: value.get()
                onchange: onchange
                class: base_class
                class: size_class
                class: shadcn_class
                class: shadcn_size_class
                children
            }
            span {
                "data-slot": "native-select-icon"
                "aria-hidden": "true"
                class: c_euv_native_select_icon()
                class: icon_class()
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
    html! {
        option {
            "data-slot": "native-select-option"
            value: value
            disabled: disabled
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
    let children: VirtualNode = node.get_children().into();
    html! {
        optgroup {
            "data-slot": "native-select-optgroup"
            label: label
            disabled: disabled
            class: c_euv_native_select_optgroup()
            children
        }
    }
}
