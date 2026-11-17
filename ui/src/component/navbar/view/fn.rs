use super::*;

/// A generic fixed top navigation bar aligned with common site frameworks.
///
/// Renders the mobile hamburger button (when `drawer_open` is provided), the
/// brand block, the link list and a trailing actions area taken from the
/// children.
///
/// # Arguments
///
/// - `VirtualNode<EuvNavbarProps>` - The props node.
///
/// # Returns
///
/// - `VirtualNode` - The navbar virtual DOM tree.
#[component]
pub fn euv_navbar(node: VirtualNode<EuvNavbarProps>) -> VirtualNode {
    let EuvNavbarProps {
        route_signal,
        brand_logo,
        brand_title,
        brand_href,
        items,
        drawer_open,
    }: EuvNavbarProps = node.try_get_props().unwrap_or_default();
    let children: VirtualNode = node.get_children().into();
    let menu_button: VirtualNode = match drawer_open {
        Some(signal) => html! {
            button {
                class: if { signal } {
                    c_euv_navbar_menu_button_active()
                } else {
                    c_euv_navbar_menu_button()
                }
                onclick: UseEuvLayout::use_drawer_toggle(signal)
                "≡"
            }
        },
        None => html! {
            ""
        },
    };
    html! {
        nav {
            class: c_euv_navbar()
            menu_button
            a {
                class: c_euv_navbar_brand()
                // OPT 30: avoid `format!("#{...}")` per render; concatenate
                // the constant `#` prefix with `brand_href` once.
                href: {
                    let mut
                    href: String =
                    String::with_capacity(ROUTE_HASH_PREFIX.len() + brand_href.len());
                    href.push_str(ROUTE_HASH_PREFIX);
                    href.push_str(brand_href);
                    href
                }
                onclick: Router::link_handler(brand_href)
                span {
                    class: c_euv_navbar_logo()
                    {
                        brand_logo
                    }
                }
                span {
                    {
                        brand_title
                    }
                }
            }
            div {
                class: c_euv_navbar_links()
                for item in items.iter() {
                    euv_navbar_link {
                        route_signal
                        item: *item
                    }
                }
            }
            div {
                class: c_euv_navbar_actions()
                children
            }
        }
    }
}

/// Renders a single navbar link (internal hash route or external URL).
///
/// # Arguments
///
/// - `VirtualNode<EuvNavbarLinkProps>` - The props node.
///
/// # Returns
///
/// - `VirtualNode` - The link virtual DOM tree.
#[component]
pub fn euv_navbar_link(node: VirtualNode<EuvNavbarLinkProps>) -> VirtualNode {
    let EuvNavbarLinkProps { route_signal, item }: EuvNavbarLinkProps =
        node.try_get_props().unwrap_or_default();
    let external: bool = item.link.starts_with(NAVBAR_LINK_SCHEME_HTTP_PREFIX);
    let route: String = route_signal.get();
    let path: &str = strip_hash_anchor(&route);
    let active: bool = !external && item.link != "/" && path.starts_with(item.link);
    let link_class: fn() -> &'static Css = if active {
        c_euv_navbar_link_active
    } else {
        c_euv_navbar_link
    };
    if external {
        html! {
            a {
                class: c_euv_navbar_link()
                href: item.link
                target: "_blank"
                onclick: Router::external_link_handler(item.link)
                {
                    item.text
                }
            }
        }
    } else {
        html! {
            a {
                class: {
                    link_class()
                }
                // OPT 30: see comment in `euv_navbar` above — the same
                // `#`-prefix concatenation replaces `format!("#{}", ...)`
                // here.
                href: {
                    let mut href: String =
                        String::with_capacity(ROUTE_HASH_PREFIX.len() + item.link.len());
                    href.push_str(ROUTE_HASH_PREFIX);
                    href.push_str(item.link);
                    href
                }
                onclick: Router::link_handler(item.link)
                {
                    item.text
                }
            }
        }
    }
}

/// Strips the `#anchor` suffix from a hash route.
///
/// # Arguments
///
/// - `&str` - The raw route string.
///
/// # Returns
///
/// - `&str` - The route path without the anchor.
fn strip_hash_anchor(route: &str) -> &str {
    match route.split_once('#') {
        Some((path, _)) => path,
        None => route,
    }
}

/// The bar that holds the menus of an application.
///
/// The bar carries `role="menubar"`, which is what tells the screen
/// reader that the triggers inside it open menus rather than navigate.
///
/// # Arguments
///
/// - `VirtualNode<EuvMenubarProps>` - The props node carrying the component settings.
///
/// # Returns
///
/// - `VirtualNode` - The euv menubar virtual DOM tree.
#[component]
pub fn euv_menubar(node: VirtualNode<EuvMenubarProps>) -> VirtualNode {
    let EuvMenubarProps { label }: EuvMenubarProps = node.try_get_props().unwrap_or_default();
    let children: VirtualNode = node.get_children().into();
    html! {
        div {
            role: "menubar"
            "data-slot": "menubar"
            "aria-label": label
            class: c_euv_menubar()
            onclick: close_other_menubar_menus()
            onfocusin: update_menubar_tab_stop()
            children
        }
    }
}

/// One menu of the bar: its trigger and its content.
///
/// # Arguments
///
/// - `VirtualNode<EuvMenubarMenuProps>` - The props node carrying the component settings.
///
/// # Returns
///
/// - `VirtualNode` - The euv menubar menu virtual DOM tree.
#[component]
pub fn euv_menubar_menu(node: VirtualNode<EuvMenubarMenuProps>) -> VirtualNode {
    let EuvMenubarMenuProps { open }: EuvMenubarMenuProps =
        node.try_get_props().unwrap_or_default();
    let children: VirtualNode = node.get_children().into();
    html! {
        div {
            "data-slot": "menubar-menu"
            class: c_euv_menubar_menu()
            class: if { open } {
                c_euv_menubar_menu_open()
            } else {
                c_euv_menubar_menu_closed()
            }
            children
        }
    }
}

