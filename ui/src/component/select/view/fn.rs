use super::*;

/// A single-choice select control with an always-mounted option panel.
///
/// Renders a `c_euv_select` wrapper holding the optional group label, a
/// `c_euv_select_trigger` button that shows the selected option's label
/// (or the placeholder while no option matches) next to a trailing
/// `c_euv_select_caret` marker, and the `c_euv_select_panel` option
/// panel.
///
/// The panel is always mounted: opening and closing it swaps the
/// `c_euv_select_panel_open` and `c_euv_select_panel_closed` classes
/// rather than removing the options from the tree, so the option nodes
/// survive every re-render.
///
/// The options are grouped by their `group` caption: the ungrouped options
/// come first without a caption, and every other group is introduced by a
/// `c_euv_select_group` caption row. A selected option carries
/// `c_euv_select_option_active`, a disabled option carries
/// `c_euv_select_option_disabled` and cannot be picked, and an option can
/// request a `c_euv_select_separator` before itself. An empty option list
/// renders a single `c_euv_select_empty` row.
///
/// # Arguments
///
/// - `VirtualNode<EuvSelectProps>` - The props node holding the selection signal, the options, invalid state and change callback.
///
/// # Returns
///
/// - `VirtualNode` - The select control virtual DOM tree.
#[component]
pub fn euv_select(node: VirtualNode<EuvSelectProps>) -> VirtualNode {
    let EuvSelectProps {
        value,
        options,
        placeholder,
        label: label_text,
        id,
        name,
        required,
        invalid,
        disabled,
        align_item_with_trigger,
        on_change,
    }: EuvSelectProps = node.try_get_props().unwrap_or_default();
    let open: Signal<bool> = use_euv_select_open();
    let align_offset: Signal<String> = use_euv_select_alignment();
    let (scroll_up_visible, scroll_down_visible): (Signal<bool>, Signal<bool>) =
        use_euv_select_scroll_affordance();
    let (typeahead_buffer, typeahead_generation): (Signal<String>, Signal<u32>) =
        use_euv_select_typeahead_state();
    let typeahead_options: Rc<Vec<EuvSelectOption>> = Rc::new(options.clone());
    let select_id: UseId = use_id("select");
    let select_id_value: String = select_id.get_value().get();
    let trigger_id: String = if id.is_empty() {
        select_id_value.clone()
    } else {
        String::from(id)
    };
    let listbox_id: String = format!("{trigger_id}-list");
    let selected: String = value.get();
    let selected_label: Option<&'static str> = options
        .iter()
        .find(|option: &&EuvSelectOption| option.value == selected)
        .map(|option: &EuvSelectOption| option.label);
    let trigger_text: &'static str = selected_label.unwrap_or(placeholder);
    let has_selection: bool = selected_label.is_some();
    let is_empty: bool = options.is_empty();
    let form_input: VirtualNode = select_form_input(name, value.get(), required, disabled);
    let option_context: EuvSelectOptionContext = EuvSelectOptionContext {
        value,
        options: typeahead_options,
        open,
        align_item_with_trigger,
        align_offset,
        scroll_up_visible,
        scroll_down_visible,
        typeahead_buffer,
        typeahead_generation,
        on_change,
    };
    let trigger_keydown: Option<Rc<dyn Fn(Event)>> =
        select_trigger_keyboard(option_context.clone());
    let scroll_handler: Option<Rc<dyn Fn(Event)>> =
        select_panel_scroll_handler(scroll_up_visible, scroll_down_visible);
    let trigger_class: &Css = c_euv_select_trigger();
    let shadcn_trigger_class: Css = if cfg!(feature = "theme-shadcn") {
        c_euv_select_trigger_shadcn().clone()
    } else {
        Css::default()
    };
    let invalid_trigger_class: Css = if invalid {
        c_euv_select_trigger_invalid().clone()
    } else {
        Css::default()
    };
    let panel_class: &Css = c_euv_select_panel();
    let panel_open_class: &Css = c_euv_select_panel_open();
    let panel_closed_class: &Css = c_euv_select_panel_closed();
    let shadcn_panel_class: Css = if cfg!(feature = "theme-shadcn") {
        c_euv_select_panel_shadcn().clone()
    } else {
        Css::default()
    };
    let align_offset_value: String = align_offset.get();
    let aligned_panel_class: Css = if align_item_with_trigger && !align_offset_value.is_empty() {
        c_euv_select_panel_aligned(&align_offset_value)
    } else {
        Css::default()
    };
    let entries: Vec<SelectPanelEntry> = select_panel_entries(options);
    let panel_items: Vec<VirtualNode> = if is_empty {
        vec![html! {
            div {
                class: c_euv_select_empty()
                "No options available"
            }
        }]
    } else {
        entries
            .into_iter()
            .enumerate()
            .map(
                |(entry_index, entry): (usize, SelectPanelEntry)| match entry {
                    SelectPanelEntry::Group { label, options } => select_option_group(
                        label,
                        select_id_value.as_str(),
                        entry_index,
                        options,
                        selected.as_str(),
                        option_context.clone(),
                    ),
                    SelectPanelEntry::Separator => html! {
                        div {
                            "data-slot": "select-separator"
                            role: "separator"
                            aria-orientation: "horizontal"
                            class: c_euv_select_separator()
                        }
                    },
                    SelectPanelEntry::Option(option) => {
                        select_option_row(option, option.value == selected, option_context.clone())
                    }
                },
            )
            .collect()
    };
    html! {
        div {
            "data-slot": "select"
            "data-invalid": if invalid {
                SELECT_ATTRIBUTE_ON
            } else {
                SELECT_ATTRIBUTE_OFF
            }
            "data-required": if required {
                SELECT_ATTRIBUTE_ON
            } else {
                SELECT_ATTRIBUTE_OFF
            }
            class: c_euv_select()
            if !label_text.is_empty() {
                label {
                    r#for: trigger_id.clone()
                    class: c_form_label()
                    label_text
                }
            }
            button {
                type: "button"
                "data-slot": "select-trigger"
                id: trigger_id
                role: "combobox"
                "aria-haspopup": "listbox"
                "aria-controls": listbox_id.clone()
                "aria-invalid": if invalid {
                    SELECT_ATTRIBUTE_ON
                } else {
                    SELECT_ATTRIBUTE_OFF
                }
                "aria-required": if required {
                    SELECT_ATTRIBUTE_ON
                } else {
                    SELECT_ATTRIBUTE_OFF
                }
                aria-expanded: if { open.get() } {
                    "true"
                } else {
                    "false"
                }
                class: trigger_class
                class: shadcn_trigger_class
                class: invalid_trigger_class
                disabled: boolean_attr(disabled)
                onclick: select_toggle(disabled, option_context.clone())
                onkeydown: trigger_keydown
                span {
                    "data-slot": "select-value"
                    class: if has_selection {
                        c_euv_select_value()
                    } else {
                        c_euv_select_placeholder()
                    }
                    trigger_text
                }
                span {
                    class: c_euv_select_caret()
                    "▾"
                }
            }
            form_input
            div {
                "data-slot": "select-content"
                role: "presentation"
                class: panel_class
                class: if { open.get() } {
                    panel_open_class
                } else {
                    panel_closed_class
                }
                class: shadcn_panel_class
                class: aligned_panel_class
                if { scroll_up_visible.get() } {
                    euv_select_scroll_up_button {}
                }
                div {
                    "data-slot": "select-viewport"
                    id: listbox_id
                    role: "listbox"
                    class: c_euv_select_viewport()
                    onscroll: scroll_handler
                    panel_items
                }
                if { scroll_down_visible.get() } {
                    euv_select_scroll_down_button {}
                }
            }
        }
    }
}

