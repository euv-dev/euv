use super::*;

fn total_elements(node: &VirtualNode) -> usize {
    match node {
        VirtualNode::Element { children, .. } => {
            1 + children.iter().map(total_elements).sum::<usize>()
        }
        VirtualNode::Fragment(children) => children.iter().map(total_elements).sum(),
        _ => 0,
    }
}

fn drawer_node(open: bool) -> VirtualNode<EuvDrawerProps> {
    VirtualNode::Element {
        tag: Tag::Element(Cow::Borrowed("div")),
        attributes: Vec::new(),
        children: Vec::new(),
        key: None,
        props: Some(Box::new(EuvDrawerProps {
            open: Signal::create(open),
        })),
    }
}

fn popover_node(open: bool) -> VirtualNode<EuvPopoverProps> {
    VirtualNode::Element {
        tag: Tag::Element(Cow::Borrowed("div")),
        attributes: Vec::new(),
        children: Vec::new(),
        key: None,
        props: Some(Box::new(EuvPopoverProps {
            open: Signal::create(open),
            title: "Tip",
            placement: EuvTooltipPlacement::default(),
        })),
    }
}

fn panel_node(subtitle: &'static str) -> VirtualNode<EuvPanelProps> {
    VirtualNode::Element {
        tag: Tag::Element(Cow::Borrowed("div")),
        attributes: Vec::new(),
        children: Vec::new(),
        key: None,
        props: Some(Box::new(EuvPanelProps {
            title: "Title",
            subtitle,
            variant: EuvPanelVariant::default(),
        })),
    }
}

#[test]
fn a_drawer_renders_a_scrim_and_a_sheet() {
    let rendered: VirtualNode = euv_drawer(drawer_node(true));
    match &rendered {
        VirtualNode::Fragment(children) => assert_eq!(
            children.len(),
            2,
            "an overlay is a backdrop plus the sheet, got {rendered:?}"
        ),
        _ => panic!("a drawer renders siblings, not one wrapper, got {rendered:?}"),
    }
    assert_eq!(
        format!("{rendered:?}").matches("Element(\"div\")").count(),
        2,
        "the scrim and the sheet are the only two divs in the tree"
    );
}

#[test]
fn an_open_drawer_differs_from_a_shut_one() {
    let open: VirtualNode = euv_drawer(drawer_node(true));
    let shut: VirtualNode = euv_drawer(drawer_node(false));
    assert_ne!(
        open, shut,
        "the open flag is carried by the class on the scrim and the sheet"
    );
}

#[test]
fn the_open_flag_does_not_change_the_drawer_shape() {
    let open: VirtualNode = euv_drawer(drawer_node(true));
    let shut: VirtualNode = euv_drawer(drawer_node(false));
    assert_eq!(
        total_elements(&open),
        total_elements(&shut),
        "a closed drawer is hidden by styling, not by removing nodes"
    );
}

#[test]
fn a_drawer_without_props_renders_the_shut_form() {
    let bare: VirtualNode<EuvDrawerProps> = VirtualNode::Element {
        tag: Tag::Element(Cow::Borrowed("div")),
        attributes: Vec::new(),
        children: Vec::new(),
        key: None,
        props: None,
    };
    let rendered: VirtualNode = euv_drawer(bare);
    let expected: VirtualNode = euv_drawer(VirtualNode::Element {
        tag: Tag::Element(Cow::Borrowed("div")),
        attributes: Vec::new(),
        children: Vec::new(),
        key: None,
        props: Some(Box::new(EuvDrawerProps {
            open: Signal::create(false),
        })),
    });
    assert_eq!(
        rendered, expected,
        "the open signal defaults to false, so a bare props slot is a closed drawer"
    );
}

#[test]
fn a_popover_renders_its_content_whether_open_or_shut() {
    let open: VirtualNode = euv_popover(popover_node(true));
    let shut: VirtualNode = euv_popover(popover_node(false));
    assert_eq!(
        total_elements(&open),
        total_elements(&shut),
        "a popover is revealed by class, so both states keep every node"
    );
    assert_ne!(
        open, shut,
        "even though the two states must still be distinguishable"
    );
}

#[test]
fn a_popover_always_renders_its_title_as_text() {
    let rendered: VirtualNode = euv_popover(popover_node(false));
    assert!(
        format!("{rendered:?}").contains("Tip"),
        "the title is a static text node, so it is directly readable, got {rendered:?}"
    );
}

#[test]
fn a_popover_body_is_always_present_and_never_empty() {
    let rendered: VirtualNode = euv_popover(popover_node(false));
    assert!(
        format!("{rendered:?}").contains("Empty"),
        "the popover keeps an Empty placeholder for its caller-supplied \
         children whether it is open or not, got {rendered:?}"
    );
}

#[test]
fn a_panel_renders_as_a_section_with_a_heading() {
    let rendered: VirtualNode = euv_panel(panel_node(""));
    let debugged: String = format!("{rendered:?}");
    assert!(
        debugged.contains("Element(\"section\")"),
        "a panel is a section for document outline semantics, got {debugged}"
    );
    assert!(
        debugged.contains("Element(\"h3\")"),
        "and it carries a heading, got {debugged}"
    );
}

#[test]
fn a_panel_with_a_subtitle_differs_from_one_without() {
    let with: VirtualNode = euv_panel(panel_node("extra"));
    let without: VirtualNode = euv_panel(panel_node(""));
    assert_ne!(
        with, without,
        "the subtitle reaches the rendered tree as a Dynamic child"
    );
    assert_eq!(
        total_elements(&with) - total_elements(&without),
        1,
        "a non-empty subtitle gets its own paragraph instead of filling an \
         existing slot, unlike the popover's children"
    );
}

#[test]
fn a_panel_without_props_still_renders_its_frame() {
    let bare: VirtualNode<EuvPanelProps> = VirtualNode::Element {
        tag: Tag::Element(Cow::Borrowed("div")),
        attributes: Vec::new(),
        children: Vec::new(),
        key: None,
        props: None,
    };
    let rendered: VirtualNode = euv_panel(bare);
    assert!(
        total_elements(&rendered) > 0,
        "the panel frame renders even with no props, got {rendered:?}"
    );
}