/// The button that opens one menu of the bar.
///
/// # Arguments
///
/// - `VirtualNode<EuvMenubarTriggerProps>` - The props node carrying the component settings.
///
/// # Returns
///
/// - `VirtualNode` - The euv menubar trigger virtual DOM tree.
#[component]
pub fn euv_menubar_trigger(node: VirtualNode<EuvMenubarTriggerProps>) -> VirtualNode {
    let EuvMenubarTriggerProps {
        label,
        open,
        id,
        index,
        disabled,
    }: EuvMenubarTriggerProps = node.try_get_props().unwrap_or_default();
    let trigger_id: String = if id.is_empty() {
        format!("{}{}", MENUBAR_TRIGGER_ID_PREFIX, label.replace(' ', "-"))
    } else {
        id.to_string()
    };
    let content_id: String = format!("{}{}", trigger_id, MENUBAR_CONTENT_ID_SUFFIX);
    let trigger_class: fn() -> &'static Css = if open.get() {
        c_euv_menubar_trigger_open
    } else {
        c_euv_menubar_trigger
    };
    let open_shadcn_class: &Css = if open.get() {
        c_euv_menubar_trigger_open()
    } else {
        &EMPTY_CLASS
    };
    reconcile_menubar_after_render(trigger_id.clone(), open, disabled);
    let on_click: Option<Rc<dyn Fn(Event)>> = if disabled {
        None
    } else {
        Some(Rc::new(move |_: Event| {
            open.set(!open.get());
        }))
    };
    let on_keydown: Option<Rc<dyn Fn(Event)>> = if disabled {
        None
    } else {
        Some(menubar_trigger_keydown(open, trigger_id.clone()))
    };
    html! {
        button {
            id: trigger_id
            type: "button"
            role: "menuitem"
            "data-slot": "menubar-trigger"
            "data-menu-index": index.to_string()
            "data-state": if { open } {
                MENUBAR_STATE_OPEN
            } else {
                MENUBAR_STATE_CLOSED
            }
            "aria-haspopup": "menu"
            "aria-controls": content_id
            "aria-expanded": if { open } {
                MENUBAR_TRUE_VALUE
            } else {
                MENUBAR_FALSE_VALUE
            }
            "aria-disabled": if disabled {
                MENUBAR_TRUE_VALUE
            } else {
                MENUBAR_FALSE_VALUE
            }
            tabindex: "-1"
            disabled: if disabled {
                MENUBAR_TRUE_VALUE
            } else {
                MENUBAR_FALSE_VALUE
            }
            class: trigger_class()
            class: {
                c_euv_menubar_trigger()
            }
            class: open_shadcn_class
            onclick: on_click
            onkeydown: on_keydown
            label
        }
    }
}

/// The panel that holds the rows of one menu.
///
/// # Arguments
///
/// - `VirtualNode<EuvMenubarContentProps>` - The props node carrying the component settings.
///
/// # Returns
///
/// - `VirtualNode` - The euv menubar content virtual DOM tree.
#[component]
pub fn euv_menubar_content(node: VirtualNode<EuvMenubarContentProps>) -> VirtualNode {
    let EuvMenubarContentProps {
        open,
        trigger_id,
        label,
    }: EuvMenubarContentProps = node.try_get_props().unwrap_or_default();
    let content_id: String = format!("{}{}", trigger_id, MENUBAR_CONTENT_ID_SUFFIX);
    let children: VirtualNode = node.get_children().into();
    html! {
        div {
            id: content_id
            role: "menu"
            "data-slot": "menubar-content"
            "data-state": if { open } {
                MENUBAR_STATE_OPEN
            } else {
                MENUBAR_STATE_CLOSED
            }
            "aria-labelledby": trigger_id
            "aria-label": label
            class: c_euv_menubar_content()
            class: if { open } {
                c_euv_menubar_content_open()
            } else {
                c_euv_menubar_content_closed()
            }
            onclick: close_menubar_after_item(open, trigger_id)
            onkeydown: menubar_content_keydown(open, trigger_id)
            children
        }
    }
}

/// Closes every sibling menu when a pointer opens a different menubar trigger.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - `Some(...)` on success, `None` otherwise.
fn close_other_menubar_menus() -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |event: Event| {
        if !event.is_trusted() {
            return;
        }
        let Some(target) = event
            .target()
            .and_then(|target: EventTarget| target.dyn_into::<Element>().ok())
        else {
            return;
        };
        let Some(active_trigger) = target.closest(MENUBAR_TRIGGER_SELECTOR).ok().flatten() else {
            return;
        };
        let Some(root) = active_trigger.closest(MENUBAR_ROOT_SELECTOR).ok().flatten() else {
            return;
        };
        if menubar_element_disabled(&active_trigger) {
            return;
        }
        for trigger in menubar_root_triggers(&root) {
            if trigger.id() != active_trigger.id()
                && trigger
                    .get_attribute(MENUBAR_ARIA_EXPANDED_ATTRIBUTE)
                    .as_deref()
                    == Some(MENUBAR_TRUE_VALUE)
                && let Ok(trigger_element) = trigger.dyn_into::<HtmlElement>()
            {
                trigger_element.click();
            }
        }
    }))
}

/// Chooses one enabled DOM position, preserving the previous roving entry.
///
/// # Arguments
///
/// - `&[bool]` - Disabled states in DOM order, independent of caller indices.
/// - `Option<usize>` - The preferred DOM position.
///
/// # Returns
///
/// - `Option<usize>` - One enabled position, or none for an all-disabled root.
pub(crate) fn menubar_enabled_tab_stop(
    disabled: &[bool],
    preferred: Option<usize>,
) -> Option<usize> {
    preferred
        .filter(|index: &usize| disabled.get(*index) == Some(&false))
        .or_else(|| disabled.iter().position(|disabled: &bool| !disabled))
}

