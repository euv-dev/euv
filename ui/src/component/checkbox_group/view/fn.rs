use super::*;

/// A group of checkboxes bound to a set of checked values.
///
/// Renders a `c_euv_checkbox_group` container holding the optional
/// `c_euv_checkbox_group_label` and one `c_euv_checkbox_group_item` row
/// per option. Every row carries a real `<input type="checkbox">` — hidden
/// by `c_euv_checkbox_group_input` but still focusable and form
/// submittable — so keyboard and screen-reader behaviour is the browser's
/// own, plus the `c_euv_checkbox_group_box` marker that draws the square
/// and the `c_euv_checkbox_group_text` label.
///
/// The marker is `c_euv_checkbox_group_box_checked` while the option's
/// value is in the checked-values signal and
/// `c_euv_checkbox_group_box_unchecked` otherwise; the ticked marker
/// carries a "✓" glyph, which the checked class paints in the
/// `text-on-accent` colour.
///
/// # Arguments
///
/// - `VirtualNode<EuvCheckboxGroupProps>` - The props node holding the options, the checked-values signal and the change callback.
///
/// # Returns
///
/// - `VirtualNode` - The checkbox group virtual DOM tree.
#[component]
pub fn euv_checkbox_group(node: VirtualNode<EuvCheckboxGroupProps>) -> VirtualNode {
    let EuvCheckboxGroupProps {
        options,
        values,
        label: label_text,
        on_change,
    }: EuvCheckboxGroupProps = node.try_get_props().unwrap_or_default();
    let checked_values: Vec<String> = values.get();
    let items: Vec<VirtualNode> = options
        .into_iter()
        .map(|option: EuvCheckboxGroupOption| {
            let is_checked: bool = checked_values
                .iter()
                .any(|value: &String| value.as_str() == option.value);
            checkbox_item(option, is_checked, values, on_change.clone())
        })
        .collect();
    html! {
        div {
            class: c_euv_checkbox_group()
            if !label_text.is_empty() {
                div {
                    class: c_euv_checkbox_group_label()
                    label_text
                }
            }
            items
        }
    }
}

/// Renders one option row of the group.
///
/// The row is a `<label>`, so clicking the marker or the text toggles the
/// real checkbox inside it and the browser reports the new state through
/// the change handler.
///
/// # Arguments
///
/// - `EuvCheckboxGroupOption` - The option this row renders.
/// - `bool` - Whether the option's value is currently checked.
/// - `Signal<Vec<String>>` - The checked-values signal owned by the caller.
/// - `Option<Rc<dyn Fn(Vec<String>)>>` - The change callback.
///
/// # Returns
///
/// - `VirtualNode` - The option row virtual DOM tree.
fn checkbox_item(
    option: EuvCheckboxGroupOption,
    is_checked: bool,
    values: Signal<Vec<String>>,
    on_change: Option<Rc<dyn Fn(Vec<String>)>>,
) -> VirtualNode {
    let disabled: bool = option.disabled;
    let handler: Option<Rc<dyn Fn(Event)>> = if disabled {
        None
    } else {
        checkbox_change(values, option.value, on_change)
    };
    html! {
        label {
            class: c_euv_checkbox_group_item()
            input {
                type: "checkbox"
                value: option.value
                checked: boolean_attr(is_checked)
                disabled: boolean_attr(disabled)
                class: c_euv_checkbox_group_input()
                onchange: handler
            }
            span {
                class: c_euv_checkbox_group_box()
                class: if is_checked {
                    c_euv_checkbox_group_box_checked()
                } else {
                    c_euv_checkbox_group_box_unchecked()
                }
                if is_checked {
                    "✓"
                }
            }
            span {
                class: c_euv_checkbox_group_text()
                option.label
            }
        }
    }
}

/// Builds the change handler that toggles one option in the checked set.
///
/// The handler reads the state of the target input rather than trusting
/// the closure capture, so the browser's own toggle behaviour stays the
/// source of truth for what the new checked set should be.
///
/// # Arguments
///
/// - `Signal<Vec<String>>` - The checked-values signal owned by the caller.
/// - `&'static str` - The value of the option that owns the handler.
/// - `Option<Rc<dyn Fn(Vec<String>)>>` - The change callback.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - The change handler.
fn checkbox_change(
    values: Signal<Vec<String>>,
    option_value: &'static str,
    on_change: Option<Rc<dyn Fn(Vec<String>)>>,
) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |event: Event| {
        let is_checked: bool = event
            .target()
            .and_then(|target: EventTarget| target.dyn_into::<HtmlInputElement>().ok())
            .map(|input: HtmlInputElement| input.checked())
            .unwrap_or(true);
        let mut next: Vec<String> = values.get();
        if is_checked {
            if !next
                .iter()
                .any(|value: &String| value.as_str() == option_value)
            {
                next.push(String::from(option_value));
            }
        } else {
            next.retain(|value: &String| value.as_str() != option_value);
        }
        values.set(next.clone());
        if let Some(callback) = &on_change {
            callback(next);
        }
    }))
}

/// Maps a boolean flag onto the attribute value the renderer expects.
///
/// The renderer writes the boolean DOM attributes (`disabled`, `checked`)
/// through `set_attribute_or_property`, which compares the value against
/// `"true"`, so the flag has to travel as that literal rather than as a
/// Rust `bool` — a plain `bool` attribute value never reaches the DOM.
///
/// # Arguments
///
/// - `bool` - The flag to render.
///
/// # Returns
///
/// - `&'static str` - The attribute value for the flag.
fn boolean_attr(flag: bool) -> &'static str {
    if flag { "true" } else { "" }
}
