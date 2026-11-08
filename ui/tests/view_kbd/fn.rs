use super::*;

fn kbd_node(text: &'static str) -> VirtualNode<EuvKbdProps> {
    VirtualNode::Element {
        tag: Tag::Element(Cow::Borrowed("div")),
        attributes: Vec::new(),
        children: Vec::new(),
        key: None,
        props: Some(Box::new(EuvKbdProps { text })),
    }
}

fn tag_name(node: &VirtualNode) -> String {
    match node {
        VirtualNode::Element { tag, .. } => format!("{tag:?}"),
        _ => panic!("unexpected node variant: {node:?}"),
    }
}

#[test]
fn a_kbd_renders_a_kbd_element() {
    let rendered: VirtualNode = euv_kbd(kbd_node("K"));
    assert_eq!(
        tag_name(&rendered),
        "Element(\"kbd\")",
        "a key is a kbd element, so the browser reads it as one"
    );
}

#[test]
fn a_kbd_carries_the_key_class() {
    let rendered: VirtualNode = euv_kbd(kbd_node("K"));
    assert!(
        format!("{rendered:?}").contains("class"),
        "the key carries the registry class: {rendered:?}"
    );
}
