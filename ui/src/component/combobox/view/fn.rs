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
        open,
        value: _value,
        label,
    }: EuvComboboxProps = node.try_get_props().unwrap_or_default();
    let children: VirtualNode = node.get_children().into();
    let outside: UseClickOutside = use_click_outside();
    let outside_target: NodeRef<Element> = outside.get_target().clone();
    let outside_triggered: Signal<bool> = outside.get_triggered();
    watch!(outside_triggered, |triggered: bool| {
        if triggered {
            if open.get() {
                open.set(false);
            }
            outside_triggered.set(false);
        }
    });
    html! {
        div {
            "data-slot": "combobox"
            ref: outside_target
            role: "group"
            class: c_euv_combobox()
            "aria-label": label
            children
        }
    }
}

/// The row that holds the chosen values and the query field.
///
/// The row is the group the chips and the query field live in, so a
/// screen reader announces the chosen values as one group.
///
/// # Arguments
///
/// - `VirtualNode<EuvComboboxChipsProps>` - The props node carrying the component settings.
///
/// # Returns
///
/// - `VirtualNode` - The euv combobox chips virtual DOM tree.
#[component]
pub fn euv_combobox_chips(node: VirtualNode<EuvComboboxChipsProps>) -> VirtualNode {
    let EuvComboboxChipsProps { label }: EuvComboboxChipsProps =
        node.try_get_props().unwrap_or_default();
    let children: VirtualNode = node.get_children().into();

    html! {
            div {
                role: "group"
                "data-slot": "combobox-chips"
                "aria-label": label
                class: c_euv_combobox_chips()

                children
            }
    }
}

/// The list that holds the chips.
///
/// The list is a wrapper: the chips themselves sit inside it, and the
/// list only names the group they form.
///
/// # Arguments
///
/// - `VirtualNode<EuvComboboxChipListProps>` - The props node carrying the component settings.
///
/// # Returns
///
/// - `VirtualNode` - The euv combobox chip list virtual DOM tree.
#[component]
pub fn euv_combobox_chip_list(node: VirtualNode<EuvComboboxChipListProps>) -> VirtualNode {
    let EuvComboboxChipListProps { label }: EuvComboboxChipListProps =
        node.try_get_props().unwrap_or_default();
    let children: VirtualNode = node.get_children().into();
    html! {
        div {
            role: "list"
            "data-slot": "combobox-chip-list"
            "aria-label": label
            class: c_euv_combobox_chip_list()
            children
        }
    }
}

/// One chosen value, shown as a chip.
///
/// The chip carries its own remove button, so the caller decides what
/// removing a value means.
///
/// # Arguments
///
/// - `VirtualNode<EuvComboboxChipProps>` - The props node carrying the component settings.
///
/// # Returns
///
/// - `VirtualNode` - The euv combobox chip virtual DOM tree.
#[component]
pub fn euv_combobox_chip(node: VirtualNode<EuvComboboxChipProps>) -> VirtualNode {
    let EuvComboboxChipProps {
        label,
        value,
        disabled,
        on_remove,
    }: EuvComboboxChipProps = node.try_get_props().unwrap_or_default();
    let on_click: Option<Rc<dyn Fn(Event)>> = if disabled {
        None
    } else {
        Some(Rc::new(move |_: Event| {
            if let Some(callback) = &on_remove {
                callback(value);
            }
        }))
    };
    let combobox_chip_shadcn: Css = { c_euv_combobox_chip().clone() };
    html! {
            div {
                role: "listitem"
                "data-slot": "combobox-chip"
                "data-disabled": if disabled {
                    "true"
                } else {
                    "false"
                }
                class: if disabled {
                    c_euv_combobox_chip_disabled()
                } else {
                    c_euv_combobox_chip()
                }
                class: combobox_chip_shadcn
                {
                    label
                }
                button {
                    type: "button"
                    "data-slot": "combobox-chip-remove"
                    "aria-label": "Remove"
                    class: c_euv_combobox_chip_remove()
                    onclick: on_click
                    {
                        "×"
                    }
                }
            }
    }
}

/// The query field that lives inside the chip row.
///
/// # Arguments
///
/// - `VirtualNode<EuvComboboxChipsInputProps>` - The props node carrying the component settings.
///
/// # Returns
///
/// - `VirtualNode` - The euv combobox chips input virtual DOM tree.
#[component]
pub fn euv_combobox_chips_input(node: VirtualNode<EuvComboboxChipsInputProps>) -> VirtualNode {
    let EuvComboboxChipsInputProps {
        value,
        placeholder,
        disabled,
    }: EuvComboboxChipsInputProps = node.try_get_props().unwrap_or_default();
    let text: String = value.get();
    html! {
        input {
            type: "text"
            "data-slot": "combobox-chip-input"
            value: text
            placeholder: placeholder
            disabled: disabled
            class: c_euv_combobox_chips_input()
            oninput: on_combobox_input(value)
        }
    }
}

