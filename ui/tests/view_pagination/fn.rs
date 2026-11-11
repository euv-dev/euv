use super::*;

fn item(text: &'static str, link: &'static str) -> EuvPaginationItem {
    EuvPaginationItem { text, link }
}

fn props_node<T: Clone + 'static>(props: T) -> VirtualNode<T> {
    VirtualNode::Element {
        tag: Tag::Element(Cow::Borrowed("div")),
        attributes: Vec::new(),
        children: Vec::new(),
        key: None,
        props: Some(Box::new(props)),
    }
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

fn content_node(children: Vec<VirtualNode>) -> VirtualNode<EuvPaginationContentProps> {
    VirtualNode::Element {
        tag: Tag::Element(Cow::Borrowed("ul")),
        attributes: Vec::new(),
        children,
        key: None,
        props: Some(Box::new(EuvPaginationContentProps {})),
    }
}

fn class_values(node: &VirtualNode) -> Vec<String> {
    match node {
        VirtualNode::Element {
            attributes,
            children,
            ..
        } => {
            let mut values: Vec<String> = Vec::new();
            for attribute in attributes.iter() {
                if attribute.get_name() == "class" {
                    values.extend(attribute.class_names());
                }
            }
            for child in children.iter() {
                values.extend(class_values(child));
            }
            values
        }
        VirtualNode::Fragment(children) => children.iter().flat_map(class_values).collect(),
        _ => Vec::new(),
    }
}

fn count_anchors(node: &VirtualNode) -> usize {
    format!("{node:?}").matches("Element(\"a\")").count()
}

fn slot_values(node: &VirtualNode) -> Vec<String> {
    match node {
        VirtualNode::Element {
            attributes,
            children,
            ..
        } => {
            let mut values: Vec<String> = attributes
                .iter()
                .filter(|attribute: &&AttributeEntry| attribute.get_name() == "data-slot")
                .filter_map(AttributeEntry::text_value)
                .collect();
            for child in children.iter() {
                values.extend(slot_values(child));
            }
            values
        }
        VirtualNode::Fragment(children) => children.iter().flat_map(slot_values).collect(),
        _ => Vec::new(),
    }
}

fn attribute_values(node: &VirtualNode, name: &str) -> Vec<String> {
    match node {
        VirtualNode::Element {
            attributes,
            children,
            ..
        } => {
            let mut values: Vec<String> = attributes
                .iter()
                .filter(|attribute: &&AttributeEntry| attribute.get_name() == name)
                .filter_map(AttributeEntry::text_value)
                .collect();
            for child in children.iter() {
                values.extend(attribute_values(child, name));
            }
            values
        }
        VirtualNode::Fragment(children) => children
            .iter()
            .flat_map(|child: &VirtualNode| attribute_values(child, name))
            .collect(),
        _ => Vec::new(),
    }
}

fn direct_children(node: &VirtualNode) -> Vec<VirtualNode> {
    match node {
        VirtualNode::Element { children, .. } => children.clone(),
        _ => panic!("expected the pagination root, got {node:?}"),
    }
}

fn item_children(node: &VirtualNode) -> Vec<VirtualNode> {
    let mut out: Vec<VirtualNode> = Vec::new();
    for child in direct_children(node) {
        for entry in direct_children(&child) {
            out.push(entry);
        }
    }
    out
}

#[test]
fn pagination_composition_renders_numbered_links_and_an_ellipsis() {
    let content: VirtualNode = euv_pagination_content(content_node(vec![
        euv_pagination_item(props_node(EuvPaginationItemProps {
            label: "Previous",
            link: "/previous",
            text: "Previous page",
            next: false,
        })),
        euv_pagination_link(props_node(EuvPaginationLinkProps {
            label: "1",
            link: "/page-1",
            active: true,
        })),
        euv_pagination_link(props_node(EuvPaginationLinkProps {
            label: "2",
            link: "/page-2",
            active: false,
        })),
        euv_pagination_ellipsis(props_node(EuvPaginationEllipsisProps {})),
        euv_pagination_item(props_node(EuvPaginationItemProps {
            label: "Next",
            link: "/next",
            text: "Next page",
            next: true,
        })),
    ]));
    let mut root: VirtualNode<EuvPaginationProps> = node(None, None);
    if let VirtualNode::Element { children, .. } = &mut root {
        children.push(content);
    }
    let rendered: VirtualNode = euv_pagination(root);
    let debugged: String = format!("{rendered:?}");
    let slots: Vec<String> = slot_values(&rendered);
    assert_eq!(count_anchors(&rendered), 4);
    for slot in [
        "pagination",
        "pagination-content",
        "pagination-item",
        "pagination-previous",
        "pagination-next",
        "pagination-link",
        "pagination-ellipsis",
    ] {
        assert!(
            slots.iter().any(|value: &String| value == slot),
            "missing pagination slot {slot}; rendered {slots:?}"
        );
    }
    let class_names: Vec<String> = class_values(&rendered);
    if cfg!(feature = "theme-shadcn") {
        for class_name in [
            "c_euv_pagination_shadcn",
            "c_euv_pagination_content_shadcn",
            "c_euv_pagination_page_shadcn",
            "c_euv_pagination_page_active_shadcn",
            "c_euv_pagination_link_shadcn",
            "c_euv_pagination_previous_shadcn",
            "c_euv_pagination_next_shadcn",
            "c_euv_pagination_icon",
            "c_euv_pagination_ellipsis_shadcn",
        ] {
            assert!(
                class_names.iter().any(|value: &String| value == class_name),
                "pagination must apply {class_name} in the shadcn feature"
            );
        }
    }
    assert!(
        attribute_values(&rendered, "role")
            .iter()
            .any(|value: &String| value == "button")
    );
    assert!(
        attribute_values(&rendered, "aria-label")
            .iter()
            .any(|value: &String| value == "More pages")
    );
    assert!(debugged.contains("…"));
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
        item_children(&rendered).len(),
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
                item_children(&rendered).len(),
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
        item_children(&rendered).len(),
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
fn identical_pagination_renders_have_the_same_structure() {
    let rendered: VirtualNode = euv_pagination(node(Some(item("Back", "/back")), None));
    let same_again: VirtualNode = euv_pagination(node(Some(item("Back", "/back")), None));
    assert_eq!(
        rendered, same_again,
        "the same routes and labels produce the same pagination structure"
    );
    assert_eq!(count_anchors(&rendered), count_anchors(&same_again));
}
