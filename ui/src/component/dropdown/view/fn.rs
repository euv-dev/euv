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
                    "data-slot": "dropdown-menu-item"
                    aria-disabled: DROPDOWN_ATTR_FALSE
                    tabindex: "-1"
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
            "data-slot": "dropdown-menu"
            class: c_euv_dropdown()
            children
            div {
                role: "menu"
                "aria-label": DROPDOWN_MENU_ARIA_LABEL
                "data-slot": "dropdown-menu-content"
                "data-state": if { open.get() } { DROPDOWN_STATE_OPEN } else { DROPDOWN_STATE_CLOSED }
                tabindex: "-1"
                onkeydown: dropdown_menu_keydown(open)
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

/// The button that opens the menu.
///
/// The trigger is the `dropdown-menu-trigger` part of the reference: it is
/// the element the reader presses to open the menu, so it carries the
/// open state of the menu it opens.
///
/// # Arguments
///
/// - `VirtualNode<EuvDropdownTriggerProps>` - The props node carrying the
///   component settings.
///
/// # Returns
///
/// - `VirtualNode` - The euv dropdown trigger virtual DOM tree.
#[component]
pub fn euv_dropdown_trigger(node: VirtualNode<EuvDropdownTriggerProps>) -> VirtualNode {
    let EuvDropdownTriggerProps { open }: EuvDropdownTriggerProps =
        node.try_get_props().unwrap_or_default();
    let children: VirtualNode = node.get_children().into();
    html! {
        button {
            type: "button"
            "data-slot": "dropdown-menu-trigger"
            "aria-haspopup": "menu"
            "aria-expanded": if { open.get() } {
                DROPDOWN_ATTR_TRUE
            } else {
                DROPDOWN_ATTR_FALSE
            }
            onkeydown: dropdown_trigger_keydown(open)
            onclick: toggle_dropdown(open)
            children
        }
    }
}

/// Moves focus through the enabled menu items and dismisses the menu on Escape.
///
/// # Arguments
///
/// - `Signal<bool>` - The open state signal.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - The menu keyboard handler.
fn dropdown_menu_keydown(open: Signal<bool>) -> Option<Rc<dyn Fn(Event)>> {
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
        let Some(root) = target.closest(DROPDOWN_ROOT_SELECTOR).ok().flatten() else {
            return;
        };
        if key == DROPDOWN_KEY_ESCAPE {
            keyboard_event.prevent_default();
            open.set(false);
            focus_dropdown_trigger(&root);
            return;
        }
        let Ok(nodes) = root.query_selector_all(DROPDOWN_ITEM_SELECTOR) else {
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
        if items.is_empty() {
            return;
        }
        let Some(current_item) = keyboard_event
            .target()
            .and_then(|target: EventTarget| target.dyn_into::<Element>().ok())
            .and_then(|target: Element| target.closest(DROPDOWN_ITEM_SELECTOR).ok().flatten())
            .and_then(|target: Element| target.dyn_into::<HtmlElement>().ok())
        else {
            return;
        };
        let Some(current_index) = items
            .iter()
            .position(|item: &HtmlElement| item == &current_item)
        else {
            return;
        };
        let next_index: usize = if key == DROPDOWN_KEY_ARROW_DOWN {
            (current_index + 1) % items.len()
        } else if key == DROPDOWN_KEY_ARROW_UP {
            (current_index + items.len() - 1) % items.len()
        } else if key == DROPDOWN_KEY_HOME {
            0
        } else if key == DROPDOWN_KEY_END {
            items.len() - 1
        } else {
            return;
        };
        keyboard_event.prevent_default();
        if let Some(item) = items.get(next_index) {
            let _: Result<(), JsValue> = item.focus();
        }
    }))
}

/// Opens the menu from trigger keyboard input and focuses the first or last row.
///
/// # Arguments
///
/// - `Signal<bool>` - The open state signal.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - The trigger keyboard handler.
fn dropdown_trigger_keydown(open: Signal<bool>) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |event: Event| {
        let Ok(keyboard_event) = event.dyn_into::<KeyboardEvent>() else {
            return;
        };
        let key: String = keyboard_event.key();
        if key != DROPDOWN_KEY_ARROW_DOWN && key != DROPDOWN_KEY_ARROW_UP {
            return;
        }
        let Some(root) = keyboard_event
            .target()
            .and_then(|target: EventTarget| target.dyn_into::<Element>().ok())
            .and_then(|target: Element| target.closest(DROPDOWN_ROOT_SELECTOR).ok().flatten())
        else {
            return;
        };
        keyboard_event.prevent_default();
        open.set(true);
        let focus_last: bool = key == DROPDOWN_KEY_ARROW_UP;
        schedule_dropdown_item_focus(root, focus_last);
    }))
}

