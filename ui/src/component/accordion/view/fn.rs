use super::*;

/// A stack of disclosure sections.
///
/// Each section is a header button and a body. The open sections live in
/// a caller-owned signal holding every open value, so the single mode
/// and the multiple mode are the same signal read at a different width.
///
/// The header button reports its state through `aria-expanded`, so a
/// screen reader reads the section as open or closed.
///
/// # Arguments
///
/// - `VirtualNode<EuvAccordionProps>` - The props node carrying the
///   sections, the open-values signal and the mode.
///
/// # Returns
///
/// - `VirtualNode` - The accordion virtual DOM tree.
#[component]
pub fn euv_accordion(node: VirtualNode<EuvAccordionProps>) -> VirtualNode {
    let EuvAccordionProps {
        items,
        open_values,
        accordion_type,
        collapsible,
        disabled,
        on_change,
    }: EuvAccordionProps = node.try_get_props().unwrap_or_default();
    let default_focus_value: String = items
        .iter()
        .find(|item: &&EuvAccordionItem| !disabled && !item.disabled)
        .map(|item: &EuvAccordionItem| item.value.to_string())
        .unwrap_or_default();
    let focus_value: Signal<String> =
        HookContext::use_hook(move || Signal::create(default_focus_value));
    html! {
        div {
            class: c_euv_accordion()
            role: "group"
            "data-slot": "accordion"
            "data-orientation": "vertical"
            "data-type": match accordion_type {
                EuvAccordionType::Single => "single",
                EuvAccordionType::Multiple => "multiple",
            }
            for item in items.iter() {
                accordion_section(
                *item,
                open_values,
                accordion_type,
                collapsible,
                disabled,
                focus_value,
                on_change.clone(),
                )
            }
        }
    }
}

/// Renders one section of the accordion.
///
/// The open state is read inside the reactive class conditionals rather
/// than hoisted into a local, so the header and the body each subscribe
/// to the signal on their own and can never disagree.
///
/// # Arguments
///
/// - `EuvAccordionItem` - The section to render.
/// - `Signal<Vec<String>>` - The open-values signal.
/// - `EuvAccordionType` - Whether one section or many may stay open.
/// - `bool` - Whether the open section may be closed again.
/// - `bool` - Whether every section is disabled.
/// - `Signal<String>` - The value whose trigger currently owns the tab stop.
/// - `Option<Rc<dyn Fn(Vec<String>)>>` - The change callback.
///
/// # Returns
///
/// - `VirtualNode` - The section virtual DOM tree.
fn accordion_section(
    item: EuvAccordionItem,
    open_values: Signal<Vec<String>>,
    accordion_type: EuvAccordionType,
    collapsible: bool,
    disabled: bool,
    focus_value: Signal<String>,
    on_change: Option<Rc<dyn Fn(Vec<String>)>>,
) -> VirtualNode {
    let value: &'static str = item.value;
    let is_disabled: bool = disabled || item.disabled;
    let disabled_signal: Signal<bool> = Signal::create(is_disabled);
    let trigger_id: String = format!("euv-accordion-{value}-trigger");
    let trigger_id_ref: &str = trigger_id.as_str();
    let content_id: String = format!("euv-accordion-{value}-content");
    let content_id_ref: &str = content_id.as_str();
    let handler: Option<Rc<dyn Fn(Event)>> = if is_disabled {
        None
    } else {
        accordion_toggle(
            open_values,
            accordion_type,
            collapsible,
            value,
            focus_value,
            on_change,
        )
    };
    let focus_handler: Option<Rc<dyn Fn(Event)>> = if is_disabled {
        None
    } else {
        accordion_focus(value, focus_value)
    };
    let keydown_handler: Option<Rc<dyn Fn(Event)>> = if is_disabled {
        None
    } else {
        accordion_keyboard_navigation(value, focus_value)
    };
    let trigger_class: &Css = c_euv_accordion_trigger();
    let trigger_open_class: &Css = c_euv_accordion_trigger_open();
    let shadcn_trigger_class: Css = if cfg!(feature = "theme-shadcn") {
        c_euv_accordion_trigger_shadcn().clone()
    } else {
        Css::default()
    };
    let content_class: &Css = c_euv_accordion_content();
    let content_open_class: &Css = c_euv_accordion_content_open();
    let content_closed_class: &Css = c_euv_accordion_content_closed();
    let shadcn_content_class: Css = if cfg!(feature = "theme-shadcn") {
        c_euv_accordion_content_shadcn().clone()
    } else {
        Css::default()
    };
    html! {
        div {
            key: value
            class: c_euv_accordion_item()
            "data-slot": "accordion-item"
            "data-state": if { open_values.get().iter().any(|entry: &String| entry == value) } {
                "open"
            } else {
                "closed"
            }
            div {
                class: c_euv_accordion_header()
                role: "heading"
                "aria-level": "3"
                "data-slot": "accordion-header"
                button {
                    type: "button"
                    id: trigger_id_ref
                    class: trigger_class
                    class: if { open_values.get().iter().any(|entry: &String| entry == value) } {
                        trigger_open_class
                    } else {
                        trigger_class
                    }
                    class: shadcn_trigger_class
                    "data-slot": "accordion-trigger"
                    "aria-expanded": if {
                        open_values.get().iter().any(|entry: &String| entry == value)
                    } {
                        "true"
                    } else {
                        "false"
                    }
                    "aria-controls": content_id_ref
                    "aria-disabled": if { item.disabled || disabled } { "true" } else { "false" }
                    disabled: disabled_signal
                    tabindex: if { focus_value.get() == value } { "0" } else { "-1" }
                    "data-value": value
                    onclick: handler
                    onfocus: focus_handler
                    onkeydown: keydown_handler
                    {
                        item.trigger
                    }
                    span {
                        class: c_euv_accordion_icon()
                        class: if { open_values.get().iter().any(|entry: &String| entry == value) } {
                            c_euv_accordion_icon_open()
                        } else {
                            c_euv_accordion_icon()
                        }
                        "aria-hidden": "true"
                        "data-slot": "accordion-trigger-icon"
                    }
                }
            }
            div {
                id: content_id_ref
                role: "region"
                "aria-labelledby": trigger_id_ref
                class: content_class
                class: if { open_values.get().iter().any(|entry: &String| entry == value) } {
                    content_open_class
                } else {
                    content_closed_class
                }
                class: shadcn_content_class
                "data-slot": "accordion-content"
                "data-state": if { open_values.get().iter().any(|entry: &String| entry == value) } {
                    "open"
                } else {
                    "closed"
                }
                div {
                    class: c_euv_accordion_content_inner()
                    {
                        item.content
                    }
                }
            }
        }
    }
}