/// Reads both native and accessibility disabled states.
///
/// # Arguments
///
/// - `&Element` - The interactive element.
///
/// # Returns
///
/// - `bool` - Whether the element is unavailable.
fn menubar_element_disabled(element: &Element) -> bool {
    element.has_attribute(MENUBAR_DISABLED_ATTRIBUTE)
        || element
            .get_attribute(MENUBAR_ARIA_DISABLED_ATTRIBUTE)
            .as_deref()
            == Some(MENUBAR_TRUE_VALUE)
}

/// Collects only triggers owned by the nearest bar, excluding nested roots.
///
/// # Arguments
///
/// - `&Element` - The owning menubar.
///
/// # Returns
///
/// - `Vec<Element>` - Triggers in DOM order.
fn menubar_root_triggers(root: &Element) -> Vec<Element> {
    let Ok(nodes) = root.query_selector_all(MENUBAR_TRIGGER_SELECTOR) else {
        return Vec::new();
    };
    (0..nodes.length())
        .filter_map(|index: u32| {
            let element: Element = nodes.item(index)?.dyn_into().ok()?;
            let owner: Element = element.closest(MENUBAR_ROOT_SELECTOR).ok().flatten()?;
            owner.is_same_node(Some(root)).then_some(element)
        })
        .collect()
}

/// Repairs one root's tab order without moving document focus.
///
/// # Arguments
///
/// - `&Element` - The owning bar.
/// - `Option<&Element>` - An enabled trigger receiving actual focus.
fn reconcile_menubar_tab_stop(root: &Element, focused: Option<&Element>) {
    let triggers: Vec<Element> = menubar_root_triggers(root);
    let remembered: Option<String> = root.get_attribute(MENUBAR_TAB_STOP_ATTRIBUTE);
    let preferred: Option<usize> = triggers.iter().position(|trigger: &Element| {
        focused
            .map(|focused: &Element| trigger.is_same_node(Some(focused)))
            .unwrap_or_else(|| remembered.as_deref() == Some(trigger.id().as_str()))
    });
    let disabled: Vec<bool> = triggers.iter().map(menubar_element_disabled).collect();
    let selected: Option<usize> = menubar_enabled_tab_stop(&disabled, preferred);
    for (index, trigger) in triggers.iter().enumerate() {
        let _: Result<(), JsValue> = trigger.set_attribute(
            MENUBAR_TAB_INDEX_ATTRIBUTE,
            if selected == Some(index) { "0" } else { "-1" },
        );
    }
    if let Some(index) = selected {
        let _: Result<(), JsValue> =
            root.set_attribute(MENUBAR_TAB_STOP_ATTRIBUTE, &triggers[index].id());
    } else {
        let _: Result<(), JsValue> = root.remove_attribute(MENUBAR_TAB_STOP_ATTRIBUTE);
    }
}

/// Reconciles mounted triggers after native disabled properties have updated.
///
/// # Arguments
///
/// - `String` - The stable trigger ID.
/// - `Signal<bool>` - The caller-owned open state.
/// - `bool` - The current disabled state.
fn reconcile_menubar_after_render(trigger_id: String, open: Signal<bool>, disabled: bool) {
    after_mount(move || {
        if disabled && open.get() {
            open.set(false);
        }
        #[cfg(target_arch = "wasm32")]
        {
            let Some(trigger) = browser_document()
                .and_then(|document: Document| document.get_element_by_id(&trigger_id))
            else {
                return;
            };
            if let Ok(Some(root)) = trigger.closest(MENUBAR_ROOT_SELECTOR) {
                reconcile_menubar_tab_stop(&root, None);
            }
        }
        #[cfg(not(target_arch = "wasm32"))]
        let _: String = trigger_id;
    });
}

/// Keeps actual enabled focus as the root's persistent roving entry.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - The focus handler for the menubar root.
fn update_menubar_tab_stop() -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |event: Event| {
        let Some(target) = event
            .target()
            .and_then(|target: EventTarget| target.dyn_into::<Element>().ok())
        else {
            return;
        };
        let Some(trigger) = target.closest(MENUBAR_TRIGGER_SELECTOR).ok().flatten() else {
            return;
        };
        if menubar_element_disabled(&trigger) {
            return;
        }
        if let Ok(Some(root)) = trigger.closest(MENUBAR_ROOT_SELECTOR) {
            reconcile_menubar_tab_stop(&root, Some(&trigger));
        }
    }))
}

/// Resolves logical horizontal navigation in DOM order without trusting caller indices.
///
/// # Arguments
///
/// - `&[bool]` - Disabled states in DOM order.
/// - `usize` - The current DOM position.
/// - `&str` - The navigation key.
/// - `bool` - Whether the root has right-to-left direction.
///
/// # Returns
///
/// - `Option<usize>` - The next enabled position, or none.
pub(crate) fn menubar_next_enabled_trigger(
    disabled: &[bool],
    current: usize,
    key: &str,
    rtl: bool,
) -> Option<usize> {
    let count: usize = disabled.len();
    if count == 0 {
        return None;
    }
    let backward: bool = (key == MENUBAR_KEY_ARROW_LEFT && !rtl)
        || (key == MENUBAR_KEY_ARROW_RIGHT && rtl)
        || key == MENUBAR_KEY_END;
    let step: usize = if backward { count - 1 } else { 1 };
    let mut candidate: usize = match key {
        MENUBAR_KEY_HOME => 0,
        MENUBAR_KEY_END => count - 1,
        _ => (current.min(count - 1) + step) % count,
    };
    for _ in 0..count {
        if !disabled[candidate] {
            return Some(candidate);
        }
        candidate = (candidate + step) % count;
    }
    None
}

