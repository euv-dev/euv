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

fn leaf(label: &'static str, value: &'static str) -> EuvTreeNode {
    EuvTreeNode {
        label,
        value,
        children: Vec::new(),
    }
}

fn branch(label: &'static str, value: &'static str, children: Vec<EuvTreeNode>) -> EuvTreeNode {
    EuvTreeNode {
        label,
        value,
        children,
    }
}

fn fixture() -> Vec<EuvTreeNode> {
    vec![branch(
        "Root",
        "root",
        vec![
            leaf("Alpha", "alpha"),
            branch("Beta", "beta", vec![leaf("Gamma", "gamma")]),
        ],
    )]
}

fn tree_select_with(value: &str, nodes: Vec<EuvTreeNode>) -> VirtualNode {
    euv_tree_select(props(EuvTreeSelectProps {
        nodes,
        value: Signal::create(String::from(value)),
        placeholder: "Pick a node",
        label: "",
        on_select: None,
    }))
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
                    for name in attribute.class_names() {
                        into.push(name);
                    }
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
fn the_trigger_shows_the_placeholder_when_no_node_matches() {
    let rendered: VirtualNode = tree_select_with("", fixture());
    let mut names: Vec<String> = Vec::new();
    class_names(&rendered, &mut names);
    assert!(
        names
            .iter()
            .any(|name: &String| name == "c_euv_tree_select_placeholder"),
        "an unmatched value shows the placeholder, got {names:?}"
    );
    assert!(
        !names
            .iter()
            .any(|name: &String| name == "c_euv_tree_select_value"),
        "and no value class, got {names:?}"
    );
    assert!(
        texts(&rendered)
            .iter()
            .any(|text: &String| text.contains("Pick a node")),
        "the placeholder text reaches the trigger, got {rendered:?}"
    );
}

#[test]
fn the_trigger_shows_the_label_of_a_nested_node() {
    let rendered: VirtualNode = tree_select_with("gamma", fixture());
    let mut names: Vec<String> = Vec::new();
    class_names(&rendered, &mut names);
    assert!(
        names
            .iter()
            .any(|name: &String| name == "c_euv_tree_select_value"),
        "a matched value shows the node's label, got {names:?}"
    );
    assert!(
        !names
            .iter()
            .any(|name: &String| name == "c_euv_tree_select_placeholder"),
        "and no placeholder, got {names:?}"
    );
    assert!(
        texts(&rendered)
            .iter()
            .any(|text: &String| text.contains("Gamma")),
        "the nested node's label reaches the trigger, got {rendered:?}"
    );
}

#[test]
fn the_panel_stays_mounted_while_the_control_is_closed() {
    let rendered: VirtualNode = tree_select_with("", fixture());
    assert!(
        has_classes(
            &rendered,
            &["c_euv_tree_select_panel", "c_euv_tree_select_panel_closed"]
        ),
        "the closed panel carries the closed class, got {rendered:?}"
    );
    let mut names: Vec<String> = Vec::new();
    class_names(&rendered, &mut names);
    assert_eq!(
        names
            .iter()
            .filter(|name: &&String| name.as_str() == "c_euv_tree_node")
            .count(),
        4,
        "and the tree stays mounted while the panel is closed, got {names:?}"
    );
}

#[test]
fn the_panel_reuses_the_tree_classes() {
    let rendered: VirtualNode = tree_select_with("", fixture());
    let mut names: Vec<String> = Vec::new();
    class_names(&rendered, &mut names);
    for expected in [
        "c_euv_tree",
        "c_euv_tree_node",
        "c_euv_tree_node_row",
        "c_euv_tree_node_label",
        "c_euv_tree_node_toggle_open",
        "c_euv_tree_node_toggle_leaf",
        "c_euv_tree_node_children",
        "c_euv_tree_node_children_open",
    ] {
        assert!(
            names.iter().any(|name: &String| name == expected),
            "the panel reuses the {expected} class, got {names:?}"
        );
    }
    assert_eq!(
        names
            .iter()
            .filter(|name: &&String| name.as_str() == "c_euv_tree_node_row")
            .count(),
        4,
        "one row per node of the fixture, got {names:?}"
    );
    assert!(
        !names
            .iter()
            .any(|name: &String| name.starts_with("c_euv_tree_select_node")),
        "the panel adds no second set of tree classes, got {names:?}"
    );
}

#[test]
fn the_selected_node_gets_the_active_row_class() {
    let rendered: VirtualNode = tree_select_with("beta", fixture());
    let mut names: Vec<String> = Vec::new();
    class_names(&rendered, &mut names);
    assert_eq!(
        names
            .iter()
            .filter(|name: &&String| name.as_str() == "c_euv_tree_node_row_active")
            .count(),
        1,
        "exactly one row may carry the active class, got {names:?}"
    );
    assert_eq!(
        names
            .iter()
            .filter(|name: &&String| name.as_str() == "c_euv_tree_node_row")
            .count(),
        3,
        "and every other row keeps the plain class, got {names:?}"
    );
}

#[test]
fn a_nested_node_is_found_by_its_value() {
    assert_eq!(
        tree_select_find_label(&fixture(), "gamma"),
        Some("Gamma"),
        "a nested node is found by its value"
    );
    assert_eq!(
        tree_select_find_label(&fixture(), "root"),
        Some("Root"),
        "a root node is found by its value"
    );
    assert_eq!(
        tree_select_find_label(&fixture(), "nowhere"),
        None,
        "an unknown value matches no node"
    );
}

#[test]
fn choosing_a_node_writes_the_value_reports_it_and_closes_the_panel() {
    let value: Signal<String> = Signal::create(String::new());
    let open: Signal<bool> = Signal::create(true);
    let seen: Rc<RefCell<Vec<&'static str>>> = Rc::new(RefCell::new(Vec::new()));
    let recorder: Rc<RefCell<Vec<&'static str>>> = seen.clone();
    tree_select_pick_node(
        value,
        Some(Rc::new(move |picked: &'static str| {
            recorder.borrow_mut().push(picked);
        })),
        open,
        "gamma",
    );
    assert_eq!(
        value.get(),
        String::from("gamma"),
        "the picked value lands in the caller's signal"
    );
    assert_eq!(
        seen.borrow().clone(),
        vec!["gamma"],
        "and the callback receives the same value"
    );
    assert!(!open.get(), "and the panel closes again");
}