/// The display of one chosen value.
///
/// # Arguments
///
/// - `VirtualNode<EuvComboboxValueProps>` - The props node carrying the component settings.
///
/// # Returns
///
/// - `VirtualNode` - The euv combobox value virtual DOM tree.
#[component]
pub fn euv_combobox_value(node: VirtualNode<EuvComboboxValueProps>) -> VirtualNode {
    let EuvComboboxValueProps { label }: EuvComboboxValueProps =
        node.try_get_props().unwrap_or_default();
    html! {
        div {
            "data-slot": "combobox-value"
            class: c_euv_combobox_value()
            {
                label
            }
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
    let onkeydown: Option<Rc<dyn Fn(Event)>> = combobox_open_on_keydown(open);
    let base_class: &Css = c_euv_combobox_trigger();

    html! {
        button {
            type: "button"
            "aria-haspopup": "listbox"
            "aria-expanded": if { open.get() } {
                "true"
            } else {
                "false"
            }
            disabled: disabled
            "data-slot": "combobox-trigger"
            class: base_class

            onclick: on_click
            onkeydown: onkeydown
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
        open,
        value,
        placeholder,
        disabled,
        aria_label,
        aria_controls,
    }: EuvComboboxInputProps = node.try_get_props().unwrap_or_default();
    let text: String = value.get();
    let base_class: &Css = c_euv_combobox_input();

    let onkeydown: Option<Rc<dyn Fn(Event)>> = combobox_open_on_keydown(open);
    html! {
        input {
            type: "text"
            role: "combobox"
            "aria-autocomplete": "list"
            "aria-haspopup": "listbox"
            "aria-label": aria_label
            "aria-controls": aria_controls
            "aria-expanded": if { open.get() } {
                "true"
            } else {
                "false"
            }
            value: text
            placeholder: placeholder
            disabled: disabled
            "data-slot": "combobox-input"
            class: base_class

            oninput: on_combobox_input(value)
            onkeydown: onkeydown
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

/// Opens the option panel and moves focus to its selected row from the field.
///
/// # Arguments
///
/// - `Signal<bool>` - The open-state signal shared by the Combobox parts.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - The ArrowUp/ArrowDown handler.
fn combobox_open_on_keydown(open: Signal<bool>) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |event: Event| {
        let Ok(keyboard_event) = event.dyn_into::<KeyboardEvent>() else {
            return;
        };
        let key: String = keyboard_event.key();
        if key != COMBOBOX_KEY_ARROW_DOWN && key != COMBOBOX_KEY_ARROW_UP {
            return;
        }
        let Some(root) = keyboard_event
            .target()
            .and_then(|target: EventTarget| target.dyn_into::<Element>().ok())
            .and_then(|target: Element| target.closest(COMBOBOX_ROOT_SELECTOR).ok().flatten())
        else {
            return;
        };
        keyboard_event.prevent_default();
        open.set(true);
        let focus_root: Element = root;
        let focus_last: bool = key == COMBOBOX_KEY_ARROW_UP;
        let callback: Function = Closure::once_into_js(move || {
            let Ok(nodes) = focus_root.query_selector_all(COMBOBOX_ITEM_SELECTOR) else {
                return;
            };
            let mut items: Vec<HtmlElement> = Vec::new();
            for index in 0..nodes.length() {
                if let Some(item) = nodes
                    .item(index)
                    .and_then(|node: Node| node.dyn_into::<HtmlElement>().ok())
                {
                    items.push(item);
                }
            }
            if let Some(item) = items.first() {
                let selected_index: Option<usize> = items.iter().position(|item: &HtmlElement| {
                    item.get_attribute("aria-selected").as_deref() == Some("true")
                });
                let index: usize =
                    selected_index.unwrap_or_else(|| if focus_last { items.len() - 1 } else { 0 });
                if let Some(target) = items.get(index) {
                    let _: Result<(), JsValue> = target.focus();
                } else {
                    let _: Result<(), JsValue> = item.focus();
                }
            }
        })
        .unchecked_into();
        if let Some(window) = window() {
            let _: Result<i32, JsValue> = window.request_animation_frame(&callback);
        }
    }))
}

/// Restores focus to the query input in one Combobox root.
///
/// # Arguments
///
/// - `&Element` - The Combobox root holding the input.
///
/// # Returns
///
/// - `()` - No value.
fn focus_combobox_input(root: &Element) {
    if let Some(input) = root
        .query_selector(COMBOBOX_INPUT_SELECTOR)
        .ok()
        .flatten()
        .and_then(|input: Element| input.dyn_into::<HtmlElement>().ok())
    {
        let _: Result<(), JsValue> = input.focus();
    }
}

/// Handles option navigation, selection and dismissal inside a Combobox.
///
/// # Arguments
///
/// - `&'static str` - The value of the row receiving keyboard input.
/// - `bool` - Whether the row is disabled.
/// - `Signal<bool>` - The open-state signal shared by the Combobox parts.
/// - `Option<Rc<dyn Fn(&'static str)>>` - The selection callback.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - The option keyboard handler.
fn combobox_item_keydown(
    value: &'static str,
    disabled: bool,
    open: Signal<bool>,
    on_select: Option<Rc<dyn Fn(&'static str)>>,
) -> Option<Rc<dyn Fn(Event)>> {
    if disabled {
        return None;
    }
    Some(Rc::new(move |event: Event| {
        let Ok(keyboard_event) = event.dyn_into::<KeyboardEvent>() else {
            return;
        };
        let key: String = keyboard_event.key();
        let Some(target) = keyboard_event
            .target()
            .and_then(|target: EventTarget| target.dyn_into::<Element>().ok())
        else {
            return;
        };
        let Some(root) = target.closest(COMBOBOX_ROOT_SELECTOR).ok().flatten() else {
            return;
        };
        if key == COMBOBOX_KEY_ESCAPE {
            keyboard_event.prevent_default();
            open.set(false);
            focus_combobox_input(&root);
            return;
        }
        if key == COMBOBOX_KEY_ENTER || key == COMBOBOX_KEY_SPACE {
            keyboard_event.prevent_default();
            if let Some(callback) = on_select.as_ref() {
                callback(value);
            }
            open.set(false);
            focus_combobox_input(&root);
            return;
        }
        let Ok(nodes) = root.query_selector_all(COMBOBOX_ITEM_SELECTOR) else {
            return;
        };
        let mut items: Vec<HtmlElement> = Vec::new();
        for index in 0..nodes.length() {
            if let Some(item) = nodes
                .item(index)
                .and_then(|node: Node| node.dyn_into::<HtmlElement>().ok())
            {
                items.push(item);
            }
        }
        let Some(current_index) = items.iter().position(|item: &HtmlElement| {
            item.get_attribute(COMBOBOX_VALUE_ATTRIBUTE).as_deref() == Some(value)
        }) else {
            return;
        };
        let next_index: usize = if key == COMBOBOX_KEY_ARROW_DOWN {
            (current_index + 1) % items.len()
        } else if key == COMBOBOX_KEY_ARROW_UP {
            (current_index + items.len() - 1) % items.len()
        } else if key == COMBOBOX_KEY_HOME {
            0
        } else if key == COMBOBOX_KEY_END {
            items.len() - 1
        } else {
            return;
        };
        let Some(next_item) = items.get(next_index) else {
            return;
        };
        keyboard_event.prevent_default();
        let _: Result<(), JsValue> = next_item.focus();
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
    let base_class: &Css = c_euv_combobox_content();
    let open_class: &Css = c_euv_combobox_content_open();
    let closed_class: &Css = c_euv_combobox_content_closed();

    html! {
        div {
            "data-slot": "combobox-content"
            class: base_class
            class: if { open } {
                open_class
            } else {
                closed_class
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
    let EuvComboboxListProps { id, label }: EuvComboboxListProps =
        node.try_get_props().unwrap_or_default();
    let children: VirtualNode = node.get_children().into();
    html! {
        div {
            role: "listbox"
            id: id
            "aria-label": label
            "data-slot": "combobox-list"
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
        open,
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
    let base_class: &Css = item_class();
    let shadcn_class: Css = { c_euv_combobox_item().clone() };
    let on_click: Option<Rc<dyn Fn(Event)>> = if disabled {
        None
    } else {
        let click_callback: Option<Rc<dyn Fn(&'static str)>> = on_select.clone();
        Some(Rc::new(move |event: Event| {
            if let Some(callback) = &click_callback {
                callback(value);
            }
            open.set(false);
            if let Some(root) = event
                .target()
                .and_then(|target: EventTarget| target.dyn_into::<Element>().ok())
                .and_then(|target: Element| target.closest(COMBOBOX_ROOT_SELECTOR).ok().flatten())
            {
                focus_combobox_input(&root);
            }
        }))
    };
    let keydown_handler: Option<Rc<dyn Fn(Event)>> =
        combobox_item_keydown(value, disabled, open, on_select.clone());
    html! {
        div {
            role: "option"
            "aria-selected": if selected {
                "true"
            } else {
                "false"
            }
            "aria-disabled": if disabled {
                "true"
            } else {
                "false"
            }
            "data-slot": "combobox-item"
            "data-value": value
            tabindex: "-1"
            class: base_class
            class: shadcn_class
            onclick: on_click
            onkeydown: keydown_handler
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
            "data-slot": "combobox-group"
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
            "data-slot": "combobox-label"
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
            "data-slot": "combobox-empty"
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
            "data-slot": "combobox-separator"
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
            "data-slot": "combobox-clear"
            class: c_euv_combobox_clear()
        }
    }
}