/// Implements horizontal menubar navigation and opens the active menu vertically.
///
/// # Arguments
///
/// - `Signal<bool>` - The open state of this menu.
/// - `String` - The stable trigger ID paired with the content panel.
///
/// # Returns
///
/// - `Rc<dyn Fn(Event)>` - The keyboard handler for the trigger.
fn menubar_trigger_keydown(open: Signal<bool>, trigger_id: String) -> Rc<dyn Fn(Event)> {
    Rc::new(move |event: Event| {
        let Ok(keyboard_event) = event.dyn_into::<KeyboardEvent>() else {
            return;
        };
        let key: String = keyboard_event.key();
        if key.as_str() == MENUBAR_KEY_ARROW_RIGHT
            || key.as_str() == MENUBAR_KEY_ARROW_LEFT
            || key.as_str() == MENUBAR_KEY_HOME
            || key.as_str() == MENUBAR_KEY_END
        {
            keyboard_event.prevent_default();
            let Some(target) = keyboard_event
                .target()
                .and_then(|target: EventTarget| target.dyn_into::<Element>().ok())
            else {
                return;
            };
            let Some(root) = target.closest(MENUBAR_ROOT_SELECTOR).ok().flatten() else {
                return;
            };
            let triggers: Vec<Element> = menubar_root_triggers(&root);
            let Some(current_index) = triggers
                .iter()
                .position(|trigger: &Element| trigger.id() == trigger_id)
            else {
                return;
            };
            if menubar_element_disabled(&triggers[current_index]) {
                return;
            }
            let rtl: bool = web_sys::window()
                .and_then(|window: Window| window.get_computed_style(&root).ok().flatten())
                .and_then(|style: CssStyleDeclaration| {
                    style.get_property_value(MENUBAR_DIRECTION_PROPERTY).ok()
                })
                .is_some_and(|direction: String| direction == MENUBAR_RTL_VALUE);
            let disabled: Vec<bool> = triggers.iter().map(menubar_element_disabled).collect();
            let Some(next_index) =
                menubar_next_enabled_trigger(&disabled, current_index, &key, rtl)
            else {
                return;
            };
            let Ok(next_trigger) = triggers[next_index].clone().dyn_into::<HtmlElement>() else {
                return;
            };
            let was_open: bool = open.get();
            if was_open {
                open.set(false);
            }
            let _: Result<(), JsValue> = next_trigger.focus();
            if was_open
                && next_trigger
                    .get_attribute(MENUBAR_ARIA_EXPANDED_ATTRIBUTE)
                    .as_deref()
                    != Some(MENUBAR_TRUE_VALUE)
            {
                next_trigger.click();
            }
            return;
        }
        if key == MENUBAR_KEY_ARROW_DOWN
            || key == MENUBAR_KEY_ARROW_UP
            || key == MENUBAR_KEY_ENTER
            || key == MENUBAR_KEY_SPACE
        {
            keyboard_event.prevent_default();
            if !open.get() {
                open.set(true);
            }
            let content_id: String = format!("{}{}", trigger_id, MENUBAR_CONTENT_ID_SUFFIX);
            focus_menubar_content_item_after_render(content_id, key == MENUBAR_KEY_ARROW_UP);
            return;
        }
        if key == MENUBAR_KEY_ESCAPE && open.get() {
            keyboard_event.prevent_default();
            open.set(false);
            focus_menubar_trigger_after_render(trigger_id.clone());
        }
    })
}

/// Closes a menu after an ordinary item is selected; checkbox and radio rows stay open.
///
/// # Arguments
///
/// - `Signal<bool>` - The open state of the menu.
/// - `&'static str` - The trigger ID restored after selection.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - The selection handler for menu content.
fn close_menubar_after_item(
    open: Signal<bool>,
    trigger_id: &'static str,
) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |event: Event| {
        let Some(target) = event
            .target()
            .and_then(|target: EventTarget| target.dyn_into::<Element>().ok())
        else {
            return;
        };
        let Some(item) = target.closest(MENUBAR_ITEM_SELECTOR).ok().flatten() else {
            return;
        };
        let role: Option<String> = item.get_attribute("role");
        if role.as_deref() == Some(MENUBAR_ROLE_MENU_ITEM)
            && !menubar_element_disabled(&item)
            && !item.has_attribute("aria-haspopup")
        {
            open.set(false);
            focus_menubar_trigger_after_render(trigger_id.to_string());
        }
    }))
}

/// Handles Escape and vertical movement inside one open menu.
///
/// # Arguments
///
/// - `Signal<bool>` - The open state of the menu.
/// - `&'static str` - The ID of the trigger that opened the menu.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - The keyboard handler for menu content.
fn menubar_content_keydown(
    open: Signal<bool>,
    trigger_id: &'static str,
) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |event: Event| {
        let Ok(keyboard_event) = event.dyn_into::<KeyboardEvent>() else {
            return;
        };
        let key: String = keyboard_event.key();
        if key == MENUBAR_KEY_ESCAPE {
            keyboard_event.prevent_default();
            open.set(false);
            focus_menubar_trigger_after_render(trigger_id.to_string());
            return;
        }
        if !(key.as_str() == MENUBAR_KEY_ARROW_DOWN
            || key.as_str() == MENUBAR_KEY_ARROW_UP
            || key.as_str() == MENUBAR_KEY_HOME
            || key.as_str() == MENUBAR_KEY_END)
        {
            return;
        }
        keyboard_event.prevent_default();
        let Some(target) = keyboard_event
            .target()
            .and_then(|target: EventTarget| target.dyn_into::<Element>().ok())
        else {
            return;
        };
        let Some(content) = target.closest(MENUBAR_CONTENT_SELECTOR).ok().flatten() else {
            return;
        };
        let Ok(items) = content.query_selector_all(MENUBAR_ITEM_SELECTOR) else {
            return;
        };
        focus_adjacent_menubar_item(&items, &target, &key);
    }))
}

