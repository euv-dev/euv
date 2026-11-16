use super::*;

fn pagination_style(class: &Css, property: &str) -> Option<String> {
    class
        .get_style()
        .split(';')
        .filter_map(|part: &str| part.split_once(':'))
        .filter(|(name, _): &(&str, &str)| name.trim() == property)
        .map(|(_, value): (&str, &str)| value.trim().to_string())
        .next_back()
}

#[cfg(not(feature = "theme-custom"))]
fn assert_pagination_auto_rule(class: &Css, properties: &[(&str, &str)]) {
    let selector: String = format!(":is(.c_euv_pagination_auto_content .{})", class.get_name());
    let expected: PseudoRule = PseudoRule::new(
        selector.clone(),
        format!("{} ", Css::style_string(properties)),
    );
    assert!(
        class.get_pseudo_rules().contains(&expected),
        "expected generated .{}{selector} rule {expected:?}; got {:?}",
        class.get_name(),
        class.get_pseudo_rules()
    );
}

#[test]
#[cfg(not(feature = "theme-custom"))]
fn auto_footer_uses_two_equal_stretch_tracks_without_a_mobile_collapse() {
    let auto: &Css = c_euv_pagination_auto_content();
    for (property, expected) in [
        ("display", "grid"),
        ("grid-template-columns", "minmax(0px, 1fr) minmax(0px, 1fr)"),
        ("align-items", "stretch"),
        ("width", "100%"),
        ("box-sizing", "border-box"),
        ("padding", "0px"),
        ("list-style", "none"),
    ] {
        assert_eq!(pagination_style(auto, property).as_deref(), Some(expected));
    }
    assert!(auto.get_media_rules().is_empty());
    assert_pagination_auto_rule(
        c_euv_pagination_item(),
        &[
            ("display", "flex"),
            ("min-width", "0px"),
            ("align-items", "stretch"),
        ],
    );
    let mut link_properties: Vec<(&str, &str)> = vec![
        ("display", "grid"),
        ("grid-template-columns", "auto minmax(0px, 1fr) auto"),
        ("grid-template-rows", "auto auto"),
        ("align-content", "center"),
        ("align-items", "center"),
    ];
    if cfg!(feature = "theme-shadcn") {
        link_properties.push(("justify-content", "normal"));
    }
    link_properties.extend([
        ("justify-items", "stretch"),
        ("flex", "1"),
        ("width", "100%"),
        ("height", "auto"),
        (
            "min-height",
            "calc(var(--control-height) + var(--space-lg))",
        ),
        ("box-sizing", "border-box"),
        ("padding", "var(--space-lg)"),
        ("column-gap", "var(--space-sm)"),
        ("row-gap", "var(--space-2xs)"),
        ("white-space", "normal"),
        ("text-decoration", "none"),
    ]);
    assert_pagination_auto_rule(c_euv_pagination_link(), &link_properties);
}

#[test]
#[cfg(not(feature = "theme-custom"))]
fn auto_footer_restores_two_visible_bounded_text_rows_with_qualified_selectors() {
    assert_pagination_auto_rule(
        c_euv_pagination_label(),
        &[
            ("display", "block"),
            ("grid-column", "2"),
            ("grid-row", "1"),
            ("min-width", "0px"),
            ("overflow", "hidden"),
            ("white-space", "nowrap"),
            ("text-overflow", "ellipsis"),
            ("font-size", "var(--font-xs)"),
            ("line-height", "1.5"),
            ("color", "var(--muted-foreground)"),
        ],
    );
    let mut title_properties: Vec<(&str, &str)> = vec![
        ("display", "block"),
        ("grid-column", "2"),
        ("grid-row", "2"),
        ("min-width", "0px"),
        ("overflow", "hidden"),
        ("white-space", "nowrap"),
        ("text-overflow", "ellipsis"),
        ("font-size", "var(--font-sm)"),
    ];
    if cfg!(feature = "theme-shadcn") {
        title_properties.push(("font-weight", "var(--font-weight-semibold)"));
    }
    title_properties.extend([("line-height", "1.5"), ("color", "var(--foreground)")]);
    assert_pagination_auto_rule(c_euv_pagination_text(), &title_properties);
    assert_pagination_auto_rule(
        c_euv_pagination_previous(),
        &[("padding", "var(--space-lg)"), ("text-align", "left")],
    );
    assert_pagination_auto_rule(
        c_euv_pagination_next(),
        &[("padding", "var(--space-lg)"), ("text-align", "right")],
    );
    for (direction, column) in [("previous", "1"), ("next", "3")] {
        let expected: PseudoRule = PseudoRule::new(
            format!(
                ":is(.c_euv_pagination_auto_content .c_euv_pagination_{direction} .c_euv_pagination_icon)"
            ),
            format!("{} ", Css::style_string(&[("grid-column", column)])),
        );
        assert!(
            c_euv_pagination_icon()
                .get_pseudo_rules()
                .contains(&expected)
        );
    }
}

