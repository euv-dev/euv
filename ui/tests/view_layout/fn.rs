use super::*;

fn total_elements(node: &VirtualNode) -> usize {
    match node {
        VirtualNode::Element { children, .. } => {
            1 + children.iter().map(total_elements).sum::<usize>()
        }
        _ => 0,
    }
}

fn header_node(icon: &'static str, subtitle: &'static str) -> VirtualNode<EuvHeaderProps> {
    VirtualNode::Element {
        tag: Tag::Element(Cow::Borrowed("div")),
        attributes: Vec::new(),
        children: Vec::new(),
        key: None,
        props: Some(Box::new(EuvHeaderProps {
            icon,
            title: "Heading",
            subtitle,
        })),
    }
}

const ACTIONS_TWO: [EuvHeroAction; 2] = [
    EuvHeroAction {
        text: "Start",
        link: "/start",
        primary: true,
    },
    EuvHeroAction {
        text: "Docs",
        link: "/docs",
        primary: false,
    },
];

const ACTIONS_NONE: [EuvHeroAction; 0] = [];

fn hero_node(actions: &'static [EuvHeroAction]) -> VirtualNode<EuvHeroProps> {
    VirtualNode::Element {
        tag: Tag::Element(Cow::Borrowed("div")),
        attributes: Vec::new(),
        children: Vec::new(),
        key: None,
        props: Some(Box::new(EuvHeroProps {
            title: "Hello",
            subtitle: "World",
            actions,
        })),
    }
}

const TOC_ONE: [EuvTocItem; 1] = [EuvTocItem {
    level: 2,
    text: "Section",
    href: "/s",
}];
const TOC_NONE: [EuvTocItem; 0] = [];

fn doc_node(
    toc_items: &'static [EuvTocItem],
    footer: &'static str,
) -> VirtualNode<EuvDocLayoutProps> {
    VirtualNode::Element {
        tag: Tag::Element(Cow::Borrowed("div")),
        attributes: Vec::new(),
        children: Vec::new(),
        key: None,
        props: Some(Box::new(EuvDocLayoutProps {
            toc_title: "On this page",
            toc_items,
            prev_label: "Prev",
            next_label: "Next",
            prev: None,
            next: None,
            footer,
        })),
    }
}

fn default_header() -> VirtualNode<EuvHeaderProps> {
    VirtualNode::Element {
        tag: Tag::Element(Cow::Borrowed("div")),
        attributes: Vec::new(),
        children: Vec::new(),
        key: None,
        props: Some(Box::new(EuvHeaderProps {
            icon: "",
            title: "",
            subtitle: "",
        })),
    }
}

fn default_hero() -> VirtualNode<EuvHeroProps> {
    VirtualNode::Element {
        tag: Tag::Element(Cow::Borrowed("div")),
        attributes: Vec::new(),
        children: Vec::new(),
        key: None,
        props: Some(Box::new(EuvHeroProps {
            title: "",
            subtitle: "",
            actions: &ACTIONS_NONE,
        })),
    }
}

fn default_doc() -> VirtualNode<EuvDocLayoutProps> {
    VirtualNode::Element {
        tag: Tag::Element(Cow::Borrowed("div")),
        attributes: Vec::new(),
        children: Vec::new(),
        key: None,
        props: Some(Box::new(EuvDocLayoutProps {
            toc_title: "",
            toc_items: &TOC_NONE,
            prev_label: "",
            next_label: "",
            prev: None,
            next: None,
            footer: "",
        })),
    }
}

#[test]
fn a_header_renders_a_non_empty_tree() {
    let rendered: VirtualNode = euv_header(header_node("I", "Subtitle"));
    assert!(
        total_elements(&rendered) > 0,
        "a header always renders, got {rendered:?}"
    );
}

#[test]
fn a_header_with_a_subtitle_differs_from_one_without() {
    let with: VirtualNode = euv_header(header_node("I", "Subtitle"));
    let without: VirtualNode = euv_header(header_node("I", ""));
    assert_ne!(
        with, without,
        "the subtitle is carried by a class or Dynamic node rather than by \
         adding an element, so the trees compare unequal while the element \
         count stays the same"
    );
}

#[test]
fn a_header_without_props_falls_back_to_defaults() {
    let bare: VirtualNode<EuvHeaderProps> = VirtualNode::Element {
        tag: Tag::Element(Cow::Borrowed("div")),
        attributes: Vec::new(),
        children: Vec::new(),
        key: None,
        props: None,
    };
    let rendered: VirtualNode = euv_header(bare);
    let expected: VirtualNode = euv_header(default_header());
    assert_eq!(
        rendered, expected,
        "a missing props slot renders the same thing as all-default props"
    );
}