/// Opens a nested menu on the forward key and returns to its trigger on Escape.
///
/// # Arguments
///
/// - `Signal<bool>` - A `Signal<bool>` parameter.
///
/// # Returns
///
/// - `Rc<dyn Fn(Event)>` - A `Rc<dyn Fn(Event)>` value.
fn menubar_sub_trigger_keydown(open: Signal<bool>) -> Rc<dyn Fn(Event)> {
    Rc::new(move |event: Event| {
        let Ok(keyboard_event) = event.dyn_into::<KeyboardEvent>() else {
            return;
        };
        let key: String = keyboard_event.key();
        if key.as_str() == MENUBAR_KEY_ARROW_RIGHT
            || key.as_str() == MENUBAR_KEY_ARROW_DOWN
            || key.as_str() == MENUBAR_KEY_ARROW_UP
        {
            keyboard_event.prevent_default();
            keyboard_event.stop_propagation();
            if !open.get() {
                open.set(true);
            }
            let Some(target) = keyboard_event
                .target()
                .and_then(|target: EventTarget| target.dyn_into::<Element>().ok())
            else {
                return;
            };
            let Some(parent) = target.parent_element() else {
                return;
            };
            let focus_last: bool = key == MENUBAR_KEY_ARROW_UP;
            after_mount(move || {
                #[cfg(target_arch = "wasm32")]
                {
                    if let Ok(Some(content)) = parent.query_selector(MENUBAR_SUB_CONTENT_SELECTOR) {
                        if let Ok(items) = content.query_selector_all(MENUBAR_ITEM_SELECTOR) {
                            let count: usize = items.length() as usize;
                            let mut index: usize = if focus_last {
                                count.saturating_sub(1)
                            } else {
                                0
                            };
                            for _ in 0..count {
                                let Some(node) = items.item(index as u32) else {
                                    return;
                                };
                                let Ok(item) = node.dyn_into::<Element>() else {
                                    return;
                                };
                                if !item.has_attribute(MENUBAR_DISABLED_ATTRIBUTE) {
                                    if let Ok(item) = item.dyn_into::<HtmlElement>() {
                                        let _: Result<(), JsValue> = item.focus();
                                    }
                                    return;
                                }
                                index = if focus_last {
                                    index.saturating_sub(1)
                                } else {
                                    index + 1
                                };
                            }
                        }
                    }
                }
                #[cfg(not(target_arch = "wasm32"))]
                let _: (Element, bool) = (parent, focus_last);
            });
            return;
        }
        if key == MENUBAR_KEY_ESCAPE && open.get() {
            keyboard_event.prevent_default();
            keyboard_event.stop_propagation();
            open.set(false);
            if let Some(target) = keyboard_event
                .target()
                .and_then(|target: EventTarget| target.dyn_into::<HtmlElement>().ok())
            {
                let _: Result<(), JsValue> = target.focus();
            }
        }
    })
}

/// Focuses the menu item addressed by a vertical arrow, Home or End key.
///
/// # Arguments
///
/// - `&NodeList` - Shared reference to a `NodeList`.
/// - `&Element` - Shared reference to a `Element`.
/// - `&str` - Shared reference to a `str`.
fn focus_adjacent_menubar_item(items: &NodeList, target: &Element, key: &str) {
    let item_count: usize = items.length() as usize;
    if item_count == 0 {
        return;
    }
    let current: Option<Element> = target.closest(MENUBAR_ITEM_SELECTOR).ok().flatten();
    let current_index: Option<usize> = current.as_ref().and_then(|current_item: &Element| {
        (0..item_count).find(|index: &usize| {
            items
                .item(*index as u32)
                .map(|item: Node| current_item.is_same_node(Some(&item)))
                .unwrap_or(false)
        })
    });
    let start_index: usize = match key {
        MENUBAR_KEY_HOME => 0,
        MENUBAR_KEY_END => item_count - 1,
        MENUBAR_KEY_ARROW_DOWN => current_index
            .map(|index: usize| (index + 1) % item_count)
            .unwrap_or(0),
        _ => current_index
            .map(|index: usize| (index + item_count - 1) % item_count)
            .unwrap_or(item_count - 1),
    };
    let direction: usize = if key == MENUBAR_KEY_ARROW_UP || key == MENUBAR_KEY_END {
        item_count - 1
    } else {
        1
    };
    let mut index: usize = start_index;
    for _ in 0..item_count {
        let Some(node) = items.item(index as u32) else {
            return;
        };
        let Ok(item) = node.dyn_into::<Element>() else {
            return;
        };
        if !item.has_attribute(MENUBAR_DISABLED_ATTRIBUTE)
            && item
                .get_attribute(MENUBAR_ARIA_DISABLED_ATTRIBUTE)
                .as_deref()
                != Some(MENUBAR_TRUE_VALUE)
        {
            if let Ok(item) = item.dyn_into::<HtmlElement>() {
                let _: Result<(), JsValue> = item.focus();
            }
            return;
        }
        index = (index + direction) % item_count;
    }
}

