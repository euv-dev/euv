use super::*;

fn item(key: &'static str, title: &'static str) -> EuvCollapseItem {
    EuvCollapseItem { key, title }
}

fn collapse_node(items: Vec<EuvCollapseItem>, open: Vec<&str>) -> VirtualNode<EuvCollapseProps> {
    let open_keys: Signal<Vec<String>> =
        Signal::create(open.iter().map(|k: &&str| (*k).to_string()).collect());
    VirtualNode::Element {
        tag: Tag::Element(Cow::Borrowed("div")),
        attributes: Vec::new(),
        children: Vec::new(),
        key: None,
        props: Some(Box::new(EuvCollapseProps {
            items,
            open_keys,
            allow_multiple: true,
        })),
    }
}

fn child_count(node: &VirtualNode) -> usize {
    match node {
        VirtualNode::Element { children, .. } => children.len(),
        VirtualNode::Fragment(children) => children.len(),
        _ => panic!("expected an element or fragment node, got {node:?}"),
    }
}

#[test]
fn a_collapse_renders_one_section_per_item_plus_its_body() {
    let node: VirtualNode<EuvCollapseProps> = collapse_node(
        vec![item("a", "First"), item("b", "Second"), item("c", "Third")],
        Vec::new(),
    );
    let rendered: VirtualNode = euv_collapse(node);
    assert_eq!(
        child_count(&rendered),
        4,
        "three items plus the trailing body slot"
    );
}

#[test]
fn a_collapse_with_no_items_still_renders_its_body_slot() {
    let node: VirtualNode<EuvCollapseProps> = collapse_node(Vec::new(), Vec::new());
    let rendered: VirtualNode = euv_collapse(node);
    assert_eq!(
        child_count(&rendered),
        1,
        "the body slot is pushed unconditionally, items or not"
    );
}

#[test]
fn a_single_item_collapse_renders_a_section_and_a_body() {
    let node: VirtualNode<EuvCollapseProps> = collapse_node(vec![item("only", "Only")], Vec::new());
    let rendered: VirtualNode = euv_collapse(node);
    assert_eq!(child_count(&rendered), 2, "one section plus the body slot");
}

#[test]
fn an_unrelated_open_key_still_renders_one_section() {
    let node: VirtualNode<EuvCollapseProps> = collapse_node(vec![item("a", "First")], vec!["b"]);
    let rendered: VirtualNode = euv_collapse(node);
    assert_eq!(
        child_count(&rendered),
        2,
        "an open key belonging to another section must not change the shape"
    );
}

#[test]
fn the_open_and_closed_renders_differ_only_in_unobservable_state() {
    let closed: VirtualNode = euv_collapse(collapse_node(vec![item("a", "First")], Vec::new()));
    let open: VirtualNode = euv_collapse(collapse_node(vec![item("a", "First")], vec!["a"]));
    assert_eq!(
        child_count(&closed),
        child_count(&open),
        "opening a section changes class values, not the tree shape"
    );
    assert_eq!(
        format!("{:?}", closed)
            .matches("Element(\"button\")")
            .count(),
        format!("{:?}", open).matches("Element(\"button\")").count(),
        "and the header/body element counts stay identical"
    );
}

#[test]
fn a_collapse_without_props_falls_back_to_defaults() {
    let bare: VirtualNode<EuvCollapseProps> = VirtualNode::Element {
        tag: Tag::Element(Cow::Borrowed("div")),
        attributes: Vec::new(),
        children: Vec::new(),
        key: None,
        props: None,
    };
    let rendered: VirtualNode = euv_collapse(bare);
    assert_eq!(
        child_count(&rendered),
        1,
        "default props mean no items, leaving just the body slot"
    );
}

#[test]
fn a_collapse_preserves_the_children_it_was_given() {
    let mut node: VirtualNode<EuvCollapseProps> = collapse_node(Vec::new(), Vec::new());
    if let VirtualNode::Element { children, .. } = &mut node {
        *children = vec![VirtualNode::from(String::from("footer"))];
    } else {
        panic!("the helper always builds an element");
    }
    let rendered: VirtualNode = euv_collapse(node);
    assert_eq!(
        child_count(&rendered),
        1,
        "the body slot carries the children rather than adding a second one"
    );
    let debugged: String = format!("{rendered:?}");
    assert!(
        debugged.contains("footer"),
        "the body text must survive, got {debugged}"
    );
}
