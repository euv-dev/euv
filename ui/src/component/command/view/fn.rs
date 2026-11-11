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
    let base_class: &Css = c_euv_command();
    let shadcn_class: &Css = if cfg!(feature = "theme-shadcn") {
        c_euv_command_shadcn()
    } else {
        &EMPTY_CLASS
    };
    html! {
            div {
                role: "group"
                "aria-label": label
                class: base_class
                class: shadcn_class
                "data-slot": "command"
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
    let disabled_signal: Signal<bool> = Signal::create(disabled);
    let text: String = value.get();
    let command_input_wrapper_shadcn: Css = if cfg!(feature = "theme-shadcn") {
        c_euv_command_input_wrapper_shadcn().clone()
    } else {
        Css::default()
    };
    html! {
            div {
                class: c_euv_command_input_wrapper()
                class: command_input_wrapper_shadcn
                "data-slot": "command-input-wrapper"
                input {
                    type: "text"
                    role: "combobox"
                    "aria-expanded": "true"
                    value: text
                    placeholder: placeholder
                    disabled: disabled_signal
                    class: c_euv_command_input()
                    "data-slot": "command-input"
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
            "data-slot": "command-list"
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
            "data-slot": "command-empty"
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
            "data-slot": "command-group"
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
    let children: VirtualNode = node.get_children().into();
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
            class: command_item_class(item_class)
            "data-slot": "command-item"
            onclick: on_click
            { label }
            children
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
            "data-slot": "command-separator"
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
            "data-slot": "command-shortcut"
            {
                text
            }
        }
    }
}

/// Builds the class list of a command row.
///
/// The base class is resolved first and the theme variant second, so the
/// base's rule is injected before the variant's and the variant wins the
/// cascade. A class injects its stylesheet on its first call, so the
/// order the two are resolved in is the order their rules land in.
///
/// # Arguments
///
/// - `fn() -> &'static Css` - The state class of the row.
///
/// # Returns
///
/// - `String` - The class list.
fn command_item_class(state_class: fn() -> &'static Css) -> String {
    let base_class: &str = c_euv_command_item().get_name();
    let shadcn_class: &str = if cfg!(feature = "theme-shadcn") {
        c_euv_command_item_shadcn().get_name()
    } else {
        ""
    };
    let mut class_name: String = String::with_capacity(64);
    class_name.push_str(base_class);
    class_name.push(' ');
    class_name.push_str(state_class().get_name());
    if !shadcn_class.is_empty() {
        class_name.push(' ');
        class_name.push_str(shadcn_class);
    }
    class_name
}
