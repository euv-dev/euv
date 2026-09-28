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
                    // `& true` keeps the condition a non-path expression so the
                    // `html!` auto-unwrap heuristic skips it. A bare `is_checked`
                    // would be rewritten to `is_checked.get()` and fail to
                    // compile, because `is_checked` is a plain `bool`.
                    class: if { is_checked & true } {
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
