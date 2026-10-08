use super::*;

fn element_with_children(children: Vec<VirtualNode>) -> VirtualNode {
    VirtualNode::Element {
        tag: Tag::Element(Cow::Borrowed("div")),
        attributes: Vec::new(),
        children,
        key: None,
        props: None,
    }
}

fn text(value: &str) -> VirtualNode {
    VirtualNode::Text(TextNode::new(Cow::Owned(value.to_string()), None))
}

fn attribute(name: &str, value: &str) -> AttributeEntry {
    AttributeEntry::new(
        Cow::Owned(name.to_string()),
        AttributeValue::Text(value.to_string()),
    )
}

#[test]
fn an_element_reports_its_children() {
    let node: VirtualNode = element_with_children(vec![text("a"), text("b")]);
    assert_eq!(node.get_children().len(), 2, "both children are reported");
}

#[test]
fn a_childless_element_reports_an_empty_child_slice() {
    let node: VirtualNode = element_with_children(Vec::new());
    assert!(
        node.get_children().is_empty(),
        "no children means an empty slice"
    );
}

#[test]
fn an_empty_node_reports_an_empty_child_slice() {
    let node: VirtualNode = VirtualNode::Empty;
    assert!(node.get_children().is_empty(), "Empty never has children");
}

#[test]
fn a_text_node_reports_an_empty_child_slice() {
    let node: VirtualNode = text("hello");
    assert!(node.get_children().is_empty(), "a leaf has no children");
}

#[test]
fn a_fragment_reports_its_children() {
    let node: VirtualNode = VirtualNode::Fragment(vec![text("x"), text("y"), text("z")]);
    assert_eq!(
        node.get_children().len(),
        3,
        "a fragment exposes its children"
    );
}

#[test]
fn get_first_child_returns_the_leftmost_child() {
    let node: VirtualNode = element_with_children(vec![text("first"), text("second")]);
    let observed: Option<&VirtualNode> = node.get_first_child();
    match observed {
        Some(child) => assert_eq!(child, &text("first"), "the leftmost child wins"),
        None => panic!("a node with children must have a first child"),
    }
}

#[test]
fn get_first_child_is_none_for_a_childless_node() {
    let node: VirtualNode = element_with_children(Vec::new());
    assert_eq!(
        node.get_first_child(),
        None,
        "no children means no first child"
    );
}

#[test]
fn get_first_child_is_none_for_an_empty_node() {
    let node: VirtualNode = VirtualNode::Empty;
    assert_eq!(node.get_first_child(), None, "Empty has no first child");
}

#[test]
fn has_children_is_true_for_a_populated_element() {
    let node: VirtualNode = element_with_children(vec![text("a")]);
    assert!(node.has_children(), "one child is enough to be true");
}

#[test]
fn has_children_is_false_for_a_childless_element() {
    let node: VirtualNode = element_with_children(Vec::new());
    assert!(!node.has_children(), "an empty child list is not populated");
}

#[test]
fn has_children_is_false_for_an_empty_node() {
    let node: VirtualNode = VirtualNode::Empty;
    assert!(!node.has_children(), "Empty is never populated");
}

#[test]
fn has_children_is_true_for_a_populated_fragment() {
    let node: VirtualNode = VirtualNode::Fragment(vec![text("a")]);
    assert!(node.has_children(), "a fragment counts as populated");
}

#[test]
fn try_get_children_distinguishes_absent_from_empty() {
    let populated: VirtualNode = element_with_children(vec![text("a")]);
    let childless: VirtualNode = element_with_children(Vec::new());
    let leaf: VirtualNode = VirtualNode::Empty;
    assert!(
        populated.try_get_children().is_some(),
        "a populated element has a child list"
    );
    assert!(
        childless.try_get_children().is_some(),
        "a childless element still HAS a child list, it is just empty"
    );
    assert_eq!(
        leaf.try_get_children(),
        None,
        "Empty has no child list at all, which the plain accessor cannot express"
    );
}

#[test]
fn try_get_children_agrees_with_get_children_on_an_element() {
    let node: VirtualNode = element_with_children(vec![text("a"), text("b")]);
    let optional: Option<&[VirtualNode]> = node.try_get_children();
    match optional {
        Some(slice) => assert_eq!(slice, node.get_children(), "both views agree"),
        None => panic!("an element always has a child list"),
    }
}

