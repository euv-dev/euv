use super::*;

/// A single- or multiple-choice select control with an always-mounted panel.
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
        multiple,
        values,
        size,
        options,
        placeholder,
        label: label_text,
        aria_label,
        aria_describedby,
        id,
        name,
        form,
        auto_complete,
        required,
        invalid,
        disabled,
        read_only,
        open: open_prop,
        default_open,
        align_item_with_trigger,
        on_change,
        on_change_values,
        on_open_change,
    }: EuvSelectProps = node.try_get_props().unwrap_or_default();
    let internal_open: Signal<bool> = use_euv_select_open_with_default(default_open);
    let open: Signal<bool> = open_prop.unwrap_or(internal_open);
    let align_offset: Signal<String> = use_euv_select_alignment();
    let panel_position_style: Signal<String> = use_euv_select_panel_position();
    let viewport_position_style: Signal<String> = use_euv_select_viewport_position();
    #[cfg(target_arch = "wasm32")]
    let panel_gap: Signal<f64> = HookContext::use_hook(|| Signal::create(0.0));
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
    let initial_single_value: String = value.get();
    let initial_multiple_values: Vec<String> = values.get();
    let reset_single_signal: Signal<String> = value;
    let reset_multiple_signal: Signal<Vec<String>> = values;
    let root_id: String = format!("{trigger_id}-root");
    let reset_root_id: String = root_id.clone();
    let form_reset_callback: Rc<dyn Fn()> = if multiple {
        let reset_required: bool = required;
        Rc::new(move || {
            let reset_values: Vec<String> = initial_multiple_values.clone();
            reset_multiple_signal.set(reset_values.clone());
            let controls: Option<HtmlCollection> = window()
                .and_then(|window: Window| window.document())
                .and_then(|document: Document| document.get_element_by_id(&reset_root_id))
                .map(|root: Element| root.get_elements_by_tag_name(SELECT_FORM_INPUT_TAG_NAME));
            if let Some(controls) = controls {
                let mut input_index: u32 = 0;
                if reset_required {
                    let validation_value: &str = if reset_values.is_empty() {
                        ""
                    } else {
                        SELECT_FORM_VALIDATION_VALUE_SELECTED
                    };
                    if let Some(input) = controls
                        .item(input_index)
                        .and_then(|element: Element| element.dyn_into::<HtmlInputElement>().ok())
                    {
                        input.set_value(validation_value);
                    }
                    input_index += 1;
                }
                for selected_value in reset_values.iter() {
                    if let Some(input) = controls
                        .item(input_index)
                        .and_then(|element: Element| element.dyn_into::<HtmlInputElement>().ok())
                    {
                        input.set_value(selected_value);
                    }
                    input_index += 1;
                }
            }
        })
    } else {
        Rc::new(move || {
            reset_single_signal.set(initial_single_value.clone());
            let input: Option<HtmlInputElement> = window()
                .and_then(|window: Window| window.document())
                .and_then(|document: Document| document.get_element_by_id(&reset_root_id))
                .and_then(|root: Element| {
                    root.get_elements_by_tag_name(SELECT_FORM_INPUT_TAG_NAME)
                        .item(0)
                })
                .and_then(|element: Element| element.dyn_into::<HtmlInputElement>().ok());
            if let Some(input) = input {
                input.set_value(&initial_single_value);
            }
        })
    };
    use_form_reset(trigger_id.clone(), form, form_reset_callback);
    let panel_id: String = format!("{trigger_id}-content");
    let listbox_id: String = format!("{trigger_id}-list");
    let selected: String = value.get();
    let selected_values: Vec<String> = if multiple {
        values.get()
    } else if selected.is_empty() {
        Vec::new()
    } else {
        vec![selected.clone()]
    };
    let selected_labels: Vec<&str> = selected_values
        .iter()
        .filter_map(|selected_value: &String| {
            options
                .iter()
                .find(|option: &&EuvSelectOption| option.value == selected_value)
                .map(|option: &EuvSelectOption| option.label)
        })
        .collect();
    let trigger_text: String = if selected_labels.is_empty() {
        String::from(placeholder)
    } else {
        selected_labels.join(", ")
    };
    let has_selection: bool = !selected_labels.is_empty();
    let mut active_value: Option<String> =
        selected_values.iter().find_map(|selected_value: &String| {
            options
                .iter()
                .find(|option: &&EuvSelectOption| option.value == selected_value)
                .map(|option: &EuvSelectOption| String::from(option.value))
        });
    if multiple && active_value.is_none() {
        active_value = options
            .iter()
            .find(|option: &&EuvSelectOption| !option.disabled)
            .map(|option: &EuvSelectOption| String::from(option.value));
    }
    let is_empty: bool = options.is_empty();
    let form_input: VirtualNode = select_form_input(EuvSelectFormInputProps {
        name,
        value: selected,
        values: selected_values.clone(),
        multiple,
        required,
        disabled,
        form,
        auto_complete,
    });
    let option_context: EuvSelectOptionContext = EuvSelectOptionContext {
        value,
        values,
        multiple,
        options: typeahead_options,
        open,
        on_open_change,
        read_only,
        align_item_with_trigger,
        align_offset,
        scroll_up_visible,
        scroll_down_visible,
        typeahead_buffer,
        typeahead_generation,
        on_change,
        on_change_values,
    };
    #[cfg(target_arch = "wasm32")]
    let open_effect_trigger_id: Signal<String> =
        HookContext::use_hook(|| Signal::create(trigger_id.clone()));
    #[cfg(target_arch = "wasm32")]
    open_effect_trigger_id.set(trigger_id.clone());
    #[cfg(target_arch = "wasm32")]
    let open_effect: EuvSelectOpenEffect = EuvSelectOpenEffect {
        trigger_id: open_effect_trigger_id,
        panel_position_style,
        viewport_position_style,
        panel_gap,
        open,
        value: option_context.value,
        values: option_context.values,
        multiple,
        align_item_with_trigger,
        align_offset,
        scroll_up_visible,
        scroll_down_visible,
    };
    #[cfg(not(target_arch = "wasm32"))]
    let open_effect: EuvSelectOpenEffect = EuvSelectOpenEffect;
    #[cfg(target_arch = "wasm32")]
    use_select_outside_click_listener(option_context.clone(), open_effect_trigger_id);
    #[cfg(target_arch = "wasm32")]
    use_select_scroll_position_listener(open_effect);
    watch!(open, |is_open: bool| {
        if is_open {
            schedule_select_open_effect(open_effect);
        } else {
            panel_position_style.set(String::from(SELECT_PANEL_STYLE_HIDDEN));
            viewport_position_style.set(String::new());
            align_offset.set(String::new());
            scroll_up_visible.set(false);
            scroll_down_visible.set(false);
            clear_select_typeahead(&typeahead_buffer, &typeahead_generation);
        }
    });
    let trigger_keydown: Option<Rc<dyn Fn(Event)>> =
        select_trigger_keyboard(option_context.clone());
    let scroll_handler: Option<Rc<dyn Fn(Event)>> =
        select_panel_scroll_handler(scroll_up_visible, scroll_down_visible);
    let trigger_class: &Css = c_euv_select_trigger();

    let invalid_trigger_class: Css = if invalid {
        c_euv_select_trigger_invalid().clone()
    } else {
        Css::default()
    };
    let size_attr: &'static str = match size {
        EuvSelectSize::Default => SELECT_SIZE_DEFAULT,
        EuvSelectSize::Sm => SELECT_SIZE_SM,
    };
    let small_trigger_class: Css = match size {
        EuvSelectSize::Default => Css::default(),
        EuvSelectSize::Sm => c_euv_select_trigger_size_sm().clone(),
    };
    let panel_class: &Css = c_euv_select_panel();
    let panel_open_class: &Css = c_euv_select_panel_open();
    let panel_closed_class: &Css = c_euv_select_panel_closed();

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
                        &selected_values,
                        active_value.as_deref(),
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
                        let is_selected: bool = selected_values
                            .iter()
                            .any(|selected_value: &String| selected_value == option.value);
                        select_option_row(
                            option,
                            is_selected,
                            active_value.as_deref() == Some(option.value),
                            option_context.clone(),
                        )
                    }
                },
            )
            .collect()
    };
    html! {
        div {
            "data-slot": "select"
            id: root_id.clone()
            "data-select-panel-id": panel_id.clone()
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
            "data-readonly": if read_only {
                SELECT_ATTRIBUTE_ON
            } else {
                SELECT_ATTRIBUTE_OFF
            }
            "data-multiple": if multiple {
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
                "data-size": size_attr
                id: trigger_id
                role: "combobox"
                "aria-haspopup": "listbox"
                "aria-controls": listbox_id.clone()
                "aria-label": aria_label.clone()
                "aria-describedby": aria_describedby
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
                "aria-readonly": if read_only {
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

                class: small_trigger_class
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
                    "data-slot": "select-icon"
                    "aria-hidden": "true"
                    class: c_euv_select_caret()
                    inner_html: crate::component::icon::ICON_CHEVRON_DOWN_MARKUP
                }
            }
            form_input
            portal {
                target: SELECT_PORTAL_TARGET
                div {
                    "data-slot": "select-content"
                    id: panel_id
                    "data-select-root-id": root_id
                    style: panel_position_style
                    role: "presentation"
                    class: panel_class
                    class: if { open.get() } {
                        panel_open_class
                    } else {
                        panel_closed_class
                    }

                    class: aligned_panel_class
                    if { scroll_up_visible.get() } {
                        euv_select_scroll_up_button {}
                    }
                    div {
                        "data-slot": "select-viewport"
                        id: listbox_id
                        role: "listbox"
                        "aria-multiselectable": if multiple {
                            SELECT_ATTRIBUTE_ON
                        } else {
                            SELECT_ATTRIBUTE_OFF
                        }
                        class: c_euv_select_viewport()
                        style: viewport_position_style
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
}

/// Renders the native form control paired with a custom Select trigger.
///
/// # Arguments
///
/// - `EuvSelectFormInputProps` - The values and form behavior for the native control.
///
/// # Returns
///
/// - `VirtualNode` - The visually hidden native text input or an empty node.
fn select_form_input(props: EuvSelectFormInputProps) -> VirtualNode {
    let EuvSelectFormInputProps {
        name,
        value,
        values,
        multiple,
        required,
        disabled,
        form,
        auto_complete,
    }: EuvSelectFormInputProps = props;
    if multiple {
        let mut inputs: Vec<VirtualNode> = Vec::with_capacity(values.len() + 1);
        if required {
            let validation_value: String = if values.is_empty() {
                String::new()
            } else {
                String::from(SELECT_FORM_VALIDATION_VALUE_SELECTED)
            };
            inputs.push(select_validation_form_input(
                form,
                validation_value,
                disabled,
            ));
        }
        if !name.is_empty() {
            for selected_value in values {
                inputs.push(select_hidden_form_input(
                    form,
                    name,
                    selected_value,
                    disabled,
                ));
            }
        }
        return if inputs.is_empty() {
            VirtualNode::Empty
        } else {
            VirtualNode::Fragment(inputs)
        };
    }
    if name.is_empty() && !required {
        return VirtualNode::Empty;
    }
    select_text_form_input(form, name, value, required, disabled, auto_complete)
}

/// Renders the required validation proxy for a multiple Select.
///
/// # Arguments
///
/// - `&'static str` - The external form id; empty means ancestor association.
/// - `String` - The validation value; empty means no selection.
/// - `bool` - Whether the validation input is disabled.
///
/// # Returns
///
/// - `VirtualNode` - A name-less, visually hidden required input.
fn select_validation_form_input(form: &'static str, value: String, disabled: bool) -> VirtualNode {
    if form.is_empty() {
        html! {
            input {
                r#type: "text"
                value: value
                required: boolean_attr(true)
                disabled: boolean_attr(disabled)
                "aria-hidden": "true"
                tabindex: "-1"
                class: c_euv_select_form_input()
                autocomplete: "off"
            }
        }
    } else {
        html! {
            input {
                form: form
                r#type: "text"
                value: value
                required: boolean_attr(true)
                disabled: boolean_attr(disabled)
                "aria-hidden": "true"
                tabindex: "-1"
                class: c_euv_select_form_input()
                autocomplete: "off"
            }
        }
    }
}

/// Renders the visually hidden text input that submits or validates Select state.
///
/// # Arguments
///
/// - `&'static str` - The external form id; empty means ancestor association.
/// - `&'static str` - The submitted field name; empty means validation-only.
/// - `String` - The submitted or validation value.
/// - `bool` - Whether the input participates in required validation.
/// - `bool` - Whether the input is disabled.
/// - `&'static str` - The browser autocomplete hint; empty defaults to `on`.
///
/// # Returns
///
/// - `VirtualNode` - The visually hidden native text input.
fn select_text_form_input(
    form: &'static str,
    name: &'static str,
    value: String,
    required: bool,
    disabled: bool,
    auto_complete: &'static str,
) -> VirtualNode {
    let autocomplete_value: &'static str = if auto_complete.is_empty() {
        "on"
    } else {
        auto_complete
    };
    if form.is_empty() {
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
                autocomplete: autocomplete_value
            }
        }
    } else {
        html! {
            input {
                form: form
                r#type: "text"
                name: name
                value: value
                required: boolean_attr(required)
                disabled: boolean_attr(disabled)
                "aria-hidden": "true"
                tabindex: "-1"
                class: c_euv_select_form_input()
                autocomplete: autocomplete_value
            }
        }
    }
}

/// Renders one visually hidden submitted value for a multiple Select.
///
/// # Arguments
///
/// - `&'static str` - The external form id; empty means ancestor association.
/// - `&'static str` - The submitted field name.
/// - `String` - The selected option value.
/// - `bool` - Whether the input is disabled.
///
/// # Returns
///
/// - `VirtualNode` - The visually hidden native input.
fn select_hidden_form_input(
    form: &'static str,
    name: &'static str,
    value: String,
    disabled: bool,
) -> VirtualNode {
    if form.is_empty() {
        html! {
            input {
                r#type: "hidden"
                name: name
                value: value
                disabled: boolean_attr(disabled)
                "aria-hidden": "true"
                tabindex: "-1"
                class: c_euv_select_form_input()
            }
        }
    } else {
        html! {
            input {
                form: form
                r#type: "hidden"
                name: name
                value: value
                disabled: boolean_attr(disabled)
                "aria-hidden": "true"
                tabindex: "-1"
                class: c_euv_select_form_input()
            }
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
    use_euv_select_open_with_default(false)
}

/// Obtains the component-owned open signal with its configured initial state.
///
/// # Arguments
///
/// - `bool` - Whether the panel starts open when the caller does not control it.
///
/// # Returns
///
/// - `Signal<bool>` - The open-state signal retained by the current hook context.
fn use_euv_select_open_with_default(default_open: bool) -> Signal<bool> {
    HookContext::use_hook(move || Signal::create(default_open))
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

/// Holds the fixed-position style for the Select's portaled panel.
///
/// # Returns
///
/// - `Signal<String>` - The inline panel position, hidden until the trigger is measured.
fn use_euv_select_panel_position() -> Signal<String> {
    HookContext::use_hook(|| Signal::create(String::from(SELECT_PANEL_STYLE_HIDDEN)))
}

/// Holds a measured height cap for a Select near the viewport edge.
///
/// # Returns
///
/// - `Signal<String>` - The scroll viewport's inline maximum-height override.
fn use_euv_select_viewport_position() -> Signal<String> {
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

/// Resolves the portaled panel owned by a Select root.
///
/// # Arguments
///
/// - `&Element` - The Select root carrying the panel id.
///
/// # Returns
///
/// - `Option<Element>` - The content panel when it is mounted.
fn select_panel_for_root(root: &Element) -> Option<Element> {
    let panel_id: String = root.get_attribute(SELECT_PANEL_ID_ATTRIBUTE)?;
    root.owner_document()?.get_element_by_id(&panel_id)
}

/// Resolves the owning Select root from either its trigger or portaled panel.
///
/// # Arguments
///
/// - `&Element` - The trigger or panel descendant that received an event.
///
/// # Returns
///
/// - `Option<Element>` - The Select root that owns the event target.
fn select_root_for_event_target(target: &Element) -> Option<Element> {
    if let Some(root) = target.closest(SELECT_ROOT_SELECTOR).ok().flatten() {
        return Some(root);
    }
    let panel: Element = target.closest(SELECT_CONTENT_SELECTOR).ok().flatten()?;
    let root_id: String = panel.get_attribute(SELECT_ROOT_ID_ATTRIBUTE)?;
    panel.owner_document()?.get_element_by_id(&root_id)
}

/// Moves keyboard focus to the option row matching a Select value.
///
/// # Arguments
///
/// - `&Element` - The Select root containing the option rows.
/// - `&str` - The option value to focus.
fn select_focus_option(root: &Element, option_value: &str) {
    let Some(panel) = select_panel_for_root(root) else {
        return;
    };
    let Ok(nodes) = panel.query_selector_all(SELECT_OPTION_SELECTOR) else {
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
/// - `&[String]` - The currently selected values.
/// - `Option<&str>` - The option that owns the listbox tab stop.
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
    selected_values: &[String],
    tab_stop_value: Option<&str>,
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
        let is_selected: bool = selected_values
            .iter()
            .any(|selected_value: &String| selected_value == option.value);
        let is_tab_stop: bool = tab_stop_value == Some(option.value);
        group_items.push(select_option_row(
            option,
            is_selected,
            is_tab_stop,
            context.clone(),
        ));
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
/// - `bool` - Whether the option owns the listbox tab stop.
/// - `EuvSelectOptionContext` - The reactive selection, panel and scroll state shared by the row.
///
/// # Returns
///
/// - `VirtualNode` - The option row virtual DOM tree.
fn select_option_row(
    option: EuvSelectOption,
    is_selected: bool,
    is_tab_stop: bool,
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
    let tabindex: &str = if is_tab_stop { "0" } else { "-1" };
    let default_option_class: &Css = c_euv_select_option();
    let state_option_class: &Css = if disabled {
        c_euv_select_option_disabled()
    } else if is_selected {
        c_euv_select_option_active()
    } else {
        c_euv_select_option()
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

            disabled: boolean_attr(disabled)
            onclick: handler
            onkeydown: keydown_handler
            option.label
            span {
                "data-slot": "select-item-indicator"
                aria-hidden: "true"
                class: c_euv_select_option_indicator()
                inner_html: crate::component::icon::ICON_CHECK_MARKUP
            }
        }
    }
}

/// Returns the single or first selected value used for focus and alignment.
///
/// # Arguments
///
/// - `&EuvSelectOptionContext` - The active selection context.
///
/// # Returns
///
/// - `String` - The current value used to focus an option.
fn select_current_value(context: &EuvSelectOptionContext) -> String {
    if context.multiple {
        context.values.get().into_iter().next().unwrap_or_default()
    } else {
        context.value.get()
    }
}

/// Applies a picked value to the active single- or multiple-selection signal.
///
/// # Arguments
///
/// - `&'static str` - The option value that was activated.
/// - `&EuvSelectOptionContext` - The selection signals and change callbacks.
fn select_update_value(option_value: &'static str, context: &EuvSelectOptionContext) {
    if context.read_only {
        return;
    }
    if context.multiple {
        let mut next_values: Vec<String> = context.values.get();
        if next_values
            .iter()
            .any(|selected_value: &String| selected_value == option_value)
        {
            next_values.retain(|selected_value: &String| selected_value != option_value);
        } else {
            next_values.push(String::from(option_value));
        }
        context.values.set(next_values.clone());
        if let Some(callback) = &context.on_change_values {
            callback(next_values);
        }
    } else {
        context.value.set(String::from(option_value));
        if let Some(callback) = &context.on_change {
            callback(option_value);
        }
    }
}

/// Re-aligns and refreshes scroll affordances while a multiple Select stays open.
///
/// # Arguments
///
/// - `&Element` - The Select root containing the trigger and option panel.
/// - `&EuvSelectOptionContext` - The selection, alignment and scroll state.
fn select_refresh_open_panel(root: &Element, context: &EuvSelectOptionContext) {
    let selected_value: String = select_current_value(context);
    schedule_select_alignment(
        root.clone(),
        selected_value,
        context.align_item_with_trigger,
        context.align_offset,
    );
    schedule_select_scroll_affordance(
        root.clone(),
        context.scroll_up_visible,
        context.scroll_down_visible,
    );
}

/// Keeps the portaled panel attached to its trigger while the page scrolls.
///
/// The document listener uses capture so scrolls in nested ancestors are also
/// observed. It is attached only while this Select is open and removed on
/// close or unmount.
///
/// # Arguments
///
/// - `EuvSelectOpenEffect` - The trigger, panel and open-state signals.
#[cfg(target_arch = "wasm32")]
fn use_select_scroll_position_listener(effect: EuvSelectOpenEffect) {
    let listener: Rc<RefCell<Option<Closure<dyn FnMut(Event)>>>> =
        HookContext::use_hook(|| Rc::new(RefCell::new(None)));
    let mounted: Rc<Cell<bool>> = HookContext::use_hook(|| Rc::new(Cell::new(true)));
    let scheduled: Rc<Cell<bool>> = HookContext::use_hook(|| Rc::new(Cell::new(false)));
    let document: Option<Document> = window().and_then(|window: Window| window.document());
    let window_value: Option<Window> = window();
    let open: Signal<bool> = effect.open;
    let listener_for_subscription: Rc<RefCell<Option<Closure<dyn FnMut(Event)>>>> =
        listener.clone();
    let mounted_for_subscription: Rc<Cell<bool>> = mounted.clone();
    let scheduled_for_subscription: Rc<Cell<bool>> = scheduled.clone();
    let document_for_subscription: Option<Document> = document.clone();
    let window_for_subscription: Option<Window> = window_value.clone();
    let effect_for_subscription: EuvSelectOpenEffect = effect;
    let subscription_id: usize = HookContext::use_hook(move || {
        let callback_listener: Rc<RefCell<Option<Closure<dyn FnMut(Event)>>>> =
            listener_for_subscription.clone();
        let callback_mounted: Rc<Cell<bool>> = mounted_for_subscription.clone();
        let callback_scheduled: Rc<Cell<bool>> = scheduled_for_subscription.clone();
        let callback_document: Option<Document> = document_for_subscription.clone();
        let callback_window: Option<Window> = window_for_subscription.clone();
        let callback_effect: EuvSelectOpenEffect = effect_for_subscription;
        let callback_open: Signal<bool> = open;
        let subscription: usize = open.subscribe(move || {
            update_select_scroll_position_listener(
                callback_document.clone(),
                callback_window.clone(),
                &callback_listener,
                callback_effect,
                callback_scheduled.clone(),
                callback_mounted.clone(),
                callback_open.get(),
            );
        });
        update_select_scroll_position_listener(
            document_for_subscription,
            window_for_subscription,
            &listener_for_subscription,
            effect_for_subscription,
            scheduled_for_subscription,
            mounted_for_subscription,
            open.get(),
        );
        subscription
    });
    App::use_cleanup({
        let cleanup_document: Option<Document> = document;
        let cleanup_window: Option<Window> = window_value;
        let cleanup_listener: Rc<RefCell<Option<Closure<dyn FnMut(Event)>>>> = listener;
        let cleanup_mounted: Rc<Cell<bool>> = mounted;
        let cleanup_scheduled: Rc<Cell<bool>> = scheduled;
        move || {
            cleanup_mounted.set(false);
            cleanup_scheduled.set(false);
            open.unsubscribe(subscription_id);
            detach_select_scroll_position_listener(
                cleanup_document,
                cleanup_window,
                &cleanup_listener,
            );
        }
    });
}

/// Attaches or detaches the scroll and resize observers for one Select.
///
/// # Arguments
///
/// - `Option<Document>` - The document that captures nested scroll events.
/// - `Option<Window>` - The window that reports viewport resizing.
/// - `&Rc<RefCell<Option<Closure<dyn FnMut(Event)>>>>` - The listener slot.
/// - `EuvSelectOpenEffect` - The panel position state.
/// - `Rc<Cell<bool>>` - Whether a reposition frame is already queued.
/// - `Rc<Cell<bool>>` - Whether the component is still mounted.
/// - `bool` - Whether the Select is open.
#[cfg(target_arch = "wasm32")]
fn update_select_scroll_position_listener(
    document: Option<Document>,
    window: Option<Window>,
    listener: &Rc<RefCell<Option<Closure<dyn FnMut(Event)>>>>,
    effect: EuvSelectOpenEffect,
    scheduled: Rc<Cell<bool>>,
    mounted: Rc<Cell<bool>>,
    should_attach: bool,
) {
    if !should_attach || !mounted.get() {
        scheduled.set(false);
        detach_select_scroll_position_listener(document, window, listener);
        return;
    }
    if listener.borrow().is_some() {
        return;
    }
    let (Some(document), Some(window)) = (document, window) else {
        return;
    };
    let callback_effect: EuvSelectOpenEffect = effect;
    let callback_open: Signal<bool> = effect.open;
    let callback_window: Window = window.clone();
    let callback_scheduled: Rc<Cell<bool>> = scheduled.clone();
    let callback_mounted: Rc<Cell<bool>> = mounted.clone();
    let callback: Closure<dyn FnMut(Event)> = Closure::wrap(Box::new(move |event: Event| {
        if !callback_open.get() || !callback_mounted.get() {
            return;
        }
        let target_is_select_panel: bool = event
            .target()
            .and_then(|target: EventTarget| target.dyn_into::<Element>().ok())
            .and_then(|target: Element| target.closest(SELECT_CONTENT_SELECTOR).ok().flatten())
            .is_some();
        if target_is_select_panel {
            return;
        }
        schedule_select_reposition(
            callback_effect,
            callback_window.clone(),
            callback_scheduled.clone(),
            callback_mounted.clone(),
        );
    }) as Box<dyn FnMut(Event)>);
    let scroll_result: Result<(), JsValue> = document.add_event_listener_with_callback_and_bool(
        SELECT_SCROLL_EVENT,
        callback.as_ref().unchecked_ref(),
        true,
    );
    let resize_result: Result<(), JsValue> = window
        .add_event_listener_with_callback(SELECT_RESIZE_EVENT, callback.as_ref().unchecked_ref());
    if scroll_result.is_ok() && resize_result.is_ok() {
        *listener.borrow_mut() = Some(callback);
        return;
    }
    if scroll_result.is_ok() {
        let _: Result<(), JsValue> = document.remove_event_listener_with_callback_and_bool(
            SELECT_SCROLL_EVENT,
            callback.as_ref().unchecked_ref(),
            true,
        );
    }
    if resize_result.is_ok() {
        let _: Result<(), JsValue> = window.remove_event_listener_with_callback(
            SELECT_RESIZE_EVENT,
            callback.as_ref().unchecked_ref(),
        );
    }
}

/// Removes the scroll and resize observers for one Select.
///
/// # Arguments
///
/// - `Option<Document>` - The document that captured scroll events.
/// - `Option<Window>` - The window that reported viewport resizing.
/// - `&Rc<RefCell<Option<Closure<dyn FnMut(Event)>>>>` - The listener slot.
#[cfg(target_arch = "wasm32")]
fn detach_select_scroll_position_listener(
    document: Option<Document>,
    window: Option<Window>,
    listener: &Rc<RefCell<Option<Closure<dyn FnMut(Event)>>>>,
) {
    let callback: Option<Closure<dyn FnMut(Event)>> = listener.borrow_mut().take();
    let Some(callback) = callback else {
        return;
    };
    if let Some(document) = document {
        let _: Result<(), JsValue> = document.remove_event_listener_with_callback_and_bool(
            SELECT_SCROLL_EVENT,
            callback.as_ref().unchecked_ref(),
            true,
        );
    }
    if let Some(window) = window {
        let _: Result<(), JsValue> = window.remove_event_listener_with_callback(
            SELECT_RESIZE_EVENT,
            callback.as_ref().unchecked_ref(),
        );
    }
}

/// Schedules a throttled panel reposition after scroll or viewport resize.
///
/// # Arguments
///
/// - `EuvSelectOpenEffect` - The trigger, panel and open-state signals.
/// - `Window` - The browser window scheduling animation frames.
/// - `Rc<Cell<bool>>` - Whether a reposition frame is already queued.
/// - `Rc<Cell<bool>>` - Whether the component is still mounted.
#[cfg(target_arch = "wasm32")]
fn schedule_select_reposition(
    effect: EuvSelectOpenEffect,
    window: Window,
    scheduled: Rc<Cell<bool>>,
    mounted: Rc<Cell<bool>>,
) {
    if scheduled.replace(true) {
        return;
    }
    let initial_effect: EuvSelectOpenEffect = effect;
    let initial_window: Window = window.clone();
    let initial_scheduled: Rc<Cell<bool>> = scheduled.clone();
    let initial_mounted: Rc<Cell<bool>> = mounted.clone();
    let initial_callback: Function = Closure::once_into_js(move || {
        if !initial_mounted.get() || !initial_effect.open.get() {
            initial_scheduled.set(false);
            return;
        }
        let Some(document) = initial_window.document() else {
            initial_scheduled.set(false);
            return;
        };
        let Some(trigger) = document.get_element_by_id(&initial_effect.trigger_id.get()) else {
            initial_scheduled.set(false);
            return;
        };
        let trigger_rect: DomRect = trigger.get_bounding_client_rect();
        let panel_top: f64 = trigger_rect.bottom() + initial_effect.panel_gap.get();
        initial_effect.viewport_position_style.set(String::new());
        initial_effect.panel_position_style.set(format!(
            "position:fixed;left:{:.2}px;top:{:.2}px;right:auto;width:{:.2}px;margin-top:0px;",
            trigger_rect.left(),
            panel_top,
            trigger_rect.width(),
        ));
        let final_effect: EuvSelectOpenEffect = initial_effect;
        let final_window: Window = initial_window.clone();
        let final_scheduled: Rc<Cell<bool>> = initial_scheduled.clone();
        let final_mounted: Rc<Cell<bool>> = initial_mounted.clone();
        let final_callback: Function = Closure::once_into_js(move || {
            if final_mounted.get() && final_effect.open.get() {
                finalize_select_open_position(final_effect, final_window, false);
            }
            final_scheduled.set(false);
        })
        .unchecked_into();
        let result: Result<i32, JsValue> = initial_window.request_animation_frame(&final_callback);
        if result.is_err() {
            initial_scheduled.set(false);
        }
    })
    .unchecked_into();
    let result: Result<i32, JsValue> = window.request_animation_frame(&initial_callback);
    if result.is_err() {
        scheduled.set(false);
    }
}

/// Schedules fixed positioning, collision handling, focus and scrolling after open.
///
/// # Arguments
///
/// - `EuvSelectOpenEffect` - The trigger, selection, alignment and scroll signals.
#[cfg(target_arch = "wasm32")]
fn schedule_select_open_effect(effect: EuvSelectOpenEffect) {
    let Some(window) = window() else {
        return;
    };
    let effect_window: Window = window.clone();
    let callback: Function = Closure::once_into_js(move || {
        if !effect.open.get() {
            return;
        }
        let Some(document) = effect_window.document() else {
            return;
        };
        let Some(trigger) = document.get_element_by_id(&effect.trigger_id.get()) else {
            return;
        };
        let Some(root) = trigger.closest(SELECT_ROOT_SELECTOR).ok().flatten() else {
            return;
        };
        let Some(_panel) = select_panel_for_root(&root) else {
            return;
        };
        let trigger_rect: DomRect = trigger.get_bounding_client_rect();
        effect.panel_position_style.set(format!(
            "position:fixed;left:{:.2}px;top:{:.2}px;right:auto;width:{:.2}px;visibility:hidden;",
            trigger_rect.left(),
            trigger_rect.bottom(),
            trigger_rect.width(),
        ));
        let position_effect: EuvSelectOpenEffect = effect;
        let position_window: Window = effect_window.clone();
        let position_callback: Function = Closure::once_into_js(move || {
            finalize_select_open_position(position_effect, position_window, true);
        })
        .unchecked_into();
        let _: Result<i32, JsValue> = effect_window.request_animation_frame(&position_callback);
    })
    .unchecked_into();
    let _: Result<i32, JsValue> = window.request_animation_frame(&callback);
}

/// Finalizes the portaled panel position after its anchor width has been applied.
///
/// # Arguments
///
/// - `EuvSelectOpenEffect` - The trigger, selection, panel and layout signals.
/// - `Window` - The browser window used to measure available vertical space.
/// - `bool` - Whether this is the initial open and should focus the selection.
#[cfg(target_arch = "wasm32")]
fn finalize_select_open_position(
    effect: EuvSelectOpenEffect,
    window: Window,
    focus_after_position: bool,
) {
    if !effect.open.get() {
        return;
    }
    let Some(document) = window.document() else {
        return;
    };
    let Some(trigger) = document.get_element_by_id(&effect.trigger_id.get()) else {
        return;
    };
    let Some(root) = trigger.closest(SELECT_ROOT_SELECTOR).ok().flatten() else {
        return;
    };
    let Some(panel) = select_panel_for_root(&root) else {
        return;
    };
    let Some(viewport) = panel
        .query_selector(SELECT_VIEWPORT_SELECTOR)
        .ok()
        .flatten()
        .and_then(|element: Element| element.dyn_into::<HtmlElement>().ok())
    else {
        return;
    };
    let trigger_rect: DomRect = trigger.get_bounding_client_rect();
    let panel_rect: DomRect = panel.get_bounding_client_rect();
    let viewport_height: f64 = window
        .inner_height()
        .ok()
        .and_then(|height: JsValue| height.as_f64())
        .unwrap_or_default();
    let panel_gap: f64 = if focus_after_position {
        (panel_rect.top() - trigger_rect.bottom()).max(0.0)
    } else {
        effect.panel_gap.get()
    };
    if focus_after_position {
        effect.panel_gap.set(panel_gap);
    }
    let (position_style, viewport_style): (String, String) = select_panel_position_style(
        &trigger_rect,
        &panel_rect,
        viewport_height,
        f64::from(viewport.client_height()),
        panel_gap,
    );
    effect.panel_position_style.set(position_style);
    effect.viewport_position_style.set(viewport_style);
    let selected_value: Option<String> = if focus_after_position {
        let value: String = if effect.multiple {
            effect.values.get().into_iter().next().unwrap_or_default()
        } else {
            effect.value.get()
        };
        schedule_select_alignment(
            root.clone(),
            value.clone(),
            effect.align_item_with_trigger,
            effect.align_offset,
        );
        Some(value)
    } else {
        None
    };
    schedule_select_scroll_affordance(
        root.clone(),
        effect.scroll_up_visible,
        effect.scroll_down_visible,
    );
    if let Some(selected_value) = selected_value {
        schedule_select_focus(root, selected_value);
    }
}

/// Chooses a side and viewport height that keep the Select panel inside the viewport.
///
/// # Arguments
///
/// - `&DomRect` - The trigger's viewport rectangle.
/// - `&DomRect` - The initially measured panel rectangle.
/// - `f64` - The browser viewport height.
/// - `f64` - The scroll viewport's current client height.
/// - `f64` - The measured trigger-to-panel gap.
///
/// # Returns
///
/// - `(String, String)` - The panel position and viewport height styles.
#[cfg(target_arch = "wasm32")]
fn select_panel_position_style(
    trigger_rect: &DomRect,
    panel_rect: &DomRect,
    viewport_height: f64,
    viewport_client_height: f64,
    gap: f64,
) -> (String, String) {
    let max_side_space: f64 =
        (viewport_height - SELECT_ALIGNMENT_VIEWPORT_GUTTER_PX * 2.0).max(0.0);
    let available_below: f64 =
        (viewport_height - panel_rect.top() - SELECT_ALIGNMENT_VIEWPORT_GUTTER_PX)
            .max(0.0)
            .min(max_side_space);
    let available_above: f64 = (trigger_rect.top() - gap - SELECT_ALIGNMENT_VIEWPORT_GUTTER_PX)
        .max(0.0)
        .min(max_side_space);
    let open_above: bool =
        panel_rect.height() > available_below && available_above > available_below;
    let available_side: f64 = if open_above {
        available_above
    } else {
        available_below
    };
    let available_viewport_height: f64 =
        (available_side - 2.0).max(0.0).min(viewport_client_height);
    let final_panel_height: f64 = (available_viewport_height + 2.0).min(panel_rect.height());
    let desired_top: f64 = if open_above {
        trigger_rect.top() - gap - final_panel_height
    } else {
        trigger_rect.bottom() + gap
    };
    let maximum_top: f64 =
        (viewport_height - SELECT_ALIGNMENT_VIEWPORT_GUTTER_PX - final_panel_height)
            .max(SELECT_ALIGNMENT_VIEWPORT_GUTTER_PX);
    let top: f64 = desired_top.clamp(SELECT_ALIGNMENT_VIEWPORT_GUTTER_PX, maximum_top);
    let panel_style: String = format!(
        "position:fixed;left:{:.2}px;top:{:.2}px;right:auto;width:{:.2}px;margin-top:0px;",
        trigger_rect.left(),
        top,
        trigger_rect.width(),
    );
    let viewport_style: String = if available_viewport_height < viewport_client_height {
        format!("max-height:{available_viewport_height:.2}px;")
    } else {
        String::new()
    };
    (panel_style, viewport_style)
}

/// Body of the `schedule_select_open_effect` free function.
///
/// # Arguments
///
/// - `EuvSelectOpenEffect` - A `EuvSelectOpenEffect` parameter.
#[cfg(not(target_arch = "wasm32"))]
fn schedule_select_open_effect(_effect: EuvSelectOpenEffect) {}

/// Requests an open-state change and notifies the caller once if it changes.
///
/// # Arguments
///
/// - `&EuvSelectOptionContext` - The open signal and callback.
/// - `bool` - The requested open state.
fn select_set_open(context: &EuvSelectOptionContext, next_open: bool) {
    if context.open.get() == next_open {
        return;
    }
    context.open.set(next_open);
    if let Some(callback) = &context.on_open_change {
        callback(next_open);
    }
}

/// Registers a temporary document listener while a portaled Select is open.
///
/// # Arguments
///
/// - `EuvSelectOptionContext` - The current Select state and callbacks.
/// - `Signal<String>` - The trigger id used to resolve the Select DOM regions.
#[cfg(target_arch = "wasm32")]
fn use_select_outside_click_listener(context: EuvSelectOptionContext, trigger_id: Signal<String>) {
    let initial_context: EuvSelectOptionContext = context.clone();
    let context_ref: Rc<RefCell<EuvSelectOptionContext>> =
        HookContext::use_hook(move || Rc::new(RefCell::new(initial_context)));
    *context_ref.borrow_mut() = context;
    let listener: Rc<RefCell<Option<Closure<dyn FnMut(Event)>>>> =
        HookContext::use_hook(|| Rc::new(RefCell::new(None)));
    let mounted: Rc<Cell<bool>> = HookContext::use_hook(|| Rc::new(Cell::new(true)));
    let document: Option<Document> = window().and_then(|window: Window| window.document());
    let open: Signal<bool> = context_ref.borrow().open;
    let context_for_subscription: Rc<RefCell<EuvSelectOptionContext>> = context_ref.clone();
    let listener_for_subscription: Rc<RefCell<Option<Closure<dyn FnMut(Event)>>>> =
        listener.clone();
    let document_for_subscription: Option<Document> = document.clone();
    let mounted_for_subscription: Rc<Cell<bool>> = mounted.clone();
    let trigger_id_for_subscription: Signal<String> = trigger_id;
    let subscription_id: usize = HookContext::use_hook(move || {
        let callback_context: Rc<RefCell<EuvSelectOptionContext>> =
            context_for_subscription.clone();
        let callback_listener: Rc<RefCell<Option<Closure<dyn FnMut(Event)>>>> =
            listener_for_subscription.clone();
        let callback_document: Option<Document> = document_for_subscription.clone();
        let callback_mounted: Rc<Cell<bool>> = mounted_for_subscription.clone();
        let callback_trigger_id: Signal<String> = trigger_id_for_subscription;
        let callback_open: Signal<bool> = open;
        let subscription: usize = open.subscribe(move || {
            let should_attach: bool = callback_open.get();
            update_select_outside_click_listener(
                callback_document.clone(),
                &callback_listener,
                callback_context.clone(),
                callback_trigger_id,
                should_attach,
                callback_mounted.clone(),
            );
        });
        update_select_outside_click_listener(
            document_for_subscription,
            &listener_for_subscription,
            context_for_subscription,
            trigger_id_for_subscription,
            open.get(),
            mounted_for_subscription,
        );
        subscription
    });
    App::use_cleanup({
        let cleanup_document: Option<Document> = document;
        let cleanup_listener: Rc<RefCell<Option<Closure<dyn FnMut(Event)>>>> = listener;
        let cleanup_mounted: Rc<Cell<bool>> = mounted;
        move || {
            cleanup_mounted.set(false);
            open.unsubscribe(subscription_id);
            detach_select_outside_click_listener(cleanup_document, &cleanup_listener);
        }
    });
}

/// Attaches or schedules removal of the outside-click listener for a Select.
///
/// # Arguments
///
/// - `Option<Document>` - The document that owns the Select and panel.
/// - `&Rc<RefCell<Option<Closure<dyn FnMut(Event)>>>>` - The listener slot.
/// - `Rc<RefCell<EuvSelectOptionContext>>` - The latest Select state.
/// - `Signal<String>` - The trigger id.
/// - `bool` - Whether the Select is currently open.
/// - `Rc<Cell<bool>>` - Whether the component is still mounted.
#[cfg(target_arch = "wasm32")]
fn update_select_outside_click_listener(
    document: Option<Document>,
    listener: &Rc<RefCell<Option<Closure<dyn FnMut(Event)>>>>,
    context: Rc<RefCell<EuvSelectOptionContext>>,
    trigger_id: Signal<String>,
    should_attach: bool,
    mounted: Rc<Cell<bool>>,
) {
    if !should_attach {
        if listener.borrow().is_none() {
            return;
        }
        let Some(window) = window() else {
            detach_select_outside_click_listener(document, listener);
            return;
        };
        let callback_document: Option<Document> = document;
        let callback_listener: Rc<RefCell<Option<Closure<dyn FnMut(Event)>>>> = listener.clone();
        let callback_context: Rc<RefCell<EuvSelectOptionContext>> = context.clone();
        let callback_mounted: Rc<Cell<bool>> = mounted.clone();
        let callback: Function = Closure::once_into_js(move || {
            if !callback_mounted.get() || !callback_context.borrow().open.get() {
                detach_select_outside_click_listener(callback_document, &callback_listener);
            }
        })
        .unchecked_into();
        let _: Result<i32, JsValue> = window.request_animation_frame(&callback);
        return;
    }
    if !mounted.get() || listener.borrow().is_some() {
        return;
    }
    let (Some(document), Some(window)) = (document, window()) else {
        return;
    };
    let callback_document: Document = document.clone();
    let callback_listener: Rc<RefCell<Option<Closure<dyn FnMut(Event)>>>> = listener.clone();
    let callback_context: Rc<RefCell<EuvSelectOptionContext>> = context.clone();
    let callback_mounted: Rc<Cell<bool>> = mounted.clone();
    let callback_open: Signal<bool> = context.borrow().open;
    let callback: Function = Closure::once_into_js(move || {
        if callback_mounted.get() && callback_open.get() {
            attach_select_outside_click_listener(
                callback_document,
                &callback_listener,
                callback_context,
                trigger_id,
            );
        }
    })
    .unchecked_into();
    let _: Result<i32, JsValue> = window.request_animation_frame(&callback);
}

/// Attaches the document click callback that dismisses a Select on outside presses.
///
/// # Arguments
///
/// - `Document` - The document to observe.
/// - `&Rc<RefCell<Option<Closure<dyn FnMut(Event)>>>>` - The listener slot.
/// - `Rc<RefCell<EuvSelectOptionContext>>` - The latest Select state.
/// - `Signal<String>` - The trigger id.
#[cfg(target_arch = "wasm32")]
fn attach_select_outside_click_listener(
    document: Document,
    listener: &Rc<RefCell<Option<Closure<dyn FnMut(Event)>>>>,
    context: Rc<RefCell<EuvSelectOptionContext>>,
    trigger_id: Signal<String>,
) {
    if listener.borrow().is_some() {
        return;
    }
    let callback_context: Rc<RefCell<EuvSelectOptionContext>> = context;
    let callback: Closure<dyn FnMut(Event)> = Closure::wrap(Box::new(move |event: Event| {
        let context: EuvSelectOptionContext = callback_context.borrow().clone();
        if !context.open.get() {
            return;
        }
        let id: String = trigger_id.get();
        let root_id: String = format!("{id}-root");
        let panel_id: String = format!("{id}-content");
        let target_is_inside: bool = event
            .target()
            .and_then(|target: EventTarget| target.dyn_into::<Element>().ok())
            .is_some_and(|target: Element| {
                target
                    .closest(SELECT_ROOT_SELECTOR)
                    .ok()
                    .flatten()
                    .is_some_and(|root: Element| {
                        root.get_attribute("id").as_deref() == Some(root_id.as_str())
                    })
                    || target
                        .closest(SELECT_CONTENT_SELECTOR)
                        .ok()
                        .flatten()
                        .is_some_and(|panel: Element| {
                            panel.get_attribute("id").as_deref() == Some(panel_id.as_str())
                        })
            });
        if !target_is_inside {
            select_set_open(&context, false);
        }
    }) as Box<dyn FnMut(Event)>);
    if document
        .add_event_listener_with_callback_and_bool(
            SELECT_OUTSIDE_CLICK_EVENT,
            callback.as_ref().unchecked_ref(),
            SELECT_OUTSIDE_CLICK_CAPTURE,
        )
        .is_ok()
    {
        *listener.borrow_mut() = Some(callback);
    }
}

/// Removes the document click listener and drops its callback closure.
///
/// # Arguments
///
/// - `Option<Document>` - The document that owns the listener.
/// - `&Rc<RefCell<Option<Closure<dyn FnMut(Event)>>>>` - The listener slot.
#[cfg(target_arch = "wasm32")]
fn detach_select_outside_click_listener(
    document: Option<Document>,
    listener: &Rc<RefCell<Option<Closure<dyn FnMut(Event)>>>>,
) {
    let callback: Option<Closure<dyn FnMut(Event)>> = listener.borrow_mut().take();
    let (Some(document), Some(callback)) = (document, callback) else {
        return;
    };
    let _: Result<(), JsValue> = document.remove_event_listener_with_callback_and_bool(
        SELECT_OUTSIDE_CLICK_EVENT,
        callback.as_ref().unchecked_ref(),
        SELECT_OUTSIDE_CLICK_CAPTURE,
    );
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
            let is_open: bool = context.open.get();
            let typeahead_enabled: bool = is_open || (!context.multiple && !context.read_only);
            let current_value: String = select_current_value(&context);
            if typeahead_enabled
                && let Some(option_value) = select_typeahead_value(
                    context.options.as_ref(),
                    &key,
                    &current_value,
                    context.typeahead_buffer,
                    context.typeahead_generation,
                )
            {
                keyboard_event.prevent_default();
                if is_open {
                    select_focus_option(&root, option_value);
                } else if option_value != current_value.as_str() {
                    // Base UI commits closed, single-select typeahead matches without opening.
                    select_update_value(option_value, &context);
                }
            }
            return;
        }
        if key != SELECT_KEY_ARROW_DOWN && key != SELECT_KEY_ARROW_UP {
            return;
        }
        keyboard_event.prevent_default();
        let was_open: bool = context.open.get();
        select_set_open(&context, true);
        clear_select_typeahead(&context.typeahead_buffer, &context.typeahead_generation);
        if was_open {
            let selected_value: String = select_current_value(&context);
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
        }
    }))
}

/// Focuses the selected option, falling back to the first enabled item.
///
/// # Arguments
///
/// - `&Element` - The Select root containing its options.
/// - `&str` - The selected option value, if one is set.
fn select_focus_selected_or_first(root: &Element, selected_value: &str) {
    let Some(panel) = select_panel_for_root(root) else {
        return;
    };
    let Ok(nodes) = panel.query_selector_all(SELECT_OPTION_SELECTOR) else {
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
        let Some(panel) = select_panel_for_root(&root) else {
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
        let viewport: Option<HtmlElement> = select_panel_for_root(&root)
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
        let Some(root) = select_root_for_event_target(&target) else {
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
            select_set_open(&context, false);
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
        if key == SELECT_KEY_TAB {
            select_set_open(&context, false);
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
        if key == SELECT_KEY_TAB {
            select_set_open(&context, false);
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
            select_update_value(value, &context);
            let keep_open: bool = context.multiple && !context.read_only;
            if keep_open {
                select_refresh_open_panel(&root, &context);
            } else {
                select_set_open(&context, false);
                context.align_offset.set(String::new());
                context.scroll_up_visible.set(false);
                context.scroll_down_visible.set(false);
                if let Some(trigger) = root
                    .query_selector(SELECT_TRIGGER_SELECTOR)
                    .ok()
                    .flatten()
                    .and_then(|trigger: Element| trigger.dyn_into::<HtmlElement>().ok())
                {
                    let _: Result<(), JsValue> = trigger.focus();
                }
            }
            clear_select_typeahead(&context.typeahead_buffer, &context.typeahead_generation);
            return;
        }
        let Some(panel) = select_panel_for_root(&root) else {
            return;
        };
        let Ok(nodes) = panel.query_selector_all(SELECT_OPTION_SELECTOR) else {
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
    Some(Rc::new(move |_event: Event| {
        if disabled {
            return;
        }
        let should_open: bool = !context.open.get();
        select_set_open(&context, should_open);
        clear_select_typeahead(&context.typeahead_buffer, &context.typeahead_generation);
    }))
}

/// Builds the click handler that picks one option.
///
/// The handler toggles multiple values without closing, or commits one value
/// and closes. A read-only Select closes without changing its selection.
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
        let root: Option<Element> = event
            .target()
            .and_then(|target: EventTarget| target.dyn_into::<Element>().ok())
            .and_then(|target: Element| select_root_for_event_target(&target));
        select_update_value(option_value, &context);
        let keep_open: bool = context.multiple && !context.read_only;
        if keep_open {
            if let Some(root) = root.as_ref() {
                select_refresh_open_panel(root, &context);
            }
        } else {
            select_set_open(&context, false);
            context.align_offset.set(String::new());
            context.scroll_up_visible.set(false);
            context.scroll_down_visible.set(false);
            if let Some(trigger) = root
                .as_ref()
                .and_then(|root: &Element| {
                    root.query_selector(SELECT_TRIGGER_SELECTOR).ok().flatten()
                })
                .and_then(|trigger: Element| trigger.dyn_into::<HtmlElement>().ok())
            {
                let _: Result<(), JsValue> = trigger.focus();
            }
        }
        clear_select_typeahead(&context.typeahead_buffer, &context.typeahead_generation);
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

/// Resolves and scrolls only the viewport belonging to the clicked edge control.
///
/// # Arguments
///
/// - `bool` - Whether the control moves toward the top of its viewport.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - The edge control click handler.
fn select_scroll_button_handler(is_upward: bool) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |event: Event| {
        event.prevent_default();
        let viewport: Option<HtmlElement> = event
            .target()
            .and_then(|target: EventTarget| target.dyn_into::<Element>().ok())
            .and_then(|target: Element| target.closest(SELECT_CONTENT_SELECTOR).ok().flatten())
            .and_then(|panel: Element| {
                panel
                    .query_selector(SELECT_VIEWPORT_SELECTOR)
                    .ok()
                    .flatten()
            })
            .and_then(|viewport: Element| viewport.dyn_into::<HtmlElement>().ok());
        if let Some(viewport) = viewport {
            let target: i32 = select_scroll_button_target(
                viewport.scroll_top(),
                viewport.client_height(),
                viewport.scroll_height(),
                is_upward,
            );
            viewport.set_scroll_top(target);
        }
    }))
}

/// Computes a bounded half-page movement for a Select viewport.
///
/// # Arguments
///
/// - `i32` - The current vertical scroll offset.
/// - `i32` - The visible viewport height.
/// - `i32` - The complete option-list height.
/// - `bool` - Whether to move toward the top.
///
/// # Returns
///
/// - `i32` - An offset inside the viewport's scrollable range.
pub(crate) fn select_scroll_button_target(
    scroll_top: i32,
    client_height: i32,
    scroll_height: i32,
    is_upward: bool,
) -> i32 {
    let visible: i32 = client_height.max(0);
    let maximum: i32 = scroll_height.saturating_sub(visible).max(0);
    let current: i32 = scroll_top.clamp(0, maximum);
    if visible == 0 {
        return current;
    }
    let step: i32 = (visible / 2).max(1);
    if is_upward {
        current.saturating_sub(step).max(0)
    } else {
        current.saturating_add(step).min(maximum)
    }
}

/// Scrolls a long option list upward without changing its selection.
///
/// Pointer activation preserves the listbox focus; keyboard users retain
/// the option list's existing navigation commands.
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
            onmousedown: |event: Event| event.prevent_default()
            onclick: select_scroll_button_handler(true)
            span {
                class: c_euv_select_scroll_icon()
                inner_html: crate::component::icon::ICON_CHEVRON_UP_MARKUP
            }
        }
    }
}

/// Scrolls a long option list downward without changing its selection.
///
/// Pointer activation preserves the listbox focus; keyboard users retain
/// the option list's existing navigation commands.
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
            onmousedown: |event: Event| event.prevent_default()
            onclick: select_scroll_button_handler(false)
            span {
                class: c_euv_select_scroll_icon()
                inner_html: crate::component::icon::ICON_CHEVRON_DOWN_MARKUP
            }
        }
    }
}