/// Renders the native form control paired with a custom Select trigger.
///
/// # Arguments
///
/// - `&'static str` - The form field name; empty means no form control is needed.
/// - `String` - The current selected value.
/// - `bool` - Whether the native control participates in required validation.
/// - `bool` - Whether the control is disabled.
///
/// # Returns
///
/// - `VirtualNode` - The visually hidden native text input or an empty node.
fn select_form_input(
    name: &'static str,
    value: String,
    required: bool,
    disabled: bool,
) -> VirtualNode {
    if name.is_empty() && !required {
        return VirtualNode::Empty;
    }
    html! {
        input {
            r#type: "text"
            name: name
            value: value
            required: boolean_attr(required)
            disabled: boolean_attr(disabled)
            "aria-hidden": "true"
            tabindex: "-1"
            class: c_euv_select_form_input()
        }
    }
}

/// Obtains the panel open-state signal registered against the current hook
/// context slot.
///
/// Behaves like [`HookContext::use_hook`] — the same signal is returned on
/// every render at the same hook index, so the panel keeps its open state
/// across re-renders. The factory is used directly when no hook context is
/// active (e.g. when called outside a render cycle).
///
/// # Returns
///
/// - `Signal<bool>` - The signal holding the panel's open state.
pub fn use_euv_select_open() -> Signal<bool> {
    HookContext::use_hook(|| Signal::create(false))
}

/// Holds the computed translation of a Select panel while it is open.
///
/// The signal is hook-scoped so alignment survives the render between opening
/// the panel and measuring the selected option.
///
/// # Returns
///
/// - `Signal<String>` - The CSS translation of the open panel, or an empty string.
fn use_euv_select_alignment() -> Signal<String> {
    HookContext::use_hook(|| Signal::create(String::new()))
}

/// Holds whether either end of the open Select list can scroll further.
///
/// # Returns
///
/// - `(Signal<bool>, Signal<bool>)` - Whether the panel can scroll up and down.
fn use_euv_select_scroll_affordance() -> (Signal<bool>, Signal<bool>) {
    HookContext::use_hook(|| (Signal::create(false), Signal::create(false)))
}

