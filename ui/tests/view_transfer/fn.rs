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

fn item(value: &'static str, label: &'static str) -> EuvTransferItem {
    EuvTransferItem {
        value,
        label,
        disabled: false,
    }
}

fn source_items() -> Vec<EuvTransferItem> {
    vec![item("alpha", "Alpha"), item("beta", "Beta")]
}

fn target_items() -> Vec<EuvTransferItem> {
    vec![item("gamma", "Gamma")]
}

fn transfer_with(
    marked: Vec<&str>,
    source: Vec<EuvTransferItem>,
    target: Vec<EuvTransferItem>,
) -> VirtualNode {
    euv_transfer(props(EuvTransferProps {
        source,
        target,
        selected: Signal::create(marked.into_iter().map(String::from).collect()),
        source_title: "Available",
        target_title: "Chosen",
        on_move_to_target: None,
        on_move_to_source: None,
    }))
}

fn class_lists(node: &VirtualNode, into: &mut Vec<Vec<String>>) {
    match node {
        VirtualNode::Element {
            attributes,
            children,
            ..
        } => {
            for attribute in attributes {
                if attribute.get_name() == "class" {
                    into.push(attribute.class_names());
                }
            }
            for child in children {
                class_lists(child, into);
            }
        }
        VirtualNode::Fragment(children) => {
            for child in children {
                class_lists(child, into);
            }
        }
        _ => {}
    }
}

