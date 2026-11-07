use super::*;

/// The frame that holds a context menu.
///
/// The frame owns the open state, so the trigger and the panel always
/// agree on whether the menu is showing.
///
/// # Arguments
///
/// - `VirtualNode<EuvContextMenuProps>` - The props node carrying the component settings.
///
/// # Returns
///
/// - `VirtualNode` - The euv context menu virtual DOM tree.
#[component]
pub fn euv_context_menu(node: VirtualNode<EuvContextMenuProps>) -> VirtualNode {
    let EuvContextMenuProps { open: _open }: EuvContextMenuProps =
        node.try_get_props().unwrap_or_default();
    let children: VirtualNode = node.get_children().into();
    html! {
        div {
            class: c_euv_context_menu()
            children
        }
    }
}

/// The surface the menu is opened from.
///
/// # Arguments
///
/// - `VirtualNode<EuvContextMenuTriggerProps>` - The props node carrying the component settings.
///
/// # Returns
///
/// - `VirtualNode` - The euv context menu trigger virtual DOM tree.
#[component]
pub fn euv_context_menu_trigger(node: VirtualNode<EuvContextMenuTriggerProps>) -> VirtualNode {
    let EuvContextMenuTriggerProps { open, label }: EuvContextMenuTriggerProps =
        node.try_get_props().unwrap_or_default();
    let on_click: Option<Rc<dyn Fn(Event)>> = Some(Rc::new(move |_: Event| {
        open.set(!open.get());
    }));
    html! {
        div {
            role: "button"
            "aria-haspopup": "menu"
            "aria-expanded": if { open.get() } {
                "true"
            } else {
                "false"
            }
            class: c_euv_context_menu_trigger()
            onclick: on_click
            {
                label
            }
        }
    }
}

/// The panel that holds the rows of the menu.
///
/// # Arguments
///
/// - `VirtualNode<EuvContextMenuContentProps>` - The props node carrying the component settings.
///
/// # Returns
///
/// - `VirtualNode` - The euv context menu content virtual DOM tree.
#[component]
pub fn euv_context_menu_content(node: VirtualNode<EuvContextMenuContentProps>) -> VirtualNode {
    let EuvContextMenuContentProps { open }: EuvContextMenuContentProps =
        node.try_get_props().unwrap_or_default();
    let children: VirtualNode = node.get_children().into();
    html! {
        div {
            role: "menu"
            class: c_euv_context_menu_content()
            class: if { open } {
                c_euv_context_menu_content_open()
            } else {
                c_euv_context_menu_content_closed()
            }
            children
        }
    }
}

/// One row of the menu.
///
/// # Arguments
///
/// - `VirtualNode<EuvContextMenuItemProps>` - The props node carrying the component settings.
///
/// # Returns
///
/// - `VirtualNode` - The euv context menu item virtual DOM tree.
#[component]
pub fn euv_context_menu_item(node: VirtualNode<EuvContextMenuItemProps>) -> VirtualNode {
    let EuvContextMenuItemProps {
        value,
        label,
        disabled,
        on_select,
    }: EuvContextMenuItemProps = node.try_get_props().unwrap_or_default();
    let item_class: fn() -> &'static Css = if disabled {
        c_euv_context_menu_item_disabled
    } else {
        c_euv_context_menu_item
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
            role: "menuitem"
            "aria-disabled": if disabled { "true" } else { "false" }
            class: item_class()
            onclick: on_click
            {
                label
            }
        }
    }
}

/// A row that carries a box the reader ticks.
///
/// # Arguments
///
///
/// # Returns
///
/// - `VirtualNode` - The euv context menu checkbox item virtual DOM tree.
#[component]
pub fn euv_context_menu_checkbox_item(
    node: VirtualNode<EuvContextMenuCheckboxItemProps>,
) -> VirtualNode {
    let EuvContextMenuCheckboxItemProps {
        label,
        checked,
        on_select,
    }: EuvContextMenuCheckboxItemProps = node.try_get_props().unwrap_or_default();
    let is_checked: bool = checked.get();
    let on_click: Option<Rc<dyn Fn(Event)>> = Some(Rc::new(move |_: Event| {
        if let Some(callback) = &on_select {
            callback(!is_checked);
        }
    }));
    html! {
        div {
            role: "menuitemcheckbox"
            "aria-checked": if is_checked { "true" } else { "false" }
            class: c_euv_context_menu_checkbox_item()
            onclick: on_click
            {
                label
            }
        }
    }
}

/// A row that carries a dot the reader moves.
///
/// # Arguments
///
/// - `VirtualNode<EuvContextMenuRadioItemProps>` - The props node carrying the component settings.
///
/// # Returns
///
/// - `VirtualNode` - The euv context menu radio item virtual DOM tree.
#[component]
pub fn euv_context_menu_radio_item(node: VirtualNode<EuvContextMenuRadioItemProps>) -> VirtualNode {
    let EuvContextMenuRadioItemProps { label, checked }: EuvContextMenuRadioItemProps =
        node.try_get_props().unwrap_or_default();
    let is_checked: bool = checked.get();
    html! {
        div {
            role: "menuitemradio"
            "aria-checked": if is_checked { "true" } else { "false" }
            class: c_euv_context_menu_radio_item()
            {
                label
            }
        }
    }
}

/// The heading that names a group of rows.
///
/// # Arguments
///
/// - `VirtualNode<EuvContextMenuLabelProps>` - The props node carrying the component settings.
///
/// # Returns
///
/// - `VirtualNode` - The euv context menu label virtual DOM tree.
#[component]
pub fn euv_context_menu_label(node: VirtualNode<EuvContextMenuLabelProps>) -> VirtualNode {
    let EuvContextMenuLabelProps { text }: EuvContextMenuLabelProps =
        node.try_get_props().unwrap_or_default();
    html! {
        div {
            class: c_euv_context_menu_label()
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
/// - `VirtualNode<EuvContextMenuSeparatorProps>` - The props node carrying the component settings.
///
/// # Returns
///
/// - `VirtualNode` - The euv context menu separator virtual DOM tree.
#[component]
pub fn euv_context_menu_separator(node: VirtualNode<EuvContextMenuSeparatorProps>) -> VirtualNode {
    let EuvContextMenuSeparatorProps { label }: EuvContextMenuSeparatorProps =
        node.try_get_props().unwrap_or_default();
    html! {
        div {
            role: "separator"
            class: c_euv_context_menu_separator()
            "aria-label": label
        }
    }
}

/// The key hint shown at the trailing edge of a row.
///
/// # Arguments
///
/// - `VirtualNode<EuvContextMenuShortcutProps>` - The props node carrying the component settings.
///
/// # Returns
///
/// - `VirtualNode` - The euv context menu shortcut virtual DOM tree.
#[component]
pub fn euv_context_menu_shortcut(node: VirtualNode<EuvContextMenuShortcutProps>) -> VirtualNode {
    let EuvContextMenuShortcutProps { text }: EuvContextMenuShortcutProps =
        node.try_get_props().unwrap_or_default();
    html! {
        span {
            class: c_euv_context_menu_shortcut()
            {
                text
            }
        }
    }
}