#[test]
fn try_get_children_is_none_for_a_text_node() {
    let node: VirtualNode = text("leaf");
    assert_eq!(
        node.try_get_children(),
        None,
        "a leaf has no child list even though get_children returns an empty slice"
    );
}

#[test]
fn try_get_props_returns_the_boxed_props_of_an_element() {
    let node: VirtualNode<u32> = VirtualNode::Element {
        tag: Tag::Component(Cow::Borrowed("Counter")),
        attributes: Vec::new(),
        children: Vec::new(),
        key: None,
        props: Some(Box::new(7)),
    };
    let observed: Option<u32> = node.try_get_props();
    assert_eq!(observed, Some(7), "the props come back by value");
}

#[test]
fn try_get_props_is_none_when_no_props_were_attached() {
    let node: VirtualNode<u32> = VirtualNode::Element {
        tag: Tag::Component(Cow::Borrowed("Counter")),
        attributes: Vec::new(),
        children: Vec::new(),
        key: None,
        props: None,
    };
    let observed: Option<u32> = node.try_get_props();
    assert_eq!(observed, None, "an element without props yields None");
}

#[test]
fn try_get_props_is_none_for_a_non_element_variant() {
    let node: VirtualNode<u32> = VirtualNode::Empty;
    let observed: Option<u32> = node.try_get_props();
    assert_eq!(observed, None, "only elements can carry props");
}

#[test]
fn has_key_is_true_when_a_key_is_attached() {
    let node: VirtualNode = VirtualNode::Element {
        tag: Tag::Element(Cow::Borrowed("li")),
        attributes: Vec::new(),
        children: Vec::new(),
        key: Some(String::from("row-1")),
        props: None,
    };
    assert!(node.has_key(), "an attached key is reported");
}

#[test]
fn has_key_is_false_when_no_key_is_attached() {
    let node: VirtualNode = element_with_children(Vec::new());
    assert!(!node.has_key(), "an absent key is reported as absent");
}

#[test]
fn key_returns_the_attached_key_text() {
    let node: VirtualNode = VirtualNode::Element {
        tag: Tag::Element(Cow::Borrowed("li")),
        attributes: Vec::new(),
        children: Vec::new(),
        key: Some(String::from("row-9")),
        props: None,
    };
    let observed: Option<&str> = node.key();
    assert_eq!(observed, Some("row-9"), "the key text comes back");
}

#[test]
fn extend_attributes_appends_to_an_element() {
    let entry: AttributeEntry = attribute("id", "root");
    let node: VirtualNode = element_with_children(Vec::new());
    let extended: VirtualNode = node.extend_attributes(vec![entry]);
    match &extended {
        VirtualNode::Element { attributes, .. } => {
            assert_eq!(attributes.len(), 1, "the extra entry lands on the element")
        }
        _ => panic!("extend_attributes must keep the element variant"),
    }
}

#[test]
fn extend_attributes_keeps_the_existing_entries_first() {
    let first: AttributeEntry = attribute("id", "root");
    let second: AttributeEntry = attribute("class", "btn");
    let node: VirtualNode = VirtualNode::Element {
        tag: Tag::Element(Cow::Borrowed("div")),
        attributes: vec![first],
        children: Vec::new(),
        key: None,
        props: None,
    };
    let extended: VirtualNode = node.extend_attributes(vec![second]);
    let first_entry: AttributeEntry = attribute("id", "root");
    let second_entry: AttributeEntry = attribute("class", "btn");
    match &extended {
        VirtualNode::Element { attributes, .. } => {
            assert_eq!(
                attributes,
                &vec![first_entry, second_entry],
                "the original entry stays first and the appended one lands last"
            );
        }
        _ => panic!("extend_attributes must keep the element variant"),
    }
}

#[test]
fn extend_attributes_with_nothing_leaves_the_node_alone() {
    let node: VirtualNode = element_with_children(Vec::new());
    let extended: VirtualNode = node.extend_attributes(Vec::new());
    match &extended {
        VirtualNode::Element { attributes, .. } => {
            assert!(attributes.is_empty(), "appending nothing adds nothing")
        }
        _ => panic!("extend_attributes must keep the element variant"),
    }
}

#[test]
fn extend_attributes_drops_the_entries_on_a_non_element_node() {
    let entry: AttributeEntry = attribute("id", "root");
    let node: VirtualNode = VirtualNode::Empty;
    let extended: VirtualNode = node.extend_attributes(vec![entry]);
    assert_eq!(
        extended,
        VirtualNode::Empty,
        "a non-element is returned as is"
    );
}
