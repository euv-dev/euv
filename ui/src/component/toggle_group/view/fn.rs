use super::*;

/// A group of toggle buttons.
///
/// The group is a row of toggles that share one selection: a
/// single-selection group keeps one value, a multiple-selection group
/// keeps every picked value. The selection lives in a caller-owned
/// signal, so the group takes part in two-way binding with the rest of
/// the page.
///
/// A disabled entry carries no click handler at all, so it can never
/// write into the signal.
///
/// # Arguments
///
/// - `VirtualNode<EuvToggleGroupProps>` - The props node.
///
/// # Returns
///
/// - `VirtualNode` - The toggle group virtual DOM tree.
#[component]
pub fn euv_toggle_group(node: VirtualNode<EuvToggleGroupProps>) -> VirtualNode {
    let EuvToggleGroupProps {
        items,
        values,
        multiple,
        collapsible,
        variant,
        size,
        on_change,
    }: EuvToggleGroupProps = node.try_get_props().unwrap_or_default();
    let default_focus_value: String = items
        .iter()
        .find(|item: &&EuvToggleGroupItem| !item.disabled)
        .map(|item: &EuvToggleGroupItem| item.value.to_string())
        .unwrap_or_default();
    let focus_value: Signal<String> =
        HookContext::use_hook(move || Signal::create(default_focus_value));
    let group_class: &Css = c_euv_toggle_group();
    let shadcn_group_class: Css = if cfg!(feature = "theme-shadcn") {
        c_euv_toggle_group_shadcn().clone()
    } else {
        Css::default()
    };
    let selected: Vec<String> = values.get();
    let entries: Vec<VirtualNode> = items
        .into_iter()
        .map(|item: EuvToggleGroupItem| {
            let is_pressed: bool = selected.iter().any(|value: &String| value == item.value);
            toggle_group_entry(ToggleGroupEntry {
                item,
                is_pressed,
                values,
                multiple,
                collapsible,
                variant,
                size,
                focus_value,
                on_change: on_change.clone(),
            })
        })
        .collect();
    html! {
        div {
            "data-slot": "toggle-group"
            class: group_class
            class: shadcn_group_class
            role: "group"
            entries
        }
    }
}

/// Draws one entry of a toggle group.
///
/// The entry carries everything the row needs, so the row is drawn by
/// one call and the group keeps the shape of its own data.
///
/// # Arguments
///
/// - `ToggleGroupEntry` - The settings the row is drawn with.
///
/// # Returns
///
/// - `VirtualNode` - The entry virtual DOM tree.
fn toggle_group_entry(entry: ToggleGroupEntry) -> VirtualNode {
    let ToggleGroupEntry {
        item,
        is_pressed,
        values,
        multiple,
        collapsible,
        variant,
        size,
        focus_value,
        on_change,
    }: ToggleGroupEntry = entry;
    let disabled: bool = item.disabled;
    let pressed_attr: &str = if is_pressed {
        TOGGLE_GROUP_ATTRIBUTE_ON
    } else {
        TOGGLE_GROUP_ATTRIBUTE_OFF
    };
    let disabled_attr: &str = if disabled {
        TOGGLE_GROUP_ATTRIBUTE_ON
    } else {
        TOGGLE_GROUP_ATTRIBUTE_EMPTY
    };
    let handler: Option<Rc<dyn Fn(Event)>> = if disabled {
        None
    } else {
        toggle_group_pick(
            item.value,
            values,
            multiple,
            collapsible,
            focus_value,
            on_change,
        )
    };
    let focus_handler: Option<Rc<dyn Fn(Event)>> = if disabled {
        None
    } else {
        toggle_group_focus(item.value, focus_value)
    };
    let keydown_handler: Option<Rc<dyn Fn(Event)>> = if disabled {
        None
    } else {
        toggle_group_keyboard_navigation(item.value, focus_value)
    };
    html! {
        button {
            type: "button"
            "data-slot": "toggle-group-item"
            "data-value": item.value
            class: toggle_class(variant, size, is_pressed)
            aria-pressed: pressed_attr
            disabled: disabled_attr
            tabindex: if { focus_value.get() == item.value } {
                "0"
            } else {
                "-1"
            }
            onfocus: focus_handler
            onkeydown: keydown_handler
            onclick: handler
            item.label
        }
    }
}

