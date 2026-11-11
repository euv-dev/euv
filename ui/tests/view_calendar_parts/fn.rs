use super::*;

fn props_node<T: Clone + 'static>(props: T) -> VirtualNode<T> {
    VirtualNode::Element {
        tag: Tag::Element(Cow::Borrowed("div")),
        attributes: Vec::new(),
        children: Vec::new(),
        key: None,
        props: Some(Box::new(props)),
    }
}

fn elements(node: &VirtualNode) -> Vec<VirtualNode> {
    let mut out: Vec<VirtualNode> = Vec::new();
    collect(node, &mut out);
    out
}

fn collect(node: &VirtualNode, out: &mut Vec<VirtualNode>) {
    match node {
        VirtualNode::Element { children, .. } => {
            out.push(node.clone());
            for child in children.iter() {
                collect(child, out);
            }
        }
        VirtualNode::Fragment(children) => {
            for child in children.iter() {
                collect(child, out);
            }
        }
        _ => {}
    }
}

fn class_names(node: &VirtualNode) -> Vec<String> {
    let mut out: Vec<String> = Vec::new();
    for element in elements(node) {
        if let VirtualNode::Element { attributes, .. } = &element {
            for entry in attributes.iter() {
                if entry.get_name() == "class" {
                    out.extend(entry.class_names());
                }
            }
        }
    }
    out
}

fn has_class(node: &VirtualNode, name: &str) -> bool {
    class_names(node).iter().any(|entry: &String| entry == name)
}

fn has_attribute(node: &VirtualNode, name: &str) -> bool {
    for element in elements(node) {
        if let VirtualNode::Element { attributes, .. } = &element {
            for entry in attributes.iter() {
                if entry.get_name() == name {
                    return true;
                }
            }
        }
    }
    false
}

fn has_slot(node: &VirtualNode, slot: &str) -> bool {
    for element in elements(node) {
        if let VirtualNode::Element { attributes, .. } = &element {
            for entry in attributes.iter() {
                if entry.get_name() == "data-slot" && entry.text_value().as_deref() == Some(slot) {
                    return true;
                }
            }
        }
    }
    false
}

fn attribute_value_count(node: &VirtualNode, name: &str, value: &str) -> usize {
    let mut count: usize = 0;
    for element in elements(node) {
        if let VirtualNode::Element { attributes, .. } = &element
            && attributes.iter().any(|entry: &AttributeEntry| {
                entry.get_name() == name && entry.text_value().as_deref() == Some(value)
            })
        {
            count += 1;
        }
    }
    count
}

#[test]
fn a_calendar_day_button_carries_its_class_and_slot() {
    let node: VirtualNode = euv_calendar_day_button(VirtualNode::Empty);
    assert!(has_class(&node, "c_euv_calendar_day_button"));
    assert!(has_attribute(&node, "data-slot"));
    assert!(has_slot(&node, "calendar-day-button"));
}

#[test]
fn a_disabled_calendar_day_button_is_native_and_not_tab_stoppable() {
    let node: VirtualNode = euv_calendar_day_button(props_node(EuvCalendarDayButtonProps {
        day: 3,
        selected: false,
        disabled: true,
        on_select: None,
    }));
    assert!(has_attribute(&node, "disabled"));
    assert_eq!(attribute_value_count(&node, "tabindex", "-1"), 1);
    assert!(has_attribute(&node, "aria-disabled"));
}

#[test]
fn a_calendar_grid_has_accessible_rows_and_keyboard_day_controls() {
    let rendered: VirtualNode = euv_calendar(props_node(EuvCalendarProps {
        title: "November 2026",
        weekdays: vec![EuvCalendarWeekday::Mon, EuvCalendarWeekday::Tue],
        days: vec![
            EuvCalendarDay {
                day: 1,
                muted: false,
                today: true,
            },
            EuvCalendarDay {
                day: 2,
                muted: false,
                today: false,
            },
        ],
        selected: Signal::create(0),
        on_day_click: None,
    }));
    assert!(has_slot(&rendered, "calendar"));
    assert!(has_slot(&rendered, "calendar-grid"));
    assert!(has_slot(&rendered, "calendar-day-button"));
    assert_eq!(attribute_value_count(&rendered, "role", "grid"), 1);
    assert_eq!(attribute_value_count(&rendered, "role", "row"), 2);
    assert_eq!(attribute_value_count(&rendered, "role", "columnheader"), 2);
    assert_eq!(attribute_value_count(&rendered, "role", "gridcell"), 2);
    assert_eq!(
        attribute_value_count(&rendered, "aria-label", "November 2026"),
        1
    );
    assert_eq!(
        attribute_value_count(&rendered, "aria-label", "November 2026 1"),
        1
    );
    assert_eq!(
        attribute_value_count(&rendered, "aria-label", "November 2026 2"),
        1
    );
    assert!(has_attribute(&rendered, "aria-live"));
    assert_eq!(
        attribute_value_count(&rendered, "aria-selected", "false"),
        2
    );
    assert_eq!(attribute_value_count(&rendered, "aria-current", "date"), 1);
    assert!(has_attribute(&rendered, "type"));
    assert_eq!(attribute_value_count(&rendered, "tabindex", "0"), 1);
    assert_eq!(attribute_value_count(&rendered, "tabindex", "-1"), 1);
    assert!(has_attribute(&rendered, "onkeydown"));
}

#[test]
fn a_date_picker_announces_month_changes_and_names_navigation_controls() {
    let rendered: VirtualNode = HookContext::with(HookContext::default(), || {
        euv_date_picker(props_node(EuvDatePickerProps {
            value: Signal::create(String::from("2026-10-21")),
            placeholder: "Pick a date",
            label: "Date",
            min: "",
            max: "",
            on_change: None,
        }))
    });

    assert_eq!(attribute_value_count(&rendered, "aria-live", "polite"), 1);
    assert_eq!(attribute_value_count(&rendered, "aria-atomic", "true"), 1);
    assert_eq!(
        attribute_value_count(&rendered, "aria-label", "Previous month"),
        1
    );
    assert_eq!(
        attribute_value_count(&rendered, "aria-label", "Next month"),
        1
    );
}
