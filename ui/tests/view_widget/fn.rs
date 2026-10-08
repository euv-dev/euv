use super::*;

fn total_elements(node: &VirtualNode) -> usize {
    match node {
        VirtualNode::Element { children, .. } => {
            1 + children.iter().map(total_elements).sum::<usize>()
        }
        _ => 0,
    }
}

fn count_spans(node: &VirtualNode) -> usize {
    format!("{node:?}").matches("Element(\"span\")").count()
}

fn icon_node(size: EuvIconSize, label: &'static str) -> VirtualNode<EuvIconProps> {
    VirtualNode::Element {
        tag: Tag::Element(Cow::Borrowed("div")),
        attributes: Vec::new(),
        children: Vec::new(),
        key: None,
        props: Some(Box::new(EuvIconProps {
            name: "star",
            label,
            size,
        })),
    }
}

fn space_node(size: EuvSpaceSize, vertical: bool) -> VirtualNode<EuvSpaceProps> {
    VirtualNode::Element {
        tag: Tag::Element(Cow::Borrowed("div")),
        attributes: Vec::new(),
        children: Vec::new(),
        key: None,
        props: Some(Box::new(EuvSpaceProps {
            size,
            vertical: Signal::create(vertical),
        })),
    }
}

fn result_node(code: &'static str) -> VirtualNode<EuvResultProps> {
    VirtualNode::Element {
        tag: Tag::Element(Cow::Borrowed("div")),
        attributes: Vec::new(),
        children: Vec::new(),
        key: None,
        props: Some(Box::new(EuvResultProps {
            code,
            title: "Title",
            description: "Description",
        })),
    }
}

#[test]
fn an_icon_renders_a_single_span() {
    let rendered: VirtualNode = euv_icon(icon_node(EuvIconSize::default(), ""));
    assert_eq!(
        count_spans(&rendered),
        1,
        "an icon is one span, got {rendered:?}"
    );
    assert_eq!(total_elements(&rendered), 1, "and nothing nests inside it");
}

#[test]
fn an_icon_carries_its_glyph_as_the_span_text() {
    let rendered: VirtualNode = euv_icon(icon_node(EuvIconSize::default(), ""));
    assert!(
        format!("{rendered:?}").contains("star"),
        "a static glyph reaches the tree as plain text, got {rendered:?}"
    );
}

#[test]
fn a_labelled_icon_differs_from_an_unlabelled_one() {
    let labelled: VirtualNode = euv_icon(icon_node(EuvIconSize::default(), "Star"));
    let plain: VirtualNode = euv_icon(icon_node(EuvIconSize::default(), ""));
    assert_ne!(
        labelled, plain,
        "the accessibility label is carried by the rendered tree"
    );
    assert_eq!(
        total_elements(&labelled) - total_elements(&plain),
        1,
        "the label adds exactly one element, the usual visually hidden \
         accessibility text, rather than an attribute"
    );
}

#[test]
fn an_icon_without_props_renders_the_default_size() {
    let bare: VirtualNode<EuvIconProps> = VirtualNode::Element {
        tag: Tag::Element(Cow::Borrowed("div")),
        attributes: Vec::new(),
        children: Vec::new(),
        key: None,
        props: None,
    };
    let rendered: VirtualNode = euv_icon(bare);
    let expected: VirtualNode = euv_icon(VirtualNode::Element {
        tag: Tag::Element(Cow::Borrowed("div")),
        attributes: Vec::new(),
        children: Vec::new(),
        key: None,
        props: Some(Box::new(EuvIconProps {
            name: "",
            label: "",
            size: EuvIconSize::default(),
        })),
    });
    assert_eq!(
        rendered, expected,
        "an icon wires no handlers, so two renders compare directly"
    );
}

#[test]
fn a_space_renders_one_empty_div() {
    let rendered: VirtualNode = euv_space(space_node(EuvSpaceSize::default(), false));
    assert_eq!(
        total_elements(&rendered),
        1,
        "a spacer is a single empty element, got {rendered:?}"
    );
    assert!(rendered.get_children().is_empty(), "with nothing inside it");
}

#[test]
fn a_vertical_space_differs_from_a_horizontal_one() {
    let vertical: VirtualNode = euv_space(space_node(EuvSpaceSize::default(), true));
    let horizontal: VirtualNode = euv_space(space_node(EuvSpaceSize::default(), false));
    assert_ne!(
        vertical, horizontal,
        "the direction is carried by the class and inline style"
    );
    assert_eq!(
        total_elements(&vertical),
        total_elements(&horizontal),
        "while the element count is identical either way"
    );
}

#[test]
fn a_space_without_props_renders_the_horizontal_default() {
    let bare: VirtualNode<EuvSpaceProps> = VirtualNode::Element {
        tag: Tag::Element(Cow::Borrowed("div")),
        attributes: Vec::new(),
        children: Vec::new(),
        key: None,
        props: None,
    };
    let rendered: VirtualNode = euv_space(bare);
    let expected: VirtualNode = euv_space(space_node(EuvSpaceSize::default(), false));
    assert_eq!(
        rendered, expected,
        "a spacer has no handlers, so the trees compare"
    );
}

#[test]
fn a_result_renders_its_three_text_slots() {
    let rendered: VirtualNode = euv_result(result_node("404"));
    let debugged: String = format!("{rendered:?}");
    assert_eq!(
        debugged.matches("Dynamic").count(),
        3,
        "code, title and description are the three signal-backed slots, got {debugged}"
    );
}

#[test]
fn a_result_with_a_code_differs_from_one_without() {
    let with_code: VirtualNode = euv_result(result_node("404"));
    let without: VirtualNode = euv_result(result_node(""));
    assert_ne!(
        with_code, without,
        "an empty code drops the status slot entirely"
    );
    assert_eq!(
        total_elements(&with_code),
        total_elements(&without),
        "the slot becomes empty rather than disappearing"
    );
}

#[test]
fn a_result_keeps_a_stable_shape_with_and_without_a_code() {
    let shapes: Vec<usize> = [&result_node(""), &result_node("404"), &result_node("500")]
        .iter()
        .map(|n: &&VirtualNode<EuvResultProps>| total_elements(&euv_result((*n).clone())))
        .collect();
    assert!(
        shapes.windows(2).all(|w: &[usize]| w[0] == w[1]),
        "the code only fills an existing slot, so the shape is constant, got {shapes:?}"
    );
}

#[test]
fn a_result_without_props_still_renders_its_frame() {
    let bare: VirtualNode<EuvResultProps> = VirtualNode::Element {
        tag: Tag::Element(Cow::Borrowed("div")),
        attributes: Vec::new(),
        children: Vec::new(),
        key: None,
        props: None,
    };
    let rendered: VirtualNode = euv_result(bare);
    assert!(
        total_elements(&rendered) > 0,
        "the result frame always renders"
    );
}

#[test]
fn two_results_with_different_codes_render_different_trees() {
    let not_found: VirtualNode = euv_result(result_node("404"));
    let server_error: VirtualNode = euv_result(result_node("500"));
    assert_ne!(
        not_found, server_error,
        "different codes must be distinguishable downstream"
    );
}