/// Toggles the clicked section, honouring the mode of the accordion.
///
/// In the single mode the list is narrowed to at most one value: an
/// already-open section collapses to the empty list when the accordion
/// is collapsible, and a closed one replaces the whole list. In the
/// multiple mode the value is added to or removed from the list in
/// place.
///
/// The list is read and written back through the signal rather than
/// captured by the handler, so no `Vec` borrow ever escapes into a
/// render closure.
///
/// # Arguments
///
/// - `Signal<Vec<String>>` - The open-values signal.
/// - `EuvAccordionType` - Whether one section or many may stay open.
/// - `bool` - Whether the open section may be closed again.
/// - `&'static str` - The clicked section value.
/// - `Signal<String>` - The value whose trigger owns the tab stop.
/// - `Option<Rc<dyn Fn(Vec<String>)>>` - The change callback.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - The click handler.
fn accordion_toggle(
    open_values: Signal<Vec<String>>,
    accordion_type: EuvAccordionType,
    collapsible: bool,
    value: &'static str,
    focus_value: Signal<String>,
    on_change: Option<Rc<dyn Fn(Vec<String>)>>,
) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |_: Event| {
        let mut values: Vec<String> = open_values.get();
        let is_open: bool = values.iter().any(|entry: &String| entry == value);
        match accordion_type {
            EuvAccordionType::Multiple => {
                if is_open {
                    values.retain(|entry: &String| entry != value);
                } else {
                    values.push(value.to_string());
                }
            }
            EuvAccordionType::Single => {
                if is_open {
                    if collapsible {
                        values.clear();
                    }
                } else {
                    values.clear();
                    values.push(value.to_string());
                }
            }
        }
        focus_value.set(value.to_string());
        open_values.set(values.clone());
        if let Some(callback) = on_change.as_ref() {
            callback(values);
        }
    }))
}

/// Updates the keyboard tab stop when an accordion trigger receives focus.
///
/// # Arguments
///
/// - `&'static str` - The value of the focused section.
/// - `Signal<String>` - The value whose trigger owns the tab stop.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - The focus handler.
fn accordion_focus(value: &'static str, focus_value: Signal<String>) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |_: Event| {
        focus_value.set(value.to_string());
    }))
}

/// Moves keyboard focus among enabled accordion triggers.
///
/// # Arguments
///
/// - `&'static str` - The value of the trigger receiving the key.
/// - `Signal<String>` - The value whose trigger owns the tab stop.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - The keyboard navigation handler.
fn accordion_keyboard_navigation(
    value: &'static str,
    focus_value: Signal<String>,
) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |event: Event| {
        let Ok(keyboard_event) = event.dyn_into::<KeyboardEvent>() else {
            return;
        };
        let Some(target) = keyboard_event
            .target()
            .and_then(|target: EventTarget| target.dyn_into::<Element>().ok())
        else {
            return;
        };
        let Some(trigger) = target.closest(ACCORDION_TRIGGER_SELECTOR).ok().flatten() else {
            return;
        };
        let Some(root) = trigger.closest(ACCORDION_ROOT_SELECTOR).ok().flatten() else {
            return;
        };
        let Ok(nodes) = root.query_selector_all(ACCORDION_TRIGGER_SELECTOR) else {
            return;
        };
        let mut triggers: Vec<HtmlElement> = Vec::new();
        for index in 0..nodes.length() {
            if let Some(button) = nodes
                .item(index)
                .and_then(|node: Node| node.dyn_into::<HtmlElement>().ok())
            {
                triggers.push(button);
            }
        }
        let Some(current_index) = triggers.iter().position(|button: &HtmlElement| {
            button.get_attribute(ACCORDION_VALUE_ATTRIBUTE).as_deref() == Some(value)
        }) else {
            return;
        };
        let next_index: usize = match keyboard_event.key().as_str() {
            "ArrowDown" => (current_index + 1) % triggers.len(),
            "ArrowUp" => (current_index + triggers.len() - 1) % triggers.len(),
            "Home" => 0,
            "End" => triggers.len() - 1,
            _ => return,
        };
        let Some(next_trigger) = triggers.get(next_index) else {
            return;
        };
        let Some(next_value) = next_trigger.get_attribute(ACCORDION_VALUE_ATTRIBUTE) else {
            return;
        };
        keyboard_event.prevent_default();
        focus_value.set(next_value);
        let _: Result<(), JsValue> = next_trigger.focus();
    }))
}
