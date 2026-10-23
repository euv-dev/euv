use super::*;

/// The character that opens a mention.
const MENTION_TRIGGER: char = '@';

/// A text field that suggests names.
///
/// Renders a `c_euv_mentions` wrapper holding the optional group label, a
/// real `<input type="text">` carrying the `c_euv_mentions_input` class and
/// the `c_euv_mentions_panel` suggestion panel.
///
/// The panel is always mounted: opening and closing it swaps the
/// `c_euv_mentions_panel_open` and `c_euv_mentions_panel_closed` classes
/// rather than removing the suggestions from the tree, so the suggestion
/// rows survive every re-render.
///
/// The panel is open while the text ends in a mention trigger — that is,
/// while the text behind the last `@` is a non-empty fragment without a
/// space. That fragment is the query: it keeps the mentions whose label or
/// value contains it, compared case-insensitively. The first match carries
/// `c_euv_mentions_option_active`, the rest carry `c_euv_mentions_option`.
///
/// Picking a suggestion writes the mention's label over the trigger
/// fragment, reports the mention's value to `on_select` and closes the
/// panel.
///
/// The text in the field lives in a caller-owned [`Signal`], so the
/// component takes part in two-way binding.
///
/// # Arguments
///
/// - `VirtualNode<EuvMentionsProps>` - The props node holding the mentions, the text and the callback.
///
/// # Returns
///
/// - `VirtualNode` - The mentions field with its suggestion panel.
#[component]
pub fn euv_mentions(node: VirtualNode<EuvMentionsProps>) -> VirtualNode {
    let EuvMentionsProps {
        items,
        value,
        placeholder,
        label: label_text,
        on_select,
    }: EuvMentionsProps = node.try_get_props().unwrap_or_default();
    let text: String = value.get();
    let query: Option<&str> = mentions_trigger(&text);
    let has_trigger: bool = query.is_some();
    let open: Signal<bool> = HookContext::use_hook(|| Signal::create(has_trigger));
    let filtered: Vec<EuvMentionItem> = match query {
        Some(fragment) => mentions_filter(&items, fragment),
        None => Vec::new(),
    };
    let rows: Vec<VirtualNode> = filtered
        .iter()
        .enumerate()
        .map(|entry: (usize, &EuvMentionItem)| {
            let index: usize = entry.0;
            let item: &EuvMentionItem = entry.1;
            let is_active: bool = index == 0;
            let item_value: &'static str = item.value;
            let item_label: &'static str = item.label;
            html! {
                button {
                    type: "button"
                    class: if is_active {
                        c_euv_mentions_option_active()
                    } else {
                        c_euv_mentions_option()
                    }
                    onclick: on_mentions_select(value, open, item_value, item_label, on_select.clone())
                    item_label
                }
            }
        })
        .collect();
    html! {
        div {
            class: c_euv_mentions()
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
                class: c_euv_mentions_input()
                oninput: on_mentions_input(value, open)
            }
            div {
                class: c_euv_mentions_panel()
                class: if { open } {
                    c_euv_mentions_panel_open()
                } else {
                    c_euv_mentions_panel_closed()
                }
                rows
            }
        }
    }
}

/// Returns the fragment the mention trigger opens.
///
/// The trigger is the text behind the last `@`: the panel is open while
/// that fragment is not empty and holds no space, so a mention is only
/// suggested while its name is being typed.
///
/// # Arguments
///
/// - `&str` - The text of the field.
///
/// # Returns
///
/// - `Option<&str>` - The fragment behind the last `@`, or `None` when the
///   text does not end in a mention trigger.
pub fn mentions_trigger(text: &str) -> Option<&str> {
    let at: usize = text.rfind(MENTION_TRIGGER)?;
    let fragment: &str = &text[at + MENTION_TRIGGER.len_utf8()..];
    if fragment.is_empty() || fragment.chars().any(char::is_whitespace) {
        return None;
    }
    Some(fragment)
}