/// Holds the short-lived prefix used to match keyboard typeahead input.
///
/// # Returns
///
/// - `(Signal<String>, Signal<u32>)` - The prefix and its reset-timer generation.
fn use_euv_select_typeahead_state() -> (Signal<String>, Signal<u32>) {
    HookContext::use_hook(|| (Signal::create(String::new()), Signal::create(0_u32)))
}

/// Recognizes a printable typeahead key without consuming modified shortcuts.
///
/// # Arguments
///
/// - `&KeyboardEvent` - The event whose key and modifiers are checked.
/// - `&str` - The key value reported by the browser.
///
/// # Returns
///
/// - `bool` - Whether the key contributes to the Select typeahead prefix.
fn is_select_typeahead_key(event: &KeyboardEvent, key: &str) -> bool {
    key.chars().count() == 1
        && key != " "
        && !event.ctrl_key()
        && !event.alt_key()
        && !event.meta_key()
}

/// Finds the next enabled option whose label starts with the typeahead prefix.
///
/// Search continues after the current option and wraps once to preserve the
/// expected cycling behavior for labels that share an initial character.
///
/// # Arguments
///
/// - `&[EuvSelectOption]` - The options in their rendered order.
/// - `&str` - The case-insensitive prefix to match.
/// - `&str` - The currently active or selected option value.
///
/// # Returns
///
/// - `Option<&'static str>` - The value of the next enabled matching option.
fn select_typeahead_match(
    options: &[EuvSelectOption],
    query: &str,
    current_value: &str,
) -> Option<&'static str> {
    if options.is_empty() || query.is_empty() {
        return None;
    }
    let normalized_query: String = query.to_lowercase();
    let current_index: Option<usize> = options
        .iter()
        .position(|option: &EuvSelectOption| option.value == current_value);
    let start_index: usize = current_index
        .map(|index: usize| (index + 1) % options.len())
        .unwrap_or(0);
    for offset in 0..options.len() {
        let option_index: usize = (start_index + offset) % options.len();
        let option: &EuvSelectOption = &options[option_index];
        if option.matches_typeahead(&normalized_query) {
            return Some(option.value);
        }
    }
    None
}

/// Clears a typeahead prefix and invalidates any pending timeout that would restore it.
///
/// # Arguments
///
/// - `&Signal<String>` - The typeahead prefix to clear.
/// - `&Signal<u32>` - The generation used to cancel older reset callbacks.
fn clear_select_typeahead(buffer: &Signal<String>, generation: &Signal<u32>) {
    buffer.set(String::new());
    generation.set(generation.get().wrapping_add(1));
}

/// Schedules the typeahead prefix to clear after the input idle window.
///
/// # Arguments
///
/// - `Signal<String>` - The prefix signal cleared by the timer.
/// - `Signal<u32>` - The generation used to reject stale timers.
/// - `u32` - The generation this timer belongs to.
fn schedule_select_typeahead_reset(
    buffer: Signal<String>,
    generation: Signal<u32>,
    expected_generation: u32,
) {
    let Some(window) = window() else {
        return;
    };
    let callback: Function = Closure::once_into_js(move || {
        if generation.get() == expected_generation {
            buffer.set(String::new());
        }
    })
    .unchecked_into();
    let _: Result<i32, JsValue> = window.set_timeout_with_callback_and_timeout_and_arguments_0(
        &callback,
        SELECT_TYPEAHEAD_RESET_MS,
    );
}

/// Extends the active typeahead prefix, falling back to a new single-key search when needed.
///
/// # Arguments
///
/// - `&[EuvSelectOption]` - The options and labels available for matching.
/// - `&str` - The newly typed printable key.
/// - `&str` - The currently active or selected value.
/// - `Signal<String>` - The prefix held between consecutive key events.
/// - `Signal<u32>` - The generation used to invalidate prior reset timers.
///
/// # Returns
///
/// - `Option<&'static str>` - The value of the next enabled matching option.
fn select_typeahead_value(
    options: &[EuvSelectOption],
    typed_key: &str,
    current_value: &str,
    buffer: Signal<String>,
    generation: Signal<u32>,
) -> Option<&'static str> {
    let normalized_key: String = typed_key.to_lowercase();
    let current_prefix: String = buffer.get();
    let combined_prefix: String = format!("{current_prefix}{normalized_key}");
    let combined_match: Option<&'static str> =
        select_typeahead_match(options, &combined_prefix, current_value);
    let (next_prefix, match_value): (String, Option<&'static str>) =
        if let Some(matched_value) = combined_match {
            (combined_prefix, Some(matched_value))
        } else {
            let single_match: Option<&'static str> =
                select_typeahead_match(options, &normalized_key, current_value);
            (normalized_key, single_match)
        };
    if match_value.is_some() {
        buffer.set(next_prefix);
        let next_generation: u32 = generation.get().wrapping_add(1);
        generation.set(next_generation);
        schedule_select_typeahead_reset(buffer, generation, next_generation);
    } else {
        clear_select_typeahead(&buffer, &generation);
    }
    match_value
}

