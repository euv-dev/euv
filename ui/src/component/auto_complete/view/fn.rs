use super::*;

/// The text shown in the panel when the typed text matches no suggestion.
const NO_MATCHES_TEXT: &str = "No matches";

/// Returns whether a suggestion matches the typed text.
///
/// The comparison is case-insensitive and looks at both the label and the
/// value of the suggestion, so `rust` matches the label "Rust" and the
/// value `rust-lang` alike. An empty query matches every suggestion.
///
/// # Arguments
///
/// - `&EuvAutoCompleteOption` - The suggestion under test.
/// - `&str` - The typed text.
///
/// # Returns
///
/// - `bool` - `true` when the suggestion belongs in the filtered list.
pub fn auto_complete_matches(option: &EuvAutoCompleteOption, query: &str) -> bool {
    matches_lowered(option, &query.to_lowercase())
}

/// Returns whether a suggestion matches an already lower-cased query.
///
/// The component lower-cases the typed text once per render and hands the
/// result to this helper, so the filter does not allocate a copy of the
/// query for every suggestion it walks.
///
/// # Arguments
///
/// - `&EuvAutoCompleteOption` - The suggestion under test.
/// - `&str` - The typed text, already lower-cased.
///
/// # Returns
///
/// - `bool` - `true` when the suggestion belongs in the filtered list.
fn matches_lowered(option: &EuvAutoCompleteOption, lowered_query: &str) -> bool {
    if lowered_query.is_empty() {
        return true;
    }
    option.label.to_lowercase().contains(lowered_query)
        || option.value.to_lowercase().contains(lowered_query)
}

/// Returns the suggestion that carries the active marker.
///
/// The active suggestion is the one whose label or value equals the typed
/// text exactly; when no suggestion matches exactly the first one in the
/// filtered list is active.
///
/// # Arguments
///
/// - `&'a [EuvAutoCompleteOption]` - The suggestions left after filtering.
/// - `&str` - The typed text.
///
/// # Returns
///
/// - `Option<&'a EuvAutoCompleteOption>` - The active suggestion, or `None`
///   when the filtered list is empty.
pub fn auto_complete_active<'a>(
    filtered: &'a [EuvAutoCompleteOption],
    query: &str,
) -> Option<&'a EuvAutoCompleteOption> {
    let exact: Option<&'a EuvAutoCompleteOption> = filtered
        .iter()
        .find(|option: &&EuvAutoCompleteOption| option.label == query || option.value == query);
    exact.or_else(|| filtered.first())
}

/// Builds the input handler of the auto-complete field.
///
/// The handler reads the value off the input element, publishes it to the
/// caller-owned signal and opens the suggestion panel as soon as the field
/// holds text and there is something to suggest.
///
/// # Arguments
///
/// - `Signal<String>` - The signal holding the text in the field.
/// - `Signal<bool>` - The panel's open state.
/// - `bool` - Whether the option list holds any suggestion at all.
/// - `Option<Rc<dyn Fn(&'static str)>>` - The callback receiving the typed text.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - The input handler of the field.
fn on_auto_complete_input(
    value: Signal<String>,
    open: Signal<bool>,
    has_options: bool,
    on_change: Option<Rc<dyn Fn(&'static str)>>,
) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |event: Event| {
        let reported: Option<String> = event.target().and_then(|target: EventTarget| {
            if let Ok(input) = target.dyn_into::<HtmlInputElement>() {
                return Some(input.value());
            }
            None
        });
        let Some(reported) = reported else {
            return;
        };
        value.set(reported.clone());
        open.set(has_options && !reported.is_empty());
        if let Some(callback) = &on_change {
            let leaked: &'static str = Box::leak(reported.into_boxed_str());
            callback(leaked);
        }
    }))
}

