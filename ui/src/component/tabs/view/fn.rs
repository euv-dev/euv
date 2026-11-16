use super::*;

/// A generic tab bar with a single panel, aligned with common site
/// frameworks.
///
/// Renders the tab bar from `items` plus a panel holding the children. The
/// selected tab is owned by the caller through `active`, so the selection
/// survives re-renders and can be shared with page-level state. The panel
/// is always mounted: switching the panel content is left to the caller's
/// own `match` / `if` on the same `active` signal. Every trigger controls
/// this shared panel; its accessible name follows the selected trigger.
/// The panel is tabbable for ordinary content.
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
    let EuvTabsProps {
        items,
        active,
        aria_label,
        activation_mode,
    }: EuvTabsProps = node.try_get_props().unwrap_or_default();
    let children: VirtualNode = node.get_children().into();
    let tabs_id: UseId = use_id("tabs");
    let root_id: String = tabs_id.get_value().get();
    let focus: Signal<(String, String)> = use_tabs_focus(active, &items);
    let panel_id: String = format!("{root_id}-panel");
    let shadcn_tabs_class: &Css = { c_euv_tabs() };
    let shadcn_bar_class: &Css = { c_euv_tabs_bar() };
    let shadcn_panel_class: &Css = { c_euv_tabs_panel() };
    if items.is_empty() {
        return html! {
            div {
                "data-slot": "tabs-content"
                class: shadcn_panel_class
                children
            }
        };
    }
    let items: Rc<Vec<EuvTabItem>> = Rc::new(items);
    let tab_buttons: Vec<VirtualNode> = items
        .iter()
        .copied()
        .enumerate()
        .map(|(index, item): (usize, EuvTabItem)| {
            tab_button(
                active,
                item,
                Rc::clone(&items),
                &root_id,
                index,
                focus,
                activation_mode,
            )
        })
        .collect();
    let labelledby: String = items
        .iter()
        .position(|item: &EuvTabItem| active.get() == item.key)
        .map(|index: usize| format!("{root_id}-trigger-{index}"))
        .unwrap_or_default();
    html! {
        div {
            "data-slot": "tabs"
            class: shadcn_tabs_class
            div {
                role: "tablist"
                "aria-label": aria_label
                "data-slot": "tabs-list"
                class: shadcn_bar_class
                tab_buttons
            }
            div {
                role: "tabpanel"
                id: panel_id
                "aria-labelledby": labelledby
                tabindex: "0"
                "data-slot": "tabs-content"
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
/// - `&str` - The stable identifier scoped to this tab root.
/// - `usize` - The trigger position within the root.
/// - `Signal<(String, String)>` - The observed selection and roving focus key.
/// - `EuvTabsActivationMode` - The keyboard activation policy.
///
/// # Returns
///
/// - `VirtualNode` - The tab button virtual DOM tree.
fn tab_button(
    active: Signal<String>,
    item: EuvTabItem,
    items: Rc<Vec<EuvTabItem>>,
    root_id: &str,
    index: usize,
    focus: Signal<(String, String)>,
    activation_mode: EuvTabsActivationMode,
) -> VirtualNode {
    let key: &'static str = item.key;
    let selected: bool = active.get() == key;
    let selected_attr: &str = if selected { "true" } else { "false" };
    let tabbable: bool = if activation_mode == EuvTabsActivationMode::Manual {
        focus.get().1 == key
    } else {
        selected
    };
    let tabindex: &str = if tabbable { "0" } else { "-1" };
    let trigger_id: String = format!("{root_id}-trigger-{index}");
    let panel_id: String = format!("{root_id}-panel");
    let tab_class: &Css = {
        if selected {
            c_euv_tab_item_active()
        } else {
            c_euv_tab_item()
        }
    };
    html! {
        button {
            key: key
            type: "button"
            role: "tab"
            id: trigger_id
            "aria-controls": panel_id
            "data-slot": "tabs-trigger"
            aria-selected: selected_attr
            tabindex: tabindex
            class: c_euv_tab_item()
            class: tab_class
            onclick: select_tab(active, key, focus)
            onfocus: track_tab_focus(active, key, focus)
            onkeydown: select_tab_by_keyboard(active, key, items, focus, activation_mode)
            onkeyup: prevent_tab_activation_default(activation_mode)
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
/// - `Signal<(String, String)>` - The observed selection and roving focus key.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - The click handler.
fn select_tab(
    active: Signal<String>,
    key: &'static str,
    focus: Signal<(String, String)>,
) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |_: Event| {
        focus.set((key.to_string(), key.to_string()));
        active.set(key.to_string());
    }))
}

/// Focuses the adjacent tab and selects it only in automatic mode.
///
/// # Arguments
///
/// - `Signal<String>` - The selected tab key signal.
/// - `&'static str` - The key of the tab receiving keyboard input.
/// - `Rc<Vec<EuvTabItem>>` - The entries in their display order.
/// - `Signal<(String, String)>` - The observed selection and roving focus key.
/// - `EuvTabsActivationMode` - The keyboard activation policy.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - The keyboard handler.
fn select_tab_by_keyboard(
    active: Signal<String>,
    key: &'static str,
    items: Rc<Vec<EuvTabItem>>,
    focus: Signal<(String, String)>,
    activation_mode: EuvTabsActivationMode,
) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |event: Event| {
        let Ok(keyboard_event) = event.dyn_into::<KeyboardEvent>() else {
            return;
        };
        let Some(current_index) = items.iter().position(|item: &EuvTabItem| item.key == key) else {
            return;
        };
        let Some(current_tab) = keyboard_event
            .target()
            .and_then(|target: EventTarget| target.dyn_into::<Element>().ok())
            .and_then(|target: Element| target.closest(TAB_SELECTOR).ok().flatten())
            .and_then(|target: Element| target.dyn_into::<HtmlElement>().ok())
        else {
            return;
        };
        let Some(tab_list) = current_tab.closest(TAB_LIST_SELECTOR).ok().flatten() else {
            return;
        };
        if activation_mode == EuvTabsActivationMode::Manual
            && matches!(keyboard_event.key().as_str(), TAB_KEY_ENTER | TAB_KEY_SPACE)
        {
            keyboard_event.prevent_default();
            if !keyboard_event.repeat() {
                current_tab.click();
            }
            return;
        }
        let rtl: bool = tab_list
            .owner_document()
            .and_then(|document: Document| document.default_view())
            .and_then(|window: Window| window.get_computed_style(&tab_list).ok().flatten())
            .and_then(|style: CssStyleDeclaration| {
                style.get_property_value(TAB_DIRECTION_PROPERTY).ok()
            })
            .is_some_and(|direction: String| direction == TAB_DIRECTION_RTL);
        let Some(next_index) =
            tab_index_for_key(current_index, &keyboard_event.key(), items.len(), rtl)
        else {
            return;
        };
        let Some(next_item) = items.get(next_index) else {
            return;
        };
        keyboard_event.prevent_default();
        if activation_mode == EuvTabsActivationMode::Automatic {
            active.set(next_item.key.to_string());
        }
        focus.set((active.get(), next_item.key.to_string()));
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
/// - `bool` - Whether the list has effective right-to-left direction.
///
/// # Returns
///
/// - `Option<usize>` - The destination index for a supported key.
pub(super) fn tab_index_for_key(
    current_index: usize,
    key: &str,
    length: usize,
    rtl: bool,
) -> Option<usize> {
    if length == 0 {
        return None;
    }
    match key {
        TAB_KEY_RIGHT if rtl => Some((current_index + length - 1) % length),
        TAB_KEY_LEFT if rtl => Some((current_index + 1) % length),
        TAB_KEY_RIGHT => Some((current_index + 1) % length),
        TAB_KEY_LEFT => Some((current_index + length - 1) % length),
        TAB_KEY_HOME => Some(0),
        TAB_KEY_END => Some(length - 1),
        _ => None,
    }
}

/// Retains a root's roving entrance and reconciles caller-owned selection changes.
///
/// # Arguments
///
/// - `Signal<String>` - The caller-owned selected key.
/// - `&[EuvTabItem]` - The currently available entries.
///
/// # Returns
///
/// - `Signal<(String, String)>` - The last observed selection and focused key.
fn use_tabs_focus(active: Signal<String>, items: &[EuvTabItem]) -> Signal<(String, String)> {
    let selected: String = active.get();
    let focus: Signal<(String, String)> =
        HookContext::use_hook(|| Signal::create((selected.clone(), selected.clone())));
    let (observed, focused): (String, String) = focus.get();
    if observed != selected || !items.iter().any(|item: &EuvTabItem| item.key == focused) {
        let entrance: String = items
            .iter()
            .find(|item: &&EuvTabItem| item.key == selected)
            .or_else(|| items.first())
            .map(|item: &EuvTabItem| item.key.to_string())
            .unwrap_or_default();
        focus.set((selected, entrance));
    }
    focus
}

/// Tracks native or programmatic focus without changing the selection.
///
/// # Arguments
///
/// - `Signal<String>` - The caller-owned selected key.
/// - `&'static str` - The focused trigger key.
/// - `Signal<(String, String)>` - The observed selection and roving focus key.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - The focus handler.
fn track_tab_focus(
    active: Signal<String>,
    key: &'static str,
    focus: Signal<(String, String)>,
) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |_: Event| {
        focus.set((active.get(), key.to_string()));
    }))
}

/// Suppresses native key-up activation after manual key-down selection.
///
/// # Arguments
///
/// - `EuvTabsActivationMode` - The keyboard activation policy.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - The manual activation key-up handler.
fn prevent_tab_activation_default(
    activation_mode: EuvTabsActivationMode,
) -> Option<Rc<dyn Fn(Event)>> {
    if activation_mode != EuvTabsActivationMode::Manual {
        return None;
    }
    Some(Rc::new(move |event: Event| {
        if let Ok(keyboard_event) = event.dyn_into::<KeyboardEvent>()
            && matches!(keyboard_event.key().as_str(), TAB_KEY_ENTER | TAB_KEY_SPACE)
        {
            keyboard_event.prevent_default();
        }
    }))
}