#[test]
fn the_trigger_toggles_the_open_state() {
    let open: Signal<bool> = Signal::create(false);
    let handler: Option<Rc<dyn Fn(())>> = tree_select_toggle_handler(open);
    let Some(handler) = handler else {
        panic!("the trigger carries a click handler");
    };
    handler(());
    assert!(open.get(), "the first click opens the panel");
    handler(());
    assert!(!open.get(), "the second click closes it again");
}

#[test]
fn a_tree_select_with_a_label_renders_the_label() {
    let rendered: VirtualNode = euv_tree_select(props(EuvTreeSelectProps {
        nodes: fixture(),
        value: Signal::create(String::new()),
        placeholder: "Pick a node",
        label: "Node",
        on_select: None,
    }));
    assert!(
        texts(&rendered)
            .iter()
            .any(|text: &String| text.contains("Node")),
        "the group label reaches the control, got {rendered:?}"
    );
}

#[test]
fn a_tree_select_without_props_falls_back_to_defaults() {
    let bare: VirtualNode<EuvTreeSelectProps> = VirtualNode::Element {
        tag: Tag::Element(Cow::Borrowed("div")),
        attributes: Vec::new(),
        children: Vec::new(),
        key: None,
        props: None,
    };
    let rendered: VirtualNode = euv_tree_select(bare);
    assert!(
        matches!(rendered, VirtualNode::Element { .. }),
        "defaults still render a tree select"
    );
}