/// Moves keyboard focus to the option row matching a Select value.
///
/// # Arguments
///
/// - `&Element` - The Select root containing the option rows.
/// - `&str` - The option value to focus.
fn select_focus_option(root: &Element, option_value: &str) {
    let Ok(nodes) = root.query_selector_all(SELECT_OPTION_SELECTOR) else {
        return;
    };
    for index in 0..nodes.length() {
        let Some(option) = nodes
            .item(index)
            .and_then(|node: Node| node.dyn_into::<HtmlElement>().ok())
        else {
            continue;
        };
        if option.get_attribute(SELECT_VALUE_ATTRIBUTE).as_deref() == Some(option_value) {
            let _: Result<(), JsValue> = option.focus();
            return;
        }
    }
}

/// Flattens the option list into the rows the panel renders.
///
/// The ungrouped options come first. Each named group is emitted once in the
/// order its label first appears, with its options kept together inside it.
///
/// # Arguments
///
/// - `Vec<EuvSelectOption>` - The options in declaration order.
///
/// # Returns
///
/// - `Vec<SelectPanelEntry>` - The caption and option rows in render order.
fn select_panel_entries(options: Vec<EuvSelectOption>) -> Vec<SelectPanelEntry> {
    let mut entries: Vec<SelectPanelEntry> = Vec::with_capacity(options.len());
    let mut groups: Vec<(&'static str, Vec<EuvSelectOption>)> = Vec::new();
    for option in options {
        if option.group.is_empty() {
            if option.separator_before {
                entries.push(SelectPanelEntry::Separator);
            }
            entries.push(SelectPanelEntry::Option(option));
            continue;
        }
        let existing: Option<usize> =
            groups
                .iter()
                .position(|(caption, _): &(&'static str, Vec<EuvSelectOption>)| {
                    *caption == option.group
                });
        match existing {
            Some(index) => groups[index].1.push(option),
            None => groups.push((option.group, vec![option])),
        }
    }
    for (label, options) in groups {
        entries.push(SelectPanelEntry::Group { label, options });
    }
    entries
}

/// Renders a labelled group and the option rows it owns.
///
/// # Arguments
///
/// - `&'static str` - The accessible label of the group.
/// - `&str` - The unique Select id prefix used by the group's label reference.
/// - `usize` - The group row's stable index within the panel.
/// - `Vec<EuvSelectOption>` - The grouped option rows in declaration order.
/// - `&str` - The currently selected value.
/// - `EuvSelectOptionContext` - The reactive context shared by each row.
///
/// # Returns
///
/// - `VirtualNode` - The labelled Select group subtree.
fn select_option_group(
    label: &'static str,
    group_id_prefix: &str,
    group_index: usize,
    options: Vec<EuvSelectOption>,
    selected: &str,
    context: EuvSelectOptionContext,
) -> VirtualNode {
    let group_label_id: String = format!("{group_id_prefix}-label-{group_index}");
    let group_label_id_ref: &str = group_label_id.as_str();
    let mut group_items: Vec<VirtualNode> = Vec::with_capacity(options.len().saturating_mul(2));
    for option in options {
        if option.separator_before {
            group_items.push(html! {
                div {
                    "data-slot": "select-separator"
                    role: "separator"
                    aria-orientation: "horizontal"
                    class: c_euv_select_separator()
                }
            });
        }
        let is_selected: bool = option.value == selected;
        group_items.push(select_option_row(option, is_selected, context.clone()));
    }
    html! {
        div {
            "data-slot": "select-group"
            role: "group"
            "aria-labelledby": group_label_id_ref
            div {
                "data-slot": "select-label"
                id: group_label_id_ref
                class: c_euv_select_group()
                label
            }
            group_items
        }
    }
}

/// Renders one option row of the panel.
///
/// A disabled option carries the muted class, is marked `disabled` on the
/// button and gets no click handler at all, so it can never write into the
/// selection signal.
///
/// # Arguments
///
/// - `EuvSelectOption` - The option this row renders.
/// - `bool` - Whether the option is the selected one.
/// - `EuvSelectOptionContext` - The reactive selection, panel and scroll state shared by the row.
///
/// # Returns
///
/// - `VirtualNode` - The option row virtual DOM tree.
fn select_option_row(
    option: EuvSelectOption,
    is_selected: bool,
    context: EuvSelectOptionContext,
) -> VirtualNode {
    let disabled: bool = option.disabled;
    let selected_attr: &str = if is_selected {
        SELECT_ATTRIBUTE_ON
    } else {
        SELECT_ATTRIBUTE_OFF
    };
    let handler: Option<Rc<dyn Fn(Event)>> = if disabled {
        None
    } else {
        select_option(option.value, context.clone())
    };
    let keydown_handler: Option<Rc<dyn Fn(Event)>> = if disabled {
        None
    } else {
        select_option_keyboard(option.value, context)
    };
    let tabindex: &str = if is_selected { "0" } else { "-1" };
    let default_option_class: &Css = c_euv_select_option();
    let state_option_class: &Css = if disabled {
        c_euv_select_option_disabled()
    } else if is_selected {
        c_euv_select_option_active()
    } else {
        c_euv_select_option()
    };
    let shadcn_option_class: Css = if cfg!(feature = "theme-shadcn") {
        c_euv_select_option_shadcn().clone()
    } else {
        Css::default()
    };
    let shadcn_selected_class: Css = if cfg!(feature = "theme-shadcn") && is_selected && !disabled {
        c_euv_select_option_active_shadcn().clone()
    } else {
        Css::default()
    };
    html! {
        button {
            type: "button"
            "data-slot": "select-item"
            "data-value": option.value
            role: "option"
            aria-selected: selected_attr
            tabindex: tabindex
            class: default_option_class
            class: state_option_class
            class: shadcn_option_class
            class: shadcn_selected_class
            disabled: boolean_attr(disabled)
            onclick: handler
            onkeydown: keydown_handler
            option.label
            if { cfg!(feature = "theme-shadcn") && is_selected && !disabled } {
                span {
                    "data-slot": "select-item-indicator"
                    aria-hidden: "true"
                    class: c_euv_select_option_indicator_shadcn()
                    SELECT_CHECKMARK
                }
            }
        }
    }
}

/// Handles keyboard opening and typeahead selection from the trigger.
///
/// # Arguments
///
/// - `EuvSelectOptionContext` - The reactive selection, options, panel and typeahead state.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - The trigger keyboard handler.
fn select_trigger_keyboard(context: EuvSelectOptionContext) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |event: Event| {
        let Ok(keyboard_event) = event.dyn_into::<KeyboardEvent>() else {
            return;
        };
        let key: String = keyboard_event.key();
        let Some(root) = keyboard_event
            .target()
            .and_then(|target: EventTarget| target.dyn_into::<Element>().ok())
            .and_then(|target: Element| target.closest(SELECT_ROOT_SELECTOR).ok().flatten())
        else {
            return;
        };
        if is_select_typeahead_key(&keyboard_event, &key) {
            let current_value: String = context.value.get();
            if context.open.get()
                && let Some(option_value) = select_typeahead_value(
                    context.options.as_ref(),
                    &key,
                    &current_value,
                    context.typeahead_buffer,
                    context.typeahead_generation,
                )
            {
                keyboard_event.prevent_default();
                select_focus_option(&root, option_value);
            }
            return;
        }
        if key != SELECT_KEY_ARROW_DOWN && key != SELECT_KEY_ARROW_UP {
            return;
        }
        keyboard_event.prevent_default();
        context.open.set(true);
        clear_select_typeahead(&context.typeahead_buffer, &context.typeahead_generation);
        let selected_value: String = context.value.get();
        schedule_select_alignment(
            root.clone(),
            selected_value.clone(),
            context.align_item_with_trigger,
            context.align_offset,
        );
        schedule_select_scroll_affordance(
            root.clone(),
            context.scroll_up_visible,
            context.scroll_down_visible,
        );
        schedule_select_focus(root, selected_value);
    }))
}