/// Focuses the first or last enabled item after the panel has been displayed.
///
/// # Arguments
///
/// - `String` - A `String` parameter.
/// - `bool` - A boolean (`bool`).
fn focus_menubar_content_item_after_render(content_id: String, last: bool) {
    after_mount(move || {
        #[cfg(target_arch = "wasm32")]
        {
            let Some(document) = browser_document() else {
                return;
            };
            let Some(content) = document.get_element_by_id(&content_id) else {
                return;
            };
            let Ok(items) = content.query_selector_all(MENUBAR_ITEM_SELECTOR) else {
                return;
            };
            let count: usize = items.length() as usize;
            let mut index: usize = if last { count.saturating_sub(1) } else { 0 };
            for _ in 0..count {
                let Some(node) = items.item(index as u32) else {
                    return;
                };
                let Ok(item) = node.dyn_into::<Element>() else {
                    return;
                };
                if !item.has_attribute(MENUBAR_DISABLED_ATTRIBUTE)
                    && item
                        .get_attribute(MENUBAR_ARIA_DISABLED_ATTRIBUTE)
                        .as_deref()
                        != Some(MENUBAR_TRUE_VALUE)
                {
                    if let Ok(item) = item.dyn_into::<HtmlElement>() {
                        let _: Result<(), JsValue> = item.focus();
                    }
                    return;
                }
                index = if last {
                    index.saturating_sub(1)
                } else {
                    index + 1
                };
            }
        }
        #[cfg(not(target_arch = "wasm32"))]
        let _: (String, bool) = (content_id, last);
    });
}

/// Restores focus to the trigger after the menu has closed.
///
/// # Arguments
///
/// - `String` - A `String` parameter.
fn focus_menubar_trigger_after_render(trigger_id: String) {
    after_mount(move || {
        #[cfg(target_arch = "wasm32")]
        {
            let Some(document) = browser_document() else {
                return;
            };
            if let Some(trigger) = document
                .get_element_by_id(&trigger_id)
                .and_then(|trigger: Element| trigger.dyn_into::<HtmlElement>().ok())
            {
                let _: Result<(), JsValue> = trigger.focus();
            }
        }
        #[cfg(not(target_arch = "wasm32"))]
        let _: String = trigger_id;
    });
}

/// One row of a menu panel.
///
/// # Arguments
///
/// - `VirtualNode<EuvMenubarItemProps>` - The props node carrying the component settings.
///
/// # Returns
///
/// - `VirtualNode` - The euv menubar item virtual DOM tree.
#[component]
pub fn euv_menubar_item(node: VirtualNode<EuvMenubarItemProps>) -> VirtualNode {
    let EuvMenubarItemProps {
        label,
        value,
        disabled,
        on_select,
    }: EuvMenubarItemProps = node.try_get_props().unwrap_or_default();
    let on_click: Option<Rc<dyn Fn(Event)>> = if disabled {
        None
    } else {
        Some(Rc::new(move |_: Event| {
            if let Some(callback) = &on_select {
                callback(value);
            }
        }))
    };
    if disabled {
        html! {
            button {
                type: "button"
                role: "menuitem"
                "data-slot": "menubar-item"
                { MENUBAR_DATA_DISABLED_ATTRIBUTE.to_string() }: ""
                "aria-disabled": MENUBAR_TRUE_VALUE
                tabindex: "-1"
                disabled: MENUBAR_TRUE_VALUE
                class: c_euv_menubar_item()
                onclick: on_click
                label
            }
        }
    } else {
        html! {
            button {
                type: "button"
                role: "menuitem"
                "data-slot": "menubar-item"
                "aria-disabled": MENUBAR_FALSE_VALUE
                tabindex: "-1"
                disabled: MENUBAR_FALSE_VALUE
                class: c_euv_menubar_item()
                onclick: on_click
                label
            }
        }
    }
}

/// A set of rows that belong together.
///
/// # Arguments
///
/// - `VirtualNode<EuvMenubarGroupProps>` - The props node carrying the component settings.
///
/// # Returns
///
/// - `VirtualNode` - The euv menubar group virtual DOM tree.
#[component]
pub fn euv_menubar_group(node: VirtualNode<EuvMenubarGroupProps>) -> VirtualNode {
    let EuvMenubarGroupProps { label }: EuvMenubarGroupProps =
        node.try_get_props().unwrap_or_default();
    let children: VirtualNode = node.get_children().into();
    html! {
        div {
            role: "group"
            "data-slot": "menubar-group"
            "aria-label": label
            class: c_euv_menubar_group()
            children
        }
    }
}

/// The rule that divides one set of rows from the next.
///
/// # Arguments
///
/// - `VirtualNode<EuvMenubarSeparatorProps>` - The props node carrying the component settings.
///
/// # Returns
///
/// - `VirtualNode` - The euv menubar separator virtual DOM tree.
#[component]
pub fn euv_menubar_separator(node: VirtualNode<EuvMenubarSeparatorProps>) -> VirtualNode {
    let EuvMenubarSeparatorProps { label }: EuvMenubarSeparatorProps =
        node.try_get_props().unwrap_or_default();
    html! {
        div {
            role: "separator"
            "data-slot": "menubar-separator"
            "aria-label": label
            class: c_euv_menubar_separator()
        }
    }
}

/// The heading that names a set of rows.
///
/// # Arguments
///
/// - `VirtualNode<EuvMenubarLabelProps>` - The props node carrying the component settings.
///
/// # Returns
///
/// - `VirtualNode` - The euv menubar label virtual DOM tree.
#[component]
pub fn euv_menubar_label(node: VirtualNode<EuvMenubarLabelProps>) -> VirtualNode {
    let EuvMenubarLabelProps { text, inset }: EuvMenubarLabelProps =
        node.try_get_props().unwrap_or_default();
    let label_class: fn() -> &'static Css = if inset {
        c_euv_menubar_label_inset
    } else {
        c_euv_menubar_label
    };
    html! {
        div {
            "data-slot": "menubar-label"
            class: label_class()
            {
                text
            }
        }
    }
}

