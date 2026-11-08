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
    on_change: Option<Rc<dyn Fn(Vec<String>)>>,
) -> VirtualNode {
    let value: &'static str = item.value;
    let is_disabled: bool = disabled || item.disabled;
    let trigger_id: String = format!("euv-accordion-{value}-trigger");
    let trigger_id_ref: &str = trigger_id.as_str();
    let handler: Option<Rc<dyn Fn(Event)>> = if is_disabled {
        None
    } else {
        accordion_toggle(open_values, accordion_type, collapsible, value, on_change)
    };
    html! {
        div {
            key: value
            class: c_euv_accordion_item()
            role: "region"
            "aria-labelledby": trigger_id_ref
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
                    class: c_euv_accordion_trigger()
                    class: if { open_values.get().iter().any(|entry: &String| entry == value) } {
                        c_euv_accordion_trigger_open()
                    } else {
                        c_euv_accordion_trigger()
                    }
                    "data-slot": "accordion-trigger"
                    "aria-expanded": if {
                        open_values.get().iter().any(|entry: &String| entry == value)
                    } {
                        "true"
                    } else {
                        "false"
                    }
                    "aria-disabled": if { item.disabled || disabled } { "true" } else { "false" }
                    disabled: is_disabled
                    onclick: handler
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
                    }
                }
            }
            div {
                class: c_euv_accordion_content()
                class: if { open_values.get().iter().any(|entry: &String| entry == value) } {
                    c_euv_accordion_content_open()
                } else {
                    c_euv_accordion_content_closed()
                }
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
        open_values.set(values.clone());
        if let Some(callback) = on_change.as_ref() {
            callback(values);
        }
    }))
}