/// Focuses the selected option, falling back to the first enabled item.
///
/// # Arguments
///
/// - `&Element` - The Select root containing its options.
/// - `&str` - The selected option value, if one is set.
fn select_focus_selected_or_first(root: &Element, selected_value: &str) {
    let Ok(nodes) = root.query_selector_all(SELECT_OPTION_SELECTOR) else {
        return;
    };
    let mut first: Option<HtmlElement> = None;
    let mut selected: Option<HtmlElement> = None;
    for index in 0..nodes.length() {
        let Some(option) = nodes
            .item(index)
            .and_then(|node: Node| node.dyn_into::<HtmlElement>().ok())
        else {
            continue;
        };
        if first.is_none() {
            first = Some(option.clone());
        }
        if option.get_attribute(SELECT_VALUE_ATTRIBUTE).as_deref() == Some(selected_value) {
            selected = Some(option);
            break;
        }
    }
    if let Some(option) = selected.or(first) {
        let _: Result<(), JsValue> = option.focus();
    }
}

/// Focuses the selected or first enabled item after the panel mount commits.
///
/// # Arguments
///
/// - `Element` - The Select root containing its option rows.
/// - `String` - The selected option value, or an empty string when none is selected.
fn schedule_select_focus(root: Element, selected_value: String) {
    let callback: Function = Closure::once_into_js(move || {
        select_focus_selected_or_first(&root, &selected_value);
    })
    .unchecked_into();
    if let Some(window) = window() {
        let _: Result<i32, JsValue> = window.request_animation_frame(&callback);
    }
}

