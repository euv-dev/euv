use super::*;

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

/// Returns the active suggestion index for the current query.
///
/// # Arguments
///
/// - `&[EuvAutoCompleteOption]` - The suggestions left after filtering.
/// - `&str` - The typed text.
///
/// # Returns
///
/// - `Option<usize>` - The active option index, or `None` when no options remain.
fn auto_complete_active_index(filtered: &[EuvAutoCompleteOption], query: &str) -> Option<usize> {
    let active: Option<&EuvAutoCompleteOption> = auto_complete_active(filtered, query);
    active.and_then(|active_option: &EuvAutoCompleteOption| {
        filtered.iter().position(|option: &EuvAutoCompleteOption| {
            option.value == active_option.value && option.label == active_option.label
        })
    })
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
/// - `Signal<Option<usize>>` - The active suggestion index.
/// - `bool` - Whether the option list holds any suggestion at all.
/// - `bool` - Whether the input is editable.
/// - `Option<Rc<dyn Fn(&'static str)>>` - The callback receiving the typed text.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - The input handler of the field.
fn on_auto_complete_input(
    value: Signal<String>,
    open: Signal<bool>,
    active_index: Signal<Option<usize>>,
    has_options: bool,
    can_edit: bool,
    on_change: Option<Rc<dyn Fn(&'static str)>>,
) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |event: Event| {
        if !can_edit {
            return;
        }
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
        active_index.set(None);
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
/// - `Signal<Option<usize>>` - The active suggestion index.
/// - `&'static str` - The value of the picked suggestion.
/// - `Option<Rc<dyn Fn(&'static str)>>` - The callback receiving the value.
/// - `bool` - Whether the component is disabled.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - The click handler of the suggestion.
fn on_auto_complete_select(
    value: Signal<String>,
    open: Signal<bool>,
    active_index: Signal<Option<usize>>,
    option_value: &'static str,
    on_select: Option<Rc<dyn Fn(&'static str)>>,
    disabled: bool,
) -> Option<Rc<dyn Fn(Event)>> {
    if disabled {
        return None;
    }
    Some(Rc::new(move |_event: Event| {
        value.set(String::from(option_value));
        open.set(false);
        active_index.set(None);
        if let Some(callback) = &on_select {
            callback(option_value);
        }
    }))
}

/// Builds the keyboard handler of the editable combobox input.
///
/// # Arguments
///
/// - `Signal<String>` - The signal holding the text in the field.
/// - `Signal<bool>` - The panel's open state.
/// - `Signal<Option<usize>>` - The active suggestion index.
/// - `Vec<EuvAutoCompleteOption>` - The suggestions currently in the popup.
/// - `Option<Rc<dyn Fn(&'static str)>>` - The callback receiving a picked value.
/// - `bool` - Whether the component is disabled.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - The keyboard handler of the field.
fn on_auto_complete_keydown(
    value: Signal<String>,
    open: Signal<bool>,
    active_index: Signal<Option<usize>>,
    filtered: Vec<EuvAutoCompleteOption>,
    on_select: Option<Rc<dyn Fn(&'static str)>>,
    disabled: bool,
) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |event: Event| {
        if disabled {
            return;
        }
        let Ok(keyboard_event) = event.dyn_into::<KeyboardEvent>() else {
            return;
        };
        if keyboard_event.ctrl_key()
            || keyboard_event.shift_key()
            || keyboard_event.alt_key()
            || keyboard_event.meta_key()
        {
            return;
        }
        let key: String = keyboard_event.key();
        let is_open: bool = open.get();
        let query: String = value.get();
        let preferred_index: Option<usize> = auto_complete_active_index(&filtered, &query);
        match key.as_str() {
            AUTO_COMPLETE_KEY_ARROW_DOWN => {
                if filtered.is_empty() {
                    return;
                }
                keyboard_event.prevent_default();
                let next_index: Option<usize> = if is_open {
                    match active_index
                        .get()
                        .filter(|index: &usize| *index < filtered.len())
                    {
                        Some(index) if index + 1 < filtered.len() => Some(index + 1),
                        Some(_) => None,
                        None => Some(0),
                    }
                } else {
                    preferred_index.or(Some(0))
                };
                active_index.set(next_index);
                open.set(true);
            }
            AUTO_COMPLETE_KEY_ARROW_UP => {
                if filtered.is_empty() {
                    return;
                }
                keyboard_event.prevent_default();
                let next_index: Option<usize> = if is_open {
                    match active_index
                        .get()
                        .filter(|index: &usize| *index < filtered.len())
                    {
                        Some(index) if index > 0 => Some(index - 1),
                        Some(_) => None,
                        None => Some(filtered.len() - 1),
                    }
                } else {
                    Some(filtered.len() - 1)
                };
                active_index.set(next_index);
                open.set(true);
            }
            AUTO_COMPLETE_KEY_ENTER => {
                if !is_open || keyboard_event.is_composing() || keyboard_event.key_code() == 229 {
                    return;
                }
                let selected_option: Option<&EuvAutoCompleteOption> = active_index
                    .get()
                    .filter(|index: &usize| *index < filtered.len())
                    .and_then(|index: usize| filtered.get(index));
                let Some(option) = selected_option else {
                    open.set(false);
                    active_index.set(None);
                    return;
                };
                keyboard_event.prevent_default();
                value.set(String::from(option.value));
                open.set(false);
                active_index.set(None);
                if let Some(callback) = &on_select {
                    callback(option.value);
                }
            }
            AUTO_COMPLETE_KEY_ESCAPE => {
                if is_open {
                    keyboard_event.prevent_default();
                    open.set(false);
                    active_index.set(None);
                }
            }
            AUTO_COMPLETE_KEY_TAB if is_open => {
                open.set(false);
                active_index.set(None);
            }
            _ => {}
        }
    }))
}

/// Prevents a suggestion click from moving focus away from the input.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - The mousedown handler of a suggestion.
fn on_auto_complete_option_mousedown() -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(|event: Event| event.prevent_default()))
}