/// Finds this event's containing dropdown root.
///
/// # Arguments
///
/// - `&Event` - The event whose target is inside the dropdown.
///
/// # Returns
///
/// - `Option<Element>` - The containing dropdown root, if found.
fn dropdown_root_from_event(event: &Event) -> Option<Element> {
    event
        .target()
        .and_then(|target: EventTarget| target.dyn_into::<Element>().ok())
        .and_then(|target: Element| target.closest(DROPDOWN_ROOT_SELECTOR).ok().flatten())
}

/// Schedules focus on the first or last enabled menu item.
///
/// The signal update changes the open class before the animation frame runs,
/// so the target is visible when focus is moved to it.
///
/// # Arguments
///
/// - `Element` - The dropdown root containing the menu and trigger.
/// - `bool` - Whether to focus the last enabled item instead of the first.
fn schedule_dropdown_item_focus(root: Element, focus_last: bool) {
    let callback: Function = Closure::once_into_js(move || {
        let Ok(nodes) = root.query_selector_all(DROPDOWN_ITEM_SELECTOR) else {
            return;
        };
        let index: u32 = if focus_last {
            nodes.length().saturating_sub(1)
        } else {
            0
        };
        if let Some(node) = nodes.item(index)
            && let Ok(item) = node.dyn_into::<HtmlElement>()
        {
            let _: Result<(), JsValue> = item.focus();
        }
    })
    .unchecked_into();
    if let Some(window) = window() {
        let _: Result<i32, JsValue> = window.request_animation_frame(&callback);
    }
}

/// Restores focus to the trigger of a dropdown root.
///
/// # Arguments
///
/// - `&Element` - The dropdown root containing the trigger.
fn focus_dropdown_trigger(root: &Element) {
    if let Ok(Some(node)) = root.query_selector(DROPDOWN_TRIGGER_SELECTOR)
        && let Ok(trigger) = node.dyn_into::<HtmlElement>()
    {
        let _: Result<(), JsValue> = trigger.focus();
    }
}

/// Toggles the menu's open state when its trigger is activated.
///
/// # Arguments
///
/// - `Signal<bool>` - The open state signal owned by the caller.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - The toggle click handler.
fn toggle_dropdown(open: Signal<bool>) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |event: Event| {
        let should_open: bool = !open.get();
        open.set(should_open);
        if let Some(root) = dropdown_root_from_event(&event) {
            if should_open {
                schedule_dropdown_item_focus(root, false);
            } else {
                focus_dropdown_trigger(&root);
            }
        }
    }))
}

/// Emits the chosen value, closes the menu, and restores trigger focus.
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
    Some(Rc::new(move |event: Event| {
        if let Some(callback) = &on_select {
            callback(value);
        }
        open.set(false);
        if let Some(root) = dropdown_root_from_event(&event) {
            focus_dropdown_trigger(&root);
        }
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
            "data-slot": "dropdown-menu-item"
            class: item_class()
            "aria-disabled": if disabled {
                DROPDOWN_ATTR_TRUE
            } else {
                DROPDOWN_ATTR_FALSE
            }
            onclick: on_click
            children
        }
    }
}

/// The group of rows that belong to one heading.
///
/// # Arguments
///
/// - `VirtualNode<EuvDropdownMenuGroupProps>` - The props node carrying the component settings.
///
/// # Returns
///
/// - `VirtualNode` - The euv dropdown menu group virtual DOM tree.
#[component]
pub fn euv_dropdown_menu_group(node: VirtualNode<EuvDropdownMenuGroupProps>) -> VirtualNode {
    let EuvDropdownMenuGroupProps { label }: EuvDropdownMenuGroupProps =
        node.try_get_props().unwrap_or_default();
    let children: VirtualNode = node.get_children().into();
    html! {
        div {
            role: "group"
            "data-slot": "dropdown-menu-group"
            "aria-label": label
            class: c_euv_dropdown_menu_group()
            children
        }
    }
}

/// The row that carries a box the reader ticks.
///
/// # Arguments
///
/// - `VirtualNode<EuvDropdownMenuCheckboxItemProps>` - The props node carrying the component settings.
///
/// # Returns
///
/// - `VirtualNode` - The euv dropdown menu checkbox item virtual DOM tree.
#[component]
pub fn euv_dropdown_menu_checkbox_item(
    node: VirtualNode<EuvDropdownMenuCheckboxItemProps>,
) -> VirtualNode {
    let EuvDropdownMenuCheckboxItemProps { label, checked }: EuvDropdownMenuCheckboxItemProps =
        node.try_get_props().unwrap_or_default();
    let is_checked: bool = checked.get();
    html! {
        div {
            role: "menuitemcheckbox"
            "data-slot": "dropdown-menu-checkbox-item"
            "aria-checked": if is_checked { "true" } else { "false" }
            class: c_euv_dropdown_menu_checkbox_item()
            {
                label
            }
        }
    }
}