#[test]
fn auto_footer_geometry_does_not_change_manual_numbered_or_custom_pagination() {
    let classes: [&Css; 9] = [
        c_euv_pagination_auto_content(),
        c_euv_pagination_item(),
        c_euv_pagination_link(),
        c_euv_pagination_label(),
        c_euv_pagination_text(),
        c_euv_pagination_icon(),
        c_euv_pagination_previous(),
        c_euv_pagination_next(),
        c_euv_pagination_content(),
    ];
    if cfg!(feature = "theme-custom") {
        for class in classes {
            assert!(class.get_style().is_empty(), "{}", class.get_name());
            assert!(class.get_pseudo_rules().is_empty(), "{}", class.get_name());
            assert!(class.get_media_rules().is_empty(), "{}", class.get_name());
        }
        return;
    }
    assert_eq!(
        pagination_style(c_euv_pagination_content(), "display").as_deref(),
        Some("flex")
    );
    assert_eq!(
        pagination_style(c_euv_pagination_content(), "grid-template-columns"),
        None
    );
    assert_eq!(pagination_style(c_euv_pagination_item(), "display"), None);
    assert_eq!(
        pagination_style(c_euv_pagination_page(), "display").as_deref(),
        Some("inline-flex")
    );
    assert!(c_euv_pagination_content().get_pseudo_rules().is_empty());
    assert!(c_euv_pagination_page().get_pseudo_rules().is_empty());
    if cfg!(feature = "theme-shadcn") {
        assert_eq!(
            pagination_style(c_euv_pagination_link(), "display").as_deref(),
            Some("inline-flex")
        );
        assert_eq!(
            pagination_style(c_euv_pagination_text(), "display").as_deref(),
            Some("none")
        );
    }
    let hover: String = format!("{:?}", c_euv_pagination_link().get_pseudo_rules());
    assert!(hover.contains(":hover"));
    assert!(!hover.contains("transform"));
}

#[test]
fn pagination_keeps_one_root_content_structure_and_both_title_props_in_every_build() {
    let rendered: VirtualNode = euv_pagination(node(
        Some(item("A previous title", "/previous")),
        Some(item("A next title", "/next")),
    ));
    let VirtualNode::Element {
        tag,
        attributes,
        children,
        ..
    } = &rendered
    else {
        panic!("expected the navigation landmark");
    };
    assert_eq!(format!("{tag:?}"), "Element(\"nav\")");
    let root_classes: Vec<String> = attributes
        .iter()
        .filter(|attribute: &&AttributeEntry| attribute.get_name() == "class")
        .flat_map(AttributeEntry::class_names)
        .collect();
    assert_eq!(root_classes, vec![String::from("c_euv_pagination")]);
    assert_eq!(children.len(), 1);
    let VirtualNode::Element {
        tag, attributes, ..
    } = &children[0]
    else {
        panic!("expected the pagination list");
    };
    assert_eq!(format!("{tag:?}"), "Element(\"ul\")");
    let content_classes: Vec<String> = attributes
        .iter()
        .filter(|attribute: &&AttributeEntry| attribute.get_name() == "class")
        .flat_map(AttributeEntry::class_names)
        .collect();
    assert_eq!(
        content_classes,
        vec![String::from("c_euv_pagination_auto_content")]
    );
    assert_eq!(slot_values(&children[0])[0], "pagination-content");
    let debugged: String = format!("{rendered:?}");
    for text in ["Previous", "Next", "A previous title", "A next title"] {
        assert!(debugged.contains(text), "missing {text}");
    }
    let classes: Vec<String> = class_values(&rendered);
    for name in [
        "c_euv_pagination_label",
        "c_euv_pagination_text",
        "c_euv_pagination_icon",
    ] {
        assert_eq!(
            classes
                .iter()
                .filter(|value: &&String| value.as_str() == name)
                .count(),
            2
        );
    }
}

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
    assert!(
        class_names
            .iter()
            .any(|name: &String| name == "c_euv_pagination_content")
    );
    assert!(
        !class_names
            .iter()
            .any(|name: &String| name == "c_euv_pagination_auto_content")
    );
    if cfg!(feature = "theme-shadcn") {
        for class_name in [
            "c_euv_pagination",
            "c_euv_pagination_content",
            "c_euv_pagination_page",
            "c_euv_pagination_page_active",
            "c_euv_pagination_link",
            "c_euv_pagination_previous",
            "c_euv_pagination_next",
            "c_euv_pagination_icon",
            "c_euv_pagination_ellipsis",
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