/// The keyboard hint shown at the end of a row.
///
/// # Arguments
///
/// - `VirtualNode<EuvMenubarShortcutProps>` - The props node carrying the component settings.
///
/// # Returns
///
/// - `VirtualNode` - The euv menubar shortcut virtual DOM tree.
#[component]
pub fn euv_menubar_shortcut(node: VirtualNode<EuvMenubarShortcutProps>) -> VirtualNode {
    let EuvMenubarShortcutProps { text }: EuvMenubarShortcutProps =
        node.try_get_props().unwrap_or_default();
    html! {
        span {
            "data-slot": "menubar-shortcut"
            class: c_euv_menubar_shortcut()
            {
                text
            }
        }
    }
}

/// A row that carries a box the reader ticks.
///
/// The row reports the ticked state the way the reference does: the role
/// is `menuitemcheckbox` and the state lives in `data-state`.
///
/// # Arguments
///
/// - `VirtualNode<EuvMenubarCheckboxItemProps>` - The props node carrying the component settings.
///
/// # Returns
///
/// - `VirtualNode` - The euv menubar checkbox item virtual DOM tree.
#[component]
pub fn euv_menubar_checkbox_item(node: VirtualNode<EuvMenubarCheckboxItemProps>) -> VirtualNode {
    let EuvMenubarCheckboxItemProps {
        label,
        checked,
        disabled,
        on_select,
    }: EuvMenubarCheckboxItemProps = node.try_get_props().unwrap_or_default();
    let is_checked: bool = checked.get();
    let on_click: Option<Rc<dyn Fn(Event)>> = if disabled {
        None
    } else {
        Some(Rc::new(move |_: Event| {
            if let Some(callback) = &on_select {
                callback(!checked.get());
            }
        }))
    };
    if disabled {
        html! {
            button {
                type: "button"
                role: "menuitemcheckbox"
                "data-slot": "menubar-checkbox-item"
                {
                    MENUBAR_DATA_DISABLED_ATTRIBUTE.to_string()
                }
                : ""
                "aria-checked": if is_checked {
                    MENUBAR_TRUE_VALUE
                } else {
                    MENUBAR_FALSE_VALUE
                }
                "aria-disabled": MENUBAR_TRUE_VALUE
                "data-state": if is_checked {
                    "checked"
                } else {
                    "unchecked"
                }
                tabindex: "-1"
                disabled: MENUBAR_TRUE_VALUE
                class: c_euv_menubar_checkbox_item()
                onclick: on_click
                span {
                    class: c_euv_menubar_checkbox_item_indicator()
                    if is_checked {
                        "✓"
                    }
                }
                label
            }
        }
    } else {
        html! {
            button {
                type: "button"
                role: "menuitemcheckbox"
                "data-slot": "menubar-checkbox-item"
                "aria-checked": if is_checked {
                    MENUBAR_TRUE_VALUE
                } else {
                    MENUBAR_FALSE_VALUE
                }
                "aria-disabled": MENUBAR_FALSE_VALUE
                "data-state": if is_checked {
                    "checked"
                } else {
                    "unchecked"
                }
                tabindex: "-1"
                disabled: MENUBAR_FALSE_VALUE
                class: c_euv_menubar_checkbox_item()
                onclick: on_click
                span {
                    class: c_euv_menubar_checkbox_item_indicator()
                    if is_checked {
                        "✓"
                    }
                }
                label
            }
        }
    }
}

/// A row that carries a dot the reader moves.
///
/// The row reports the chosen state the way the reference does: the role
/// is `menuitemradio` and the state lives in `data-state`.
///
/// # Arguments
///
/// - `VirtualNode<EuvMenubarRadioItemProps>` - The props node carrying the component settings.
///
/// # Returns
///
/// - `VirtualNode` - The euv menubar radio item virtual DOM tree.
#[component]
pub fn euv_menubar_radio_item(node: VirtualNode<EuvMenubarRadioItemProps>) -> VirtualNode {
    let EuvMenubarRadioItemProps {
        label,
        value,
        checked,
        disabled,
        on_select,
    }: EuvMenubarRadioItemProps = node.try_get_props().unwrap_or_default();
    let is_checked: bool = checked.get();
    let on_click: Option<Rc<dyn Fn(Event)>> = if disabled {
        None
    } else {
        Some(Rc::new(move |_: Event| {
            if let Some(callback) = &on_select {
                callback(value);
            }
        }))
    };
    if disabled {
        html! {
            button {
                type: "button"
                role: "menuitemradio"
                "data-slot": "menubar-radio-item"
                {
                    MENUBAR_DATA_DISABLED_ATTRIBUTE.to_string()
                }
                : ""
                "aria-checked": if is_checked {
                    MENUBAR_TRUE_VALUE
                } else {
                    MENUBAR_FALSE_VALUE
                }
                "aria-disabled": MENUBAR_TRUE_VALUE
                "data-state": if is_checked {
                    "checked"
                } else {
                    "unchecked"
                }
                tabindex: "-1"
                disabled: MENUBAR_TRUE_VALUE
                class: c_euv_menubar_radio_item()
                onclick: on_click
                span {
                    class: c_euv_menubar_radio_item_indicator()
                    if is_checked {
                        span {
                            class: c_euv_menubar_radio_item_indicator_dot()
                        }
                    }
                }
                label
            }
        }
    } else {
        html! {
            button {
                type: "button"
                role: "menuitemradio"
                "data-slot": "menubar-radio-item"
                "aria-checked": if is_checked {
                    MENUBAR_TRUE_VALUE
                } else {
                    MENUBAR_FALSE_VALUE
                }
                "aria-disabled": MENUBAR_FALSE_VALUE
                "data-state": if is_checked {
                    "checked"
                } else {
                    "unchecked"
                }
                tabindex: "-1"
                disabled: MENUBAR_FALSE_VALUE
                class: c_euv_menubar_radio_item()
                onclick: on_click
                span {
                    class: c_euv_menubar_radio_item_indicator()
                    if is_checked {
                        span {
                            class: c_euv_menubar_radio_item_indicator_dot()
                        }
                    }
                }
                label
            }
        }
    }
}