/// Positions the open panel so its selected, or first enabled, option aligns with the trigger.
///
/// The default edge-aligned position remains in place when the aligned panel
/// would cross the configured viewport gutter.
///
/// # Arguments
///
/// - `Element` - The Select root whose trigger, panel and options are measured.
/// - `String` - The selected option value, or an empty string when none is selected.
/// - `bool` - Whether item alignment is enabled.
/// - `Signal<String>` - The CSS translation stored for the panel.
fn schedule_select_alignment(
    root: Element,
    selected_value: String,
    align_item_with_trigger: bool,
    align_offset: Signal<String>,
) {
    if !align_item_with_trigger {
        align_offset.set(String::new());
        return;
    }
    let Some(window) = window() else {
        align_offset.set(String::new());
        return;
    };
    let callback_window: Window = window.clone();
    let callback: Function = Closure::once_into_js(move || {
        let Some(trigger) = root.query_selector(SELECT_TRIGGER_SELECTOR).ok().flatten() else {
            align_offset.set(String::new());
            return;
        };
        let Some(panel) = root.query_selector(SELECT_CONTENT_SELECTOR).ok().flatten() else {
            align_offset.set(String::new());
            return;
        };
        let Some(viewport) = panel
            .query_selector(SELECT_VIEWPORT_SELECTOR)
            .ok()
            .flatten()
        else {
            align_offset.set(String::new());
            return;
        };
        let Ok(viewport_scroll) = viewport.clone().dyn_into::<HtmlElement>() else {
            align_offset.set(String::new());
            return;
        };
        let Ok(nodes) = viewport.query_selector_all(SELECT_OPTION_SELECTOR) else {
            align_offset.set(String::new());
            return;
        };
        let mut first: Option<HtmlElement> = None;
        let mut selected: Option<HtmlElement> = None;
        for index in 0..nodes.length() {
            let Some(option) = nodes
                .item(index)
                .and_then(|node: Node| node.dyn_into::<HtmlElement>().ok())
            else {
                continue;
            };
            if first.is_none() {
                first = Some(option.clone());
            }
            if option.get_attribute(SELECT_VALUE_ATTRIBUTE).as_deref()
                == Some(selected_value.as_str())
            {
                selected = Some(option);
                break;
            }
        }
        let Some(option) = selected.or(first) else {
            align_offset.set(String::new());
            return;
        };
        let panel_height: f64 = f64::from(viewport_scroll.client_height());
        let max_scroll: f64 =
            f64::from(viewport_scroll.scroll_height() - viewport_scroll.client_height()).max(0.0);
        let option_center: f64 =
            f64::from(option.offset_top()) + f64::from(option.offset_height()) / 2.0;
        let scroll_top: f64 = (option_center - panel_height / 2.0).clamp(0.0, max_scroll);
        viewport_scroll.set_scroll_top(scroll_top.round() as i32);
        let trigger_rect: DomRect = trigger.get_bounding_client_rect();
        let option_rect: DomRect = option.get_bounding_client_rect();
        let panel_rect: DomRect = panel.get_bounding_client_rect();
        let offset: f64 = trigger_rect.top() + trigger_rect.height() / 2.0
            - option_rect.top()
            - option_rect.height() / 2.0;
        let viewport_height: f64 = callback_window
            .inner_height()
            .ok()
            .and_then(|height: JsValue| height.as_f64())
            .unwrap_or_default();
        let aligned_top: f64 = panel_rect.top() + offset;
        let aligned_bottom: f64 = panel_rect.bottom() + offset;
        if aligned_top < SELECT_ALIGNMENT_VIEWPORT_GUTTER_PX
            || aligned_bottom > viewport_height - SELECT_ALIGNMENT_VIEWPORT_GUTTER_PX
        {
            align_offset.set(String::new());
            return;
        }
        align_offset.set(format!("{offset:.2}px"));
    })
    .unchecked_into();
    let _: Result<i32, JsValue> = window.request_animation_frame(&callback);
}

/// Updates the scroll indicators from the panel's current viewport position.
///
/// # Arguments
///
/// - `&HtmlElement` - The Select panel that owns the scroll position.
/// - `&Signal<bool>` - Whether to show the scroll-up indicator.
/// - `&Signal<bool>` - Whether to show the scroll-down indicator.
fn update_select_scroll_affordance(
    panel: &HtmlElement,
    scroll_up_visible: &Signal<bool>,
    scroll_down_visible: &Signal<bool>,
) {
    let client_height: i32 = panel.client_height();
    let scroll_height: i32 = panel.scroll_height();
    let max_scroll: i32 = (scroll_height - client_height).max(0);
    let scroll_top: i32 = panel.scroll_top().clamp(0, max_scroll);
    let has_overflow: bool = max_scroll > 0;
    scroll_up_visible.set(has_overflow && scroll_top > 0);
    scroll_down_visible.set(has_overflow && scroll_top < max_scroll);
}

/// Updates the scroll indicators when the list viewport moves.
///
/// # Arguments
///
/// - `Signal<bool>` - Whether to show the scroll-up indicator.
/// - `Signal<bool>` - Whether to show the scroll-down indicator.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - The panel scroll handler.
fn select_panel_scroll_handler(
    scroll_up_visible: Signal<bool>,
    scroll_down_visible: Signal<bool>,
) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |event: Event| {
        let panel: Option<HtmlElement> = event
            .target()
            .and_then(|target: EventTarget| target.dyn_into::<HtmlElement>().ok());
        if let Some(panel) = panel {
            update_select_scroll_affordance(&panel, &scroll_up_visible, &scroll_down_visible);
        }
    }))
}

