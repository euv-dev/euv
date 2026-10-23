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

fn option(value: &'static str, label: &'static str) -> EuvCheckboxGroupOption {
    EuvCheckboxGroupOption {
        value,
        label,
        disabled: false,
    }
}

fn topping_options() -> Vec<EuvCheckboxGroupOption> {
    vec![
        option("cheese", "Cheese"),
        option("olive", "Olive"),
        option("basil", "Basil"),
    ]
}

fn group_with(values: Vec<&str>, options: Vec<EuvCheckboxGroupOption>) -> VirtualNode {
    euv_checkbox_group(props(EuvCheckboxGroupProps {
        options,
        values: Signal::create(values.into_iter().map(String::from).collect()),
        label: "",
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

fn texts_of(node: &VirtualNode, into: &mut Vec<String>) {
    match node {
        VirtualNode::Text(text_node) => {
            into.push(format!("{text_node:?}"));
        }
        VirtualNode::Element { children, .. } => {
            for child in children {
                texts_of(child, into);
            }
        }
        VirtualNode::Fragment(children) => {
            for child in children {
                texts_of(child, into);
            }
        }
        _ => {}
    }
}

fn texts(node: &VirtualNode) -> Vec<String> {
    let mut into: Vec<String> = Vec::new();
    texts_of(node, &mut into);
    into
}

fn carries_no_handler(entry: &AttributeEntry) -> bool {
    let empty: AttributeEntry = AttributeEntry::new(
        Cow::Borrowed("onchange"),
        AttributeValue::Text(String::new()),
    );
    entry == &empty
}

#[test]
fn a_checkbox_group_renders_a_row_per_option() {
    let rendered: VirtualNode = group_with(Vec::new(), topping_options());
    let mut labels: Vec<AttributeEntry> = Vec::new();
    classes_of_tag(&rendered, "label", &mut labels);
    assert_eq!(labels.len(), 3, "one row per option, got {rendered:?}");
    assert!(
        labels
            .iter()
            .all(|entry: &AttributeEntry| carries_classes(entry, &["c_euv_checkbox_group_item"])),
        "every row carries the item class, got {labels:?}"
    );
    let mut inputs: Vec<Vec<AttributeEntry>> = Vec::new();
    attributes_of_tag(&rendered, "input", &mut inputs);
    assert_eq!(
        inputs.len(),
        3,
        "every row carries a real checkbox input, got {rendered:?}"
    );
}

#[test]
fn a_checked_option_carries_the_checked_marker_and_a_tick() {
    let rendered: VirtualNode = group_with(vec!["olive"], topping_options());
    let mut spans: Vec<AttributeEntry> = Vec::new();
    classes_of_tag(&rendered, "span", &mut spans);
    let checked: usize = spans
        .iter()
        .filter(|entry: &&AttributeEntry| {
            carries_classes(
                entry,
                &[
                    "c_euv_checkbox_group_box",
                    "c_euv_checkbox_group_box_checked",
                ],
            )
        })
        .count();
    assert_eq!(
        checked, 1,
        "only the ticked option is marked, got {spans:?}"
    );
    assert!(
        texts(&rendered)
            .iter()
            .any(|text: &String| text.contains("✓")),
        "the checked marker carries the tick glyph, got {rendered:?}"
    );
}

#[test]
fn an_unchecked_option_carries_the_unchecked_marker() {
    let rendered: VirtualNode = group_with(Vec::new(), topping_options());
    let mut spans: Vec<AttributeEntry> = Vec::new();
    classes_of_tag(&rendered, "span", &mut spans);
    let unchecked: usize = spans
        .iter()
        .filter(|entry: &&AttributeEntry| {
            carries_classes(
                entry,
                &[
                    "c_euv_checkbox_group_box",
                    "c_euv_checkbox_group_box_unchecked",
                ],
            )
        })
        .count();
    assert_eq!(
        unchecked, 3,
        "every option is unticked while the checked set is empty, got {spans:?}"
    );
    assert!(
        !texts(&rendered)
            .iter()
            .any(|text: &String| text.contains("✓")),
        "the tick glyph only shows on a ticked marker, got {rendered:?}"
    );
}

#[test]
fn every_option_carries_a_real_checkbox_input() {
    let rendered: VirtualNode = group_with(vec!["cheese"], topping_options());
    let mut inputs: Vec<Vec<AttributeEntry>> = Vec::new();
    attributes_of_tag(&rendered, "input", &mut inputs);
    for expected in ["type", "value", "checked", "disabled", "class", "onchange"] {
        let count: usize = inputs
            .iter()
            .flat_map(|attributes: &Vec<AttributeEntry>| attributes.iter())
            .filter(|attribute: &&AttributeEntry| attribute.get_name() == expected)
            .count();
        assert_eq!(
            count, 3,
            "each of the three inputs carries a {expected} attribute, got {inputs:?}"
        );
    }
}

#[test]
fn a_disabled_option_carries_no_change_handler() {
    let options: Vec<EuvCheckboxGroupOption> = vec![
        option("cheese", "Cheese"),
        EuvCheckboxGroupOption {
            value: "olive",
            label: "Olive",
            disabled: true,
        },
    ];
    let rendered: VirtualNode = group_with(Vec::new(), options);
    let mut inputs: Vec<Vec<AttributeEntry>> = Vec::new();
    attributes_of_tag(&rendered, "input", &mut inputs);
    let handler: Option<&AttributeEntry> =
        inputs.get(1).and_then(|attributes: &Vec<AttributeEntry>| {
            attributes
                .iter()
                .find(|attribute: &&AttributeEntry| attribute.get_name() == "onchange")
        });
    assert!(
        handler.is_some_and(carries_no_handler),
        "a disabled option cannot be toggled, got {inputs:?}"
    );
    let enabled_handler: Option<&AttributeEntry> =
        inputs.first().and_then(|attributes: &Vec<AttributeEntry>| {
            attributes
                .iter()
                .find(|attribute: &&AttributeEntry| attribute.get_name() == "onchange")
        });
    assert!(
        enabled_handler.is_some_and(|entry: &AttributeEntry| !carries_no_handler(entry)),
        "an enabled option carries a change handler, got {inputs:?}"
    );
}

#[test]
fn a_checkbox_group_with_a_label_renders_the_label() {
    let rendered: VirtualNode = euv_checkbox_group(props(EuvCheckboxGroupProps {
        options: topping_options(),
        values: Signal::create(Vec::new()),
        label: "Toppings",
        on_change: None,
    }));
    let mut divs: Vec<AttributeEntry> = Vec::new();
    classes_of_tag(&rendered, "div", &mut divs);
    assert!(
        divs.iter()
            .any(|entry: &AttributeEntry| carries_classes(entry, &["c_euv_checkbox_group_label"])),
        "the group label has its own class, got {divs:?}"
    );
    assert!(
        texts(&rendered)
            .iter()
            .any(|text: &String| text.contains("Toppings")),
        "the group label text reaches the tree, got {rendered:?}"
    );
}

#[test]
fn a_checkbox_group_without_a_label_renders_no_label_row() {
    let rendered: VirtualNode = group_with(Vec::new(), topping_options());
    let mut divs: Vec<AttributeEntry> = Vec::new();
    classes_of_tag(&rendered, "div", &mut divs);
    assert!(
        !divs
            .iter()
            .any(|entry: &AttributeEntry| carries_classes(entry, &["c_euv_checkbox_group_label"])),
        "an empty label renders no label row, got {divs:?}"
    );
}

#[test]
fn a_checkbox_group_without_props_falls_back_to_defaults() {
    let bare: VirtualNode<EuvCheckboxGroupProps> = VirtualNode::Element {
        tag: Tag::Element(Cow::Borrowed("div")),
        attributes: Vec::new(),
        children: Vec::new(),
        key: None,
        props: None,
    };
    let rendered: VirtualNode = euv_checkbox_group(bare);
    assert!(
        matches!(rendered, VirtualNode::Element { .. }),
        "defaults still render a checkbox group"
    );
}
