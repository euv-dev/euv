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
            "data-slot": "context-menu"
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
    let on_context_menu: Option<Rc<dyn Fn(Event)>> = Some(Rc::new(move |event: Event| {
        event.prevent_default();
        open.set(true);
    }));
    let on_keydown: Option<Rc<dyn Fn(Event)>> = Some(Rc::new(move |event: Event| {
        let Some(keyboard) = event.dyn_ref::<web_sys::KeyboardEvent>() else {
            return;
        };
        if keyboard.key() == CONTEXT_MENU_KEY
            || (keyboard.key() == CONTEXT_MENU_F10_KEY && keyboard.shift_key())
        {
            event.prevent_default();
            open.set(true);
        }
    }));
    let shadcn_class: Css = if cfg!(feature = "theme-shadcn") {
        c_euv_context_menu_trigger_shadcn().clone()
    } else {
        Css::default()
    };
    html! {
            div {
                role: "button"
                tabindex: "0"
                "aria-haspopup": "menu"
                "aria-expanded": if { open.get() } {
                    "true"
                } else {
                    "false"
                }
                "data-slot": "context-menu-trigger"
                class: c_euv_context_menu_trigger()
                class: shadcn_class
                oncontextmenu: on_context_menu
                onkeydown: on_keydown
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
            "data-slot": "context-menu-content"
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

/// The group of rows that belong to one heading.
///
/// # Arguments
///
/// - `VirtualNode<EuvContextMenuGroupProps>` - The props node carrying the component settings.
///
/// # Returns
///
/// - `VirtualNode` - The euv context menu group virtual DOM tree.
#[component]
pub fn euv_context_menu_group(node: VirtualNode<EuvContextMenuGroupProps>) -> VirtualNode {
    let EuvContextMenuGroupProps { label }: EuvContextMenuGroupProps =
        node.try_get_props().unwrap_or_default();
    let children: VirtualNode = node.get_children().into();
    html! {
        div {
            role: "group"
            "data-slot": "context-menu-group"
            "aria-label": label
            class: c_euv_context_menu_group()
            children
        }
    }
}

/// The group of rows that share one radio selection.
///
/// # Arguments
///
/// - `VirtualNode<EuvContextMenuRadioGroupProps>` - The props node carrying the component settings.
///
/// # Returns
///
/// - `VirtualNode` - The euv context menu radio group virtual DOM tree.
#[component]
pub fn euv_context_menu_radio_group(
    node: VirtualNode<EuvContextMenuRadioGroupProps>,
) -> VirtualNode {
    let EuvContextMenuRadioGroupProps { label }: EuvContextMenuRadioGroupProps =
        node.try_get_props().unwrap_or_default();
    let children: VirtualNode = node.get_children().into();
    html! {
        div {
            role: "group"
            "data-slot": "context-menu-radio-group"
            "aria-label": label
            class: c_euv_context_menu_radio_group()
            children
        }
    }
}

/// The nested menu of a row.
///
/// # Arguments
///
/// - `VirtualNode<EuvContextMenuSubProps>` - The props node carrying the component settings.
///
/// # Returns
///
/// - `VirtualNode` - The euv context menu sub virtual DOM tree.
#[component]
pub fn euv_context_menu_sub(node: VirtualNode<EuvContextMenuSubProps>) -> VirtualNode {
    let EuvContextMenuSubProps { open }: EuvContextMenuSubProps =
        node.try_get_props().unwrap_or_default();
    let children: VirtualNode = node.get_children().into();
    html! {
        div {
            "data-slot": "context-menu-sub"
            "data-state": if { open.get() } {
                "open"
            } else {
                "closed"
            }
            class: c_euv_context_menu_sub()
            children
        }
    }
}

/// The row that opens a nested menu.
///
/// # Arguments
///
/// - `VirtualNode<EuvContextMenuSubTriggerProps>` - The props node carrying the component settings.
///
/// # Returns
///
/// - `VirtualNode` - The euv context menu sub trigger virtual DOM tree.
#[component]
pub fn euv_context_menu_sub_trigger(
    node: VirtualNode<EuvContextMenuSubTriggerProps>,
) -> VirtualNode {
    let EuvContextMenuSubTriggerProps { label, open }: EuvContextMenuSubTriggerProps =
        node.try_get_props().unwrap_or_default();
    html! {
        button {
            type: "button"
            role: "menuitem"
            "data-slot": "context-menu-sub-trigger"
            "aria-haspopup": "menu"
            "aria-expanded": if { open.get() } {
                "true"
            } else {
                "false"
            }
            class: c_euv_context_menu_sub_trigger()
            {
                label
            }
        }
    }
}

/// The panel of a nested menu.
///
/// # Arguments
///
/// - `VirtualNode<EuvContextMenuSubContentProps>` - The props node carrying the component settings.
///
/// # Returns
///
/// - `VirtualNode` - The euv context menu sub content virtual DOM tree.
#[component]
pub fn euv_context_menu_sub_content(
    node: VirtualNode<EuvContextMenuSubContentProps>,
) -> VirtualNode {
    let EuvContextMenuSubContentProps { open }: EuvContextMenuSubContentProps =
        node.try_get_props().unwrap_or_default();
    let children: VirtualNode = node.get_children().into();
    html! {
        div {
            role: "menu"
            "data-slot": "context-menu-sub-content"
            "data-state": if { open.get() } {
                "open"
            } else {
                "closed"
            }
            class: c_euv_context_menu_sub_content()
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
            "data-slot": "context-menu-item"
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
            "data-slot": "context-menu-checkbox-item"
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
            "data-slot": "context-menu-radio-item"
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
            "data-slot": "context-menu-label"
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
            "data-slot": "context-menu-separator"
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
            "data-slot": "context-menu-shortcut"
            class: c_euv_context_menu_shortcut()
            {
                text
            }
        }
    }
}
