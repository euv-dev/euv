use super::*;

/// A generic dropdown menu aligned with common component libraries.
///
/// The trigger element is supplied as children and toggles the caller-owned
/// `open` signal; the menu lists `items` and closes after a selection. The
/// menu is always mounted and toggled via reactive classes. Document capture
/// listeners dismiss outside presses without moving focus; Escape and item
/// selection restore trigger focus. Use one open signal per root/trigger pair
/// and supply plain text or icon children, never another button.
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
            let item_class: fn() -> &'static Css = if item.active && !item.disabled {
                c_euv_dropdown_item_active
            } else {
                c_euv_dropdown_item
            };
            let on_click: Option<Rc<dyn Fn(Event)>> = if item.disabled {
                None
            } else {
                select_item(open, on_select.clone(), item.value, true)
            };
            html! {
                button {
                    type: "button"
                    role: "menuitem"
                    "data-slot": "dropdown-menu-item"
                    "data-label": item.label
                    disabled: if item.disabled {
                        DROPDOWN_ATTR_TRUE
                    } else {
                        DROPDOWN_ATTR_FALSE
                    }
                    aria-disabled: if item.disabled {
                        DROPDOWN_ATTR_TRUE
                    } else {
                        DROPDOWN_ATTR_FALSE
                    }
                    tabindex: "-1"
                    class: item_class()
                    class: if item.disabled {
                        c_euv_dropdown_menu_item_disabled()
                    } else {
                        item_class()
                    }
                    onclick: on_click
                    {
                        item.label
                    }
                }
            }
        })
        .collect();
    let content: VirtualNode = dropdown_content(open, menu_items.into());
    let root_children: Vec<VirtualNode> = vec![children, content];
    dropdown_root(open, root_children.into())
}

/// A composable dropdown lifecycle container. Share one open signal with its trigger and content.
///
/// # Arguments
///
/// - `VirtualNode<EuvDropdownMenuProps>` - The root props and children.
///
/// # Returns
///
/// - `VirtualNode` - The lifecycle-managed dropdown root.
#[component]
pub fn euv_dropdown_menu(node: VirtualNode<EuvDropdownMenuProps>) -> VirtualNode {
    let EuvDropdownMenuProps { open, parent_open }: EuvDropdownMenuProps =
        node.try_get_props().unwrap_or_default();
    use_dropdown_menu_parent_binding(open, parent_open);
    dropdown_root(open, node.get_children().into())
}

/// A mounted popup supporting mixed action and checkbox rows.
///
/// # Arguments
///
/// - `VirtualNode<EuvDropdownMenuContentProps>` - The shared open signal and popup children.
///
/// # Returns
///
/// - `VirtualNode` - The scoped keyboard-enabled menu content.
#[component]
pub fn euv_dropdown_menu_content(node: VirtualNode<EuvDropdownMenuContentProps>) -> VirtualNode {
    let EuvDropdownMenuContentProps { open }: EuvDropdownMenuContentProps =
        node.try_get_props().unwrap_or_default();
    dropdown_content(open, node.get_children().into())
}

/// Selects the nearest sheet containing the original trigger, never a popup descendant.
/// The trigger must exist before either alternative matches. The existing portal
/// pending-target lifecycle resolves detached trees and recursively resolves submenus
/// after their parent wrapper enters the modal. No renderer-owned node is moved here.
///
/// # Arguments
/// - `Signal<bool>` - The popup/trigger pair identity.
///
/// # Returns
/// - `String` - A stable selector for the owning sheet or the ordinary app host.
pub(crate) fn dropdown_portal_target(open: Signal<bool>) -> String {
    let trigger: String = format!("#euv-dropdown-{}-trigger", open.get_inner());
    let modal: &str = DROPDOWN_MODAL_CONTENT_SELECTOR;
    format!(
        "{modal}:has({trigger}):not(:has({modal} {trigger})), {DROPDOWN_PORTAL_TARGET}:has({trigger}):not(:has({modal} {trigger}))"
    )
}

/// Owns optional surface subscriptions without adding that surface to menu ancestry.
///
/// # Arguments
/// - `Signal<bool>` - The menu root state.
/// - `Option<Signal<bool>>` - The optional containing surface state.
pub(crate) fn use_dropdown_menu_parent_binding(
    open: Signal<bool>,
    parent_open: Option<Signal<bool>>,
) {
    let binding: Rc<RefCell<Option<DropdownParentBinding>>> =
        HookContext::use_hook(|| Rc::new(RefCell::new(None)));
    let cleanup_binding: Rc<RefCell<Option<DropdownParentBinding>>> = binding.clone();
    App::use_cleanup(move || {
        if let Some((_, _, cleanup)) = cleanup_binding.borrow_mut().take() {
            cleanup();
        }
    });
    if binding
        .borrow()
        .as_ref()
        .is_some_and(|(previous, parent, _): &DropdownParentBinding| {
            *previous == open && *parent == parent_open
        })
    {
        return;
    }
    if let Some((_, _, cleanup)) = binding.borrow_mut().take() {
        cleanup();
    }
    let cleanup: Box<dyn FnOnce()> = dropdown_menu_parent_binding(open, parent_open);
    *binding.borrow_mut() = Some((open, parent_open, cleanup));
}

/// Guards a root in both directions and detaches before resetting its descendants.
///
/// # Arguments
/// - `Signal<bool>` - The menu root state.
/// - `Option<Signal<bool>>` - Optional non-menu lifecycle parent.
///
/// # Returns
/// - `Box<dyn FnOnce()>` - Subscription teardown.
pub(crate) fn dropdown_menu_parent_binding(
    open: Signal<bool>,
    parent_open: Option<Signal<bool>>,
) -> Box<dyn FnOnce()> {
    let Some(parent) = parent_open.filter(|parent: &Signal<bool>| *parent != open) else {
        return Box::new(|| {});
    };
    if !parent.get() {
        open.set(false);
    }
    let parent_id: usize = parent.subscribe(move || {
        if !parent.get() && open.get() {
            open.set(false);
        }
    });
    let child_id: usize = open.subscribe(move || {
        if open.get() && !parent.get() {
            open.set(false);
        }
    });
    Box::new(move || {
        parent.unsubscribe(parent_id);
        open.unsubscribe(child_id);
        open.set(false);
    })
}

/// Shares outside-press cleanup between the legacy and composable roots.
///
/// # Arguments
///
/// - `Signal<bool>` - The caller-owned open state.
/// - `VirtualNode` - The trigger and content children.
///
/// # Returns
///
/// - `VirtualNode` - The dropdown lifecycle container.
fn dropdown_root(open: Signal<bool>, children: VirtualNode) -> VirtualNode {
    let outside_target: NodeRef<Element> = App::use_node_ref();
    #[cfg(not(target_arch = "wasm32"))]
    let _: Signal<bool> = open;
    #[cfg(target_arch = "wasm32")]
    use_dropdown_outside(open, outside_target.clone());
    html! {
        div {
            "data-slot": "dropdown-menu"
            ref: outside_target
            class: c_euv_dropdown()
            children
        }
    }
}

/// Shares popup semantics, styles and search state between both APIs.
///
/// # Arguments
///
/// - `Signal<bool>` - The caller-owned open state.
/// - `VirtualNode` - The popup rows.
///
/// # Returns
///
/// - `VirtualNode` - The always-mounted popup.
fn dropdown_content(open: Signal<bool>, children: VirtualNode) -> VirtualNode {
    dropdown_panel(open, children, false)
}