/// Builds the click handler that picks one entry.
///
/// A multiple-selection group adds the value to the selection or removes
/// it again. A single-selection group replaces the selection, and only
/// empties it when the group is collapsible and the picked entry is the
/// selected one.
///
/// # Arguments
///
/// - `&'static str` - The value of the picked entry.
/// - `Signal<Vec<String>>` - The selection signal owned by the caller.
/// - `bool` - Whether more than one entry can be selected.
/// - `bool` - Whether a single-selection group can be emptied.
/// - `Signal<String>` - The value whose entry owns the roving tab stop.
/// - `Option<Rc<dyn Fn(Vec<String>)>>` - The change callback.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - The pick click handler.
fn toggle_group_pick(
    value: &'static str,
    values: Signal<Vec<String>>,
    multiple: bool,
    collapsible: bool,
    focus_value: Signal<String>,
    on_change: Option<Rc<dyn Fn(Vec<String>)>>,
) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |_: Event| {
        let current: Vec<String> = values.get();
        let is_selected: bool = current.iter().any(|entry: &String| entry == value);
        let next: Vec<String> = if multiple {
            let mut kept: Vec<String> = Vec::with_capacity(current.len() + 1);
            for entry in current.iter() {
                if entry != value {
                    kept.push(entry.clone());
                }
            }
            if !is_selected {
                kept.push(String::from(value));
            }
            kept
        } else if collapsible && is_selected {
            Vec::new()
        } else {
            vec![String::from(value)]
        };
        focus_value.set(value.to_string());
        values.set(next.clone());
        if let Some(callback) = on_change.as_ref() {
            callback(next);
        }
    }))
}

/// Moves the tab stop to an entry when it receives focus.
///
/// # Arguments
///
/// - `&'static str` - The value of the focused entry.
/// - `Signal<String>` - The roving tab stop value.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - The entry focus handler.
fn toggle_group_focus(
    value: &'static str,
    focus_value: Signal<String>,
) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |_: Event| {
        focus_value.set(value.to_string());
    }))
}

/// Moves keyboard focus through enabled entries in a toggle group.
///
/// # Arguments
///
/// - `&'static str` - The value of the trigger receiving keyboard input.
/// - `Signal<String>` - The roving tab stop value.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - The keyboard navigation handler.
fn toggle_group_keyboard_navigation(
    value: &'static str,
    focus_value: Signal<String>,
) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |event: Event| {
        let Ok(keyboard_event) = event.dyn_into::<KeyboardEvent>() else {
            return;
        };
        let Some(target) = keyboard_event
            .target()
            .and_then(|target: EventTarget| target.dyn_into::<Element>().ok())
        else {
            return;
        };
        let Some(trigger) = target.closest(TOGGLE_GROUP_TRIGGER_SELECTOR).ok().flatten() else {
            return;
        };
        let Some(root) = trigger.closest(TOGGLE_GROUP_ROOT_SELECTOR).ok().flatten() else {
            return;
        };
        let Ok(nodes) = root.query_selector_all(TOGGLE_GROUP_TRIGGER_SELECTOR) else {
            return;
        };
        let mut triggers: Vec<HtmlElement> = Vec::new();
        for index in 0..nodes.length() {
            if let Some(button) = nodes
                .item(index)
                .and_then(|node: Node| node.dyn_into::<HtmlElement>().ok())
            {
                triggers.push(button);
            }
        }
        let Some(current_index) = triggers.iter().position(|button: &HtmlElement| {
            button
                .get_attribute(TOGGLE_GROUP_VALUE_ATTRIBUTE)
                .as_deref()
                == Some(value)
        }) else {
            return;
        };
        let next_index: usize = match keyboard_event.key().as_str() {
            "ArrowRight" | "ArrowDown" => (current_index + 1) % triggers.len(),
            "ArrowLeft" | "ArrowUp" => (current_index + triggers.len() - 1) % triggers.len(),
            "Home" => 0,
            "End" => triggers.len() - 1,
            _ => return,
        };
        let Some(next_trigger) = triggers.get(next_index) else {
            return;
        };
        let Some(next_value) = next_trigger.get_attribute(TOGGLE_GROUP_VALUE_ATTRIBUTE) else {
            return;
        };
        keyboard_event.prevent_default();
        focus_value.set(next_value);
        let _: Result<(), JsValue> = next_trigger.focus();
    }))
}