/// The set of radio rows that share one choice.
///
/// # Arguments
///
/// - `VirtualNode<EuvMenubarRadioGroupProps>` - The props node carrying the component settings.
///
/// # Returns
///
/// - `VirtualNode` - The euv menubar radio group virtual DOM tree.
#[component]
pub fn euv_menubar_radio_group(node: VirtualNode<EuvMenubarRadioGroupProps>) -> VirtualNode {
    let EuvMenubarRadioGroupProps {
        label,
        value: _value,
    }: EuvMenubarRadioGroupProps = node.try_get_props().unwrap_or_default();
    let children: VirtualNode = node.get_children().into();
    html! {
        div {
            role: "group"
            "data-slot": "menubar-radio-group"
            "aria-label": label
            class: c_euv_menubar_radio_group()
            children
        }
    }
}

/// The container of a nested menu.
///
/// # Arguments
///
/// - `VirtualNode<EuvMenubarSubProps>` - The props node carrying the component settings.
///
/// # Returns
///
/// - `VirtualNode` - The euv menubar sub virtual DOM tree.
#[component]
pub fn euv_menubar_sub(node: VirtualNode<EuvMenubarSubProps>) -> VirtualNode {
    let EuvMenubarSubProps { label }: EuvMenubarSubProps = node.try_get_props().unwrap_or_default();
    let children: VirtualNode = node.get_children().into();
    html! {
        div {
            "data-slot": "menubar-sub"
            "aria-label": label
            class: c_euv_menubar_sub()
            children
        }
    }
}

/// The button that opens a nested menu.
///
/// # Arguments
///
/// - `VirtualNode<EuvMenubarSubTriggerProps>` - The props node carrying the component settings.
///
/// # Returns
///
/// - `VirtualNode` - The euv menubar sub trigger virtual DOM tree.
#[component]
pub fn euv_menubar_sub_trigger(node: VirtualNode<EuvMenubarSubTriggerProps>) -> VirtualNode {
    let EuvMenubarSubTriggerProps {
        label,
        id,
        open,
        disabled,
        inset,
    }: EuvMenubarSubTriggerProps = node.try_get_props().unwrap_or_default();
    let trigger_id: String = if id.is_empty() {
        format!(
            "{}{}-submenu",
            MENUBAR_TRIGGER_ID_PREFIX,
            label.replace(' ', "-")
        )
    } else {
        id.to_string()
    };
    let content_id: String = format!("{}{}", trigger_id, MENUBAR_CONTENT_ID_SUFFIX);
    let on_click: Option<Rc<dyn Fn(Event)>> = if disabled {
        None
    } else {
        Some(Rc::new(move |_: Event| {
            open.set(!open.get());
        }))
    };
    let on_keydown: Option<Rc<dyn Fn(Event)>> = if disabled {
        None
    } else {
        Some(menubar_sub_trigger_keydown(open))
    };
    let trigger_class: fn() -> &'static Css = if inset {
        c_euv_menubar_sub_trigger_inset
    } else {
        c_euv_menubar_sub_trigger
    };
    html! {
        button {
            id: trigger_id
            type: "button"
            role: "menuitem"
            "data-slot": "menubar-sub-trigger"
            "aria-controls": content_id
            "aria-haspopup": "menu"
            "aria-expanded": if { open } {
                MENUBAR_TRUE_VALUE
            } else {
                MENUBAR_FALSE_VALUE
            }
            "data-state": if { open } {
                MENUBAR_STATE_OPEN
            } else {
                MENUBAR_STATE_CLOSED
            }
            "aria-disabled": if disabled {
                MENUBAR_TRUE_VALUE
            } else {
                MENUBAR_FALSE_VALUE
            }
            disabled: if disabled {
                MENUBAR_TRUE_VALUE
            } else {
                MENUBAR_FALSE_VALUE
            }
            class: trigger_class()
            class: if { open } {
                c_euv_menubar_sub_trigger_open()
            } else {
                &EMPTY_CLASS
            }
            class: {
                c_euv_menubar_sub_trigger()
            }
            onclick: on_click
            onkeydown: on_keydown
            label
            span {
                "aria-hidden": MENUBAR_TRUE_VALUE
                class: c_euv_menubar_sub_trigger_indicator()
                MENUBAR_SUBMENU_INDICATOR
            }
        }
    }
}

/// The panel that holds the rows of a nested menu.
///
/// # Arguments
///
/// - `VirtualNode<EuvMenubarSubContentProps>` - The props node carrying the component settings.
///
/// # Returns
///
/// - `VirtualNode` - The euv menubar sub content virtual DOM tree.
#[component]
pub fn euv_menubar_sub_content(node: VirtualNode<EuvMenubarSubContentProps>) -> VirtualNode {
    let EuvMenubarSubContentProps {
        open,
        trigger_id,
        label,
    }: EuvMenubarSubContentProps = node.try_get_props().unwrap_or_default();
    let content_id: String = format!("{}{}", trigger_id, MENUBAR_CONTENT_ID_SUFFIX);
    let children: VirtualNode = node.get_children().into();
    html! {
        div {
            id: content_id
            role: "menu"
            "data-slot": "menubar-sub-content"
            "data-state": if { open } {
                MENUBAR_STATE_OPEN
            } else {
                MENUBAR_STATE_CLOSED
            }
            "aria-labelledby": trigger_id
            "aria-label": label
            class: c_euv_menubar_sub_content()
            class: if { open } {
                c_euv_menubar_sub_content_open()
            } else {
                c_euv_menubar_sub_content_closed()
            }
            onclick: close_menubar_after_item(open, trigger_id)
            onkeydown: menubar_content_keydown(open, trigger_id)
            children
        }
    }
}
