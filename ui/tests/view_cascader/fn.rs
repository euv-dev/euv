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

fn leaf(value: &'static str, label: &'static str) -> EuvCascaderOption {
    EuvCascaderOption {
        value,
        label,
        children: Vec::new(),
    }
}

fn branch(
    value: &'static str,
    label: &'static str,
    children: Vec<EuvCascaderOption>,
) -> EuvCascaderOption {
    EuvCascaderOption {
        value,
        label,
        children,
    }
}

fn fixture() -> Vec<EuvCascaderOption> {
    vec![
        branch(
            "europe",
            "Europe",
            vec![leaf("france", "France"), leaf("spain", "Spain")],
        ),
        branch(
            "asia",
            "Asia",
            vec![branch("japan", "Japan", vec![leaf("tokyo", "Tokyo")])],
        ),
        leaf("africa", "Africa"),
    ]
}

fn cascader(options: Vec<EuvCascaderOption>, path: &[&str]) -> VirtualNode {
    HookContext::with(HookContext::default(), || {
        euv_cascader(props(EuvCascaderProps {
            options,
            value: Signal::create(
                path.iter()
                    .map(|entry: &&str| String::from(*entry))
                    .collect::<Vec<String>>(),
            ),
            placeholder: "Pick a region",
            label: "Region",
            on_select: None,
        }))
    })
}

