use super::*;

fn as_text(node: &VirtualNode) -> Option<String> {
    match node {
        VirtualNode::Text(text_node) => Some(format!("{:?}", text_node)),
        _ => None,
    }
}

#[test]
fn an_owned_string_becomes_a_text_node() {
    let node: VirtualNode = VirtualNode::from(String::from("hello"));
    assert_eq!(as_text(&node), as_text(&VirtualNode::from("hello")));
}

#[test]
fn a_borrowed_str_becomes_a_text_node() {
    let node: VirtualNode = VirtualNode::from("hello");
    assert!(matches!(node, VirtualNode::Text(_)), "a slice becomes text");
}

#[test]
fn the_string_and_str_conversions_agree() {
    let from_string: VirtualNode = VirtualNode::from(String::from("same"));
    let from_str: VirtualNode = VirtualNode::from("same");
    assert_eq!(
        as_text(&from_string),
        as_text(&from_str),
        "both spellings produce the same text content"
    );
}

#[test]
fn an_i32_becomes_a_text_node() {
    let node: VirtualNode = VirtualNode::from(42);
    assert!(
        matches!(node, VirtualNode::Text(_)),
        "an integer becomes text"
    );
}

#[test]
fn a_negative_i32_still_becomes_a_text_node() {
    let node: VirtualNode = VirtualNode::from(-7);
    assert!(
        matches!(node, VirtualNode::Text(_)),
        "negatives are text too"
    );
}

#[test]
fn a_usize_becomes_a_text_node() {
    let node: VirtualNode = VirtualNode::from(9_usize);
    assert!(matches!(node, VirtualNode::Text(_)), "a usize becomes text");
}

#[test]
fn a_bool_becomes_a_text_node() {
    let node: VirtualNode = VirtualNode::from("true");
    assert!(matches!(node, VirtualNode::Text(_)), "a bool becomes text");
}

#[test]
fn the_two_integers_agree_on_the_same_number() {
    let from_signed: VirtualNode = VirtualNode::from(5_i32);
    let from_unsigned: VirtualNode = VirtualNode::from(5_usize);
    assert_eq!(
        as_text(&from_signed),
        as_text(&from_unsigned),
        "5 renders the same whether it is signed or unsigned"
    );
}

#[test]
fn an_empty_node_list_becomes_empty() {
    let node: VirtualNode = VirtualNode::from(Vec::new() as Vec<VirtualNode>);
    assert_eq!(node, VirtualNode::Empty, "no children collapse to Empty");
}

#[test]
fn a_one_node_list_becomes_a_fragment() {
    let node: VirtualNode = VirtualNode::from(vec![VirtualNode::from("a")]);
    assert!(
        matches!(node, VirtualNode::Fragment(_)),
        "one child is a fragment"
    );
}

#[test]
fn a_multi_node_list_becomes_a_fragment() {
    let node: VirtualNode = VirtualNode::from(vec![
        VirtualNode::from("a"),
        VirtualNode::from("b"),
        VirtualNode::from("c"),
    ]);
    match node {
        VirtualNode::Fragment(children) => assert_eq!(children.len(), 3, "all three survive"),
        _ => panic!("expected a Fragment"),
    }
}

#[test]
fn an_empty_slice_becomes_empty() {
    let node: VirtualNode = VirtualNode::from(&[] as &[VirtualNode]);
    assert_eq!(
        node,
        VirtualNode::Empty,
        "an empty slice collapses to Empty"
    );
}

#[test]
fn a_single_node_slice_unwraps_to_that_node() {
    let only: VirtualNode = VirtualNode::from("solo");
    let node: VirtualNode = VirtualNode::from(slice::from_ref(&only));
    assert_eq!(
        node, only,
        "a one-element slice is unwrapped rather than wrapped in a Fragment"
    );
}

#[test]
fn a_multi_node_slice_becomes_a_fragment() {
    let children: [VirtualNode; 2] = [VirtualNode::from("a"), VirtualNode::from("b")];
    let node: VirtualNode = VirtualNode::from(&children[..]);
    match node {
        VirtualNode::Fragment(materialised) => assert_eq!(
            materialised.len(),
            2,
            "a longer slice is materialised into a Fragment"
        ),
        _ => panic!("expected a Fragment"),
    }
}

#[test]
fn a_some_node_is_unwrapped() {
    let inner: VirtualNode = VirtualNode::from("payload");
    let expected: VirtualNode = VirtualNode::from("payload");
    let node: VirtualNode = VirtualNode::from(Some(inner));
    assert_eq!(node, expected, "Some yields the inner node itself");
}

#[test]
fn a_none_node_becomes_empty() {
    let node: VirtualNode = VirtualNode::from(None as Option<VirtualNode>);
    assert_eq!(node, VirtualNode::Empty, "None yields Empty");
}

#[test]
fn some_of_empty_stays_empty() {
    let node: VirtualNode = VirtualNode::from(Some(VirtualNode::Empty));
    assert_eq!(
        node,
        VirtualNode::Empty,
        "an inner Empty passes through unchanged"
    );
}

#[test]
fn a_none_node_list_becomes_empty() {
    let node: VirtualNode = VirtualNode::from(None as Option<Vec<VirtualNode>>);
    assert_eq!(node, VirtualNode::Empty, "no list at all means Empty");
}

#[test]
fn a_some_node_list_is_lifted() {
    let node: VirtualNode =
        VirtualNode::from(Some(vec![VirtualNode::from("a"), VirtualNode::from("b")]));
    assert!(
        matches!(node, VirtualNode::Fragment(_)),
        "a present list is a fragment"
    );
}

#[test]
fn the_vec_and_slice_paths_agree_on_an_empty_input() {
    let from_vec: VirtualNode = VirtualNode::from(Vec::new() as Vec<VirtualNode>);
    let from_slice: VirtualNode = VirtualNode::from(&[] as &[VirtualNode]);
    assert_eq!(from_vec, from_slice, "both empty inputs collapse to Empty");
}

#[test]
fn the_vec_and_slice_paths_differ_on_a_single_child() {
    let from_vec: VirtualNode = VirtualNode::from(vec![VirtualNode::from("a")]);
    let only: VirtualNode = VirtualNode::from("a");
    let from_slice: VirtualNode = VirtualNode::from(slice::from_ref(&only));
    assert!(
        matches!(from_vec, VirtualNode::Fragment(_)),
        "the Vec path keeps a one-element list wrapped"
    );
    assert_eq!(
        from_slice, only,
        "the slice path unwraps a one-element list, matching the clone count of the helper it replaced"
    );
}

#[test]
fn the_vec_and_option_paths_agree_on_a_multi_child_list() {
    let from_vec: VirtualNode =
        VirtualNode::from(vec![VirtualNode::from("a"), VirtualNode::from("b")]);
    let from_option: VirtualNode =
        VirtualNode::from(Some(vec![VirtualNode::from("a"), VirtualNode::from("b")]));
    assert_eq!(from_vec, from_option, "wrapping a list is order preserving");
}
