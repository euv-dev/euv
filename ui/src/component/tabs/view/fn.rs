use super::*;

/// A generic tab bar with a single panel, aligned with common site
/// frameworks.
///
/// Renders the tab bar from `items` plus a panel holding the children. The
/// selected tab is owned by the caller through `active`, so the selection
/// survives re-renders and can be shared with page-level state. The panel
/// is always mounted: switching the panel content is left to the caller's
/// own `match` / `if` on the same `active` signal.
///
/// # Arguments
///
/// - `VirtualNode<EuvTabsProps>` - The props node carrying the component configuration.
///
/// # Returns
///
/// - `VirtualNode` - The component virtual DOM tree.
#[component]
pub fn euv_tabs(node: VirtualNode<EuvTabsProps>) -> VirtualNode {
    let EuvTabsProps { items, active }: EuvTabsProps = node.try_get_props().unwrap_or_default();
    let children: VirtualNode = node.get_children().into();
    let base_tabs_class: &Css = c_euv_tabs();
    let base_bar_class: &Css = c_euv_tabs_bar();
    let base_panel_class: &Css = c_euv_tabs_panel();
    let shadcn_tabs_class: &Css = if cfg!(feature = "theme-shadcn") {
        c_euv_tabs_shadcn()
    } else {
        c_euv_tabs()
    };
    let shadcn_bar_class: &Css = if cfg!(feature = "theme-shadcn") {
        c_euv_tabs_bar_shadcn()
    } else {
        c_euv_tabs_bar()
    };
    let shadcn_panel_class: &Css = if cfg!(feature = "theme-shadcn") {
        c_euv_tabs_panel_shadcn()
    } else {
        c_euv_tabs_panel()
    };
    if items.is_empty() {
        return html! {
            div {
                "data-slot": "tabs-content"
                class: base_panel_class
                children
            }
        };
    }
    let items: Rc<Vec<EuvTabItem>> = Rc::new(items);
    let tab_buttons: Vec<VirtualNode> = items
        .iter()
        .copied()
        .map(|item: EuvTabItem| tab_button(active, item, Rc::clone(&items)))
        .collect();
    html! {
        div {
            "data-slot": "tabs"
            class: base_tabs_class
            class: shadcn_tabs_class
            div {
                role: "tablist"
                "data-slot": "tabs-list"
                class: base_bar_class
                class: shadcn_bar_class
                tab_buttons
            }
            div {
                role: "tabpanel"
                "data-slot": "tabs-content"
                class: base_panel_class
                class: shadcn_panel_class
                children
            }
        }
    }
}

/// Renders one tab button, switching between the active and inactive
/// classes through the caller-owned `active` signal.
///
/// # Arguments
///
/// - `Signal<String>` - The selected tab key signal.
/// - `EuvTabItem` - The tab entry to render.
/// - `Rc<Vec<EuvTabItem>>` - All entries for keyboard navigation.
///
/// # Returns
///
/// - `VirtualNode` - The tab button virtual DOM tree.
fn tab_button(active: Signal<String>, item: EuvTabItem, items: Rc<Vec<EuvTabItem>>) -> VirtualNode {
    let key: &'static str = item.key;
    let selected: bool = active.get() == key;
    let selected_attr: &str = if selected { "true" } else { "false" };
    let tabindex: &str = if selected { "0" } else { "-1" };
    let base_tab_class: &Css = c_euv_tab_item();
    let base_active_tab_class: &Css = c_euv_tab_item_active();
    let shadcn_tab_class: &Css = if cfg!(feature = "theme-shadcn") {
        c_euv_tab_item_shadcn()
    } else {
        c_euv_tab_item()
    };
    let shadcn_active_class: &Css = if cfg!(feature = "theme-shadcn") {
        if selected {
            c_euv_tab_item_active_shadcn()
        } else {
            c_euv_tab_item_shadcn()
        }
    } else {
        c_euv_tab_item()
    };
    html! {
        button {
            key: key
            type: "button"
            role: "tab"
            "data-slot": "tabs-trigger"
            aria-selected: selected_attr
            tabindex: tabindex
            class: base_tab_class
            class: if { active.get() == key } {
                base_active_tab_class
            } else {
                base_tab_class
            }
            class: shadcn_tab_class
            class: shadcn_active_class
            onclick: select_tab(active, key)
            onkeydown: select_tab_by_keyboard(active, key, items)
            {
                item.label
            }
        }
    }
}

/// Makes the clicked tab the active one.
///
/// # Arguments
///
/// - `Signal<String>` - The selected tab key signal.
/// - `&'static str` - The clicked tab key.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - The click handler.
fn select_tab(active: Signal<String>, key: &'static str) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |_: Event| {
        active.set(key.to_string());
    }))
}

/// Selects and focuses the adjacent tab when an arrow or edge key is pressed.
///
/// # Arguments
///
/// - `Signal<String>` - The selected tab key signal.
/// - `&'static str` - The key of the tab receiving keyboard input.
/// - `Rc<Vec<EuvTabItem>>` - The entries in their display order.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - The keyboard handler.
fn select_tab_by_keyboard(
    active: Signal<String>,
    key: &'static str,
    items: Rc<Vec<EuvTabItem>>,
) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |event: Event| {
        let Ok(keyboard_event) = event.dyn_into::<KeyboardEvent>() else {
            return;
        };
        let Some(current_index) = items.iter().position(|item: &EuvTabItem| item.key == key) else {
            return;
        };
        let Some(next_index) = tab_index_for_key(current_index, &keyboard_event.key(), items.len())
        else {
            return;
        };
        let Some(next_item) = items.get(next_index) else {
            return;
        };
        keyboard_event.prevent_default();
        active.set(next_item.key.to_string());
        let Some(current_tab) = keyboard_event
            .target()
            .and_then(|target: EventTarget| target.dyn_into::<Element>().ok())
            .and_then(|target: Element| target.closest(TAB_SELECTOR).ok().flatten())
            .and_then(|target: Element| target.dyn_into::<HtmlElement>().ok())
        else {
            return;
        };
        let Some(tab_list) = current_tab.parent_element() else {
            return;
        };
        let Ok(tabs) = tab_list.query_selector_all(TAB_SELECTOR) else {
            return;
        };
        let Some(next_tab) = tabs
            .item(next_index as u32)
            .and_then(|node: Node| node.dyn_into::<HtmlElement>().ok())
        else {
            return;
        };
        let _: Result<(), JsValue> = next_tab.focus();
    }))
}

/// Finds the tab index addressed by a horizontal tab-list navigation key.
///
/// # Arguments
///
/// - `usize` - The index of the focused tab.
/// - `&str` - The pressed key.
/// - `usize` - The number of tabs in the list.
///
/// # Returns
///
/// - `Option<usize>` - The destination index for a supported key.
fn tab_index_for_key(current_index: usize, key: &str, length: usize) -> Option<usize> {
    if length == 0 {
        return None;
    }
    match key {
        "ArrowRight" => Some((current_index + 1) % length),
        "ArrowLeft" => Some((current_index + length - 1) % length),
        "Home" => Some(0),
        "End" => Some(length - 1),
        _ => None,
    }
}