/// Builds the click handler of one suggestion.
///
/// The handler publishes the picked value, closes the panel and notifies the
/// caller that a suggestion was chosen.
///
/// # Arguments
///
/// - `Signal<String>` - The signal holding the text in the field.
/// - `Signal<bool>` - The panel's open state.
/// - `&'static str` - The value of the picked suggestion.
/// - `Option<Rc<dyn Fn(&'static str)>>` - The callback receiving the value.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - The click handler of the suggestion.
fn on_auto_complete_select(
    value: Signal<String>,
    open: Signal<bool>,
    option_value: &'static str,
    on_select: Option<Rc<dyn Fn(&'static str)>>,
) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |_event: Event| {
        value.set(String::from(option_value));
        open.set(false);
        if let Some(callback) = &on_select {
            callback(option_value);
        }
    }))
}

/// A text field with a filtered list of suggestions.
///
/// The component renders a `c_euv_auto_complete` wrapper holding a real
/// `<input type="text">` and the suggestion panel. The panel is always
/// mounted; opening and closing it is a reactive class swap between
/// `c_euv_auto_complete_panel_open` and `c_euv_auto_complete_panel_closed`,
/// so the browser never has to rebuild the suggestion list.
///
/// The suggestions are the options whose label or value contains the typed
/// text, compared case-insensitively. The panel stays closed while the field
/// is empty or while there is nothing to suggest at all; when the field
/// holds text that matches no suggestion the panel shows a single
/// `c_euv_auto_complete_empty` row instead of an empty dropdown.
///
/// The text in the field lives in a caller-owned [`Signal`], so the component
/// takes part in two-way binding. Picking a suggestion writes its value into
/// that signal, notifies `on_select` and closes the panel.
///
/// # Arguments
///
/// - `VirtualNode<EuvAutoCompleteProps>` - The props node carrying the
///   suggestions, the text and the callbacks.
///
/// # Returns
///
/// - `VirtualNode` - The text field with its suggestion panel.
#[component]
pub fn euv_auto_complete(node: VirtualNode<EuvAutoCompleteProps>) -> VirtualNode {
    let EuvAutoCompleteProps {
        options,
        value,
        placeholder,
        label: label_text,
        on_change,
        on_select,
    }: EuvAutoCompleteProps = node.try_get_props().unwrap_or_default();
    let open: Signal<bool> = HookContext::use_hook(|| Signal::create(false));
    let query: String = value.get();
    let lowered_query: String = query.to_lowercase();
    let has_options: bool = !options.is_empty();
    let filtered: Vec<EuvAutoCompleteOption> = options
        .iter()
        .filter(|option: &&EuvAutoCompleteOption| matches_lowered(option, &lowered_query))
        .copied()
        .collect();
    let active: Option<&EuvAutoCompleteOption> = auto_complete_active(&filtered, &query);
    let mut rows: Vec<VirtualNode> = filtered
        .iter()
        .map(|option: &EuvAutoCompleteOption| {
            let option_value: &'static str = option.value;
            let option_label: &'static str = option.label;
            let is_active: bool = active
                .map(|active_option: &EuvAutoCompleteOption| eq(active_option, option))
                .unwrap_or(false);
            let option_class: fn() -> &'static Css = if is_active {
                c_euv_auto_complete_option_active
            } else {
                c_euv_auto_complete_option
            };
            html! {
                button {
                    type: "button"
                    class: option_class()
                    onclick: on_auto_complete_select(value, open, option_value, on_select.clone())
                    option_label
                }
            }
        })
        .collect();
    if rows.is_empty() && has_options && !query.is_empty() {
        rows.push(html! {
            div {
                class: c_euv_auto_complete_empty()
                NO_MATCHES_TEXT
            }
        });
    }
    html! {
        div {
            class: c_euv_auto_complete()
            if !label_text.is_empty() {
                label {
                    class: c_form_label()
                    label_text
                }
            }
            input {
                type: "text"
                placeholder: placeholder
                value: value
                class: c_euv_auto_complete_input()
                oninput: on_auto_complete_input(value, open, has_options, on_change.clone())
            }
            div {
                class: c_euv_auto_complete_panel()
                class: if { open } {
                    c_euv_auto_complete_panel_open()
                } else {
                    c_euv_auto_complete_panel_closed()
                }
                rows
            }
        }
    }
}
