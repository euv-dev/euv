use super::*;

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
        vec![leaf("A", "a"), branch("B", "b", vec![leaf("C", "c")])],
    )]
}

fn tree_node(
    nodes: Vec<EuvTreeNode>,
    selected: &'static str,
    default_expanded: bool,
) -> VirtualNode<EuvTreeProps> {
    VirtualNode::Element {
        tag: Tag::Element(Cow::Borrowed("div")),
        attributes: Vec::new(),
        children: Vec::new(),
        key: None,
        props: Some(Box::new(EuvTreeProps {
            nodes,
            selected: Signal::create(selected.to_string()),
            on_select: None,
            default_expanded,
        })),
    }
}

fn child_count(node: &VirtualNode) -> usize {
    match node {
        VirtualNode::Element { children, .. } => children.iter().map(count_one).sum(),
        VirtualNode::Fragment(children) => children.iter().map(count_one).sum(),
        _ => panic!("expected an element or fragment node, got {node:?}"),
    }
}

fn count_one(node: &VirtualNode) -> usize {
    match node {
        VirtualNode::Fragment(children) => children.iter().map(count_one).sum(),
        VirtualNode::Empty => 0,
        _ => 1,
    }
}

fn total(node: &VirtualNode) -> usize {
    match node {
        VirtualNode::Element { children, .. } => 1 + children.iter().map(total).sum::<usize>(),
        VirtualNode::Fragment(children) => children.iter().map(total).sum(),
        _ => 0,
    }
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

fn attribute_names(node: &VirtualNode, into: &mut Vec<String>) {
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
                attribute_names(child, into);
            }
        }
        VirtualNode::Fragment(children) => {
            for child in children {
                attribute_names(child, into);
            }
        }
        _ => {}
    }
}

fn carries_text(node: &VirtualNode, needle: &str) -> bool {
    format!("{node:?}").contains(needle)
}

#[test]
fn a_tree_renders_one_node_per_root() {
    let node: VirtualNode<EuvTreeProps> = tree_node(
        vec![leaf("A", "a"), leaf("B", "b"), leaf("C", "c")],
        "",
        false,
    );
    let rendered: VirtualNode = euv_tree(node);
    assert_eq!(
        child_count(&rendered),
        3,
        "one node is rendered per root, got {rendered:?}"
    );
}

#[test]
fn every_node_of_the_fixture_reaches_the_tree() {
    let node: VirtualNode<EuvTreeProps> = tree_node(fixture(), "", true);
    let rendered: VirtualNode = euv_tree(node);
    for label in ["Root", "A", "B", "C"] {
        assert!(
            carries_text(&rendered, label),
            "the node label {label} must reach the tree, got {rendered:?}"
        );
    }
}

#[test]
fn every_node_carries_a_row_a_toggle_and_a_label() {
    let node: VirtualNode<EuvTreeProps> = tree_node(fixture(), "", true);
    let rendered: VirtualNode = euv_tree(node);
    let mut names: Vec<String> = Vec::new();
    attribute_names(&rendered, &mut names);
    assert_eq!(
        names
            .iter()
            .filter(|name: &&String| name.as_str() == "onclick")
            .count(),
        8,
        "the fixture has four nodes, each with a clickable row and a toggle, got {names:?}"
    );
    let mut classes: Vec<String> = Vec::new();
    class_names(&rendered, &mut classes);
    assert_eq!(
        classes
            .iter()
            .filter(|name: &&String| name.as_str() == "c_euv_tree_node_label")
            .count(),
        4,
        "and every node carries a label, got {classes:?}"
    );
}

#[test]
fn the_selected_node_gets_the_active_row_class() {
    let node: VirtualNode<EuvTreeProps> = tree_node(fixture(), "b", true);
    let rendered: VirtualNode = euv_tree(node);
    let mut classes: Vec<String> = Vec::new();
    class_names(&rendered, &mut classes);
    assert_eq!(
        classes
            .iter()
            .filter(|name: &&String| name.as_str() == "c_euv_tree_node_row_active")
            .count(),
        1,
        "exactly one row may carry the active class, got {classes:?}"
    );
    assert_eq!(
        classes
            .iter()
            .filter(|name: &&String| name.as_str() == "c_euv_tree_node_row")
            .count(),
        3,
        "and every other row keeps the plain class, got {classes:?}"
    );
}

#[test]
fn a_tree_with_no_selection_leaves_every_row_plain() {
    let node: VirtualNode<EuvTreeProps> = tree_node(fixture(), "", true);
    let rendered: VirtualNode = euv_tree(node);
    let mut classes: Vec<String> = Vec::new();
    class_names(&rendered, &mut classes);
    assert_eq!(
        classes
            .iter()
            .filter(|name: &&String| name.as_str() == "c_euv_tree_node_row_active")
            .count(),
        0,
        "no node is selected, so no row may carry the active class, got {classes:?}"
    );
}

#[test]
fn a_leaf_node_gets_the_invisible_toggle() {
    let node: VirtualNode<EuvTreeProps> = tree_node(fixture(), "", true);
    let rendered: VirtualNode = euv_tree(node);
    let mut classes: Vec<String> = Vec::new();
    class_names(&rendered, &mut classes);
    assert_eq!(
        classes
            .iter()
            .filter(|name: &&String| name.as_str() == "c_euv_tree_node_toggle_leaf")
            .count(),
        2,
        "the fixture has two leaves, and a leaf toggle renders invisible, got {classes:?}"
    );
}