#[test]
fn a_hero_renders_a_non_empty_tree() {
    let rendered: VirtualNode = euv_hero(hero_node(&ACTIONS_TWO));
    assert!(
        total_elements(&rendered) > 0,
        "a hero always renders, got {rendered:?}"
    );
}

#[test]
fn a_hero_with_actions_differs_from_one_without() {
    let with: VirtualNode = euv_hero(hero_node(&ACTIONS_TWO));
    let without: VirtualNode = euv_hero(hero_node(&ACTIONS_NONE));
    assert_ne!(
        with, without,
        "the action list is carried into the rendered tree even though the \
         element count is unchanged"
    );
}

#[test]
fn a_hero_without_props_renders_the_same_shape_as_default_props() {
    let bare: VirtualNode<EuvHeroProps> = VirtualNode::Element {
        tag: Tag::Element(Cow::Borrowed("div")),
        attributes: Vec::new(),
        children: Vec::new(),
        key: None,
        props: None,
    };
    let rendered: VirtualNode = euv_hero(bare);
    let expected: VirtualNode = euv_hero(default_hero());
    assert_eq!(
        total_elements(&rendered),
        total_elements(&expected),
        "a hero carries event handlers, so two separate renders hold distinct \
         closures and can never compare equal; the element count is the \
         observable part"
    );
}

#[test]
fn a_doc_layout_renders_a_non_empty_tree() {
    let rendered: VirtualNode = euv_doc_layout(doc_node(&TOC_ONE, "© 2026"));
    assert!(
        total_elements(&rendered) > 0,
        "a layout always renders, got {rendered:?}"
    );
}

#[test]
fn a_doc_layout_with_a_toc_differs_from_one_without() {
    let with: VirtualNode = euv_doc_layout(doc_node(&TOC_ONE, ""));
    let without: VirtualNode = euv_doc_layout(doc_node(&TOC_NONE, ""));
    assert_ne!(
        with, without,
        "a non-empty table of contents changes the rendered tree"
    );
}

#[test]
fn a_doc_layout_with_a_footer_differs_from_one_without() {
    let with: VirtualNode = euv_doc_layout(doc_node(&TOC_NONE, "© 2026"));
    let without: VirtualNode = euv_doc_layout(doc_node(&TOC_NONE, ""));
    assert_ne!(
        with, without,
        "a non-empty footer changes the rendered tree"
    );
}

#[test]
fn a_doc_layout_without_props_renders_the_same_shape_as_default_props() {
    let bare: VirtualNode<EuvDocLayoutProps> = VirtualNode::Element {
        tag: Tag::Element(Cow::Borrowed("div")),
        attributes: Vec::new(),
        children: Vec::new(),
        key: None,
        props: None,
    };
    let rendered: VirtualNode = euv_doc_layout(bare);
    let expected: VirtualNode = euv_doc_layout(default_doc());
    assert_eq!(
        total_elements(&rendered),
        total_elements(&expected),
        "the layout wires pagination handlers, so two renders hold distinct \
         closures; the element count is what can be compared"
    );
}

#[test]
fn a_doc_layout_preserves_the_children_it_was_given() {
    let mut node: VirtualNode<EuvDocLayoutProps> = doc_node(&TOC_NONE, "");
    if let VirtualNode::Element { children, .. } = &mut node {
        *children = vec![VirtualNode::from(String::from("MARKER-BODY"))];
    } else {
        panic!("the helper always builds an element");
    }
    let rendered: VirtualNode = euv_doc_layout(node);
    assert!(
        format!("{rendered:?}").contains("MARKER-BODY"),
        "the page body must survive, got {rendered:?}"
    );
}

#[test]
fn a_doc_layout_keeps_a_stable_shape_across_toc_and_footer_combinations() {
    let shapes: Vec<usize> = [
        doc_node(&TOC_NONE, ""),
        doc_node(&TOC_ONE, ""),
        doc_node(&TOC_NONE, "© 2026"),
        doc_node(&TOC_ONE, "© 2026"),
    ]
    .iter()
    .map(|n: &VirtualNode<EuvDocLayoutProps>| total_elements(&euv_doc_layout(n.clone())))
    .collect();
    assert!(
        shapes.windows(2).all(|w: &[usize]| w[0] == w[1]),
        "the toc and footer branches swap classes rather than add elements, \
         so the element count must stay constant, got {shapes:?}"
    );
}
