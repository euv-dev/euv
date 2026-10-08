use super::*;

fn crumb(label: &'static str, href: &'static str) -> EuvBreadcrumbItem {
    EuvBreadcrumbItem { label, href }
}

fn breadcrumb_node(
    items: Vec<EuvBreadcrumbItem>,
    separator: &'static str,
) -> VirtualNode<EuvBreadcrumbProps> {
    VirtualNode::Element {
        tag: Tag::Element(Cow::Borrowed("div")),
        attributes: Vec::new(),
        children: Vec::new(),
        key: None,
        props: Some(Box::new(EuvBreadcrumbProps { items, separator })),
    }
}

fn tag_name(node: &VirtualNode) -> String {
    match node {
        VirtualNode::Element { tag, .. } => format!("{tag:?}"),
        VirtualNode::Text(_) => "text".to_string(),
        _ => panic!("unexpected node variant: {node:?}"),
    }
}

fn nav_children(node: &VirtualNode) -> Vec<VirtualNode> {
    match node {
        VirtualNode::Element { children, .. } => match children.first() {
            Some(VirtualNode::Fragment(inner)) => inner.clone(),
            Some(other) => vec![other.clone()],
            None => Vec::new(),
        },
        _ => panic!("expected the nav element, got {node:?}"),
    }
}

fn count_named(node: &VirtualNode, needle: &str) -> usize {
    format!("{node:?}").matches(needle).count()
}

#[test]
fn an_empty_breadcrumb_renders_a_single_text_node() {
    let node: VirtualNode<EuvBreadcrumbProps> = breadcrumb_node(Vec::new(), "/");
    let rendered: VirtualNode = euv_breadcrumb(node);
    assert_eq!(
        tag_name(&rendered),
        "text",
        "an empty list short-circuits to an empty string node, not a nav"
    );
}

#[test]
fn a_one_item_breadcrumb_renders_exactly_one_crumb() {
    let node: VirtualNode<EuvBreadcrumbProps> = breadcrumb_node(vec![crumb("Home", "/")], "/");
    let rendered: VirtualNode = euv_breadcrumb(node);
    assert_eq!(
        tag_name(&rendered),
        "Element(\"nav\")",
        "a breadcrumb is a nav"
    );
    assert_eq!(
        nav_children(&rendered).len(),
        1,
        "one item means one crumb and no separator"
    );
}

#[test]
fn a_breadcrumb_interleaves_separators_between_crumbs() {
    let node: VirtualNode<EuvBreadcrumbProps> =
        breadcrumb_node(vec![crumb("Home", "/"), crumb("Docs", "/docs")], "/");
    let rendered: VirtualNode = euv_breadcrumb(node);
    assert_eq!(
        nav_children(&rendered).len(),
        3,
        "two crumbs plus the single separator between them"
    );
}

#[test]
fn a_four_item_breadcrumb_produces_seven_nodes() {
    let node: VirtualNode<EuvBreadcrumbProps> = breadcrumb_node(
        vec![
            crumb("A", "/a"),
            crumb("B", "/b"),
            crumb("C", "/c"),
            crumb("D", "/d"),
        ],
        "/",
    );
    let rendered: VirtualNode = euv_breadcrumb(node);
    assert_eq!(
        nav_children(&rendered).len(),
        7,
        "four crumbs plus three separators"
    );
}

#[test]
fn the_node_count_grows_as_two_per_item_minus_one() {
    for count in 1_usize..=5 {
        let items: Vec<EuvBreadcrumbItem> = (0..count).map(|_: usize| crumb("L", "/l")).collect();
        let node: VirtualNode<EuvBreadcrumbProps> = breadcrumb_node(items, "/");
        let rendered: VirtualNode = euv_breadcrumb(node);
        assert_eq!(
            nav_children(&rendered).len(),
            count * 2 - 1,
            "{count} items should interleave into 2n-1 nodes"
        );
    }
}

#[test]
fn only_the_non_final_crumbs_are_anchors() {
    let node: VirtualNode<EuvBreadcrumbProps> = breadcrumb_node(
        vec![
            crumb("Home", "/"),
            crumb("Docs", "/docs"),
            crumb("Page", "/page"),
        ],
        "/",
    );
    let rendered: VirtualNode = euv_breadcrumb(node);
    assert_eq!(
        count_named(&rendered, "Element(\"a\")"),
        2,
        "the current page is not clickable, so three crumbs give two anchors"
    );
}

#[test]
fn a_one_item_breadcrumb_has_no_anchor_at_all() {
    let node: VirtualNode<EuvBreadcrumbProps> = breadcrumb_node(vec![crumb("Home", "/")], "/");
    let rendered: VirtualNode = euv_breadcrumb(node);
    assert_eq!(
        count_named(&rendered, "Element(\"a\")"),
        0,
        "a single crumb is the current page, not a link"
    );
}

#[test]
fn a_breadcrumb_without_props_falls_back_to_defaults() {
    let bare: VirtualNode<EuvBreadcrumbProps> = VirtualNode::Element {
        tag: Tag::Element(Cow::Borrowed("div")),
        attributes: Vec::new(),
        children: Vec::new(),
        key: None,
        props: None,
    };
    let rendered: VirtualNode = euv_breadcrumb(bare);
    assert_eq!(
        tag_name(&rendered),
        "text",
        "default props mean no items, which short-circuits to an empty text node"
    );
}

#[test]
fn two_breadcrumbs_with_different_hrefs_render_different_trees() {
    let first: VirtualNode = euv_breadcrumb(breadcrumb_node(
        vec![crumb("A", "/first"), crumb("B", "/b")],
        "/",
    ));
    let second: VirtualNode = euv_breadcrumb(breadcrumb_node(
        vec![crumb("A", "/second"), crumb("B", "/b")],
        "/",
    ));
    assert_ne!(
        first, second,
        "the href travels into the anchor's key, so the trees differ"
    );
}