/// Returns whether a mention matches the typed fragment.
///
/// The comparison is case-insensitive and looks at both the label and the
/// value of the mention, so `ali` matches the label "Alice" and the value
/// `alice` alike.
///
/// # Arguments
///
/// - `&EuvMentionItem` - The mention under test.
/// - `&str` - The typed fragment.
///
/// # Returns
///
/// - `bool` - `true` when the mention belongs in the suggestion list.
pub fn mentions_matches(item: &EuvMentionItem, query: &str) -> bool {
    mentions_matches_lowered(item, &query.to_lowercase())
}

/// Returns whether a mention matches an already lower-cased fragment.
///
/// The component lower-cases the fragment once per render and hands the
/// result to this helper, so the filter does not allocate a copy of the
/// fragment for every mention it walks.
///
/// # Arguments
///
/// - `&EuvMentionItem` - The mention under test.
/// - `&str` - The typed fragment, already lower-cased.
///
/// # Returns
///
/// - `bool` - `true` when the mention belongs in the suggestion list.
fn mentions_matches_lowered(item: &EuvMentionItem, lowered_query: &str) -> bool {
    if lowered_query.is_empty() {
        return true;
    }
    item.label.to_lowercase().contains(lowered_query)
        || item.value.to_lowercase().contains(lowered_query)
}

/// Returns the mentions that match the typed fragment.
///
/// The mentions keep the order they were given in, so the first match is
/// the one the field suggests first.
///
/// # Arguments
///
/// - `&[EuvMentionItem]` - The mentions the field can suggest.
/// - `&str` - The typed fragment.
///
/// # Returns
///
/// - `Vec<EuvMentionItem>` - The matching mentions, in declaration order.
pub fn mentions_filter(items: &[EuvMentionItem], query: &str) -> Vec<EuvMentionItem> {
    let lowered: String = query.to_lowercase();
    items
        .iter()
        .filter(|item: &&EuvMentionItem| mentions_matches_lowered(item, &lowered))
        .copied()
        .collect()
}

/// Writes a mention over the trigger fragment of the text.
///
/// The fragment behind the last `@` is replaced, so the mention keeps the
/// `@` it was typed with. Text without a trigger is returned unchanged.
///
/// # Arguments
///
/// - `&str` - The text of the field.
///
/// # Returns
///
/// - `String` - The text with the mention written into it.
pub fn mentions_apply(text: &str, label: &str) -> String {
    let Some(at) = text.rfind(MENTION_TRIGGER) else {
        return String::from(text);
    };
    let mut applied: String = String::with_capacity(text.len() + label.len());
    applied.push_str(&text[..at + MENTION_TRIGGER.len_utf8()]);
    applied.push_str(label);
    applied
}

/// Builds the input handler of the mentions field.
///
/// The handler reads the value off the input element, publishes it to the
/// caller-owned signal and opens the suggestion panel while the text ends
/// in a mention trigger.
///
/// # Arguments
///
/// - `Signal<String>` - The signal holding the text in the field.
/// - `Signal<bool>` - The panel's open state.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - The input handler of the field.
fn on_mentions_input(value: Signal<String>, open: Signal<bool>) -> Option<Rc<dyn Fn(Event)>> {
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
        open.set(mentions_trigger(&reported).is_some());
        value.set(reported);
    }))
}

/// Builds the click handler of one suggestion.
///
/// The handler writes the mention's label over the trigger fragment,
/// reports the mention's value to the callback and closes the panel.
///
/// The event type is a parameter so the DOM-free test suite can drive the
/// handler with a stand-in event; the component always instantiates it with
/// [`Event`].
///
/// # Arguments
///
/// - `Signal<String>` - The signal holding the text in the field.
/// - `Signal<bool>` - The panel's open state.
/// - `&'static str` - The value of the picked mention.
/// - `Option<Rc<dyn Fn(&'static str)>>` - The callback receiving the value.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(E)>>` - The click handler of the suggestion.
pub fn on_mentions_select<E: 'static>(
    value: Signal<String>,
    open: Signal<bool>,
    item_value: &'static str,
    item_label: &'static str,
    on_select: Option<Rc<dyn Fn(&'static str)>>,
) -> Option<Rc<dyn Fn(E)>> {
    Some(Rc::new(move |_: E| {
        value.set(mentions_apply(&value.get(), item_label));
        if let Some(callback) = &on_select {
            callback(item_value);
        }
        open.set(false);
    }))
}
