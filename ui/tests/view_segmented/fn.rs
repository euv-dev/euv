use super::*;

fn props<T>(value: T) -> VirtualNode<T> {
    VirtualNode::Element {
        tag: Tag::Element(Cow::Borrowed("div")),
        attributes: Vec::new(),
        children: Vec::new(),
        key: None,
        props: Some(Box::new(value)),
    }
}

fn option(value: &'static str, label: &'static str) -> EuvSegmentedOption {
    EuvSegmentedOption {
        value,
        label,
        disabled: false,
    }
}

fn view_options() -> Vec<EuvSegmentedOption> {
    vec![
        option("grid", "Grid"),
        option("list", "List"),
        option("board", "Board"),
    ]
}

fn segmented_with(value: &str, options: Vec<EuvSegmentedOption>) -> VirtualNode {
    euv_segmented(props(EuvSegmentedProps {
        options,
        value: Signal::create(String::from(value)),
        on_change: None,
    }))
}

fn carries_classes(entry: &AttributeEntry, names: &[&str]) -> bool {
    if names.len() == 1 {
        let single: AttributeEntry = AttributeEntry::new(
            Cow::Borrowed("class"),
            AttributeValue::Css(Css::new(
                String::from(names[0]),
                String::new(),
                Vec::new(),
                Vec::new(),
            )),
        );
        if entry == &single {
            return true;
        }
    }
    let merged: AttributeEntry = AttributeEntry::new(
        Cow::Borrowed("class"),
        AttributeValue::Text(names.join(" ")),
    );
    entry == &merged
}

fn classes_of_tag(node: &VirtualNode, tag: &str, into: &mut Vec<AttributeEntry>) {
    match node {
        VirtualNode::Element {
            tag: own_tag,
            attributes,
            children,
            ..
        } => {
            if format!("{own_tag:?}") == format!("Element(\"{tag}\")") {
                for attribute in attributes {
                    if attribute.get_name() == "class" {
                        into.push(attribute.clone());
                    }
                }
            }
            for child in children {
                classes_of_tag(child, tag, into);
            }
        }
        VirtualNode::Fragment(children) => {
            for child in children {
                classes_of_tag(child, tag, into);
            }
        }
        _ => {}
    }
}

fn attributes_of_tag(node: &VirtualNode, tag: &str, into: &mut Vec<Vec<AttributeEntry>>) {
    match node {
        VirtualNode::Element {
            tag: own_tag,
            attributes,
            children,
            ..
        } => {
            if format!("{own_tag:?}") == format!("Element(\"{tag}\")") {
                into.push(attributes.clone());
            }
            for child in children {
                attributes_of_tag(child, tag, into);
            }
        }
        VirtualNode::Fragment(children) => {
            for child in children {
                attributes_of_tag(child, tag, into);
            }
        }
        _ => {}
    }
}

fn carries_no_handler(entry: &AttributeEntry) -> bool {
    let empty: AttributeEntry = AttributeEntry::new(
        Cow::Borrowed("onclick"),
        AttributeValue::Text(String::new()),
    );
    entry == &empty
}

#[test]
fn a_segmented_control_renders_a_button_per_option() {
    let rendered: VirtualNode = segmented_with("grid", view_options());
    let mut buttons: Vec<AttributeEntry> = Vec::new();
    classes_of_tag(&rendered, "button", &mut buttons);
    assert_eq!(buttons.len(), 3, "one button per option, got {rendered:?}");
    assert!(
        buttons
            .get(1)
            .is_some_and(|entry: &AttributeEntry| carries_classes(
                entry,
                &["c_euv_segmented_item"]
            )),
        "every segment carries the base class, got {buttons:?}"
    );
    assert!(
        buttons
            .get(2)
            .is_some_and(|entry: &AttributeEntry| carries_classes(
                entry,
                &["c_euv_segmented_item"]
            )),
        "every segment carries the base class, got {buttons:?}"
    );
    let mut divs: Vec<AttributeEntry> = Vec::new();
    classes_of_tag(&rendered, "div", &mut divs);
    assert!(
        divs.iter()
            .any(|entry: &AttributeEntry| carries_classes(entry, &["c_euv_segmented"])),
        "the segments sit in the segmented container, got {divs:?}"
    );
}

