use super::*;

/// The frame that holds a combobox.
///
/// The frame is the anchor of the panel: the panel is positioned against
/// it, so the frame and the panel travel together.
///
/// # Arguments
///
/// - `VirtualNode<EuvComboboxProps>` - The props node carrying the component settings.
///
/// # Returns
///
/// - `VirtualNode` - The euv combobox virtual DOM tree.
#[component]
pub fn euv_combobox(node: VirtualNode<EuvComboboxProps>) -> VirtualNode {
    let EuvComboboxProps {
        open: _open,
        value: _value,
        label,
    }: EuvComboboxProps = node.try_get_props().unwrap_or_default();
    let children: VirtualNode = node.get_children().into();
    html! {
        div {
            class: c_euv_combobox()
            "aria-label": label
            children
        }
    }
}

/// The button that opens the list of options.
///
/// The button reports whether the list is open through `aria-expanded`,
/// so the screen reader says so too.
///
/// # Arguments
///
/// - `VirtualNode<EuvComboboxTriggerProps>` - The props node carrying the component settings.
///
/// # Returns
///
/// - `VirtualNode` - The euv combobox trigger virtual DOM tree.
#[component]
pub fn euv_combobox_trigger(node: VirtualNode<EuvComboboxTriggerProps>) -> VirtualNode {
    let EuvComboboxTriggerProps {
        open,
        label,
        disabled,
    }: EuvComboboxTriggerProps = node.try_get_props().unwrap_or_default();
    let on_click: Option<Rc<dyn Fn(Event)>> = Some(Rc::new(move |_: Event| {
        open.set(!open.get());
    }));
    html! {
        button {
            type: "button"
            role: "combobox"
            "aria-haspopup": "listbox"
            "aria-expanded": if { open.get() } {
                "true"
            } else {
                "false"
            }
            disabled: disabled
            class: c_euv_combobox_trigger()
            onclick: on_click
            {
                label
            }
        }
    }
}

/// The field the reader types the query into.
///
/// # Arguments
///
/// - `VirtualNode<EuvComboboxInputProps>` - The props node carrying the component settings.
///
/// # Returns
///
/// - `VirtualNode` - The euv combobox input virtual DOM tree.
#[component]
pub fn euv_combobox_input(node: VirtualNode<EuvComboboxInputProps>) -> VirtualNode {
    let EuvComboboxInputProps {
        value,
        placeholder,
        disabled,
    }: EuvComboboxInputProps = node.try_get_props().unwrap_or_default();
    let text: String = value.get();
    html! {
        input {
            type: "text"
            role: "combobox"
            "aria-expanded": "true"
            value: text
            placeholder: placeholder
            disabled: disabled
            class: c_euv_combobox_input()
            oninput: on_combobox_input(value)
        }
    }
}

/// Builds the handler that keeps the query in the caller's signal.
///
/// # Arguments
///
/// - `Signal<String>` - The signal holding the query.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - The input handler.
fn on_combobox_input(value: Signal<String>) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |event: Event| {
        let Some(target) = event.target() else {
            return;
        };
        let Ok(input) = target.dyn_into::<HtmlInputElement>() else {
            return;
        };
        value.set(input.value());
    }))
}

/// The panel that holds the list of options.
///
/// # Arguments
///
/// - `VirtualNode<EuvComboboxContentProps>` - The props node carrying the component settings.
///
/// # Returns
///
/// - `VirtualNode` - The euv combobox content virtual DOM tree.
#[component]
pub fn euv_combobox_content(node: VirtualNode<EuvComboboxContentProps>) -> VirtualNode {
    let EuvComboboxContentProps { open }: EuvComboboxContentProps =
        node.try_get_props().unwrap_or_default();
    let children: VirtualNode = node.get_children().into();
    html! {
        div {
            class: c_euv_combobox_content()
            class: if { open } {
                c_euv_combobox_content_open()
            } else {
                c_euv_combobox_content_closed()
            }
            children
        }
    }
}

/// The rows of the combobox.
///
/// # Arguments
///
/// - `VirtualNode<EuvComboboxListProps>` - The props node carrying the component settings.
///
/// # Returns
///
/// - `VirtualNode` - The euv combobox list virtual DOM tree.
#[component]
pub fn euv_combobox_list(node: VirtualNode<EuvComboboxListProps>) -> VirtualNode {
    let EuvComboboxListProps { label }: EuvComboboxListProps =
        node.try_get_props().unwrap_or_default();
    let children: VirtualNode = node.get_children().into();
    html! {
        div {
            role: "listbox"
            "aria-label": label
            class: c_euv_combobox_list()
            children
        }
    }
}