/// Builds the pointer-move handler that highlights one suggestion.
///
/// # Arguments
///
/// - `Signal<Option<usize>>` - The active suggestion index.
/// - `usize` - The suggestion index under the pointer.
/// - `bool` - Whether the component is disabled.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - The pointer-move handler of the suggestion.
fn on_auto_complete_option_pointer_move(
    active_index: Signal<Option<usize>>,
    option_index: usize,
    disabled: bool,
) -> Option<Rc<dyn Fn(Event)>> {
    if disabled {
        return None;
    }
    Some(Rc::new(move |_event: Event| {
        active_index.set(Some(option_index))
    }))
}

/// Converts a state flag to the DOM renderer's native boolean value.
///
/// # Arguments
///
/// - `bool` - The state flag to render.
///
/// # Returns
///
/// - `&'static str` - `"true"` when enabled, or an empty value when disabled.
fn auto_complete_boolean_attr(flag: bool) -> &'static str {
    if flag {
        AUTO_COMPLETE_ARIA_TRUE
    } else {
        AUTO_COMPLETE_ATTRIBUTE_EMPTY
    }
}

/// Renders the native input, omitting an empty `form` attribute so ancestor
/// form ownership remains intact.
///
/// # Arguments
///
/// - `EuvAutoCompleteInputProps` - The input values, states and handlers.
///
/// # Returns
///
/// - `VirtualNode` - The labeled combobox input.
fn auto_complete_input(props: EuvAutoCompleteInputProps) -> VirtualNode {
    let EuvAutoCompleteInputProps {
        id,
        name,
        form,
        autocomplete,
        aria_label,
        aria_describedby,
        invalid,
        required,
        disabled,
        read_only,
        panel_open,
        listbox_id,
        active_descendant_id,
        placeholder,
        value,
        input_class,
        default_class,
        invalid_class,
        disabled_class,
        on_input,
        on_keydown,
    }: EuvAutoCompleteInputProps = props;
    let required_attribute: &'static str = auto_complete_boolean_attr(required);
    let disabled_attribute: &'static str = auto_complete_boolean_attr(disabled);
    let read_only_attribute: &'static str = auto_complete_boolean_attr(read_only);
    let has_active_descendant: bool = !active_descendant_id.is_empty();
    if form.is_empty() {
        if has_active_descendant {
            html! {
                input {
                    "data-slot": "auto-complete-input"
                    id: id
                    type: "text"
                    name: name
                    autocomplete: autocomplete
                    role: AUTO_COMPLETE_ROLE_COMBOBOX
                    "aria-autocomplete": AUTO_COMPLETE_ARIA_AUTOCOMPLETE_LIST
                    "aria-haspopup": AUTO_COMPLETE_ROLE_LISTBOX
                    "aria-controls": listbox_id
                    "aria-label": aria_label
                    "aria-describedby": aria_describedby
                    "aria-invalid": if invalid {
                        AUTO_COMPLETE_ARIA_TRUE
                    } else {
                        AUTO_COMPLETE_ARIA_FALSE
                    }
                    "aria-required": if required {
                        AUTO_COMPLETE_ARIA_TRUE
                    } else {
                        AUTO_COMPLETE_ARIA_FALSE
                    }
                    "aria-disabled": if disabled {
                        AUTO_COMPLETE_ARIA_TRUE
                    } else {
                        AUTO_COMPLETE_ARIA_FALSE
                    }
                    "aria-readonly": if read_only {
                        AUTO_COMPLETE_ARIA_TRUE
                    } else {
                        AUTO_COMPLETE_ARIA_FALSE
                    }
                    "aria-expanded": if panel_open {
                        AUTO_COMPLETE_ARIA_TRUE
                    } else {
                        AUTO_COMPLETE_ARIA_FALSE
                    }
                    "aria-activedescendant": active_descendant_id
                    placeholder: placeholder
                    value: value
                    required: required_attribute
                    disabled: disabled_attribute
                    readonly: read_only_attribute
                    class: input_class
                    class: default_class
                    class: invalid_class
                    class: disabled_class
                    oninput: on_input
                    onkeydown: on_keydown
                }
            }
        } else {
            html! {
                input {
                    "data-slot": "auto-complete-input"
                    id: id
                    type: "text"
                    name: name
                    autocomplete: autocomplete
                    role: AUTO_COMPLETE_ROLE_COMBOBOX
                    "aria-autocomplete": AUTO_COMPLETE_ARIA_AUTOCOMPLETE_LIST
                    "aria-haspopup": AUTO_COMPLETE_ROLE_LISTBOX
                    "aria-controls": listbox_id
                    "aria-label": aria_label
                    "aria-describedby": aria_describedby
                    "aria-invalid": if invalid {
                        AUTO_COMPLETE_ARIA_TRUE
                    } else {
                        AUTO_COMPLETE_ARIA_FALSE
                    }
                    "aria-required": if required {
                        AUTO_COMPLETE_ARIA_TRUE
                    } else {
                        AUTO_COMPLETE_ARIA_FALSE
                    }
                    "aria-disabled": if disabled {
                        AUTO_COMPLETE_ARIA_TRUE
                    } else {
                        AUTO_COMPLETE_ARIA_FALSE
                    }
                    "aria-readonly": if read_only {
                        AUTO_COMPLETE_ARIA_TRUE
                    } else {
                        AUTO_COMPLETE_ARIA_FALSE
                    }
                    "aria-expanded": if panel_open {
                        AUTO_COMPLETE_ARIA_TRUE
                    } else {
                        AUTO_COMPLETE_ARIA_FALSE
                    }
                    placeholder: placeholder
                    value: value
                    required: required_attribute
                    disabled: disabled_attribute
                    readonly: read_only_attribute
                    class: input_class
                    class: default_class
                    class: invalid_class
                    class: disabled_class
                    oninput: on_input
                    onkeydown: on_keydown
                }
            }
        }
    } else {
        if has_active_descendant {
            html! {
                input {
                    "data-slot": "auto-complete-input"
                    id: id
                    type: "text"
                    name: name
                    form: form
                    autocomplete: autocomplete
                    role: AUTO_COMPLETE_ROLE_COMBOBOX
                    "aria-autocomplete": AUTO_COMPLETE_ARIA_AUTOCOMPLETE_LIST
                    "aria-haspopup": AUTO_COMPLETE_ROLE_LISTBOX
                    "aria-controls": listbox_id
                    "aria-label": aria_label
                    "aria-describedby": aria_describedby
                    "aria-invalid": if invalid {
                        AUTO_COMPLETE_ARIA_TRUE
                    } else {
                        AUTO_COMPLETE_ARIA_FALSE
                    }
                    "aria-required": if required {
                        AUTO_COMPLETE_ARIA_TRUE
                    } else {
                        AUTO_COMPLETE_ARIA_FALSE
                    }
                    "aria-disabled": if disabled {
                        AUTO_COMPLETE_ARIA_TRUE
                    } else {
                        AUTO_COMPLETE_ARIA_FALSE
                    }
                    "aria-readonly": if read_only {
                        AUTO_COMPLETE_ARIA_TRUE
                    } else {
                        AUTO_COMPLETE_ARIA_FALSE
                    }
                    "aria-expanded": if panel_open {
                        AUTO_COMPLETE_ARIA_TRUE
                    } else {
                        AUTO_COMPLETE_ARIA_FALSE
                    }
                    "aria-activedescendant": active_descendant_id
                    placeholder: placeholder
                    value: value
                    required: required_attribute
                    disabled: disabled_attribute
                    readonly: read_only_attribute
                    class: input_class
                    class: default_class
                    class: invalid_class
                    class: disabled_class
                    oninput: on_input
                    onkeydown: on_keydown
                }
            }
        } else {
            html! {
                input {
                    "data-slot": "auto-complete-input"
                    id: id
                    type: "text"
                    name: name
                    form: form
                    autocomplete: autocomplete
                    role: AUTO_COMPLETE_ROLE_COMBOBOX
                    "aria-autocomplete": AUTO_COMPLETE_ARIA_AUTOCOMPLETE_LIST
                    "aria-haspopup": AUTO_COMPLETE_ROLE_LISTBOX
                    "aria-controls": listbox_id
                    "aria-label": aria_label
                    "aria-describedby": aria_describedby
                    "aria-invalid": if invalid {
                        AUTO_COMPLETE_ARIA_TRUE
                    } else {
                        AUTO_COMPLETE_ARIA_FALSE
                    }
                    "aria-required": if required {
                        AUTO_COMPLETE_ARIA_TRUE
                    } else {
                        AUTO_COMPLETE_ARIA_FALSE
                    }
                    "aria-disabled": if disabled {
                        AUTO_COMPLETE_ARIA_TRUE
                    } else {
                        AUTO_COMPLETE_ARIA_FALSE
                    }
                    "aria-readonly": if read_only {
                        AUTO_COMPLETE_ARIA_TRUE
                    } else {
                        AUTO_COMPLETE_ARIA_FALSE
                    }
                    "aria-expanded": if panel_open {
                        AUTO_COMPLETE_ARIA_TRUE
                    } else {
                        AUTO_COMPLETE_ARIA_FALSE
                    }
                    placeholder: placeholder
                    value: value
                    required: required_attribute
                    disabled: disabled_attribute
                    readonly: read_only_attribute
                    class: input_class
                    class: default_class
                    class: invalid_class
                    class: disabled_class
                    oninput: on_input
                    onkeydown: on_keydown
                }
            }
        }
    }
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
        id,
        name,
        form,
        autocomplete,
        required,
        invalid,
        disabled,
        read_only,
        aria_label,
        aria_describedby,
        on_change,
        on_select,
    }: EuvAutoCompleteProps = node.try_get_props().unwrap_or_default();
    let open: Signal<bool> = App::use_signal(|| false);
    let active_index_signal: Signal<Option<usize>> = App::use_signal(|| None);
    let input_id: UseId = use_id("auto-complete");
    let input_id_value: String = if id.is_empty() {
        input_id.get_value().get()
    } else {
        String::from(id)
    };
    let input_label_for: String = input_id_value.clone();
    let effective_aria_label: &'static str = if aria_label.is_empty() {
        label_text
    } else {
        aria_label
    };
    let effective_autocomplete: &'static str = if autocomplete.is_empty() {
        AUTO_COMPLETE_AUTOCOMPLETE_DEFAULT
    } else {
        autocomplete
    };
    let listbox_id: String = format!("{input_id_value}{AUTO_COMPLETE_LISTBOX_ID_SUFFIX}");
    let initial_value: String = value.get();
    let reset_value_signal: Signal<String> = value;
    let reset_open_signal: Signal<bool> = open;
    let reset_active_index_signal: Signal<Option<usize>> = active_index_signal;
    let reset_input_id: String = input_id_value.clone();
    let reset_callback: Rc<dyn Fn()> = Rc::new(move || {
        let reset_value: String = initial_value.clone();
        reset_value_signal.set(reset_value.clone());
        reset_open_signal.set(false);
        reset_active_index_signal.set(None);
        let input: Option<HtmlInputElement> = window()
            .and_then(|window: Window| window.document())
            .and_then(|document: Document| document.get_element_by_id(&reset_input_id))
            .and_then(|element: Element| element.dyn_into::<HtmlInputElement>().ok());
        if let Some(input) = input {
            input.set_value(&reset_value);
        }
    });
    use_form_reset(input_id_value.clone(), form, reset_callback);
    let query: String = value.get();
    let lowered_query: String = query.to_lowercase();
    let has_options: bool = !options.is_empty();
    let filtered: Vec<EuvAutoCompleteOption> = options
        .iter()
        .filter(|option: &&EuvAutoCompleteOption| matches_lowered(option, &lowered_query))
        .copied()
        .collect();
    let panel_open: bool = open.get() && !disabled;
    let active_index: Option<usize> = if panel_open && !filtered.is_empty() {
        active_index_signal
            .get()
            .filter(|index: &usize| *index < filtered.len())
    } else {
        None
    };
    let active_descendant_id: String = active_index
        .map(|index: usize| format!("{input_id_value}{AUTO_COMPLETE_OPTION_ID_SUFFIX}{index}"))
        .unwrap_or_default();
    let invalid_input_class: Css = if invalid {
        c_euv_auto_complete_input_invalid().clone()
    } else {
        Css::default()
    };
    let disabled_input_class: Css = if disabled {
        c_euv_auto_complete_input_disabled().clone()
    } else {
        Css::default()
    };
    let default_input_class: Css = c_euv_auto_complete_input_default().clone();
    let rows: Vec<VirtualNode> = filtered
        .iter()
        .enumerate()
        .map(|(index, option): (usize, &EuvAutoCompleteOption)| {
            let option_value: &'static str = option.value;
            let option_label: &'static str = option.label;
            let is_active: bool = active_index == Some(index);
            let is_selected: bool = option_value == query;
            let option_id: String =
                format!("{input_id_value}{AUTO_COMPLETE_OPTION_ID_SUFFIX}{index}");
            let option_class: fn() -> &'static Css = if is_active {
                c_euv_auto_complete_option_active
            } else {
                c_euv_auto_complete_option
            };
            html! {
                div {
                    id: option_id
                    role: AUTO_COMPLETE_ROLE_OPTION
                    "aria-disabled": if disabled {
                        AUTO_COMPLETE_ARIA_TRUE
                    } else {
                        AUTO_COMPLETE_ARIA_FALSE
                    }
                    "aria-selected": if is_selected {
                        AUTO_COMPLETE_ARIA_TRUE
                    } else {
                        AUTO_COMPLETE_ARIA_FALSE
                    }
                    class: option_class()
                    onpointermove: on_auto_complete_option_pointer_move(
                        active_index_signal,
                        index,
                        disabled
                    )
                    onpointerdown: on_auto_complete_option_mousedown()
                    onmousedown: on_auto_complete_option_mousedown()
                    onclick: on_auto_complete_select(
                    value,
                    open,
                    active_index_signal,
                    option_value,
                    on_select.clone(),
                    disabled
                    )
                    option_label
                }
            }
        })
        .collect();
    let has_no_matches: bool = rows.is_empty() && has_options && !query.is_empty();
    let input_node: VirtualNode = auto_complete_input(EuvAutoCompleteInputProps {
        id: input_id_value.clone(),
        name,
        form,
        autocomplete: effective_autocomplete,
        aria_label: effective_aria_label,
        aria_describedby,
        invalid,
        required,
        disabled,
        read_only,
        panel_open,
        listbox_id: listbox_id.clone(),
        active_descendant_id,
        placeholder,
        value,
        input_class: c_euv_auto_complete_input().clone(),
        default_class: default_input_class,
        invalid_class: invalid_input_class,
        disabled_class: disabled_input_class,
        on_input: on_auto_complete_input(
            value,
            open,
            active_index_signal,
            has_options,
            !disabled && !read_only,
            on_change.clone(),
        ),
        on_keydown: on_auto_complete_keydown(
            value,
            open,
            active_index_signal,
            filtered.clone(),
            on_select.clone(),
            disabled,
        ),
    });
    html! {
        div {
            "data-slot": "auto-complete"
            "data-disabled": if disabled {
                AUTO_COMPLETE_ARIA_TRUE
            } else {
                AUTO_COMPLETE_ARIA_FALSE
            }
            "data-invalid": if invalid {
                AUTO_COMPLETE_ARIA_TRUE
            } else {
                AUTO_COMPLETE_ARIA_FALSE
            }
            "data-readonly": if read_only {
                AUTO_COMPLETE_ARIA_TRUE
            } else {
                AUTO_COMPLETE_ARIA_FALSE
            }
            "data-required": if required {
                AUTO_COMPLETE_ARIA_TRUE
            } else {
                AUTO_COMPLETE_ARIA_FALSE
            }
            class: c_euv_auto_complete()
            if !label_text.is_empty() {
                label {
                    r#for: input_label_for.clone()
                    class: c_form_label()
                    label_text
                }
            }
            input_node
            div {
                "aria-hidden": if panel_open {
                    AUTO_COMPLETE_ARIA_FALSE
                } else {
                    AUTO_COMPLETE_ARIA_TRUE
                }
                class: c_euv_auto_complete_panel()
                class: if panel_open {
                    c_euv_auto_complete_panel_open()
                } else {
                    c_euv_auto_complete_panel_closed()
                }
                div {
                    id: listbox_id
                    role: AUTO_COMPLETE_ROLE_LISTBOX
                    rows
                }
                div {
                    role: AUTO_COMPLETE_ROLE_STATUS
                    "aria-live": AUTO_COMPLETE_ARIA_LIVE_POLITE
                    "aria-atomic": AUTO_COMPLETE_ARIA_TRUE
                    class: if has_no_matches {
                        c_euv_auto_complete_empty().clone()
                    } else {
                        Css::default()
                    }
                    if has_no_matches {
                        NO_MATCHES_TEXT
                    }
                }
            }
        }
    }
}