/// Renders each recursive level through the same popup lifecycle.
///
/// # Arguments
/// - `Signal<bool>` - This panel's distinct open state.
/// - `VirtualNode` - Rows belonging to this level.
/// - `bool` - Whether the panel is a submenu.
///
/// # Returns
/// - `VirtualNode` - The always-mounted keyboard-enabled portal.
fn dropdown_panel(open: Signal<bool>, children: VirtualNode, submenu: bool) -> VirtualNode {
    let panel_ref: NodeRef<Element> = App::use_node_ref();
    #[cfg(target_arch = "wasm32")]
    use_dropdown_position(open, panel_ref.clone());
    let search: Signal<String> = App::use_signal(String::new);
    let searched_at: Signal<f64> = App::use_signal(|| 0.0);
    watch!(open, |_: bool| {
        search.set(String::new());
        searched_at.set(0.0);
    });
    let trigger_id: String = format!("euv-dropdown-{}-trigger", open.get_inner());
    let menu_id: String = format!("euv-dropdown-{}-menu", open.get_inner());
    let menu_class: fn() -> &'static Css = if submenu {
        c_euv_dropdown_sub_panel
    } else {
        c_euv_dropdown_menu
    };
    let portal_target: String = dropdown_portal_target(open);
    html! {
        portal {
            target: portal_target
        div {
            role: "menu"
            ref: panel_ref
            id: menu_id
            "aria-labelledby": trigger_id
            "aria-label": DROPDOWN_MENU_ARIA_LABEL
            "data-slot": if submenu { DROPDOWN_SUB_CONTENT_SLOT } else { "dropdown-menu-content" }
            "data-state": if { open.get() } {
                DROPDOWN_STATE_OPEN
            } else {
                DROPDOWN_STATE_CLOSED
            }
            tabindex: "-1"
            onkeydown: dropdown_menu_keydown(open, search, searched_at)
            class: menu_class()
            class: if { open } {
                c_euv_dropdown_menu_open()
            } else {
                c_euv_dropdown_menu_closed()
            }
            children
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
    let trigger_id: String = format!("euv-dropdown-{}-trigger", open.get_inner());
    let menu_id: String = format!("euv-dropdown-{}-menu", open.get_inner());
    html! {
        button {
            type: "button"
            id: trigger_id
            class: button_class(EuvButtonVariant::Outline, EuvButtonSize::Default)
            "aria-controls": menu_id
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
/// - `Signal<String>` - The buffered search prefix.
/// - `Signal<f64>` - The timestamp of the last search key.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - The menu keyboard handler.
fn dropdown_menu_keydown(
    open: Signal<bool>,
    search: Signal<String>,
    searched_at: Signal<f64>,
) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |event: Event| {
        let Ok(keyboard_event) = event.dyn_into::<KeyboardEvent>() else {
            return;
        };
        if !dropdown_keyboard_allowed(&keyboard_event) || !open.get() {
            return;
        }
        let key: String = keyboard_event.key();
        let Some(target) = keyboard_event
            .target()
            .and_then(|target: EventTarget| target.dyn_into::<Element>().ok())
        else {
            return;
        };
        let Some(root) = dropdown_root_from_event(keyboard_event.as_ref()) else {
            return;
        };
        let Some(content) = target.closest(DROPDOWN_CONTENT_SELECTOR).ok().flatten() else {
            return;
        };
        if content.id() != format!("euv-dropdown-{}-menu", open.get_inner()) {
            return;
        }
        if key == DROPDOWN_KEY_TAB {
            keyboard_event.prevent_default();
            open.set(false);
            if content.get_attribute(DROPDOWN_SLOT_ATTRIBUTE).as_deref()
                == Some(DROPDOWN_SUB_CONTENT_SLOT)
                && let Some(trigger) = root
                    .query_selector(DROPDOWN_TRIGGER_SELECTOR)
                    .ok()
                    .flatten()
                    .and_then(|trigger: Element| trigger.dyn_into::<HtmlElement>().ok())
            {
                trigger.click();
            }
            dropdown_tab_focus(&root, keyboard_event.shift_key());
            return;
        }
        let submenu: bool = content.get_attribute(DROPDOWN_SLOT_ATTRIBUTE).as_deref()
            == Some(DROPDOWN_SUB_CONTENT_SLOT);
        if key == DROPDOWN_KEY_ESCAPE
            || (submenu && key == dropdown_direction_keys(dropdown_is_rtl(&content)).1)
        {
            keyboard_event.prevent_default();
            keyboard_event.stop_propagation();
            open.set(false);
            focus_dropdown_panel_trigger(&content);
            return;
        }
        let items: Vec<HtmlElement> = dropdown_enabled_items(&root, &content);
        if items.is_empty() {
            return;
        }
        let row: Option<Element> = target.closest(DROPDOWN_ROW_SELECTOR).ok().flatten();
        let current_index: Option<usize> = items.iter().position(|item: &HtmlElement| {
            let element: &Element = item.as_ref();
            row.as_ref() == Some(element)
        });
        let labels: Vec<String> = items
            .iter()
            .map(|item: &HtmlElement| {
                item.get_attribute(DROPDOWN_LABEL_ATTRIBUTE)
                    .unwrap_or_default()
            })
            .collect();
        let printable: bool = key.chars().count() == 1 && key != DROPDOWN_KEY_SPACE;
        if !printable
            && ![
                DROPDOWN_KEY_ARROW_DOWN,
                DROPDOWN_KEY_ARROW_UP,
                DROPDOWN_KEY_HOME,
                DROPDOWN_KEY_END,
            ]
            .contains(&key.as_str())
        {
            return;
        }
        let query: String = if printable {
            let prefix: String = dropdown_search_prefix(
                &search.get(),
                &key,
                keyboard_event.time_stamp() - searched_at.get(),
            );
            search.set(prefix.clone());
            searched_at.set(keyboard_event.time_stamp());
            prefix
        } else {
            search.set(String::new());
            key.clone()
        };
        let Some(next_index) = dropdown_next_index(&labels, current_index, &query) else {
            return;
        };
        keyboard_event.prevent_default();
        if let Some(item) = items.get(next_index) {
            focus_dropdown_item(item, &content);
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
        if !dropdown_keyboard_allowed(&keyboard_event) {
            return;
        }
        let key: String = keyboard_event.key();
        if key == DROPDOWN_KEY_ESCAPE && open.get() {
            keyboard_event.prevent_default();
            open.set(false);
            if let Some(root) = dropdown_root_from_event(keyboard_event.as_ref()) {
                focus_dropdown_trigger(&root);
            }
            return;
        }
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
        schedule_dropdown_item_focus(root, open, focus_last);
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
    let target: Element = event
        .target()
        .and_then(|target: EventTarget| target.dyn_into::<Element>().ok())?;
    dropdown_root_for_target(&target)
}

/// Resolves portal ancestry through stable popup/trigger pairs at every level.
///
/// # Arguments
/// - `&Element` - An element inside the root or any portaled descendant.
///
/// # Returns
/// - `Option<Element>` - The original root container.
fn dropdown_root_for_target(target: &Element) -> Option<Element> {
    let mut current: Element = target.clone();
    loop {
        if let Some(root) = current.closest(DROPDOWN_ROOT_SELECTOR).ok().flatten() {
            return Some(root);
        }
        let content: Element = current.closest(DROPDOWN_CONTENT_SELECTOR).ok().flatten()?;
        let trigger_id: String = content.get_attribute(DROPDOWN_LABELLED_BY_ATTRIBUTE)?;
        let trigger: Element = content.owner_document()?.get_element_by_id(&trigger_id)?;
        if trigger == current {
            return None;
        }
        current = trigger;
    }
}

/// Focuses the trigger paired to exactly this popup, not a sibling or root.
///
/// # Arguments
/// - `&Element` - The closing popup.
fn focus_dropdown_panel_trigger(content: &Element) {
    if let Some(trigger) = content
        .get_attribute(DROPDOWN_LABELLED_BY_ATTRIBUTE)
        .and_then(|id: String| content.owner_document()?.get_element_by_id(&id))
        .and_then(|trigger: Element| trigger.dyn_into::<HtmlElement>().ok())
    {
        let _: Result<(), JsValue> = trigger.focus();
    }
}

/// Resolves local computed direction rather than assuming document direction.
///
/// # Arguments
/// - `&Element` - The local trigger or popup.
///
/// # Returns
/// - `bool` - Whether inline-end points left.
fn dropdown_is_rtl(target: &Element) -> bool {
    window()
        .and_then(|window: Window| window.get_computed_style(target).ok().flatten())
        .and_then(|style: web_sys::CssStyleDeclaration| {
            style.get_property_value(DROPDOWN_DIRECTION_PROPERTY).ok()
        })
        .is_some_and(|direction: String| direction == DROPDOWN_RTL)
}

/// Chooses logical submenu keys independently of physical collision flipping.
///
/// # Arguments
/// - `bool` - Whether this local level is RTL.
///
/// # Returns
/// - `(&'static str, &'static str)` - Entry and exit keys.
pub(crate) fn dropdown_direction_keys(rtl: bool) -> (&'static str, &'static str) {
    if rtl {
        (DROPDOWN_KEY_LEFT, DROPDOWN_KEY_RIGHT)
    } else {
        (DROPDOWN_KEY_RIGHT, DROPDOWN_KEY_LEFT)
    }
}

/// Schedules focus on the first or last enabled menu item.
///
/// The signal update changes the open class before the animation frame runs,
/// so the target is visible when focus is moved to it.
///
/// # Arguments
///
/// - `Element` - The dropdown root containing the menu and trigger.
/// - `Signal<bool>` - The current open state, checked before deferred focus.
/// - `bool` - Whether to focus the last enabled item instead of the first.
fn schedule_dropdown_item_focus(root: Element, open: Signal<bool>, focus_last: bool) {
    if !open.get() || !root.is_connected() {
        return;
    }
    if let Some(content) = dropdown_content_for_root(&root, open) {
        let _: Result<(), JsValue> = content.set_attribute(
            DROPDOWN_FOCUS_ATTRIBUTE,
            if focus_last {
                DROPDOWN_FOCUS_LAST
            } else {
                DROPDOWN_FOCUS_FIRST
            },
        );
        if let Ok(event) = Event::new(DROPDOWN_POSITION_EVENT) {
            let _: Result<bool, JsValue> = content.dispatch_event(&event);
        }
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
                schedule_dropdown_item_focus(root, open, false);
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
/// - `bool` - Whether selection closes the owning menu.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - The click handler.
fn select_item(
    open: Signal<bool>,
    on_select: Option<Rc<dyn Fn(&'static str)>>,
    value: &'static str,
    close_on_click: bool,
) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |event: Event| {
        if let Some(callback) = &on_select {
            callback(value);
        }
        if close_on_click {
            open.set(false);
            if let Some(root) = dropdown_root_from_event(&event) {
                focus_dropdown_trigger(&root);
            }
        }
    }))
}

/// One selectable row of a menu.
///
/// The row is a real button carrying `role="menuitem"`, so the browser
/// provides native Enter/Space activation. A disabled row carries both
/// native disabled state and `aria-disabled`, and has no click handler.
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
        open,
        close_on_click,
    }: EuvDropdownMenuItemProps = node.try_get_props().unwrap_or_default();
    let item_class: fn() -> &'static Css = if disabled {
        c_euv_dropdown_menu_item_disabled
    } else {
        c_euv_dropdown_menu_item
    };
    let on_click: Option<Rc<dyn Fn(Event)>> = if disabled {
        None
    } else {
        select_item(open, on_select, value, close_on_click)
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
            "data-label": label
            tabindex: "-1"
            class: c_euv_dropdown_menu_item()
            class: item_class()
            disabled: if disabled {
                DROPDOWN_ATTR_TRUE
            } else {
                DROPDOWN_ATTR_FALSE
            }
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
    let EuvDropdownMenuCheckboxItemProps {
        label,
        checked,
        disabled,
        open,
        close_on_click,
        on_checked_change,
    }: EuvDropdownMenuCheckboxItemProps = node.try_get_props().unwrap_or_default();
    let is_checked: bool = checked.get();
    let item_class: fn() -> &'static Css = if disabled {
        c_euv_dropdown_menu_item_disabled
    } else {
        c_euv_dropdown_menu_checkbox_item
    };
    let on_click: Option<Rc<dyn Fn(Event)>> = if disabled {
        None
    } else {
        Some(Rc::new(move |event: Event| {
            let next: bool = !checked.get();
            checked.set(next);
            if let Some(callback) = &on_checked_change {
                callback(next);
            }
            if close_on_click {
                open.set(false);
                if cfg!(target_arch = "wasm32")
                    && let Some(root) = dropdown_root_from_event(&event)
                {
                    focus_dropdown_trigger(&root);
                }
            }
        }))
    };
    html! {
        button {
            type: "button"
            role: "menuitemcheckbox"
            "data-slot": "dropdown-menu-checkbox-item"
            "data-label": label
            tabindex: "-1"
            "aria-checked": if is_checked {
                DROPDOWN_ATTR_TRUE
            } else {
                DROPDOWN_ATTR_FALSE
            }
            disabled: if disabled {
                DROPDOWN_ATTR_TRUE
            } else {
                DROPDOWN_ATTR_FALSE
            }
            "aria-disabled": if disabled {
                DROPDOWN_ATTR_TRUE
            } else {
                DROPDOWN_ATTR_FALSE
            }
            class: c_euv_dropdown_menu_checkbox_item()
            class: item_class()
            onclick: on_click
            span {
                "aria-hidden": DROPDOWN_ATTR_TRUE
                class: c_euv_dropdown_menu_checkbox_indicator()
                {
                    if checked.get() {
                        DROPDOWN_CHECKED_INDICATOR
                    } else {
                        ""
                    }
                }
            }
            {
                label
            }
        }
    }
}

/// Creates explicit radio selection state shared by a group and its items.
///
/// Call unconditionally. The default is read on first mount only. Controlled
/// activation requests a change; only the caller updates the supplied signal.
/// Use unique item values within each group.
///
/// # Arguments
///
/// - `Option<Signal<String>>` - Caller-owned value for controlled selection.
/// - `&str` - Initial uncontrolled selection.
/// - `Option<Rc<dyn Fn(String)>>` - Callback for a different requested value.
///
/// # Returns
///
/// - `EuvDropdownMenuRadioGroupState` - Explicit shared selection handle.
pub fn use_dropdown_menu_radio_group(
    value: Option<Signal<String>>,
    default_value: &str,
    on_value_change: Option<Rc<dyn Fn(String)>>,
) -> EuvDropdownMenuRadioGroupState {
    let initial: String = default_value.to_owned();
    let internal_value: Signal<String> = App::use_signal(move || initial);
    EuvDropdownMenuRadioGroupState {
        value: value.unwrap_or(internal_value),
        controlled: value.is_some(),
        on_value_change,
    }
}

/// Applies radio activation without reading or mutating DOM state.
///
/// # Arguments
///
/// - `&EuvDropdownMenuRadioItemProps` - Item settings and shared group state.
///
/// # Returns
///
/// - `bool` - Whether to restore focus after closing the owning menu.
pub(crate) fn activate_dropdown_menu_radio(props: &EuvDropdownMenuRadioItemProps) -> bool {
    if props.disabled {
        return false;
    }
    if props.selection.value.get() != props.value {
        let next: String = props.value.to_owned();
        if !props.selection.controlled {
            props.selection.value.set(next.clone());
        }
        if let Some(callback) = &props.selection.on_value_change {
            callback(next);
        }
    }
    if props.close_on_click {
        props.open.set(false);
    }
    props.close_on_click
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
    let EuvDropdownMenuRadioGroupProps { label, selection }: EuvDropdownMenuRadioGroupProps =
        node.try_get_props().unwrap_or_default();
    let children: VirtualNode = node.get_children().into();
    html! {
        div {
            role: "group"
            "data-slot": "dropdown-menu-radio-group"
            "data-value": selection.value.get()
            "aria-label": label
            class: c_euv_dropdown_menu_group()
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
    let props: EuvDropdownMenuRadioItemProps = node.try_get_props().unwrap_or_default();
    let EuvDropdownMenuRadioItemProps {
        label,
        value,
        selection,
        disabled,
        ..
    } = props.clone();
    let item_class: fn() -> &'static Css = if disabled {
        c_euv_dropdown_menu_item_disabled
    } else {
        c_euv_dropdown_menu_radio_item
    };
    let on_click: Option<Rc<dyn Fn(Event)>> = if disabled {
        None
    } else {
        Some(Rc::new(move |event: Event| {
            if activate_dropdown_menu_radio(&props)
                && cfg!(target_arch = "wasm32")
                && let Some(root) = dropdown_root_from_event(&event)
            {
                focus_dropdown_trigger(&root);
            }
        }))
    };
    html! {
        button {
            type: "button"
            role: "menuitemradio"
            "data-slot": "dropdown-menu-radio-item"
            "data-label": label
            tabindex: "-1"
            "aria-checked": if { selection.value.get() == value } {
                DROPDOWN_ATTR_TRUE
            } else {
                DROPDOWN_ATTR_FALSE
            }
            disabled: if disabled {
                DROPDOWN_ATTR_TRUE
            } else {
                DROPDOWN_ATTR_FALSE
            }
            "aria-disabled": if disabled {
                DROPDOWN_ATTR_TRUE
            } else {
                DROPDOWN_ATTR_FALSE
            }
            class: c_euv_dropdown_menu_radio_item()
            class: item_class()
            onclick: on_click
            span {
                "aria-hidden": DROPDOWN_ATTR_TRUE
                class: c_euv_dropdown_menu_checkbox_indicator()
                {
                    if selection.value.get() == value {
                        html! {
                            svg {
                                width: "8"
                                height: "8"
                                viewBox: "0 0 8 8"
                                fill: "currentColor"
                                circle { cx: "4" cy: "4" r: "4" }
                            }
                        }
                    } else {
                        VirtualNode::default()
                    }
                }
            }
            {
                label
            }
        }
    }
}

/// Connects a submenu to its immediate parent; unmount detaches both guards.
///
/// # Arguments
/// - `Signal<bool>` - Immediate owning popup state.
/// - `Signal<bool>` - This submenu's distinct open state.
pub(crate) fn use_dropdown_menu_sub_binding(parent_open: Signal<bool>, open: Signal<bool>) {
    #[cfg(target_arch = "wasm32")]
    {
        let parent: Rc<Cell<Signal<bool>>> = HookContext::use_hook(move || {
            let parent: Rc<Cell<Signal<bool>>> = Rc::new(Cell::new(parent_open));
            DROPDOWN_SUB_PARENTS.with(
                |parents: &RefCell<HashMap<usize, Weak<Cell<Signal<bool>>>>>| {
                    parents
                        .borrow_mut()
                        .insert(open.get_inner(), Rc::downgrade(&parent));
                },
            );
            parent
        });
        parent.set(parent_open);
        App::use_cleanup(move || {
            DROPDOWN_SUB_PARENTS.with(
                |parents: &RefCell<HashMap<usize, Weak<Cell<Signal<bool>>>>>| {
                    parents.borrow_mut().remove(&open.get_inner());
                },
            );
            drop(parent);
        });
    }
    let binding: Rc<RefCell<Option<DropdownSubParentBinding>>> =
        HookContext::use_hook(|| Rc::new(RefCell::new(None)));
    let cleanup: Rc<RefCell<Option<DropdownSubParentBinding>>> = binding.clone();
    App::use_cleanup(move || {
        if let Some((parent, child, parent_id, child_id)) = cleanup.borrow_mut().take() {
            parent.unsubscribe(parent_id);
            child.unsubscribe(child_id);
            child.set(false);
        }
    });
    if binding
        .borrow()
        .as_ref()
        .is_some_and(|(parent, child, _, _): &DropdownSubParentBinding| {
            *parent == parent_open && *child == open
        })
    {
        return;
    }
    if let Some((parent, child, parent_id, child_id)) = binding.borrow_mut().take() {
        parent.unsubscribe(parent_id);
        child.unsubscribe(child_id);
        child.set(false);
    }
    if !parent_open.get() {
        open.set(false);
    }
    let parent_id: usize = parent_open.subscribe(move || {
        if !parent_open.get() && open.get() {
            open.set(false);
        }
    });
    let child_id: usize = open.subscribe(move || {
        if open.get() && !parent_open.get() {
            open.set(false);
        }
    });
    *binding.borrow_mut() = Some((parent_open, open, parent_id, child_id));
}

/// The nested menu of a row.
///
/// Supply distinct `open` signals to every submenu and pass its immediate
/// parent's signal as `parent_open`. Share the submenu signal with its trigger
/// and content; action/checkbox/radio rows instead receive the owning root's
/// signal. Closing a parent resets descendants without resetting selection.
/// Pointer activation opens without entering children; logical forward-arrow
/// activation focuses the first enabled row. Hover opening retains trigger focus.
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
    let EuvDropdownMenuSubProps {
        open, parent_open, ..
    }: EuvDropdownMenuSubProps = node.try_get_props().unwrap_or_default();
    use_dropdown_menu_sub_binding(parent_open, open);
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
    let EuvDropdownMenuSubTriggerProps {
        label,
        open,
        disabled,
        open_on_hover,
        delay_ms,
        close_delay_ms,
    }: EuvDropdownMenuSubTriggerProps = node.try_get_props().unwrap_or_default();
    let hover_target: NodeRef<Element> = App::use_node_ref();
    let parent_search: Signal<String> = App::use_signal(String::new);
    let parent_searched_at: Signal<f64> = App::use_signal(|| 0.0);
    #[cfg(target_arch = "wasm32")]
    use_dropdown_sub_hover(
        open,
        hover_target.clone(),
        (disabled, open_on_hover, delay_ms, close_delay_ms),
    );
    #[cfg(not(target_arch = "wasm32"))]
    let _: (bool, bool, u32, u32) = (disabled, open_on_hover, delay_ms, close_delay_ms);
    let trigger_id: String = format!("euv-dropdown-{}-trigger", open.get_inner());
    let menu_id: String = format!("euv-dropdown-{}-menu", open.get_inner());
    let activate: Option<Rc<dyn Fn(Event)>> = if disabled {
        None
    } else {
        Some(Rc::new(move |_: Event| {
            open.set(true);
        }))
    };
    let keyboard_activate: Option<Rc<dyn Fn(Event)>> = activate.clone();
    let item_class: fn() -> &'static Css = if disabled {
        c_euv_dropdown_menu_item_disabled
    } else {
        c_euv_dropdown_menu_sub_trigger
    };
    html! {
        button {
            type: "button"
            role: "menuitem"
            "data-slot": "dropdown-menu-sub-trigger"
            ref: hover_target
            id: trigger_id
            "aria-controls": menu_id
            "data-label": label
            tabindex: "-1"
            disabled: if disabled { DROPDOWN_ATTR_TRUE } else { DROPDOWN_ATTR_FALSE }
            "aria-disabled": if disabled { DROPDOWN_ATTR_TRUE } else { DROPDOWN_ATTR_FALSE }
            onclick: activate
            onkeydown: move |event: Event| {
                let Some(keyboard) = event.dyn_ref::<KeyboardEvent>() else { return; };
                if dropdown_keyboard_allowed(keyboard)
                    && open.get()
                    && let Some(target) = keyboard.target().and_then(|target: EventTarget| target.dyn_into::<Element>().ok())
                    && dropdown_sub_trigger_exit(open.get(), &keyboard.key(), dropdown_is_rtl(&target)) {
                    keyboard.prevent_default();
                    keyboard.stop_propagation();
                    open.set(false);
                    return;
                }
                if dropdown_keyboard_allowed(keyboard)
                    && let Some(target) = keyboard.target().and_then(|target: EventTarget| target.dyn_into::<Element>().ok())
                    && keyboard.key() == dropdown_direction_keys(dropdown_is_rtl(&target)).0
                    && let Some(activate) = &keyboard_activate {
                    keyboard.prevent_default();
                    keyboard.stop_propagation();
                    if let Some(root) = dropdown_root_from_event(&event) {
                        activate(event);
                        schedule_dropdown_item_focus(root, open, false);
                    }
                    return;
                }
                #[cfg(target_arch = "wasm32")]
                if let Some(parent) = DROPDOWN_SUB_PARENTS.with(|parents: &RefCell<HashMap<usize, Weak<Cell<Signal<bool>>>>>| parents.borrow().get(&open.get_inner()).and_then(Weak::upgrade).map(|parent: Rc<Cell<Signal<bool>>>| parent.get()))
                    && let Some(handler) = dropdown_menu_keydown(parent, parent_search, parent_searched_at) {
                    handler(event);
                }
                #[cfg(not(target_arch = "wasm32"))]
                let _: (Signal<String>, Signal<f64>) = (parent_search, parent_searched_at);
            }
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
            class: item_class()
            {
                label
            }
            span {
                "data-slot": "dropdown-menu-sub-caret"
                "aria-hidden": "true"
                class: c_euv_dropdown_menu_sub_caret()
                inner_html: DROPDOWN_SUB_CARET_MARKUP
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
    dropdown_panel(open, node.get_children().into(), true)
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

/// Chooses enabled rows with wraparound navigation or case-insensitive typeahead.
///
/// # Arguments
///
/// - `&[String]` - Enabled labels in menu order.
/// - `Option<usize>` - The current enabled row index.
/// - `&str` - A navigation key or normalized search prefix.
///
/// # Returns
///
/// - `Option<usize>` - The matching row, or none without a match.
pub(crate) fn dropdown_next_index(
    labels: &[String],
    current: Option<usize>,
    query: &str,
) -> Option<usize> {
    let count: usize = labels.len();
    if count == 0 {
        return None;
    }
    let current: Option<usize> = current.filter(|index: &usize| *index < count);
    match query {
        DROPDOWN_KEY_ARROW_DOWN => Some(current.map_or(0, |index: usize| (index + 1) % count)),
        DROPDOWN_KEY_ARROW_UP => {
            Some(current.map_or(count - 1, |index: usize| (index + count - 1) % count))
        }
        DROPDOWN_KEY_HOME => Some(0),
        DROPDOWN_KEY_END => Some(count - 1),
        _ => {
            if query.is_empty() {
                return None;
            }
            let normalized: String = query.to_lowercase();
            let cycling: bool = normalized.chars().count() == 1;
            let start: usize = current.map_or(
                0,
                |index: usize| if cycling { (index + 1) % count } else { index },
            );
            (0..count)
                .map(|offset: usize| (start + offset) % count)
                .find(|index: &usize| {
                    labels[*index]
                        .trim()
                        .to_lowercase()
                        .starts_with(&normalized)
                })
        }
    }
}

/// Extends a timed prefix and collapses repeated letters for cycling.
///
/// # Arguments
///
/// - `&str` - The previous buffered prefix.
/// - `&str` - The new printable key.
/// - `f64` - Milliseconds elapsed since the previous key.
///
/// # Returns
///
/// - `String` - The next normalized prefix.
pub(crate) fn dropdown_search_prefix(previous: &str, key: &str, elapsed: f64) -> String {
    let normalized: String = key.to_lowercase();
    if (0.0..DROPDOWN_SEARCH_RESET_MS).contains(&elapsed) && previous != normalized {
        format!("{previous}{normalized}")
    } else {
        normalized
    }
}

/// Leaves composing input and modified shortcuts to the browser.
///
/// # Arguments
///
/// - `&KeyboardEvent` - The keyboard event to inspect.
///
/// # Returns
///
/// - `bool` - Whether menu keyboard handling may process this event.
fn dropdown_keyboard_allowed(event: &KeyboardEvent) -> bool {
    !event.is_composing()
        && event.key_code() != DROPDOWN_COMPOSITION_KEY_CODE
        && !event.ctrl_key()
        && !event.alt_key()
        && !event.meta_key()
        && (!event.shift_key()
            || event.key().chars().count() == 1
            || event.key() == DROPDOWN_KEY_TAB)
}

/// Collects enabled rows belonging only to this root and content.
///
/// # Arguments
///
/// - `&Element` - The owning dropdown root.
/// - `&Element` - The current popup content.
///
/// # Returns
///
/// - `Vec<HtmlElement>` - Enabled rows in DOM order, excluding nested popups.
fn dropdown_enabled_items(root: &Element, content: &Element) -> Vec<HtmlElement> {
    let mut items: Vec<HtmlElement> = Vec::new();
    let Ok(nodes) = content.query_selector_all(DROPDOWN_ITEM_SELECTOR) else {
        return items;
    };
    for index in 0..nodes.length() {
        if let Some(item) = nodes
            .item(index)
            .and_then(|node: Node| node.dyn_into::<HtmlElement>().ok())
            && dropdown_root_for_target(content).as_ref() == Some(root)
            && item
                .closest(DROPDOWN_CONTENT_SELECTOR)
                .ok()
                .flatten()
                .as_ref()
                == Some(content)
        {
            items.push(item);
        }
    }
    items
}

/// Resolves the paired popup without depending on its portaled DOM ancestry.
///
/// # Arguments
/// - `&Element` - The owning root.
/// - `Signal<bool>` - The unique root/popup signal.
///
/// # Returns
/// - `Option<Element>` - The connected, correctly labelled popup.
fn dropdown_content_for_root(root: &Element, open: Signal<bool>) -> Option<Element> {
    let content: Element = root
        .owner_document()?
        .get_element_by_id(&format!("euv-dropdown-{}-menu", open.get_inner()))?;
    (content.is_connected() && dropdown_root_for_target(&content).as_ref() == Some(root))
        .then_some(content)
}

/// Focuses a row without scrolling the page, then scrolls only its popup if needed.
///
/// # Arguments
/// - `&HtmlElement` - The enabled row.
/// - `&Element` - Its scrollable popup.
fn focus_dropdown_item(item: &HtmlElement, content: &Element) {
    let options: web_sys::FocusOptions = web_sys::FocusOptions::new();
    options.set_prevent_scroll(true);
    let _: Result<(), JsValue> = item.focus_with_options(&options);
    let row: web_sys::DomRect = item.get_bounding_client_rect();
    let panel: web_sys::DomRect = content.get_bounding_client_rect();
    let top: f64 = panel.top() + f64::from(content.client_top());
    let bottom: f64 = top + f64::from(content.client_height());
    let delta: f64 = if row.top() < top {
        row.top() - top
    } else if row.bottom() > bottom {
        row.bottom() - bottom
    } else {
        0.0
    };
    if delta != 0.0 {
        content.set_scroll_top(content.scroll_top() + delta.ceil() as i32);
    }
}

/// Preserves the trigger's logical tab order after the popup moves to the portal.
///
/// # Arguments
/// - `&Element` - The owning dropdown root.
/// - `bool` - Whether focus moves backwards to the trigger.
fn dropdown_tab_focus(root: &Element, backwards: bool) {
    if backwards {
        focus_dropdown_trigger(root);
        return;
    }
    let Some(document) = root.owner_document() else {
        return;
    };
    let Ok(Some(trigger)) = root.query_selector(DROPDOWN_TRIGGER_SELECTOR) else {
        return;
    };
    let Ok(nodes) = document.query_selector_all(DROPDOWN_TABBABLE_SELECTOR) else {
        return;
    };
    let modal: Option<Element> = trigger
        .closest(DROPDOWN_MODAL_CONTENT_SELECTOR)
        .ok()
        .flatten();
    let mut after_trigger: bool = false;
    for index in 0..nodes.length() {
        let Some(element) = nodes
            .item(index)
            .and_then(|node: Node| node.dyn_into::<HtmlElement>().ok())
        else {
            continue;
        };
        let candidate: &Element = element.as_ref();
        if candidate == &trigger {
            after_trigger = true;
            continue;
        }
        if after_trigger
            && modal
                .as_ref()
                .is_none_or(|modal: &Element| modal.contains(Some(candidate)))
            && element.tab_index() >= 0
            && !element.has_attribute(DROPDOWN_DISABLED_ATTRIBUTE)
            && element.get_bounding_client_rect().height() > 0.0
            && element
                .closest(DROPDOWN_CONTENT_SELECTOR)
                .ok()
                .flatten()
                .is_none()
        {
            let _: Result<(), JsValue> = element.focus();
            return;
        }
    }
    focus_dropdown_trigger(root);
}

/// Reads root class declarations without requiring a browser or feature identity.
///
/// # Arguments
/// - `&Css` - The selected menu class.
///
/// # Returns
/// - `DropdownGeometryPolicy` - Semantic geometry, with historical default fallbacks.
#[cfg(any(target_arch = "wasm32", test))]
pub(crate) fn dropdown_geometry_policy(css: &Css) -> DropdownGeometryPolicy {
    DropdownGeometryPolicy {
        align_start: class_property(css, "--euv-dropdown-align").as_deref() == Some("start"),
        anchor_trigger: class_property(css, "--euv-dropdown-anchor").as_deref() == Some("trigger"),
        content_width: class_property(css, "--euv-dropdown-width-mode").as_deref()
            == Some("content"),
        gap: class_property(css, "--euv-dropdown-gap")
            .and_then(|value: String| value.parse::<f64>().ok())
            .filter(|value: &f64| value.is_finite() && *value >= 0.0)
            .unwrap_or(DROPDOWN_DEFAULT_GAP),
    }
}

/// Computes anchor-matched width; intrinsic menus retain their CSS minimum width.
///
/// # Arguments
/// - `f64` - Horizontal anchor width in viewport pixels.
/// - `f64` - Available viewport width.
/// - `f64` - Horizontal transform scale.
/// - `bool` - Whether intrinsic content sizing is selected.
/// - `bool` - Whether this is a submenu, which always sizes intrinsically.
///
/// # Returns
/// - `Option<f64>` - CSS pixel width, or intrinsic sizing.
#[cfg(any(target_arch = "wasm32", test))]
pub(crate) fn dropdown_popup_width(
    anchor_width: f64,
    available_width: f64,
    scale: f64,
    content_width: bool,
    submenu: bool,
) -> Option<f64> {
    if content_width || submenu {
        None
    } else {
        Some(anchor_width.min(available_width) / scale)
    }
}

/// Chooses the popup side and clamps it within the padded viewport.
///
/// # Arguments
/// - `(f64, f64, f64, f64)` - Horizontal alignment edges and trigger top/bottom.
/// - `(f64, f64)` - Popup width and natural height.
/// - `(f64, f64)` - Viewport width and height.
/// - `f64` - The scheme's trigger gap.
///
/// # Returns
/// - `(f64, f64, f64)` - Left, top and available maximum height.
#[cfg(any(target_arch = "wasm32", test))]
pub(crate) fn dropdown_popup_position(
    anchor: (f64, f64, f64, f64),
    size: (f64, f64),
    viewport: (f64, f64),
    gap: f64,
) -> (f64, f64, f64) {
    let policy: DropdownGeometryPolicy = dropdown_geometry_policy(c_euv_dropdown_menu());
    dropdown_popup_position_aligned(anchor, size, viewport, gap, policy.get_align_start())
}

/// Chooses the popup side and clamps it within the padded viewport.
///
/// # Arguments
/// - `(f64, f64, f64, f64)` - Anchor left, right, top and bottom.
/// - `(f64, f64)` - Measured width and natural height.
/// - `(f64, f64)` - Viewport width and height.
/// - `f64` - Gap in viewport pixels.
/// - `bool` - Whether the popup aligns to the anchor's leading edge.
///
/// # Returns
/// - `(f64, f64, f64)` - Left, top and available maximum height.
#[cfg(any(target_arch = "wasm32", test))]
pub(crate) fn dropdown_popup_position_aligned(
    anchor: (f64, f64, f64, f64),
    size: (f64, f64),
    viewport: (f64, f64),
    gap: f64,
    align_start: bool,
) -> (f64, f64, f64) {
    let padding: f64 = DROPDOWN_VIEWPORT_PADDING;
    let bottom_edge: f64 = (viewport.1 - padding).max(padding);
    let below_top: f64 = (anchor.3 + gap).clamp(padding, bottom_edge);
    let above_bottom: f64 = (anchor.2 - gap).clamp(padding, bottom_edge);
    let below: f64 = (bottom_edge - below_top).max(0.0);
    let above: f64 = (above_bottom - padding).max(0.0);
    let flip: bool = size.1 > below && (size.1 <= above || above > below);
    let available: f64 = if flip { above } else { below };
    let top: f64 = if flip {
        above_bottom - size.1.min(available)
    } else {
        below_top
    };
    let desired_left: f64 = if align_start {
        anchor.0
    } else {
        anchor.1 - size.0
    };
    let left: f64 = desired_left.clamp(padding, (viewport.0 - padding - size.0).max(padding));
    (left, top, available)
}

/// Places inline-end submenus with the same flip/clamp policy as ContextMenu.
///
/// # Arguments
/// - `(f64, f64, f64)` - Trigger left, right and top viewport coordinates.
/// - `(f64, f64)` - Measured panel width and natural height.
/// - `(f64, f64)` - Viewport size.
/// - `bool` - Local computed RTL direction.
///
/// # Returns
/// - `(f64, f64, f64)` - Left, top and scrollable maximum height.
#[cfg(any(target_arch = "wasm32", test))]
pub(crate) fn dropdown_sub_position(
    anchor: (f64, f64, f64),
    size: (f64, f64),
    viewport: (f64, f64),
    rtl: bool,
) -> (f64, f64, f64) {
    let padding: f64 = DROPDOWN_VIEWPORT_PADDING;
    let left_space: f64 = (anchor.0 - padding).max(0.0);
    let right_space: f64 = (viewport.0 - padding - anchor.1).max(0.0);
    let use_left: bool = if rtl {
        size.0 <= left_space || left_space >= right_space
    } else {
        size.0 > right_space && left_space > right_space
    };
    let left: f64 = if use_left {
        anchor.0 - size.0
    } else {
        anchor.1
    };
    let available: f64 = (viewport.1 - 2.0 * padding).max(0.0);
    let top: f64 = anchor.2.clamp(
        padding,
        (viewport.1 - padding - size.1.min(available)).max(padding),
    );
    (
        left.clamp(padding, (viewport.0 - padding - size.0).max(padding)),
        top,
        (viewport.1 - padding - top).max(0.0),
    )
}

/// Measures the portaled popup, applies collision constraints, and consumes entry focus.
///
/// # Arguments
/// - `&Element` - The live popup.
#[cfg(target_arch = "wasm32")]
fn position_dropdown_popup(panel: &Element) {
    let Some(document) = panel.owner_document() else {
        return;
    };
    let Some(window) = document.default_view() else {
        return;
    };
    let Some(trigger) = panel
        .get_attribute(DROPDOWN_LABELLED_BY_ATTRIBUTE)
        .and_then(|id: String| document.get_element_by_id(&id))
    else {
        return;
    };
    let Some(root) = dropdown_root_for_target(&trigger) else {
        return;
    };
    let Some(element) = panel.dyn_ref::<HtmlElement>() else {
        return;
    };
    let viewport: (f64, f64) = (
        window
            .inner_width()
            .ok()
            .and_then(|value: JsValue| value.as_f64())
            .unwrap_or_default(),
        window
            .inner_height()
            .ok()
            .and_then(|value: JsValue| value.as_f64())
            .unwrap_or_default(),
    );
    let width: f64 = (viewport.0 - 2.0 * DROPDOWN_VIEWPORT_PADDING).max(0.0);
    let scroll_top: i32 = panel.scroll_top();
    let style: web_sys::CssStyleDeclaration = element.style();
    for (name, value) in DROPDOWN_MEASURE_STYLES {
        let _: Result<(), JsValue> = style.set_property(name, value);
    }
    let _: Result<(), JsValue> =
        style.set_property(DROPDOWN_MAX_WIDTH_PROPERTY, &format!("{width}px"));
    let policy: DropdownGeometryPolicy = dropdown_geometry_policy(c_euv_dropdown_menu());
    let submenu: bool =
        panel.get_attribute(DROPDOWN_SLOT_ATTRIBUTE).as_deref() == Some(DROPDOWN_SUB_CONTENT_SLOT);
    let horizontal_anchor: web_sys::DomRect = if policy.get_anchor_trigger() {
        trigger.get_bounding_client_rect()
    } else {
        root.get_bounding_client_rect()
    };
    if let Some(anchor_width) = dropdown_popup_width(
        horizontal_anchor.width(),
        width,
        1.0,
        policy.get_content_width(),
        submenu,
    ) {
        let _: Result<(), JsValue> =
            style.set_property(DROPDOWN_WIDTH_PROPERTY, &format!("{anchor_width}px"));
    } else {
        let _: Result<(), JsValue> = style.set_property(
            DROPDOWN_MIN_WIDTH_PROPERTY,
            &format!("min(calc(4 * {}), {width}px)", var!(space - 3xl)),
        );
    }
    if submenu {
        let _: Result<(), JsValue> =
            style.set_property(DROPDOWN_WIDTH_PROPERTY, DROPDOWN_MAX_CONTENT);
    }
    let anchor: web_sys::DomRect = trigger.get_bounding_client_rect();
    let initial_rect: web_sys::DomRect = panel.get_bounding_client_rect();
    // A transformed app root establishes the fixed containing block. Measure its
    // scale against fractional CSS dimensions, not integer-rounded offset sizes.
    let computed: Option<web_sys::CssStyleDeclaration> =
        window.get_computed_style(element).ok().flatten();
    let css_length: &dyn Fn(&str) -> Option<f64> = &|property: &str| -> Option<f64> {
        computed
            .as_ref()?
            .get_property_value(property)
            .ok()?
            .strip_suffix(DROPDOWN_PIXEL_SUFFIX)?
            .parse::<f64>()
            .ok()
    };
    let unscaled_width: f64 =
        css_length(DROPDOWN_WIDTH_PROPERTY).unwrap_or_else(|| f64::from(element.offset_width()));
    let unscaled_height: f64 =
        css_length(DROPDOWN_HEIGHT_PROPERTY).unwrap_or_else(|| f64::from(element.offset_height()));
    let scale_x: f64 = if unscaled_width > 0.0 {
        initial_rect.width() / unscaled_width
    } else {
        1.0
    };
    let scale_y: f64 = if unscaled_height > 0.0 {
        initial_rect.height() / unscaled_height
    } else {
        1.0
    };
    if scale_x <= 0.0 || scale_y <= 0.0 {
        return;
    }
    let css_width: f64 = width / scale_x;
    let _: Result<(), JsValue> =
        style.set_property(DROPDOWN_MAX_WIDTH_PROPERTY, &format!("{css_width}px"));
    if let Some(anchor_width) = dropdown_popup_width(
        horizontal_anchor.width(),
        width,
        scale_x,
        policy.get_content_width(),
        submenu,
    ) {
        let _: Result<(), JsValue> =
            style.set_property(DROPDOWN_WIDTH_PROPERTY, &format!("{anchor_width}px"));
    } else {
        let _: Result<(), JsValue> = style.set_property(
            DROPDOWN_MIN_WIDTH_PROPERTY,
            &format!("min(calc(4 * {}), {css_width}px)", var!(space - 3xl)),
        );
    }
    let rect: web_sys::DomRect = panel.get_bounding_client_rect();
    let gap: f64 = policy.get_gap();
    let rtl: bool = dropdown_is_rtl(&trigger);
    if submenu || !panel.has_attribute(DROPDOWN_DIR_ATTRIBUTE) {
        let _: Result<(), JsValue> = panel.set_attribute(
            DROPDOWN_DIR_ATTRIBUTE,
            if rtl { DROPDOWN_RTL } else { DROPDOWN_LTR },
        );
    }
    let (left, top, max_height): (f64, f64, f64) = if submenu {
        dropdown_sub_position(
            (anchor.left(), anchor.right(), anchor.top()),
            (rect.width(), rect.height()),
            viewport,
            rtl,
        )
    } else {
        dropdown_popup_position(
            (
                horizontal_anchor.left(),
                horizontal_anchor.right(),
                anchor.top(),
                anchor.bottom(),
            ),
            (rect.width(), rect.height()),
            viewport,
            gap,
        )
    };
    for (name, value) in [
        (DROPDOWN_LEFT_PROPERTY, (left - rect.left()) / scale_x),
        (DROPDOWN_TOP_PROPERTY, (top - rect.top()) / scale_y),
        (DROPDOWN_MAX_HEIGHT_PROPERTY, max_height / scale_y),
    ] {
        let _: Result<(), JsValue> = style.set_property(name, &format!("{value}px"));
    }
    panel.set_scroll_top(scroll_top);
    if let Some(request) = panel.get_attribute(DROPDOWN_FOCUS_ATTRIBUTE) {
        let _: Result<(), JsValue> = panel.remove_attribute(DROPDOWN_FOCUS_ATTRIBUTE);
        let items: Vec<HtmlElement> = dropdown_enabled_items(&root, panel);
        if let Some(item) = if request == DROPDOWN_FOCUS_LAST {
            items.last()
        } else {
            items.first()
        } {
            focus_dropdown_item(item, panel);
        }
    }
}

/// Owns one cancellable animation frame and capture scroll/resize listeners.
///
/// # Arguments
/// - `Signal<bool>` - The popup's open state.
/// - `NodeRef<Element>` - The lifecycle-managed popup reference.
#[cfg(target_arch = "wasm32")]
fn use_dropdown_position(open: Signal<bool>, target: NodeRef<Element>) {
    let teardown: Rc<RefCell<Option<Box<dyn FnOnce()>>>> = HookContext::use_hook(move || {
        let Some(window) = window() else {
            return Rc::new(RefCell::new(None));
        };
        let Some(document) = window.document() else {
            return Rc::new(RefCell::new(None));
        };
        let mounted: Rc<Cell<bool>> = Rc::new(Cell::new(true));
        let frame: Rc<Cell<Option<i32>>> = Rc::new(Cell::new(None));
        let frame_id: Rc<Cell<Option<i32>>> = frame.clone();
        let frame_mounted: Rc<Cell<bool>> = mounted.clone();
        let frame_target: NodeRef<Element> = target.clone();
        let callback: Rc<Closure<dyn FnMut(f64)>> =
            Rc::new(Closure::wrap(Box::new(move |_: f64| {
                frame_id.set(None);
                if frame_mounted.get()
                    && open.get()
                    && let Some(panel) = frame_target
                        .get_cloned()
                        .filter(|panel: &Element| panel.is_connected())
                {
                    position_dropdown_popup(&panel);
                }
            }) as Box<dyn FnMut(f64)>));
        let schedule_frame: Rc<Cell<Option<i32>>> = frame.clone();
        let schedule_mounted: Rc<Cell<bool>> = mounted.clone();
        let schedule_window: Window = window.clone();
        let schedule: Rc<dyn Fn()> = Rc::new(move || {
            if !schedule_mounted.get() || !open.get() || schedule_frame.get().is_some() {
                return;
            }
            if let Ok(id) =
                schedule_window.request_animation_frame(callback.as_ref().as_ref().unchecked_ref())
            {
                schedule_frame.set(Some(id));
            }
        });
        let event_schedule: Rc<dyn Fn()> = schedule.clone();
        let event_target: NodeRef<Element> = target.clone();
        let listener: Closure<dyn FnMut(Event)> = Closure::wrap(Box::new(move |event: Event| {
            if event.type_() == DROPDOWN_SCROLL_EVENT
                && event.target().is_some_and(|node: EventTarget| {
                    event_target.get_cloned().is_some_and(|panel: Element| {
                        let panel_target: &EventTarget = panel.as_ref();
                        panel_target == &node
                    })
                })
            {
                return;
            }
            event_schedule();
        })
            as Box<dyn FnMut(Event)>);
        for event in DROPDOWN_POSITION_EVENTS {
            let event_target: &EventTarget = if event == DROPDOWN_RESIZE_EVENT {
                window.as_ref()
            } else {
                document.as_ref()
            };
            let _: Result<(), JsValue> = event_target.add_event_listener_with_callback_and_bool(
                event,
                listener.as_ref().unchecked_ref(),
                true,
            );
        }
        let subscription_frame: Rc<Cell<Option<i32>>> = frame.clone();
        let subscription_window: Window = window.clone();
        let subscription_target: NodeRef<Element> = target;
        let subscription_schedule: Rc<dyn Fn()> = schedule.clone();
        let subscription: usize = open.subscribe(move || {
            if open.get() {
                subscription_schedule();
            } else {
                if let Some(id) = subscription_frame.take() {
                    let _: Result<(), JsValue> = subscription_window.cancel_animation_frame(id);
                }
                if let Some(panel) = subscription_target.get_cloned() {
                    let _: Result<(), JsValue> = panel.remove_attribute(DROPDOWN_FOCUS_ATTRIBUTE);
                }
            }
        });
        schedule();
        let cleanup: Box<dyn FnOnce()> = Box::new(move || {
            mounted.set(false);
            open.unsubscribe(subscription);
            if let Some(id) = frame.take() {
                let _: Result<(), JsValue> = window.cancel_animation_frame(id);
            }
            for event in DROPDOWN_POSITION_EVENTS {
                let event_target: &EventTarget = if event == DROPDOWN_RESIZE_EVENT {
                    window.as_ref()
                } else {
                    document.as_ref()
                };
                let _: Result<(), JsValue> = event_target
                    .remove_event_listener_with_callback_and_bool(
                        event,
                        listener.as_ref().unchecked_ref(),
                        true,
                    );
            }
        });
        Rc::new(RefCell::new(Some(cleanup)))
    });
    App::use_cleanup(move || {
        if let Some(cleanup) = teardown.borrow_mut().take() {
            cleanup();
        }
    });
}

/// Dismisses only presses outside both the original root and its own portaled popup.
///
/// # Arguments
/// - `Signal<bool>` - The owning root's open state.
/// - `NodeRef<Element>` - The original root's live reference.
#[cfg(target_arch = "wasm32")]
fn use_dropdown_outside(open: Signal<bool>, target: NodeRef<Element>) {
    let teardown: Rc<RefCell<Option<Box<dyn FnOnce()>>>> = HookContext::use_hook(move || {
        let Some(document) = window().and_then(|window: Window| window.document()) else {
            return Rc::new(RefCell::new(None));
        };
        let listener: Closure<dyn FnMut(Event)> = Closure::wrap(Box::new(move |event: Event| {
            if !open.get() {
                return;
            }
            let Some(root) = target
                .get_cloned()
                .filter(|root: &Element| root.is_connected())
            else {
                return;
            };
            let Some(pressed) = event
                .target()
                .and_then(|target: EventTarget| target.dyn_into::<Node>().ok())
            else {
                return;
            };
            if !root.contains(Some(&pressed))
                && !pressed
                    .dyn_ref::<Element>()
                    .is_some_and(|element: &Element| {
                        dropdown_root_for_target(element).as_ref() == Some(&root)
                    })
            {
                open.set(false);
            }
        })
            as Box<dyn FnMut(Event)>);
        for event in DROPDOWN_PRESS_EVENTS {
            let _: Result<(), JsValue> = document.add_event_listener_with_callback_and_bool(
                event,
                listener.as_ref().unchecked_ref(),
                true,
            );
        }
        let cleanup: Box<dyn FnOnce()> = Box::new(move || {
            for event in DROPDOWN_PRESS_EVENTS {
                let _: Result<(), JsValue> = document.remove_event_listener_with_callback_and_bool(
                    event,
                    listener.as_ref().unchecked_ref(),
                    true,
                );
            }
        });
        Rc::new(RefCell::new(Some(cleanup)))
    });
    App::use_cleanup(move || {
        if let Some(cleanup) = teardown.borrow_mut().take() {
            cleanup();
        }
    });
}

/// Handles an open submenu from its focused trigger before parent keyboard dismissal.
///
/// # Arguments
///
/// - `bool` - A boolean (`bool`).
/// - `&str` - Shared reference to a `str`.
/// - `bool` - A boolean (`bool`).
///
/// # Returns
///
/// - `bool` - A boolean.
pub(crate) fn dropdown_sub_trigger_exit(open: bool, key: &str, rtl: bool) -> bool {
    open && (key == DROPDOWN_KEY_ESCAPE || key == dropdown_direction_keys(rtl).1)
}

/// Rejects touch, disabled rows and closed parents before scheduling hover entry.
///
/// # Arguments
///
/// - `&str` - Shared reference to a `str`.
/// - `bool` - A boolean (`bool`).
/// - `bool` - A boolean (`bool`).
/// - `bool` - A boolean (`bool`).
/// - `bool` - A boolean (`bool`).
///
/// # Returns
///
/// - `bool` - A boolean.
#[cfg(any(target_arch = "wasm32", test))]
pub(crate) fn dropdown_hover_eligible(
    pointer: &str,
    hover: bool,
    disabled: bool,
    enabled: bool,
    parent: bool,
) -> bool {
    matches!(pointer, DROPDOWN_POINTER_MOUSE | DROPDOWN_POINTER_PEN)
        && hover
        && !disabled
        && enabled
        && parent
}

/// Chooses one cancellable branch intent without changing the owning root.
///
/// # Arguments
///
/// - `bool` - A boolean (`bool`).
/// - `bool` - A boolean (`bool`).
/// - `bool` - A boolean (`bool`).
/// - `bool` - A boolean (`bool`).
/// - `bool` - A boolean (`bool`).
///
/// # Returns
///
/// - `Option<bool>` - `Some(...)` on success, `None` otherwise.
#[cfg(any(target_arch = "wasm32", test))]
pub(crate) fn dropdown_hover_intent(
    open: bool,
    trigger: bool,
    branch: bool,
    bridge: bool,
    eligible: bool,
) -> Option<bool> {
    if trigger {
        (!open && eligible).then_some(true)
    } else if open && !branch && !bridge {
        Some(false)
    } else {
        None
    }
}

/// Tests a bounded triangle to the actual facing popup edge, including side flips.
/// Rectangles are viewport left/top/right/bottom; no RTL assumption or grace timer is used.
///
/// # Arguments
///
/// - `(f64, f64)` - A `(f64, f64)` parameter.
/// - `(f64, f64)` - A `(f64, f64)` parameter.
/// - `(f64, f64, f64, f64)` - A `(f64, f64, f64, f64)` parameter.
/// - `(f64, f64, f64, f64)` - A `(f64, f64, f64, f64)` parameter.
///
/// # Returns
///
/// - `bool` - A boolean.
#[cfg(any(target_arch = "wasm32", test))]
pub(crate) fn dropdown_hover_bridge(
    point: (f64, f64),
    exit: (f64, f64),
    trigger: (f64, f64, f64, f64),
    panel: (f64, f64, f64, f64),
) -> bool {
    if panel.2 <= panel.0 || panel.3 <= panel.1 {
        return false;
    }
    let edge: f64 = if panel.0 + panel.2 >= trigger.0 + trigger.2 {
        panel.0
    } else {
        panel.2
    };
    let width: f64 = edge - exit.0;
    if width.abs() < f64::EPSILON {
        return false;
    }
    let fraction: f64 = (point.0 - exit.0) / width;
    if !(0.0..=1.0).contains(&fraction) {
        return false;
    }
    let top: f64 = exit.1 + fraction * (panel.1 - exit.1);
    let bottom: f64 = exit.1 + fraction * (panel.3 - exit.1);
    point.1 >= top.min(bottom) && point.1 <= top.max(bottom)
}

/// Follows Portal ownership, not DOM containment, for recursive hover retention.
///
/// # Arguments
///
/// - `&Element` - Shared reference to a `Element`.
/// - `&str` - Shared reference to a `str`.
///
/// # Returns
///
/// - `bool` - A boolean.
#[cfg(target_arch = "wasm32")]
fn dropdown_hover_in_branch(target: &Element, panel_id: &str) -> bool {
    let mut current: Element = target.clone();
    while let Some(panel) = current.closest(DROPDOWN_CONTENT_SELECTOR).ok().flatten() {
        if panel.get_attribute(DROPDOWN_STATE_ATTRIBUTE).as_deref() != Some(DROPDOWN_STATE_OPEN) {
            return false;
        }
        if panel.id() == panel_id {
            return true;
        }
        let Some(trigger) = panel
            .get_attribute(DROPDOWN_LABELLED_BY_ATTRIBUTE)
            .and_then(|id: String| panel.owner_document()?.get_element_by_id(&id))
        else {
            return false;
        };
        if trigger == current {
            return false;
        }
        current = trigger;
    }
    false
}

/// Processes physical hover moves; sibling rows take precedence over the geometric bridge.
///
/// # Arguments
///
/// - `&DropdownSubHover` - Shared reference to a `DropdownSubHover`.
/// - `&Event` - Shared reference to a `Event`.
#[cfg(target_arch = "wasm32")]
fn dropdown_sub_hover_move(state: &DropdownSubHover, event: &Event) {
    let Some(trigger) = state
        .target
        .get_cloned()
        .filter(|trigger: &Element| trigger.is_connected())
    else {
        state.cancel();
        return;
    };
    let pointer: String = Reflect::get(event.as_ref(), &JsValue::from_str(DROPDOWN_POINTER_TYPE))
        .ok()
        .and_then(|value: JsValue| value.as_string())
        .unwrap_or_default();
    let hover: bool = window()
        .and_then(|window: Window| window.match_media(DROPDOWN_HOVER_MEDIA).ok().flatten())
        .is_some_and(|query: web_sys::MediaQueryList| query.matches());
    if !matches!(
        pointer.as_str(),
        DROPDOWN_POINTER_MOUSE | DROPDOWN_POINTER_PEN
    ) || !hover
    {
        state.cancel();
        return;
    }
    if !state.parent.get().get() {
        state.cancel();
        return;
    }
    let Some(mouse) = event.dyn_ref::<MouseEvent>() else {
        return;
    };
    let target: Option<Element> = event
        .target()
        .and_then(|target: EventTarget| target.dyn_into::<Element>().ok());
    let on_trigger: bool = target
        .as_ref()
        .is_some_and(|target: &Element| trigger.contains(Some(target)));
    let point: (f64, f64) = (f64::from(mouse.client_x()), f64::from(mouse.client_y()));
    if on_trigger {
        state.anchor.set(Some(point));
    }
    let panel_id: String = format!("euv-dropdown-{}-menu", state.open.get_inner());
    let in_branch: bool = state.open.get()
        && target
            .as_ref()
            .is_some_and(|target: &Element| dropdown_hover_in_branch(target, &panel_id));
    let sibling: bool = target
        .as_ref()
        .and_then(|target: &Element| target.closest(DROPDOWN_ROW_SELECTOR).ok().flatten())
        .is_some_and(|row: Element| {
            row != trigger
                && row.closest(DROPDOWN_CONTENT_SELECTOR).ok().flatten()
                    == trigger.closest(DROPDOWN_CONTENT_SELECTOR).ok().flatten()
        });
    let bridge: bool = state.open.get()
        && !sibling
        && state.anchor.get().is_some_and(|exit: (f64, f64)| {
            trigger
                .owner_document()
                .and_then(|document: Document| document.get_element_by_id(&panel_id))
                .is_some_and(|panel: Element| {
                    let row: web_sys::DomRect = trigger.get_bounding_client_rect();
                    let rect: web_sys::DomRect = panel.get_bounding_client_rect();
                    dropdown_hover_bridge(
                        point,
                        exit,
                        (row.left(), row.top(), row.right(), row.bottom()),
                        (rect.left(), rect.top(), rect.right(), rect.bottom()),
                    )
                })
        });
    let (disabled, enabled, _, _): (bool, bool, u32, u32) = state.options.get();
    let eligible: bool =
        dropdown_hover_eligible(&pointer, hover, disabled, enabled, state.parent.get().get());
    state.schedule(dropdown_hover_intent(
        state.open.get(),
        on_trigger,
        in_branch,
        bridge,
        eligible,
    ));
}

/// Owns pointer listeners, subscriptions and a single cancellable deadline per trigger.
///
/// # Arguments
///
/// - `Signal<bool>` - A `Signal<bool>` parameter.
/// - `NodeRef<Element>` - A `NodeRef<Element>` parameter.
/// - `(bool, bool, u32, u32)` - A `(bool, bool, u32, u32)` parameter.
#[cfg(target_arch = "wasm32")]
fn use_dropdown_sub_hover(
    open: Signal<bool>,
    target: NodeRef<Element>,
    options: (bool, bool, u32, u32),
) {
    let slot: Rc<RefCell<Option<(Rc<DropdownSubHover>, Box<dyn FnOnce()>)>>> =
        HookContext::use_hook(|| Rc::new(RefCell::new(None)));
    let cleanup_slot: Rc<RefCell<Option<(Rc<DropdownSubHover>, Box<dyn FnOnce()>)>>> = slot.clone();
    App::use_cleanup(move || {
        if let Some((_, cleanup)) = cleanup_slot.borrow_mut().take() {
            cleanup();
        }
    });
    let parent: Option<Rc<Cell<Signal<bool>>>> = DROPDOWN_SUB_PARENTS.with(
        |parents: &RefCell<HashMap<usize, Weak<Cell<Signal<bool>>>>>| {
            parents
                .borrow()
                .get(&open.get_inner())
                .and_then(Weak::upgrade)
        },
    );
    let Some(parent) = parent else {
        return;
    };
    if slot.borrow().as_ref().is_some_and(
        |(state, _): &(Rc<DropdownSubHover>, Box<dyn FnOnce()>)| {
            state.open == open && state.parent.get() == parent.get()
        },
    ) {
        if let Some((state, _)) = slot.borrow().as_ref()
            && state.options.replace(options) != options
        {
            state.cancel();
            state.anchor.set(None);
        }
        return;
    }
    if let Some((_, cleanup)) = slot.borrow_mut().take() {
        cleanup();
    }
    let Some(document) = window().and_then(|window: Window| window.document()) else {
        return;
    };
    let state: Rc<DropdownSubHover> = Rc::new(DropdownSubHover {
        open,
        parent,
        target,
        options: Cell::new(options),
        mounted: Cell::new(true),
        timer: Cell::new(None),
        intent: Cell::new(None),
        anchor: Cell::new(None),
        callback: RefCell::new(None),
    });
    let weak: Weak<DropdownSubHover> = Rc::downgrade(&state);
    *state.callback.borrow_mut() = Some(Closure::wrap(Box::new(move || {
        if let Some(state) = weak.upgrade() {
            state.commit();
        }
    }) as Box<dyn FnMut()>));
    let weak: Weak<DropdownSubHover> = Rc::downgrade(&state);
    let listener: Closure<dyn FnMut(Event)> = Closure::wrap(Box::new(move |event: Event| {
        let Some(state) = weak.upgrade() else {
            return;
        };
        if event.type_() == DROPDOWN_POINTER_MOVE {
            dropdown_sub_hover_move(&state, &event);
        } else if event.type_() == DROPDOWN_POINTER_OUT {
            if event
                .dyn_ref::<MouseEvent>()
                .is_some_and(|event: &MouseEvent| event.related_target().is_none())
            {
                state.schedule(state.open.get().then_some(false));
            }
        } else {
            state.cancel();
        }
    }) as Box<dyn FnMut(Event)>);
    for name in DROPDOWN_HOVER_EVENTS {
        let _: Result<(), JsValue> = document.add_event_listener_with_callback_and_bool(
            name,
            listener.as_ref().unchecked_ref(),
            true,
        );
    }
    let parent_signal: Signal<bool> = state.parent.get();
    let weak: Weak<DropdownSubHover> = Rc::downgrade(&state);
    let parent_id: usize = parent_signal.subscribe(move || {
        if !parent_signal.get()
            && let Some(state) = weak.upgrade()
        {
            state.cancel();
            state.anchor.set(None);
        }
    });
    let weak: Weak<DropdownSubHover> = Rc::downgrade(&state);
    let open_id: usize = open.subscribe(move || {
        if let Some(state) = weak.upgrade() {
            state.cancel();
        }
    });
    let cleanup_state: Rc<DropdownSubHover> = state.clone();
    let cleanup: Box<dyn FnOnce()> = Box::new(move || {
        cleanup_state.mounted.set(false);
        cleanup_state.cancel();
        open.unsubscribe(open_id);
        parent_signal.unsubscribe(parent_id);
        for name in DROPDOWN_HOVER_EVENTS {
            let _: Result<(), JsValue> = document.remove_event_listener_with_callback_and_bool(
                name,
                listener.as_ref().unchecked_ref(),
                true,
            );
        }
    });
    *slot.borrow_mut() = Some((state, cleanup));
}