/// One row of the combobox.
///
/// # Arguments
///
/// - `VirtualNode<EuvComboboxItemProps>` - The props node carrying the component settings.
///
/// # Returns
///
/// - `VirtualNode` - The euv combobox item virtual DOM tree.
#[component]
pub fn euv_combobox_item(node: VirtualNode<EuvComboboxItemProps>) -> VirtualNode {
    let EuvComboboxItemProps {
        value,
        label,
        selected,
        disabled,
        on_select,
    }: EuvComboboxItemProps = node.try_get_props().unwrap_or_default();
    let item_class: fn() -> &'static Css = if disabled {
        c_euv_combobox_item_disabled
    } else if selected {
        c_euv_combobox_item_selected
    } else {
        c_euv_combobox_item
    };
    let on_click: Option<Rc<dyn Fn(Event)>> = if disabled {
        None
    } else {
        Some(Rc::new(move |_: Event| {
            if let Some(callback) = &on_select {
                callback(value);
            }
        }))
    };
    html! {
        div {
            role: "option"
            "aria-selected": if selected { "true" } else { "false" }
            "aria-disabled": if disabled { "true" } else { "false" }
            class: item_class()
            onclick: on_click
            {
                label
            }
        }
    }
}

/// A named group of rows.
///
/// # Arguments
///
/// - `VirtualNode<EuvComboboxGroupProps>` - The props node carrying the component settings.
///
/// # Returns
///
/// - `VirtualNode` - The euv combobox group virtual DOM tree.
#[component]
pub fn euv_combobox_group(node: VirtualNode<EuvComboboxGroupProps>) -> VirtualNode {
    let EuvComboboxGroupProps { label }: EuvComboboxGroupProps =
        node.try_get_props().unwrap_or_default();
    let children: VirtualNode = node.get_children().into();
    html! {
        div {
            role: "group"
            "aria-label": label
            class: c_euv_combobox_group()
            children
        }
    }
}

/// The heading that names a group of rows.
///
/// # Arguments
///
/// - `VirtualNode<EuvComboboxLabelProps>` - The props node carrying the component settings.
///
/// # Returns
///
/// - `VirtualNode` - The euv combobox label virtual DOM tree.
#[component]
pub fn euv_combobox_label(node: VirtualNode<EuvComboboxLabelProps>) -> VirtualNode {
    let EuvComboboxLabelProps { text }: EuvComboboxLabelProps =
        node.try_get_props().unwrap_or_default();
    html! {
        div {
            class: c_euv_combobox_label()
            {
                text
            }
        }
    }
}

/// The message shown when the query matched nothing.
///
/// # Arguments
///
/// - `VirtualNode<EuvComboboxEmptyProps>` - The props node carrying the component settings.
///
/// # Returns
///
/// - `VirtualNode` - The euv combobox empty virtual DOM tree.
#[component]
pub fn euv_combobox_empty(node: VirtualNode<EuvComboboxEmptyProps>) -> VirtualNode {
    let EuvComboboxEmptyProps { text }: EuvComboboxEmptyProps =
        node.try_get_props().unwrap_or_default();
    html! {
        div {
            class: c_euv_combobox_empty()
            {
                text
            }
        }
    }
}

/// The rule that divides one group of rows from the next.
///
/// # Arguments
///
/// - `VirtualNode<EuvComboboxSeparatorProps>` - The props node carrying the component settings.
///
/// # Returns
///
/// - `VirtualNode` - The euv combobox separator virtual DOM tree.
#[component]
pub fn euv_combobox_separator(node: VirtualNode<EuvComboboxSeparatorProps>) -> VirtualNode {
    let EuvComboboxSeparatorProps { label }: EuvComboboxSeparatorProps =
        node.try_get_props().unwrap_or_default();
    html! {
        div {
            role: "separator"
            class: c_euv_combobox_separator()
            "aria-label": label
        }
    }
}

/// The button that clears the field.
///
/// # Arguments
///
/// - `VirtualNode<EuvComboboxClearProps>` - The props node carrying the component settings.
///
/// # Returns
///
/// - `VirtualNode` - The euv combobox clear virtual DOM tree.
#[component]
pub fn euv_combobox_clear(node: VirtualNode<EuvComboboxClearProps>) -> VirtualNode {
    let EuvComboboxClearProps { label }: EuvComboboxClearProps =
        node.try_get_props().unwrap_or_default();
    html! {
        button {
            type: "button"
            "aria-label": label
            class: c_euv_combobox_clear()
        }
    }
}
