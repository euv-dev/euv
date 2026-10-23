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

fn option(value: &'static str, label: &'static str, group: &'static str) -> EuvSelectOption {
    EuvSelectOption {
        value,
        label,
        group,
        disabled: false,
    }
}

fn fruit_options() -> Vec<EuvSelectOption> {
    vec![
        option("apple", "Apple", ""),
        option("banana", "Banana", ""),
        option("cherry", "Cherry", ""),
    ]
}

fn select_with(value: &str, options: Vec<EuvSelectOption>) -> VirtualNode {
    euv_select(props(EuvSelectProps {
        value: Signal::create(String::from(value)),
        options,
        placeholder: "Pick a fruit",
        label: "",
        disabled: false,
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

fn button_classes(node: &VirtualNode) -> Vec<AttributeEntry> {
    let mut into: Vec<AttributeEntry> = Vec::new();
    classes_of_tag(node, "button", &mut into);
    into
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

fn collect_attribute_names(node: &VirtualNode, into: &mut Vec<String>) {
    match node {
        VirtualNode::Element {
            attributes,
            children,
            ..
        } => {
            for attribute in attributes {
                into.push(attribute.get_name().to_string());
            }
            for child in children {
                collect_attribute_names(child, into);
            }
        }
        VirtualNode::Fragment(children) => {
            for child in children {
                collect_attribute_names(child, into);
            }
        }
        _ => {}
    }
}

#[test]
fn a_select_with_no_options_renders_the_empty_row() {
    let rendered: VirtualNode = select_with("", Vec::new());
    let mut divs: Vec<AttributeEntry> = Vec::new();
    classes_of_tag(&rendered, "div", &mut divs);
    assert!(
        divs.iter()
            .any(|entry: &AttributeEntry| carries_classes(entry, &["c_euv_select_empty"])),
        "a select without options must show the empty row, got {divs:?}"
    );
    assert!(
        texts(&rendered)
            .iter()
            .any(|text: &String| text.contains("No options available")),
        "the empty row still carries its message, got {rendered:?}"
    );
}

#[test]
fn a_select_without_a_selection_shows_the_placeholder() {
    let rendered: VirtualNode = select_with("", fruit_options());
    let mut spans: Vec<AttributeEntry> = Vec::new();
    classes_of_tag(&rendered, "span", &mut spans);
    assert!(
        spans
            .first()
            .is_some_and(|entry: &AttributeEntry| carries_classes(
                entry,
                &["c_euv_select_placeholder"]
            )),
        "an unselected select shows the placeholder, got {spans:?}"
    );
    assert!(
        texts(&rendered)
            .iter()
            .any(|text: &String| text.contains("Pick a fruit")),
        "the placeholder text reaches the trigger, got {rendered:?}"
    );
}

#[test]
fn a_select_with_a_selection_shows_the_option_label() {
    let rendered: VirtualNode = select_with("banana", fruit_options());
    let mut spans: Vec<AttributeEntry> = Vec::new();
    classes_of_tag(&rendered, "span", &mut spans);
    assert!(
        spans
            .first()
            .is_some_and(|entry: &AttributeEntry| carries_classes(entry, &["c_euv_select_value"])),
        "a selected select shows the value, got {spans:?}"
    );
    assert!(
        texts(&rendered)
            .iter()
            .any(|text: &String| text.contains("Banana")),
        "the selected option's label reaches the trigger, got {rendered:?}"
    );
}

#[test]
fn the_option_panel_stays_mounted_while_the_select_is_closed() {
    let rendered: VirtualNode = select_with("", fruit_options());
    let buttons: Vec<AttributeEntry> = button_classes(&rendered);
    assert_eq!(
        buttons.len(),
        4,
        "the trigger plus one button per option stay in the tree while the panel is \
         closed, got {rendered:?}"
    );
    assert!(
        buttons
            .first()
            .is_some_and(|entry: &AttributeEntry| carries_classes(
                entry,
                &["c_euv_select_trigger"]
            )),
        "the first button is the trigger, got {buttons:?}"
    );
}

#[test]
fn the_selected_option_carries_the_active_class() {
    let rendered: VirtualNode = select_with("cherry", fruit_options());
    let buttons: Vec<AttributeEntry> = button_classes(&rendered);
    assert!(
        buttons
            .get(3)
            .is_some_and(|entry: &AttributeEntry| carries_classes(
                entry,
                &["c_euv_select_option_active"]
            )),
        "the picked option is filled with the accent, got {buttons:?}"
    );
    assert!(
        buttons
            .get(1)
            .is_some_and(|entry: &AttributeEntry| carries_classes(entry, &["c_euv_select_option"])),
        "the other options keep the neutral look, got {buttons:?}"
    );
}

#[test]
fn grouped_options_render_a_caption_row_for_each_group() {
    let options: Vec<EuvSelectOption> = vec![
        option("alpha", "Alpha", ""),
        option("beta", "Beta", "Letters"),
        option("gamma", "Gamma", "Letters"),
        option("delta", "Delta", "Greek"),
    ];
    let rendered: VirtualNode = select_with("", options);
    let mut divs: Vec<AttributeEntry> = Vec::new();
    classes_of_tag(&rendered, "div", &mut divs);
    let captions: usize = divs
        .iter()
        .filter(|entry: &&AttributeEntry| carries_classes(entry, &["c_euv_select_group"]))
        .count();
    assert_eq!(captions, 2, "one caption row per group, got {divs:?}");
    let rendered_texts: Vec<String> = texts(&rendered);
    assert!(
        rendered_texts
            .iter()
            .any(|text: &String| text.contains("Letters"))
            && rendered_texts
                .iter()
                .any(|text: &String| text.contains("Greek")),
        "both group captions reach the panel, got {rendered:?}"
    );
}

#[test]
fn a_disabled_option_carries_the_disabled_class_and_cannot_be_picked() {
    let options: Vec<EuvSelectOption> = vec![
        option("apple", "Apple", ""),
        EuvSelectOption {
            value: "banana",
            label: "Banana",
            group: "",
            disabled: true,
        },
    ];
    let rendered: VirtualNode = select_with("", options);
    let buttons: Vec<AttributeEntry> = button_classes(&rendered);
    assert!(
        buttons
            .get(2)
            .is_some_and(|entry: &AttributeEntry| carries_classes(
                entry,
                &["c_euv_select_option_disabled"]
            )),
        "a disabled option is drawn muted, got {buttons:?}"
    );
    let mut names: Vec<String> = Vec::new();
    collect_attribute_names(&rendered, &mut names);
    assert_eq!(
        names
            .iter()
            .filter(|name: &&String| name.as_str() == "onclick")
            .count(),
        3,
        "the trigger and every option carry a click slot, got {names:?}"
    );
    let mut buttons: Vec<Vec<AttributeEntry>> = Vec::new();
    attributes_of_tag(&rendered, "button", &mut buttons);
    let disabled_handler: Option<&AttributeEntry> =
        buttons.get(2).and_then(|attributes: &Vec<AttributeEntry>| {
            attributes
                .iter()
                .find(|attribute: &&AttributeEntry| attribute.get_name() == "onclick")
        });
    assert!(
        disabled_handler.is_some_and(carries_no_handler),
        "the disabled option carries no click handler, got {buttons:?}"
    );
    let enabled_handler: Option<&AttributeEntry> =
        buttons.get(1).and_then(|attributes: &Vec<AttributeEntry>| {
            attributes
                .iter()
                .find(|attribute: &&AttributeEntry| attribute.get_name() == "onclick")
        });
    assert!(
        enabled_handler.is_some_and(|entry: &AttributeEntry| !carries_no_handler(entry)),
        "the enabled option carries a click handler, got {buttons:?}"
    );
}

#[test]
fn a_select_without_props_falls_back_to_defaults() {
    let bare: VirtualNode<EuvSelectProps> = VirtualNode::Element {
        tag: Tag::Element(Cow::Borrowed("div")),
        attributes: Vec::new(),
        children: Vec::new(),
        key: None,
        props: None,
    };
    let rendered: VirtualNode = euv_select(bare);
    assert!(
        matches!(rendered, VirtualNode::Element { .. }),
        "defaults still render a select"
    );
}
