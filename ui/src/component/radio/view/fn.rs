use super::*;

/// A radio group component with two-way binding via a signal.
///
/// Renders a `c_euv_radio_group` container holding the group label and
/// one `c_euv_radio_item` row per option. Each row wraps a real
/// `<input type="radio">` — visually hidden by `c_euv_radio_input` but
/// still focusable and form-submittable — so keyboard and screen-reader
/// behaviour is the browser's own. Selecting an option writes its value
/// into the caller-owned signal through [`on_radio_change`]. The item
/// rows carry `c_euv_radio_item_checked` / `c_euv_radio_item_unchecked`
/// to draw the square marker on the selected row.
///
/// The option rows are built before the `html!` block rather than with
/// a `for` loop inside it: the per-row marker class is a reactive
/// `AttributeValue`, and its closure must own its captured state for
/// `'static`, which a borrow of the local `options` vector cannot
/// satisfy.
///
/// # Arguments
///
/// - `VirtualNode<EuvRadioGroupProps>` - The props node containing group configuration.
///
/// # Returns
///
/// - `VirtualNode` - A styled radio group.
#[component]
pub fn euv_radio(node: VirtualNode<EuvRadioGroupProps>) -> VirtualNode {
    let EuvRadioGroupProps {
        name,
        options,
        value,
        label: label_text,
    }: EuvRadioGroupProps = node.try_get_props().unwrap_or_default();
    let selected: String = value.get();
    let items: Vec<VirtualNode> = options
        .into_iter()
        .map(|option: EuvRadioOption| {
            let is_checked: bool = selected == option.value;
            html! {
                label {
                    class: c_euv_radio_item()
                    class: if is_checked {
                        c_euv_radio_item_checked()
                    } else {
                        c_euv_radio_item_unchecked()
                    }
                    input {
                        name: name
                        type: "radio"
                        value: option.value
                        checked: is_checked
                        class: c_euv_radio_input()
                        onchange: on_radio_change(value, option.value)
                    }
                    {
                        option.label
                    }
                }
            }
        })
        .collect();
    html! {
        div {
            class: c_euv_radio_group()
            div {
                class: c_form_label()
                label_text
            }
            items
        }
    }
}

/// Builds a change handler that selects `option_value`.
///
/// The handler reads the target input rather than trusting the closure
/// capture, so a browser that reports a normalised value still selects
/// the matching option.
///
/// # Arguments
///
/// - `Signal<String>` - The signal updated with the selected value.
/// - `&'static str` - The value belonging to the option that owns the handler.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - A change handler selecting the option.
pub fn on_radio_change(
    signal: Signal<String>,
    option_value: &'static str,
) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |event: Event| {
        let reported: Option<String> = event.target().and_then(|target: EventTarget| {
            if let Ok(input) = target.dyn_into::<HtmlInputElement>() {
                return Some(input.value());
            }
            None
        });
        match reported {
            Some(value) if !value.is_empty() => signal.set(value),
            _ => signal.set(String::from(option_value)),
        }
    }))
}

/// The container of a set of radio rows.
///
/// The container carries `role="radiogroup"`, which is what tells the
/// screen reader that the rows inside it are one choice among several
/// rather than a list of unrelated controls. The selection itself stays
/// with the caller, so the group can take part in two-way binding with
/// the rest of the page.
///
/// # Arguments
///
/// - `VirtualNode<EuvRadioGroupRootProps>` - The props node carrying the component settings.
///
/// # Returns
///
/// - `VirtualNode` - The euv radio group virtual DOM tree.
#[component]
pub fn euv_radio_group(node: VirtualNode<EuvRadioGroupRootProps>) -> VirtualNode {
    let EuvRadioGroupRootProps { label }: EuvRadioGroupRootProps =
        node.try_get_props().unwrap_or_default();
    let children: VirtualNode = node.get_children().into();
    html! {
        div {
            role: "radiogroup"
            "aria-label": label
            class: c_euv_radio_group_root()
            children
        }
    }
}

/// One row of a radio group.
///
/// The row is a button carrying `role="radio"` and `aria-checked`, the
/// pair the reference publishes for a radio row that is not a native
/// input. The checked state is read from the caller's signal during the
/// render, so the row follows the signal in both directions.
///
/// # Arguments
///
/// - `VirtualNode<EuvRadioGroupItemProps>` - The props node carrying the component settings.
///
/// # Returns
///
/// - `VirtualNode` - The euv radio group item virtual DOM tree.
#[component]
pub fn euv_radio_group_item(node: VirtualNode<EuvRadioGroupItemProps>) -> VirtualNode {
    let EuvRadioGroupItemProps {
        label,
        value,
        checked,
        on_select,
    }: EuvRadioGroupItemProps = node.try_get_props().unwrap_or_default();
    let is_checked: bool = checked.get();
    let item_class: fn() -> &'static Css = if is_checked {
        c_euv_radio_group_item_checked
    } else {
        c_euv_radio_group_item
    };
    let on_click: Option<Rc<dyn Fn(Event)>> = Some(Rc::new(move |_: Event| {
        if let Some(callback) = &on_select {
            callback(value);
        }
    }));
    html! {
        button {
            type: "button"
            role: "radio"
            "aria-checked": if is_checked { "true" } else { "false" }
            class: item_class()
            onclick: on_click
            {
                label
            }
        }
    }
}