/// Measures overflow after an opening render so the matching edge indicator appears.
///
/// # Arguments
///
/// - `Element` - The root that owns the Select content panel.
/// - `Signal<bool>` - Whether to show the scroll-up indicator.
/// - `Signal<bool>` - Whether to show the scroll-down indicator.
fn schedule_select_scroll_affordance(
    root: Element,
    scroll_up_visible: Signal<bool>,
    scroll_down_visible: Signal<bool>,
) {
    let Some(window) = window() else {
        scroll_up_visible.set(false);
        scroll_down_visible.set(false);
        return;
    };
    let callback: Function = Closure::once_into_js(move || {
        let viewport: Option<HtmlElement> = root
            .query_selector(SELECT_CONTENT_SELECTOR)
            .ok()
            .flatten()
            .and_then(|panel: Element| {
                panel
                    .query_selector(SELECT_VIEWPORT_SELECTOR)
                    .ok()
                    .flatten()
            })
            .and_then(|viewport: Element| viewport.dyn_into::<HtmlElement>().ok());
        if let Some(viewport) = viewport {
            update_select_scroll_affordance(&viewport, &scroll_up_visible, &scroll_down_visible);
        } else {
            scroll_up_visible.set(false);
            scroll_down_visible.set(false);
        }
    })
    .unchecked_into();
    let _: Result<i32, JsValue> = window.request_animation_frame(&callback);
}

/// Handles typeahead, option navigation, dismissal and focus restoration.
///
/// # Arguments
///
/// - `&'static str` - The value of the focused option.
/// - `EuvSelectOptionContext` - The reactive options, panel and typeahead state.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - The option keyboard handler.
fn select_option_keyboard(
    value: &'static str,
    context: EuvSelectOptionContext,
) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |event: Event| {
        let Ok(keyboard_event) = event.dyn_into::<KeyboardEvent>() else {
            return;
        };
        let key: String = keyboard_event.key();
        let Some(target) = keyboard_event
            .target()
            .and_then(|target: EventTarget| target.dyn_into::<Element>().ok())
        else {
            return;
        };
        let Some(root) = target.closest(SELECT_ROOT_SELECTOR).ok().flatten() else {
            return;
        };
        if is_select_typeahead_key(&keyboard_event, &key) {
            if let Some(option_value) = select_typeahead_value(
                context.options.as_ref(),
                &key,
                value,
                context.typeahead_buffer,
                context.typeahead_generation,
            ) {
                keyboard_event.prevent_default();
                select_focus_option(&root, option_value);
            }
            return;
        }
        if key == SELECT_KEY_ESCAPE {
            keyboard_event.prevent_default();
            context.open.set(false);
            context.align_offset.set(String::new());
            context.scroll_up_visible.set(false);
            context.scroll_down_visible.set(false);
            clear_select_typeahead(&context.typeahead_buffer, &context.typeahead_generation);
            if let Some(trigger) = root
                .query_selector(SELECT_TRIGGER_SELECTOR)
                .ok()
                .flatten()
                .and_then(|trigger: Element| trigger.dyn_into::<HtmlElement>().ok())
            {
                let _: Result<(), JsValue> = trigger.focus();
            }
            return;
        }
        if key == SELECT_KEY_ENTER || key == SELECT_KEY_SPACE {
            keyboard_event.prevent_default();
            context.value.set(String::from(value));
            if let Some(callback) = &context.on_change {
                callback(value);
            }
            context.open.set(false);
            context.align_offset.set(String::new());
            context.scroll_up_visible.set(false);
            context.scroll_down_visible.set(false);
            clear_select_typeahead(&context.typeahead_buffer, &context.typeahead_generation);
            if let Some(trigger) = root
                .query_selector(SELECT_TRIGGER_SELECTOR)
                .ok()
                .flatten()
                .and_then(|trigger: Element| trigger.dyn_into::<HtmlElement>().ok())
            {
                let _: Result<(), JsValue> = trigger.focus();
            }
            return;
        }
        let Ok(nodes) = root.query_selector_all(SELECT_OPTION_SELECTOR) else {
            return;
        };
        let mut options: Vec<HtmlElement> = Vec::new();
        for index in 0..nodes.length() {
            if let Some(option) = nodes
                .item(index)
                .and_then(|node: Node| node.dyn_into::<HtmlElement>().ok())
            {
                options.push(option);
            }
        }
        let Some(current_index) = options.iter().position(|option: &HtmlElement| {
            option.get_attribute(SELECT_VALUE_ATTRIBUTE).as_deref() == Some(value)
        }) else {
            return;
        };
        let next_index: usize = if key == SELECT_KEY_ARROW_DOWN {
            (current_index + 1) % options.len()
        } else if key == SELECT_KEY_ARROW_UP {
            (current_index + options.len() - 1) % options.len()
        } else if key == SELECT_KEY_HOME {
            0
        } else if key == SELECT_KEY_END {
            options.len() - 1
        } else {
            return;
        };
        let Some(next_option) = options.get(next_index) else {
            return;
        };
        keyboard_event.prevent_default();
        let _: Result<(), JsValue> = next_option.focus();
    }))
}

