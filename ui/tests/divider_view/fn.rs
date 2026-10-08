use super::*;

fn blank_node() -> VirtualNode<EuvDividerProps> {
    VirtualNode::Element {
        tag: Tag::Element(Cow::Borrowed("div")),
        attributes: Vec::new(),
        children: Vec::new(),
        key: None,
        props: None,
    }
}

#[test]
fn a_styled_view_component_renders_without_a_dom() {
    let rendered: VirtualNode = euv_divider(blank_node());
    assert!(
        matches!(rendered, VirtualNode::Element { .. }),
        "a component that only calls c_*() class builders must build its node \
         without touching a document"
    );
}

#[test]
fn the_divider_wraps_a_label_when_one_is_supplied() {
    let node: VirtualNode<EuvDividerProps> = VirtualNode::Element {
        tag: Tag::Element(Cow::Borrowed("div")),
        attributes: Vec::new(),
        children: Vec::new(),
        key: None,
        props: Some(Box::new(EuvDividerProps {
            orientation: EuvDividerOrientation::Horizontal,
            label: "or",
        })),
    };
    let rendered: VirtualNode = euv_divider(node);
    let children: &[VirtualNode] = rendered.get_children();
    assert_eq!(
        children.len(),
        1,
        "a labelled divider renders one extra wrapper element, got {children:?}"
    );
}

#[test]
fn an_unlabelled_divider_renders_only_its_root() {
    let rendered: VirtualNode = euv_divider(blank_node());
    assert!(
        rendered.get_children().is_empty(),
        "without a label there is nothing to nest inside the rule"
    );
}
