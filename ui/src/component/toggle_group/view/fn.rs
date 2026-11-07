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
                on_change: on_change.clone(),
            })
        })
        .collect();
    html! {
        div {
            class: c_euv_toggle_group()
            role: "group"
            entries
        }
    }
}

/// Renders one entry of the group.
///
/// # Arguments
///
/// - `EuvToggleGroupItem` - The entry this row renders.
/// - `bool` - Whether the entry is pressed.
/// - `Signal<Vec<String>>` - The selection signal owned by the caller.
/// - `bool` - Whether more than one entry can be selected.
/// - `bool` - Whether a single-selection group can be emptied.
/// - `EuvToggleVariant` - The visual variant.
/// - `EuvToggleSize` - The control size.
/// - `Option<Rc<dyn Fn(Vec<String>)>>` - The change callback.
///
/// # Returns
///
/// - `VirtualNode` - The entry virtual DOM tree.

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
        toggle_group_pick(item.value, values, multiple, collapsible, on_change)
    };
    html! {
        button {
            type: "button"
            class: toggle_class(variant, size, is_pressed)
            aria-pressed: pressed_attr
            disabled: disabled_attr
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
        values.set(next.clone());
        if let Some(callback) = on_change.as_ref() {
            callback(next);
        }
    }))
}
