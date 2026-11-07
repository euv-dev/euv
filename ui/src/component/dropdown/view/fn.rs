use super::*;

/// A generic dropdown menu aligned with common component libraries.
///
/// The trigger element is supplied as children and toggles the caller-owned
/// `open` signal; the menu lists `items` and closes after a selection. The
/// menu is always mounted and toggled via reactive classes.
///
/// # Arguments
///
/// - `VirtualNode<EuvDropdownProps>` - The props node containing open, items and on_select.
///
/// # Returns
///
/// - `VirtualNode` - The dropdown virtual DOM tree.
#[component]
pub fn euv_dropdown(node: VirtualNode<EuvDropdownProps>) -> VirtualNode {
    let EuvDropdownProps {
        open,
        items,
        on_select,
    }: EuvDropdownProps = node.try_get_props().unwrap_or_default();
    let children: VirtualNode = node.get_children().into();
    let menu_items: Vec<VirtualNode> = items
        .into_iter()
        .map(|item: EuvDropdownItem| {
            let item_class: fn() -> &'static Css = if item.active {
                c_euv_dropdown_item_active
            } else {
                c_euv_dropdown_item
            };
            html! {
                button {
                    type: "button"
                    role: "menuitem"
                    class: item_class()
                    onclick: select_item(open, on_select.clone(), item.value)
                    {
                        item.label
                    }
                }
            }
        })
        .collect();
    html! {
        div {
            class: c_euv_dropdown()
            children
            div {
                role: "menu"
                class: c_euv_dropdown_menu()
                class: if { open } {
                    c_euv_dropdown_menu_open()
                } else {
                    c_euv_dropdown_menu_closed()
                }
                menu_items
            }
        }
    }
}

/// Emits the chosen value and closes the menu.
///
/// # Arguments
///
/// - `Signal<bool>` - The open state signal.
/// - `Option<Rc<dyn Fn(&'static str)>>` - The select callback.
/// - `&'static str` - The chosen item value.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - The click handler.
fn select_item(
    open: Signal<bool>,
    on_select: Option<Rc<dyn Fn(&'static str)>>,
    value: &'static str,
) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |_| {
        if let Some(callback) = &on_select {
            callback(value);
        }
        open.set(false);
    }))
}

/// One selectable row of a menu.
///
/// The row is a real button carrying `role="menuitem"`, so the browser
/// keeps it in the tab order and the screen reader announces it as a
/// command. A disabled row is marked with `aria-disabled` and takes no
/// click, which keeps it focusable but inert.
///
/// # Arguments
///
/// - `VirtualNode<EuvDropdownMenuItemProps>` - The props node carrying the component settings.
///
/// # Returns
///
/// - `VirtualNode` - The euv dropdown menu item virtual DOM tree.
#[component]
pub fn euv_dropdown_menu_item(node: VirtualNode<EuvDropdownMenuItemProps>) -> VirtualNode {
    let EuvDropdownMenuItemProps {
        label,
        shortcut,
        value,
        disabled,
        on_select,
    }: EuvDropdownMenuItemProps = node.try_get_props().unwrap_or_default();
    let item_class: fn() -> &'static Css = if disabled {
        c_euv_dropdown_menu_item_disabled
    } else {
        c_euv_dropdown_menu_item
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
    let mut children: Vec<VirtualNode> = vec![html! {
        span {
            class: c_euv_dropdown_menu_item_label()
            {
                label
            }
        }
    }];
    if !shortcut.is_empty() {
        children.push(html! {
            span {
                class: c_euv_dropdown_menu_shortcut()
                {
                    shortcut
                }
            }
        });
    }
    html! {
        button {
            type: "button"
            role: "menuitem"
            class: item_class()
            "aria-disabled": if disabled { "true" } else { "false" }
            onclick: on_click
            children
        }
    }
}

/// The heading that names a group of menu rows.
///
/// # Arguments
///
/// - `VirtualNode<EuvDropdownMenuLabelProps>` - The props node carrying the component settings.
///
/// # Returns
///
/// - `VirtualNode` - The euv dropdown menu label virtual DOM tree.
#[component]
pub fn euv_dropdown_menu_label(node: VirtualNode<EuvDropdownMenuLabelProps>) -> VirtualNode {
    let EuvDropdownMenuLabelProps { text }: EuvDropdownMenuLabelProps =
        node.try_get_props().unwrap_or_default();
    html! {
        div {
            class: c_euv_dropdown_menu_label()
            {
                text
            }
        }
    }
}

/// The rule that divides one group of menu rows from the next.
///
/// # Arguments
///
///
/// # Returns
///
/// - `VirtualNode` - The euv dropdown menu separator virtual DOM tree.
#[component]
pub fn euv_dropdown_menu_separator(
    node: VirtualNode<EuvDropdownMenuSeparatorProps>,
) -> VirtualNode {
    let EuvDropdownMenuSeparatorProps { label }: EuvDropdownMenuSeparatorProps =
        node.try_get_props().unwrap_or_default();
    html! {
        div {
            role: "separator"
            class: c_euv_dropdown_menu_separator()
            "aria-label": label
        }
    }
}

/// The key hint shown at the trailing edge of a menu row.
///
/// # Arguments
///
/// - `VirtualNode<EuvDropdownMenuShortcutProps>` - The props node carrying the component settings.
///
/// # Returns
///
/// - `VirtualNode` - The euv dropdown menu shortcut virtual DOM tree.
#[component]
pub fn euv_dropdown_menu_shortcut(node: VirtualNode<EuvDropdownMenuShortcutProps>) -> VirtualNode {
    let EuvDropdownMenuShortcutProps { text }: EuvDropdownMenuShortcutProps =
        node.try_get_props().unwrap_or_default();
    html! {
        span {
            class: c_euv_dropdown_menu_shortcut()
            {
                text
            }
        }
    }
}