#[test]
fn a_collapsed_branch_gets_the_collapsed_toggle_and_children_class() {
    let node: VirtualNode<EuvTreeProps> = tree_node(fixture(), "", false);
    let rendered: VirtualNode = euv_tree(node);
    let mut classes: Vec<String> = Vec::new();
    class_names(&rendered, &mut classes);
    assert_eq!(
        classes
            .iter()
            .filter(|name: &&String| name.as_str() == "c_euv_tree_node_toggle")
            .count(),
        2,
        "both branches of the fixture start collapsed, got {classes:?}"
    );
    assert_eq!(
        classes
            .iter()
            .filter(|name: &&String| name.as_str() == "c_euv_tree_node_children_closed")
            .count(),
        2,
        "and both branch containers start closed, got {classes:?}"
    );
    assert_eq!(
        classes
            .iter()
            .filter(|name: &&String| name.as_str() == "c_euv_tree_node_children_open")
            .count(),
        0,
        "and no branch may render the open class, got {classes:?}"
    );
}

#[test]
fn an_expanded_branch_gets_the_open_toggle_and_children_class() {
    let node: VirtualNode<EuvTreeProps> = tree_node(fixture(), "", true);
    let rendered: VirtualNode = euv_tree(node);
    let mut classes: Vec<String> = Vec::new();
    class_names(&rendered, &mut classes);
    assert_eq!(
        classes
            .iter()
            .filter(|name: &&String| name.as_str() == "c_euv_tree_node_toggle_open")
            .count(),
        2,
        "both branches of the fixture start expanded, got {classes:?}"
    );
    assert_eq!(
        classes
            .iter()
            .filter(|name: &&String| name.as_str() == "c_euv_tree_node_children_open")
            .count(),
        2,
        "and both branch containers start open, got {classes:?}"
    );
    assert_eq!(
        classes
            .iter()
            .filter(|name: &&String| name.as_str() == "c_euv_tree_node_children_closed")
            .count(),
        0,
        "and no branch may render the closed class, got {classes:?}"
    );
}

#[test]
fn collapsing_a_branch_keeps_its_children_mounted() {
    let collapsed: VirtualNode = euv_tree(tree_node(fixture(), "", false));
    let expanded: VirtualNode = euv_tree(tree_node(fixture(), "", true));
    assert_eq!(
        total(&collapsed),
        total(&expanded),
        "collapsing a branch is a class swap, never an unmount of the subtree"
    );
    assert_eq!(
        child_count(&collapsed),
        child_count(&expanded),
        "and the root level must not change either"
    );
}

#[test]
fn the_expansion_seed_lists_every_node_when_the_tree_starts_expanded() {
    let expanded: Signal<Vec<String>> = use_euv_tree_expanded(fixture().as_slice(), true);
    let values: Vec<String> = expanded.get();
    assert_eq!(
        values.len(),
        4,
        "every node of the fixture starts expanded, got {values:?}"
    );
    for value in ["root", "a", "b", "c"] {
        assert!(
            values.iter().any(|entry: &String| entry == value),
            "the seed must carry {value}, got {values:?}"
        );
    }
}

#[test]
fn the_expansion_seed_is_empty_when_the_tree_starts_collapsed() {
    let expanded: Signal<Vec<String>> = use_euv_tree_expanded(fixture().as_slice(), false);
    assert!(
        expanded.get().is_empty(),
        "a collapsed tree starts with nothing expanded"
    );
}

#[test]
fn toggling_a_value_adds_it_once_and_removes_it_again() {
    let mut values: Vec<String> = Vec::new();
    toggle_expanded_value(&mut values, "b");
    assert_eq!(
        values,
        vec![String::from("b")],
        "toggling an unknown value expands it"
    );
    toggle_expanded_value(&mut values, "b");
    assert!(
        values.is_empty(),
        "toggling a known value collapses it again, got {values:?}"
    );
}

#[test]
fn selecting_a_node_writes_the_signal_and_calls_the_callback() {
    let selected: Signal<String> = Signal::create(String::from("a"));
    let seen: Rc<RefCell<Vec<&'static str>>> = Rc::new(RefCell::new(Vec::new()));
    let recorder: Rc<RefCell<Vec<&'static str>>> = seen.clone();
    select_tree_node(
        selected,
        Some(Rc::new(move |value: &'static str| {
            recorder.borrow_mut().push(value);
        })),
        "c",
    );
    assert_eq!(
        selected.get(),
        String::from("c"),
        "the selected value must land in the caller's signal"
    );
    assert_eq!(
        seen.borrow().clone(),
        vec!["c"],
        "and the callback must receive the same value"
    );
}

#[test]
fn a_tree_without_props_falls_back_to_defaults() {
    let bare: VirtualNode<EuvTreeProps> = VirtualNode::Element {
        tag: Tag::Element(Cow::Borrowed("div")),
        attributes: Vec::new(),
        children: Vec::new(),
        key: None,
        props: None,
    };
    let rendered: VirtualNode = euv_tree(bare);
    assert_eq!(
        child_count(&rendered),
        0,
        "the default props carry no nodes, got {rendered:?}"
    );
}
