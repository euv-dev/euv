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
/// `c_euv_select_option_disabled` and cannot be picked, and an empty
/// option list renders a single `c_euv_select_empty` row.
///
/// # Arguments
///
/// - `VirtualNode<EuvSelectProps>` - The props node holding the selection signal, the options and the change callback.
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
        disabled,
        on_change,
    }: EuvSelectProps = node.try_get_props().unwrap_or_default();
    let open: Signal<bool> = use_euv_select_open();
    let selected: String = value.get();
    let selected_label: Option<&'static str> = options
        .iter()
        .find(|option: &&EuvSelectOption| option.value == selected)
        .map(|option: &EuvSelectOption| option.label);
    let trigger_text: &'static str = selected_label.unwrap_or(placeholder);
    let has_selection: bool = selected_label.is_some();
    let is_empty: bool = options.is_empty();
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
            .map(|entry: SelectPanelEntry| match entry {
                SelectPanelEntry::Caption(caption) => html! {
                    div {
                        class: c_euv_select_group()
                        caption
                    }
                },
                SelectPanelEntry::Option(option) => select_option_row(
                    option,
                    option.value == selected,
                    value,
                    open,
                    on_change.clone(),
                ),
            })
            .collect()
    };
    html! {
        div {
            class: c_euv_select()
            if !label_text.is_empty() {
                div {
                    class: c_form_label()
                    label_text
                }
            }
            button {
                type: "button"
                class: c_euv_select_trigger()
                disabled: boolean_attr(disabled)
                onclick: select_toggle(open, disabled)
                span {
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
            div {
                class: c_euv_select_panel()
                class: if { open } {
                    c_euv_select_panel_open()
                } else {
                    c_euv_select_panel_closed()
                }
                panel_items
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
    HookContext::use_hook(|| Signal::create(false))
}

/// Flattens the option list into the rows the panel renders.
///
/// The ungrouped options come first and carry no caption; every other
/// group keeps the order in which its caption first appeared and is
/// introduced by a caption row.
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
    for (caption, bucket) in groups {
        entries.push(SelectPanelEntry::Caption(caption));
        for option in bucket {
            entries.push(SelectPanelEntry::Option(option));
        }
    }
    entries
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
/// - `Signal<String>` - The selection signal owned by the caller.
/// - `Signal<bool>` - The panel open-state signal owned by the component.
/// - `Option<Rc<dyn Fn(&'static str)>>` - The change callback.
///
/// # Returns
///
/// - `VirtualNode` - The option row virtual DOM tree.
fn select_option_row(
    option: EuvSelectOption,
    is_selected: bool,
    value: Signal<String>,
    open: Signal<bool>,
    on_change: Option<Rc<dyn Fn(&'static str)>>,
) -> VirtualNode {
    let disabled: bool = option.disabled;
    let handler: Option<Rc<dyn Fn(Event)>> = if disabled {
        None
    } else {
        select_option(value, open, on_change, option.value)
    };
    html! {
        button {
            type: "button"
            class: if disabled {
                c_euv_select_option_disabled()
            } else if is_selected {
                c_euv_select_option_active()
            } else {
                c_euv_select_option()
            }
            disabled: boolean_attr(disabled)
            onclick: handler
            option.label
        }
    }
}

/// Builds the trigger click handler that opens or closes the panel.
///
/// # Arguments
///
/// - `Signal<bool>` - The panel open-state signal owned by the component.
/// - `bool` - Whether the control is disabled; a disabled trigger ignores clicks.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - The toggle click handler.
fn select_toggle(open: Signal<bool>, disabled: bool) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |_: Event| {
        if disabled {
            return;
        }
        let is_open: bool = open.get();
        open.set(!is_open);
    }))
}

/// Builds the click handler that picks one option.
///
/// The handler writes the option value into the selection signal, reports
/// it to the change callback and closes the panel.
///
/// # Arguments
///
/// - `Signal<String>` - The selection signal owned by the caller.
/// - `Signal<bool>` - The panel open-state signal owned by the component.
/// - `Option<Rc<dyn Fn(&'static str)>>` - The change callback.
/// - `&'static str` - The value of the picked option.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - The pick click handler.
fn select_option(
    value: Signal<String>,
    open: Signal<bool>,
    on_change: Option<Rc<dyn Fn(&'static str)>>,
    option_value: &'static str,
) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |_: Event| {
        value.set(String::from(option_value));
        if let Some(callback) = &on_change {
            callback(option_value);
        }
        open.set(false);
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