fn class_names(node: &VirtualNode, into: &mut Vec<String>) {
    match node {
        VirtualNode::Element {
            attributes,
            children,
            ..
        } => {
            for attribute in attributes {
                if attribute.get_name() == "class" {
                    into.extend(attribute.class_names());
                }
            }
            for child in children {
                class_names(child, into);
            }
        }
        VirtualNode::Fragment(children) => {
            for child in children {
                class_names(child, into);
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

fn has_classes(node: &VirtualNode, names: &[&str]) -> bool {
    match node {
        VirtualNode::Element {
            attributes,
            children,
            ..
        } => {
            let own: bool = attributes
                .iter()
                .filter(|entry: &&AttributeEntry| entry.get_name() == "class")
                .any(|entry: &AttributeEntry| carries_classes(entry, names));
            own || children.iter().any(|child: &VirtualNode| has_classes(child, names))
        }
        VirtualNode::Fragment(children) => children.iter().any(|child: &VirtualNode| has_classes(child, names)),
        _ => false,
    }
}

#[test]
fn a_cascader_without_a_chosen_path_shows_the_placeholder() {
    let rendered: VirtualNode = cascader(fixture(), &[]);
    let mut classes: Vec<String> = Vec::new();
    class_names(&rendered, &mut classes);
    assert!(
        classes
            .iter()
            .any(|name: &String| name == "c_euv_cascader_placeholder"),
        "an unchosen cascader shows the placeholder, got {classes:?}"
    );
    assert!(
        !classes
            .iter()
            .any(|name: &String| name == "c_euv_cascader_value"),
        "there is no chosen path to show yet, got {classes:?}"
    );
    assert!(
        texts(&rendered)
            .iter()
            .any(|text: &String| text.contains("Pick a region")),
        "the placeholder text reaches the trigger, got {rendered:?}"
    );
}

#[test]
fn the_chosen_path_shows_its_labels_joined_by_a_slash() {
    let rendered: VirtualNode = cascader(fixture(), &["europe", "france"]);
    let mut classes: Vec<String> = Vec::new();
    class_names(&rendered, &mut classes);
    assert!(
        classes
            .iter()
            .any(|name: &String| name == "c_euv_cascader_value"),
        "a chosen path shows the value, got {classes:?}"
    );
    assert!(
        texts(&rendered)
            .iter()
            .any(|text: &String| text.contains("Europe / France")),
        "the labels of the path reach the trigger, got {rendered:?}"
    );
}

#[test]
fn the_panel_stays_mounted_while_the_cascader_is_closed() {
    let rendered: VirtualNode = cascader(fixture(), &[]);
    assert!(
        has_classes(
            &rendered,
            &["c_euv_cascader_panel", "c_euv_cascader_panel_closed"]
        ),
        "the panel is rendered even while it is closed, got {rendered:?}"
    );
    assert!(
        !has_classes(
            &rendered,
            &["c_euv_cascader_panel", "c_euv_cascader_panel_open"]
        ),
        "a closed panel carries the closed class, got {rendered:?}"
    );
}

#[test]
fn only_the_columns_the_reader_opened_are_rendered() {
    let shallow: VirtualNode = cascader(fixture(), &[]);
    let mut shallow_classes: Vec<String> = Vec::new();
    class_names(&shallow, &mut shallow_classes);
    assert_eq!(
        shallow_classes
            .iter()
            .filter(|name: &&String| name.as_str() == "c_euv_cascader_column")
            .count(),
        1,
        "an untouched cascader shows the top-level column only, got {shallow_classes:?}"
    );
    let opened: VirtualNode = cascader(fixture(), &["europe"]);
    let mut opened_classes: Vec<String> = Vec::new();
    class_names(&opened, &mut opened_classes);
    assert_eq!(
        opened_classes
            .iter()
            .filter(|name: &&String| name.as_str() == "c_euv_cascader_column")
            .count(),
        2,
        "opening a choice adds the column below it, got {opened_classes:?}"
    );
    let deep: VirtualNode = cascader(fixture(), &["asia", "japan"]);
    let mut deep_classes: Vec<String> = Vec::new();
    class_names(&deep, &mut deep_classes);
    assert_eq!(
        deep_classes
            .iter()
            .filter(|name: &&String| name.as_str() == "c_euv_cascader_column")
            .count(),
        3,
        "every level the reader opened has a column, got {deep_classes:?}"
    );
}

#[test]
fn the_choice_on_the_path_carries_the_active_class() {
    let rendered: VirtualNode = cascader(fixture(), &["europe"]);
    let mut classes: Vec<String> = Vec::new();
    class_names(&rendered, &mut classes);
    assert_eq!(
        classes
            .iter()
            .filter(|name: &&String| name.as_str() == "c_euv_cascader_option_active")
            .count(),
        1,
        "exactly one choice may carry the active class, got {classes:?}"
    );
    assert!(
        classes
            .iter()
            .any(|name: &String| name == "c_euv_cascader_option"),
        "the choices keep the plain option class, got {classes:?}"
    );
    assert!(
        texts(&rendered)
            .iter()
            .any(|text: &String| text.contains("Europe")),
        "the choice on the path is rendered, got {rendered:?}"
    );
}

#[test]
fn picking_a_choice_with_children_opens_the_next_column() {
    let value: Signal<Vec<String>> = Signal::create(Vec::new());
    let browsed: Signal<Vec<String>> = Signal::create(Vec::new());
    let open: Signal<bool> = Signal::create(false);
    let handler = cascader_option_handler(value, browsed, open, None, "europe", 0, true).unwrap();
    handler(());
    assert_eq!(
        browsed.get(),
        vec![String::from("europe")],
        "the choice joins the path the reader is browsing"
    );
    assert!(
        value.get().is_empty(),
        "a choice with children does not end the walk, got {:?}",
        value.get()
    );
    assert!(open.get(), "the panel stays open for the next column");
}

#[test]
fn picking_a_choice_without_children_writes_the_whole_path() {
    let value: Signal<Vec<String>> = Signal::create(Vec::new());
    let browsed: Signal<Vec<String>> = Signal::create(vec![String::from("europe")]);
    let open: Signal<bool> = Signal::create(true);
    let picked: Rc<RefCell<Vec<String>>> = Rc::new(RefCell::new(Vec::new()));
    let recorder: Rc<RefCell<Vec<String>>> = picked.clone();
    let on_select: Option<Rc<dyn Fn(&'static str)>> =
        Some(Rc::new(move |reported: &'static str| {
            recorder.borrow_mut().push(String::from(reported));
        }));
    let handler: Rc<dyn Fn(())> =
        cascader_option_handler(value, browsed, open, on_select, "france", 1, false).unwrap();
    handler(());
    assert_eq!(
        value.get(),
        vec![String::from("europe"), String::from("france")],
        "the whole path is written into the caller's signal, got {:?}",
        value.get()
    );
    assert_eq!(
        picked.borrow().as_slice(),
        &[String::from("france")],
        "the callback receives the value of the picked choice"
    );
    assert!(!open.get(), "the panel closes on the last choice");
}

#[test]
fn the_trigger_opens_the_panel_on_the_chosen_path() {
    let value: Signal<Vec<String>> = Signal::create(vec![String::from("asia")]);
    let browsed: Signal<Vec<String>> = Signal::create(Vec::new());
    let open: Signal<bool> = Signal::create(false);
    let handler: Rc<dyn Fn(())> =
        cascader_trigger_handler(open, browsed, value).unwrap();
    handler(());
    assert!(open.get(), "the trigger opens the panel");
    assert_eq!(
        browsed.get(),
        vec![String::from("asia")],
        "the panel opens on the chosen path"
    );
    handler(());
    assert!(!open.get(), "the trigger closes the panel again");
}