/// The group of rows that share one radio selection.
///
/// # Arguments
///
/// - `VirtualNode<EuvDropdownMenuRadioGroupProps>` - The props node carrying the component settings.
///
/// # Returns
///
/// - `VirtualNode` - The euv dropdown menu radio group virtual DOM tree.
#[component]
pub fn euv_dropdown_menu_radio_group(
    node: VirtualNode<EuvDropdownMenuRadioGroupProps>,
) -> VirtualNode {
    let EuvDropdownMenuRadioGroupProps { label }: EuvDropdownMenuRadioGroupProps =
        node.try_get_props().unwrap_or_default();
    let children: VirtualNode = node.get_children().into();
    html! {
        div {
            role: "group"
            "data-slot": "dropdown-menu-radio-group"
            "aria-label": label
            class: c_euv_dropdown_menu_radio_group()
            children
        }
    }
}

/// The row that carries a dot the reader moves.
///
/// # Arguments
///
/// - `VirtualNode<EuvDropdownMenuRadioItemProps>` - The props node carrying the component settings.
///
/// # Returns
///
/// - `VirtualNode` - The euv dropdown menu radio item virtual DOM tree.
#[component]
pub fn euv_dropdown_menu_radio_item(
    node: VirtualNode<EuvDropdownMenuRadioItemProps>,
) -> VirtualNode {
    let EuvDropdownMenuRadioItemProps { label, checked }: EuvDropdownMenuRadioItemProps =
        node.try_get_props().unwrap_or_default();
    let is_checked: bool = checked.get();
    html! {
        div {
            role: "menuitemradio"
            "data-slot": "dropdown-menu-radio-item"
            "aria-checked": if is_checked { "true" } else { "false" }
            class: c_euv_dropdown_menu_radio_item()
            {
                label
            }
        }
    }
}

/// The nested menu of a row.
///
/// # Arguments
///
/// - `VirtualNode<EuvDropdownMenuSubProps>` - The props node carrying the component settings.
///
/// # Returns
///
/// - `VirtualNode` - The euv dropdown menu sub virtual DOM tree.
#[component]
pub fn euv_dropdown_menu_sub(node: VirtualNode<EuvDropdownMenuSubProps>) -> VirtualNode {
    let EuvDropdownMenuSubProps { open, .. }: EuvDropdownMenuSubProps =
        node.try_get_props().unwrap_or_default();
    let children: VirtualNode = node.get_children().into();
    html! {
        div {
            "data-slot": "dropdown-menu-sub"
            "data-state": if { open.get() } {
                DROPDOWN_STATE_OPEN
            } else {
                DROPDOWN_STATE_CLOSED
            }
            class: c_euv_dropdown_menu_sub()
            children
        }
    }
}

/// The row that opens a nested menu.
///
/// # Arguments
///
/// - `VirtualNode<EuvDropdownMenuSubTriggerProps>` - The props node carrying the component settings.
///
/// # Returns
///
/// - `VirtualNode` - The euv dropdown menu sub trigger virtual DOM tree.
#[component]
pub fn euv_dropdown_menu_sub_trigger(
    node: VirtualNode<EuvDropdownMenuSubTriggerProps>,
) -> VirtualNode {
    let EuvDropdownMenuSubTriggerProps { label, open }: EuvDropdownMenuSubTriggerProps =
        node.try_get_props().unwrap_or_default();
    html! {
        button {
            type: "button"
            role: "menuitem"
            "data-slot": "dropdown-menu-sub-trigger"
            "aria-haspopup": "menu"
            "aria-expanded": if { open.get() } {
                DROPDOWN_ATTR_TRUE
            } else {
                DROPDOWN_ATTR_FALSE
            }
            "data-state": if { open.get() } {
                DROPDOWN_STATE_OPEN
            } else {
                DROPDOWN_STATE_CLOSED
            }
            class: c_euv_dropdown_menu_sub_trigger()
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
/// - `VirtualNode<EuvDropdownMenuSubContentProps>` - The props node carrying the component settings.
///
/// # Returns
///
/// - `VirtualNode` - The euv dropdown menu sub content virtual DOM tree.
#[component]
pub fn euv_dropdown_menu_sub_content(
    node: VirtualNode<EuvDropdownMenuSubContentProps>,
) -> VirtualNode {
    let EuvDropdownMenuSubContentProps { open }: EuvDropdownMenuSubContentProps =
        node.try_get_props().unwrap_or_default();
    let children: VirtualNode = node.get_children().into();
    html! {
        div {
            role: "menu"
            "data-slot": "dropdown-menu-sub-content"
            "data-state": if { open.get() } {
                DROPDOWN_STATE_OPEN
            } else {
                DROPDOWN_STATE_CLOSED
            }
            class: c_euv_dropdown_menu_sub_content()
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
            "data-slot": "dropdown-menu-label"
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
            "data-slot": "dropdown-menu-separator"
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
            "data-slot": "dropdown-menu-shortcut"
            class: c_euv_dropdown_menu_shortcut()
            {
                text
            }
        }
    }
}