#[test]
fn the_selected_segment_carries_the_active_class() {
    let rendered: VirtualNode = segmented_with("list", view_options());
    let mut buttons: Vec<AttributeEntry> = Vec::new();
    classes_of_tag(&rendered, "button", &mut buttons);
    assert!(
        buttons
            .get(1)
            .is_some_and(|entry: &AttributeEntry| carries_classes(
                entry,
                &["c_euv_segmented_item", "c_euv_segmented_item_active"]
            )),
        "the picked segment is filled with the accent, got {buttons:?}"
    );
    assert_eq!(
        buttons
            .iter()
            .filter(|entry: &&AttributeEntry| carries_classes(
                entry,
                &["c_euv_segmented_item", "c_euv_segmented_item_active"]
            ))
            .count(),
        1,
        "exactly one segment can be the picked one, got {buttons:?}"
    );
}

#[test]
fn an_unselected_segment_does_not_carry_the_active_class() {
    let rendered: VirtualNode = segmented_with("", view_options());
    let mut buttons: Vec<AttributeEntry> = Vec::new();
    classes_of_tag(&rendered, "button", &mut buttons);
    assert!(
        !buttons.iter().any(|entry: &AttributeEntry| carries_classes(
            entry,
            &["c_euv_segmented_item", "c_euv_segmented_item_active"]
        )),
        "nothing is picked while the selection is empty, got {buttons:?}"
    );
}

#[test]
fn every_segment_carries_a_pick_handler() {
    let rendered: VirtualNode = segmented_with("grid", view_options());
    let mut buttons: Vec<Vec<AttributeEntry>> = Vec::new();
    attributes_of_tag(&rendered, "button", &mut buttons);
    for attributes in &buttons {
        let handler: Option<&AttributeEntry> = attributes
            .iter()
            .find(|attribute: &&AttributeEntry| attribute.get_name() == "onclick");
        assert!(
            handler.is_some_and(|entry: &AttributeEntry| !carries_no_handler(entry)),
            "every segment can be clicked, got {buttons:?}"
        );
    }
}

#[test]
fn a_disabled_segment_carries_no_pick_handler() {
    let options: Vec<EuvSegmentedOption> = vec![
        option("grid", "Grid"),
        EuvSegmentedOption {
            value: "list",
            label: "List",
            disabled: true,
        },
    ];
    let rendered: VirtualNode = segmented_with("grid", options);
    let mut buttons: Vec<Vec<AttributeEntry>> = Vec::new();
    attributes_of_tag(&rendered, "button", &mut buttons);
    let disabled_handler: Option<&AttributeEntry> =
        buttons.get(1).and_then(|attributes: &Vec<AttributeEntry>| {
            attributes
                .iter()
                .find(|attribute: &&AttributeEntry| attribute.get_name() == "onclick")
        });
    assert!(
        disabled_handler.is_some_and(carries_no_handler),
        "a disabled segment cannot be picked, got {buttons:?}"
    );
}

#[test]
fn a_segmented_control_without_props_falls_back_to_defaults() {
    let bare: VirtualNode<EuvSegmentedProps> = VirtualNode::Element {
        tag: Tag::Element(Cow::Borrowed("div")),
        attributes: Vec::new(),
        children: Vec::new(),
        key: None,
        props: None,
    };
    let rendered: VirtualNode = euv_segmented(bare);
    assert!(
        matches!(rendered, VirtualNode::Element { .. }),
        "defaults still render a segmented control"
    );
    let mut buttons: Vec<AttributeEntry> = Vec::new();
    classes_of_tag(&rendered, "button", &mut buttons);
    assert_eq!(
        buttons.len(),
        0,
        "a control without options has no segments, got {rendered:?}"
    );
}