fn class_names(node: &VirtualNode, into: &mut Vec<String>) {
    let mut lists: Vec<Vec<String>> = Vec::new();
    class_lists(node, &mut lists);
    for list in lists {
        for name in list {
            into.push(name);
        }
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

fn attributes_of_class(node: &VirtualNode, class_name: &str, into: &mut Vec<Vec<AttributeEntry>>) {
    match node {
        VirtualNode::Element {
            attributes,
            children,
            ..
        } => {
            let carries: bool = attributes.iter().any(|attribute: &AttributeEntry| {
                attribute.get_name() == "class"
                    && attribute
                        .class_names()
                        .iter()
                        .any(|name: &String| name == class_name)
            });
            if carries {
                into.push(attributes.clone());
            }
            for child in children {
                attributes_of_class(child, class_name, into);
            }
        }
        VirtualNode::Fragment(children) => {
            for child in children {
                attributes_of_class(child, class_name, into);
            }
        }
        _ => {}
    }
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

fn carries_no_handler(entry: &AttributeEntry) -> bool {
    let empty: AttributeEntry = AttributeEntry::new(
        Cow::Borrowed("onclick"),
        AttributeValue::Text(String::new()),
    );
    entry == &empty
}

#[test]
fn a_transfer_renders_a_row_per_item() {
    let rendered: VirtualNode = transfer_with(Vec::new(), source_items(), target_items());
    let mut names: Vec<String> = Vec::new();
    class_names(&rendered, &mut names);
    let rows: usize = names
        .iter()
        .filter(|name: &&String| name.as_str() == "c_euv_transfer_item")
        .count();
    assert_eq!(
        rows, 3,
        "one row per item across the two lists, got {names:?}"
    );
    assert_eq!(
        names
            .iter()
            .filter(|name: &&String| name.as_str() == "c_euv_transfer_panel")
            .count(),
        2,
        "and one panel per list, got {names:?}"
    );
    assert_eq!(
        names
            .iter()
            .filter(|name: &&String| name.as_str() == "c_euv_transfer_actions")
            .count(),
        1,
        "and one action column in the middle, got {names:?}"
    );
}

#[test]
fn a_marked_row_carries_the_active_class_and_the_active_box() {
    let rendered: VirtualNode = transfer_with(vec!["beta"], source_items(), target_items());
    let mut names: Vec<String> = Vec::new();
    class_names(&rendered, &mut names);
    assert_eq!(
        names
            .iter()
            .filter(|name: &&String| name.as_str() == "c_euv_transfer_item_active")
            .count(),
        1,
        "the marked row carries the active class, got {names:?}"
    );
    assert_eq!(
        names
            .iter()
            .filter(|name: &&String| name.as_str() == "c_euv_transfer_box_active")
            .count(),
        1,
        "and its marker box is the active one, got {names:?}"
    );
    assert_eq!(
        names
            .iter()
            .filter(|name: &&String| name.as_str() == "c_euv_transfer_box")
            .count(),
        2,
        "while every other row keeps the plain box, got {names:?}"
    );
    assert!(
        texts(&rendered)
            .iter()
            .any(|text: &String| text.contains("✓")),
        "the active marker carries the tick glyph, got {rendered:?}"
    );
}

#[test]
fn an_unmarked_row_keeps_the_plain_box() {
    let rendered: VirtualNode = transfer_with(Vec::new(), source_items(), target_items());
    let mut names: Vec<String> = Vec::new();
    class_names(&rendered, &mut names);
    assert_eq!(
        names
            .iter()
            .filter(|name: &&String| name.as_str() == "c_euv_transfer_item_active")
            .count(),
        0,
        "no row is marked, so no row carries the active class, got {names:?}"
    );
    assert_eq!(
        names
            .iter()
            .filter(|name: &&String| name.as_str() == "c_euv_transfer_box_active")
            .count(),
        0,
        "and no marker box is active, got {names:?}"
    );
    assert_eq!(
        names
            .iter()
            .filter(|name: &&String| name.as_str() == "c_euv_transfer_box")
            .count(),
        3,
        "every row keeps the plain box, got {names:?}"
    );
    assert!(
        !texts(&rendered)
            .iter()
            .any(|text: &String| text.contains("✓")),
        "the tick glyph only shows on an active marker, got {rendered:?}"
    );
}

#[test]
fn the_header_shows_the_title_and_the_row_count() {
    let rendered: VirtualNode = transfer_with(Vec::new(), source_items(), target_items());
    let mut names: Vec<String> = Vec::new();
    class_names(&rendered, &mut names);
    assert_eq!(
        names
            .iter()
            .filter(|name: &&String| name.as_str() == "c_euv_transfer_header")
            .count(),
        2,
        "one header per list, got {names:?}"
    );
    let all_texts: Vec<String> = texts(&rendered);
    assert!(
        all_texts
            .iter()
            .any(|text: &String| text.contains("Available")),
        "the source title reaches the header, got {rendered:?}"
    );
    assert!(
        all_texts
            .iter()
            .any(|text: &String| text.contains("Chosen")),
        "the target title reaches the header, got {rendered:?}"
    );
    assert!(
        all_texts.iter().any(|text: &String| text.contains("\"2\"")),
        "the source header counts its two rows, got {rendered:?}"
    );
    assert!(
        all_texts.iter().any(|text: &String| text.contains("\"1\"")),
        "the target header counts its single row, got {rendered:?}"
    );
}

#[test]
fn clicking_a_row_toggles_its_value_in_the_selection() {
    let selected: Signal<Vec<String>> = Signal::create(Vec::new());
    transfer_toggle_value(selected, "alpha");
    assert_eq!(
        selected.get(),
        vec![String::from("alpha")],
        "the first click marks the row"
    );
    transfer_toggle_value(selected, "alpha");
    assert!(
        selected.get().is_empty(),
        "the second click unmarks the row again, got {:?}",
        selected.get()
    );
}

#[test]
fn a_disabled_row_carries_no_click_handler() {
    let source: Vec<EuvTransferItem> = vec![
        item("alpha", "Alpha"),
        EuvTransferItem {
            value: "beta",
            label: "Beta",
            disabled: true,
        },
    ];
    let rendered: VirtualNode = transfer_with(Vec::new(), source, Vec::new());
    let mut rows: Vec<Vec<AttributeEntry>> = Vec::new();
    attributes_of_class(&rendered, "c_euv_transfer_item", &mut rows);
    assert_eq!(rows.len(), 2, "both rows are rendered, got {rows:?}");
    let disabled_handler: Option<&AttributeEntry> =
        rows.get(1).and_then(|attributes: &Vec<AttributeEntry>| {
            attributes
                .iter()
                .find(|attribute: &&AttributeEntry| attribute.get_name() == "onclick")
        });
    assert!(
        disabled_handler.is_some_and(carries_no_handler),
        "a disabled row cannot be marked, got {rows:?}"
    );
    let enabled_handler: Option<&AttributeEntry> =
        rows.first().and_then(|attributes: &Vec<AttributeEntry>| {
            attributes
                .iter()
                .find(|attribute: &&AttributeEntry| attribute.get_name() == "onclick")
        });
    assert!(
        enabled_handler.is_some_and(|entry: &AttributeEntry| !carries_no_handler(entry)),
        "an enabled row carries a click handler, got {rows:?}"
    );
}

#[test]
fn the_move_action_reports_every_marked_row_of_its_list() {
    let selected: Signal<Vec<String>> =
        Signal::create(vec![String::from("alpha"), String::from("beta")]);
    let seen: Rc<RefCell<Vec<&'static str>>> = Rc::new(RefCell::new(Vec::new()));
    let recorder: Rc<RefCell<Vec<&'static str>>> = seen.clone();
    let handler: Rc<dyn Fn(())> = transfer_move_to_target(
        selected,
        vec!["alpha", "beta", "gamma"],
        Some(Rc::new(move |value: &'static str| {
            recorder.borrow_mut().push(value);
        })),
    );
    handler(());
    assert_eq!(
        seen.borrow().clone(),
        vec!["alpha", "beta"],
        "both marked rows of the source list are reported, got {:?}",
        seen.borrow()
    );
}

#[test]
fn the_move_action_ignores_the_rows_that_are_not_marked() {
    let selected: Signal<Vec<String>> = Signal::create(vec![String::from("gamma")]);
    let seen: Rc<RefCell<Vec<&'static str>>> = Rc::new(RefCell::new(Vec::new()));
    let recorder: Rc<RefCell<Vec<&'static str>>> = seen.clone();
    let handler: Rc<dyn Fn(())> = transfer_move_to_source(
        selected,
        vec!["alpha", "beta", "gamma"],
        Some(Rc::new(move |value: &'static str| {
            recorder.borrow_mut().push(value);
        })),
    );
    handler(());
    assert_eq!(
        seen.borrow().clone(),
        vec!["gamma"],
        "only the marked row of the target list is reported, got {:?}",
        seen.borrow()
    );
}

#[test]
fn the_action_column_holds_two_outline_buttons() {
    let rendered: VirtualNode = transfer_with(Vec::new(), source_items(), target_items());
    let mut buttons: Vec<AttributeEntry> = Vec::new();
    classes_of_tag(&rendered, "button", &mut buttons);
    assert_eq!(
        buttons.len(),
        2,
        "the middle column holds the two move actions, got {rendered:?}"
    );
    for entry in &buttons {
        assert!(
            entry
                .class_names()
                .iter()
                .any(|name: &String| name == "c_euv_button_outline_md"),
            "a move action is an outline button, got {buttons:?}"
        );
    }
}

#[test]
fn a_transfer_without_props_falls_back_to_defaults() {
    let bare: VirtualNode<EuvTransferProps> = VirtualNode::Element {
        tag: Tag::Element(Cow::Borrowed("div")),
        attributes: Vec::new(),
        children: Vec::new(),
        key: None,
        props: None,
    };
    let rendered: VirtualNode = euv_transfer(bare);
    assert!(
        matches!(rendered, VirtualNode::Element { .. }),
        "defaults still render a transfer"
    );
}
