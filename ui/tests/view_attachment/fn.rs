use super::*;

fn props_node<T: Clone + 'static>(props: T) -> VirtualNode<T> {
    VirtualNode::Element {
        tag: Tag::Element(Cow::Borrowed("div")),
        attributes: Vec::new(),
        children: Vec::new(),
        key: None,
        props: Some(Box::new(props)),
    }
}

fn elements(node: &VirtualNode) -> Vec<VirtualNode> {
    let mut out: Vec<VirtualNode> = Vec::new();
    collect(node, &mut out);
    out
}

fn collect(node: &VirtualNode, out: &mut Vec<VirtualNode>) {
    match node {
        VirtualNode::Element { children, .. } => {
            out.push(node.clone());
            for child in children.iter() {
                collect(child, out);
            }
        }
        VirtualNode::Fragment(children) => {
            for child in children.iter() {
                collect(child, out);
            }
        }
        _ => {}
    }
}

fn class_names(node: &VirtualNode) -> Vec<String> {
    let mut out: Vec<String> = Vec::new();
    for element in elements(node) {
        if let VirtualNode::Element { attributes, .. } = &element {
            for entry in attributes.iter() {
                if entry.get_name() == "class" {
                    out.extend(entry.class_names());
                }
            }
        }
    }
    out
}

fn attribute_names(node: &VirtualNode) -> Vec<String> {
    let mut out: Vec<String> = Vec::new();
    for element in elements(node) {
        if let VirtualNode::Element { attributes, .. } = &element {
            for entry in attributes.iter() {
                out.push(entry.get_name().to_string());
            }
        }
    }
    out
}

fn has_class(node: &VirtualNode, name: &str) -> bool {
    class_names(node).iter().any(|entry: &String| entry == name)
}

fn has_attribute(node: &VirtualNode, name: &str) -> bool {
    attribute_names(node)
        .iter()
        .any(|entry: &String| entry == name)
}

#[test]
fn an_attachment_carries_its_state_and_orientation() {
    let node: VirtualNode = euv_attachment(props_node(EuvAttachmentProps {
        state: EuvAttachmentState::Done,
        size: EuvAttachmentSize::Default,
        orientation: EuvAttachmentOrientation::Horizontal,
    }));
    assert!(has_class(&node, "c_euv_attachment"));
    assert!(has_class(&node, "c_euv_attachment_horizontal"));
    assert!(has_class(&node, "c_euv_attachment_state_done"));
    assert!(has_attribute(&node, "data-state"));
    assert!(has_attribute(&node, "data-size"));
    assert!(has_attribute(&node, "data-orientation"));
}

#[test]
fn an_idle_attachment_draws_a_dashed_outline() {
    let node: VirtualNode = euv_attachment(props_node(EuvAttachmentProps {
        state: EuvAttachmentState::Idle,
        size: EuvAttachmentSize::Sm,
        orientation: EuvAttachmentOrientation::Vertical,
    }));
    assert!(has_class(&node, "c_euv_attachment_state_idle"));
    assert!(has_class(&node, "c_euv_attachment_vertical"));
}

#[test]
fn a_failed_attachment_says_so() {
    let node: VirtualNode = euv_attachment(props_node(EuvAttachmentProps {
        state: EuvAttachmentState::Error,
        size: EuvAttachmentSize::Xs,
        orientation: EuvAttachmentOrientation::Horizontal,
    }));
    assert!(has_class(&node, "c_euv_attachment_state_error"));
}

#[test]
fn the_attachment_media_carries_its_variant() {
    let node: VirtualNode = euv_attachment_media(props_node(EuvAttachmentMediaProps {
        variant: EuvAttachmentMediaVariant::Icon,
        size: EuvAttachmentSize::Xs,
    }));
    assert!(has_class(&node, "c_euv_attachment_media_size_xs"));
    assert!(has_attribute(&node, "data-variant"));
}

#[test]
fn the_attachment_trigger_is_a_button() {
    let node: VirtualNode = euv_attachment_trigger(VirtualNode::Empty);
    assert!(has_class(&node, "c_euv_attachment_trigger"));
    assert!(has_attribute(&node, "type"));
}