/// Builds the trigger click handler that opens or closes the panel.
///
/// # Arguments
///
/// - `bool` - Whether the trigger is disabled.
/// - `EuvSelectOptionContext` - The reactive panel, selection and typeahead state.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - The toggle click handler.
fn select_toggle(disabled: bool, context: EuvSelectOptionContext) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |event: Event| {
        if disabled {
            return;
        }
        let should_open: bool = !context.open.get();
        context.open.set(should_open);
        clear_select_typeahead(&context.typeahead_buffer, &context.typeahead_generation);
        if !should_open {
            context.align_offset.set(String::new());
            context.scroll_up_visible.set(false);
            context.scroll_down_visible.set(false);
            return;
        }
        let Some(root) = event
            .target()
            .and_then(|target: EventTarget| target.dyn_into::<Element>().ok())
            .and_then(|target: Element| target.closest(SELECT_ROOT_SELECTOR).ok().flatten())
        else {
            context.align_offset.set(String::new());
            context.scroll_up_visible.set(false);
            context.scroll_down_visible.set(false);
            return;
        };
        let selected_value: String = context.value.get();
        schedule_select_alignment(
            root.clone(),
            selected_value.clone(),
            context.align_item_with_trigger,
            context.align_offset,
        );
        schedule_select_scroll_affordance(
            root.clone(),
            context.scroll_up_visible,
            context.scroll_down_visible,
        );
        schedule_select_focus(root, selected_value);
    }))
}

/// Builds the click handler that picks one option.
///
/// The handler writes the option value into the selection signal, reports
/// it to the change callback and closes the panel.
///
/// # Arguments
///
/// - `&'static str` - The value of the picked option.
/// - `EuvSelectOptionContext` - The reactive selection and panel state.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - The pick click handler.
fn select_option(
    option_value: &'static str,
    context: EuvSelectOptionContext,
) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |event: Event| {
        context.value.set(String::from(option_value));
        if let Some(callback) = &context.on_change {
            callback(option_value);
        }
        context.open.set(false);
        context.align_offset.set(String::new());
        context.scroll_up_visible.set(false);
        context.scroll_down_visible.set(false);
        clear_select_typeahead(&context.typeahead_buffer, &context.typeahead_generation);
        if let Some(trigger) = event
            .target()
            .and_then(|target: EventTarget| target.dyn_into::<Element>().ok())
            .and_then(|target: Element| target.closest(SELECT_ROOT_SELECTOR).ok().flatten())
            .and_then(|root: Element| root.query_selector(SELECT_TRIGGER_SELECTOR).ok().flatten())
            .and_then(|trigger: Element| trigger.dyn_into::<HtmlElement>().ok())
        {
            let _: Result<(), JsValue> = trigger.focus();
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
    if flag {
        SELECT_ATTRIBUTE_ON
    } else {
        SELECT_ATTRIBUTE_EMPTY
    }
}

/// Shows that a long option list can scroll upward.
///
/// The indicator is decorative, carries no focus or click behavior, and
/// signals that more options remain above the visible viewport.
///
/// # Arguments
///
/// - `VirtualNode<EuvSelectScrollUpButtonProps>` - A `VirtualNode<EuvSelectScrollUpButtonProps>` parameter.
///
/// # Returns
///
/// - `VirtualNode` - A `VirtualNode` value.
#[component]
pub fn euv_select_scroll_up_button(node: VirtualNode<EuvSelectScrollUpButtonProps>) -> VirtualNode {
    let EuvSelectScrollUpButtonProps {}: EuvSelectScrollUpButtonProps =
        node.try_get_props().unwrap_or_default();
    html! {
        div {
            "data-slot": "select-scroll-up-button"
            "aria-hidden": "true"
            class: c_euv_select_scroll_up_button()
            span {
                class: c_euv_select_scroll_icon()
                SELECT_SCROLL_UP_GLYPH
            }
        }
    }
}

/// Shows that a long option list can scroll downward.
///
/// The indicator is decorative, carries no focus or click behavior, and
/// signals that more options remain below the visible viewport.
///
/// # Arguments
///
/// - `VirtualNode<EuvSelectScrollDownButtonProps>` - A `VirtualNode<EuvSelectScrollDownButtonProps>` parameter.
///
/// # Returns
///
/// - `VirtualNode` - A `VirtualNode` value.
#[component]
pub fn euv_select_scroll_down_button(
    node: VirtualNode<EuvSelectScrollDownButtonProps>,
) -> VirtualNode {
    let EuvSelectScrollDownButtonProps {}: EuvSelectScrollDownButtonProps =
        node.try_get_props().unwrap_or_default();
    html! {
        div {
            "data-slot": "select-scroll-down-button"
            "aria-hidden": "true"
            class: c_euv_select_scroll_down_button()
            span {
                class: c_euv_select_scroll_icon()
                SELECT_SCROLL_DOWN_GLYPH
            }
        }
    }
}
