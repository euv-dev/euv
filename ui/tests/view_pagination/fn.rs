use super::*;

fn item(text: &'static str, link: &'static str) -> EuvPaginationItem {
    EuvPaginationItem { text, link }
}

fn node(
    prev: Option<EuvPaginationItem>,
    next: Option<EuvPaginationItem>,
) -> VirtualNode<EuvPaginationProps> {
    VirtualNode::Element {
        tag: Tag::Element(Cow::Borrowed("div")),
        attributes: Vec::new(),
        children: Vec::new(),
        key: None,
        props: Some(Box::new(EuvPaginationProps {
            prev_label: "Previous",
            next_label: "Next",
            prev,
            next,
        })),
    }
}

fn count_anchors(node: &VirtualNode) -> usize {
    format!("{node:?}").matches("Element(\"a\")").count()
}

fn direct_children(node: &VirtualNode) -> Vec<VirtualNode> {
    match node {
        VirtualNode::Element { children, .. } => children.clone(),
        _ => panic!("expected the pagination root, got {node:?}"),
    }
}

#[test]
fn a_pagination_with_no_items_renders_two_spacers_and_no_links() {
    let rendered: VirtualNode = euv_pagination(node(None, None));
    assert_eq!(
        count_anchors(&rendered),
        0,
        "nothing to link to, got {rendered:?}"
    );
    assert_eq!(
        direct_children(&rendered).len(),
        2,
        "both sides still occupy a slot so the row keeps its balance"
    );
}

#[test]
fn a_pagination_with_one_item_renders_exactly_one_link() {
    let rendered: VirtualNode = euv_pagination(node(Some(item("Back", "/back")), None));
    assert_eq!(
        count_anchors(&rendered),
        1,
        "one reachable side becomes one link, got {rendered:?}"
    );
}

#[test]
fn a_pagination_with_both_items_renders_two_links() {
    let rendered: VirtualNode = euv_pagination(node(
        Some(item("Back", "/back")),
        Some(item("Forward", "/fwd")),
    ));
    assert_eq!(
        count_anchors(&rendered),
        2,
        "both sides are live, got {rendered:?}"
    );
}

#[test]
fn a_pagination_keeps_its_two_sided_shape_in_every_combination() {
    let combos: [Option<EuvPaginationItem>; 2] = [None, Some(item("L", "/l"))];
    for prev in combos {
        for next in combos {
            let rendered: VirtualNode = euv_pagination(node(prev, next));
            assert_eq!(
                direct_children(&rendered).len(),
                2,
                "a missing side becomes a spacer, never a removed slot"
            );
        }
    }
}

#[test]
fn the_link_count_follows_how_many_sides_are_reachable() {
    let reachable = Some(item("L", "/l"));
    let cases: [(Option<EuvPaginationItem>, Option<EuvPaginationItem>, usize); 4] = [
        (None, None, 0),
        (reachable, None, 1),
        (None, reachable, 1),
        (reachable, reachable, 2),
    ];
    for (prev, next, expected) in cases {
        let rendered: VirtualNode = euv_pagination(node(prev, next));
        assert_eq!(
            count_anchors(&rendered),
            expected,
            "expected {expected} links for this combination, got {rendered:?}"
        );
    }
}

#[test]
fn only_the_previous_side_being_present_differs_from_only_the_next() {
    let prev_only: VirtualNode = euv_pagination(node(Some(item("Back", "/back")), None));
    let next_only: VirtualNode = euv_pagination(node(None, Some(item("Forward", "/fwd"))));
    assert_ne!(
        prev_only, next_only,
        "the two sides are rendered differently, not mirrored blindly"
    );
}

#[test]
fn a_pagination_without_props_renders_two_spacers() {
    let bare: VirtualNode<EuvPaginationProps> = VirtualNode::Element {
        tag: Tag::Element(Cow::Borrowed("div")),
        attributes: Vec::new(),
        children: Vec::new(),
        key: None,
        props: None,
    };
    let rendered: VirtualNode = euv_pagination(bare);
    assert_eq!(
        count_anchors(&rendered),
        0,
        "default props mean no reachable sides, got {rendered:?}"
    );
    assert_eq!(
        direct_children(&rendered).len(),
        2,
        "but the two slots remain"
    );
}

#[test]
fn a_pagination_with_no_items_matches_all_default_props() {
    let bare: VirtualNode<EuvPaginationProps> = VirtualNode::Element {
        tag: Tag::Element(Cow::Borrowed("div")),
        attributes: Vec::new(),
        children: Vec::new(),
        key: None,
        props: None,
    };
    let rendered: VirtualNode = euv_pagination(bare);
    let expected: VirtualNode = euv_pagination(node(None, None));
    assert_eq!(
        rendered, expected,
        "pagination wires no click handlers when neither side is live, so \
         the two renders can be compared directly"
    );
}

#[test]
fn a_pagination_with_items_cannot_be_compared_by_tree_equality() {
    let rendered: VirtualNode = euv_pagination(node(Some(item("Back", "/back")), None));
    let same_again: VirtualNode = euv_pagination(node(Some(item("Back", "/back")), None));
    assert_ne!(
        rendered, same_again,
        "each side builds its own click handler, so two identical renders hold \
         distinct closures; compare the link count instead"
    );
    assert_eq!(
        count_anchors(&rendered),
        count_anchors(&same_again),
        "the shape is stable"
    );
}
