use super::*;

/// The frame that holds a command.
///
/// The frame is a column: the field sits at its top, the rows scroll
/// below it. It is the surface the rows are read against, so it carries
/// the popover colours of the design system.
///
/// # Arguments
///
/// - `VirtualNode<EuvCommandProps>` - The props node carrying the component settings.
///
/// # Returns
///
/// - `VirtualNode` - The euv command virtual DOM tree.
#[component]
pub fn euv_command(node: VirtualNode<EuvCommandProps>) -> VirtualNode {
    let EuvCommandProps {
        value: _value,
        label,
    }: EuvCommandProps = node.try_get_props().unwrap_or_default();
    let children: VirtualNode = node.get_children().into();
    html! {
        div {
            role: "group"
            "aria-label": label
            class: c_euv_command()
            children
        }
    }
}

/// The field the reader types the query into.
///
/// The field is wrapped in a row that carries the rule under it, which
/// is what separates the field from the rows below.
///
/// # Arguments
///
/// - `VirtualNode<EuvCommandInputProps>` - The props node carrying the component settings.
///
/// # Returns
///
/// - `VirtualNode` - The euv command input virtual DOM tree.
#[component]
pub fn euv_command_input(node: VirtualNode<EuvCommandInputProps>) -> VirtualNode {
    let EuvCommandInputProps {
        value,
        placeholder,
        disabled,
    }: EuvCommandInputProps = node.try_get_props().unwrap_or_default();
    let text: String = value.get();
    html! {
        div {
            class: c_euv_command_input_wrapper()
            input {
                type: "text"
                role: "combobox"
                "aria-expanded": "true"
                value: text
                placeholder: placeholder
                disabled: disabled
                class: c_euv_command_input()
                oninput: on_command_input(value)
            }
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
fn on_command_input(value: Signal<String>) -> Option<Rc<dyn Fn(Event)>> {
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

/// The rows of the command.
///
/// The list is the listbox the rows live in: it carries the role, and
/// every row inside it is one of its options.
///
/// # Arguments
///
/// - `VirtualNode<EuvCommandListProps>` - The props node carrying the component settings.
///
/// # Returns
///
/// - `VirtualNode` - The euv command list virtual DOM tree.
#[component]
pub fn euv_command_list(node: VirtualNode<EuvCommandListProps>) -> VirtualNode {
    let EuvCommandListProps { label }: EuvCommandListProps =
        node.try_get_props().unwrap_or_default();
    let children: VirtualNode = node.get_children().into();
    html! {
        div {
            role: "listbox"
            "aria-label": label
            class: c_euv_command_list()
            children
        }
    }
}

/// The message shown when the query matched nothing.
///
/// # Arguments
///
/// - `VirtualNode<EuvCommandEmptyProps>` - The props node carrying the component settings.
///
/// # Returns
///
/// - `VirtualNode` - The euv command empty virtual DOM tree.
#[component]
pub fn euv_command_empty(node: VirtualNode<EuvCommandEmptyProps>) -> VirtualNode {
    let EuvCommandEmptyProps { text }: EuvCommandEmptyProps =
        node.try_get_props().unwrap_or_default();
    html! {
        div {
            class: c_euv_command_empty()
            {
                text
            }
        }
    }
}

/// A named group of rows.
///
/// # Arguments
///
/// - `VirtualNode<EuvCommandGroupProps>` - The props node carrying the component settings.
///
/// # Returns
///
/// - `VirtualNode` - The euv command group virtual DOM tree.
#[component]
pub fn euv_command_group(node: VirtualNode<EuvCommandGroupProps>) -> VirtualNode {
    let EuvCommandGroupProps { heading }: EuvCommandGroupProps =
        node.try_get_props().unwrap_or_default();
    let children: VirtualNode = node.get_children().into();
    html! {
        div {
            role: "group"
            class: c_euv_command_group()
            div {
                class: c_euv_command_group_heading()
                {
                    heading
                }
            }
            children
        }
    }
}

/// One row of the command.
///
/// The row is an option of the listbox, so the screen reader reads it as
/// one of the choices the query narrows down.
///
/// # Arguments
///
/// - `VirtualNode<EuvCommandItemProps>` - The props node carrying the component settings.
///
/// # Returns
///
/// - `VirtualNode` - The euv command item virtual DOM tree.
#[component]
pub fn euv_command_item(node: VirtualNode<EuvCommandItemProps>) -> VirtualNode {
    let EuvCommandItemProps {
        value,
        label,
        selected,
        disabled,
        on_select,
    }: EuvCommandItemProps = node.try_get_props().unwrap_or_default();
    let item_class: fn() -> &'static Css = if disabled {
        c_euv_command_item_disabled
    } else if selected {
        c_euv_command_item_selected
    } else {
        c_euv_command_item
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

/// The rule that divides one group of rows from the next.
///
/// # Arguments
///
/// - `VirtualNode<EuvCommandSeparatorProps>` - The props node carrying the component settings.
///
/// # Returns
///
/// - `VirtualNode` - The euv command separator virtual DOM tree.
#[component]
pub fn euv_command_separator(node: VirtualNode<EuvCommandSeparatorProps>) -> VirtualNode {
    let EuvCommandSeparatorProps { label }: EuvCommandSeparatorProps =
        node.try_get_props().unwrap_or_default();
    html! {
        div {
            role: "separator"
            class: c_euv_command_separator()
            "aria-label": label
        }
    }
}

/// The key hint shown at the trailing edge of a row.
///
/// # Arguments
///
/// - `VirtualNode<EuvCommandShortcutProps>` - The props node carrying the component settings.
///
/// # Returns
///
/// - `VirtualNode` - The euv command shortcut virtual DOM tree.
#[component]
pub fn euv_command_shortcut(node: VirtualNode<EuvCommandShortcutProps>) -> VirtualNode {
    let EuvCommandShortcutProps { text }: EuvCommandShortcutProps =
        node.try_get_props().unwrap_or_default();
    html! {
        span {
            class: c_euv_command_shortcut()
            {
                text
            }
        }
    }
}
