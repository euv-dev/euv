use super::*;

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

fn has_class(node: &VirtualNode, name: &str) -> bool {
    class_names(node).iter().any(|entry: &String| entry == name)
}

fn has_attribute(node: &VirtualNode, name: &str) -> bool {
    for element in elements(node) {
        if let VirtualNode::Element { attributes, .. } = &element {
            for entry in attributes.iter() {
                if entry.get_name() == name {
                    return true;
                }
            }
        }
    }
    false
}

fn has_slot(node: &VirtualNode, slot: &str) -> bool {
    for element in elements(node) {
        if let VirtualNode::Element { attributes, .. } = &element {
            for entry in attributes.iter() {
                if entry.get_name() == "data-slot" && entry.text_value().as_deref() == Some(slot) {
                    return true;
                }
            }
        }
    }
    false
}

#[test]
fn a_message_header_carries_its_class_and_slot() {
    let node: VirtualNode = euv_message_header(VirtualNode::Empty);
    assert!(has_class(&node, "c_euv_message_header"));
    assert!(has_attribute(&node, "data-slot"));
    assert!(has_slot(&node, "message-header"));
}

#[test]
fn a_message_group_carries_its_class_and_slot() {
    let node: VirtualNode = euv_message_group(VirtualNode::Empty);
    assert!(has_class(&node, "c_euv_message_group"));
    assert!(has_attribute(&node, "data-slot"));
    assert!(has_slot(&node, "message-group"));
}

#[test]
fn a_message_avatar_carries_its_class_and_slot() {
    let node: VirtualNode = euv_message_avatar(VirtualNode::Empty);
    assert!(has_class(&node, "c_euv_message_avatar"));
    assert!(has_attribute(&node, "data-slot"));
    assert!(has_slot(&node, "message-avatar"));
}

#[test]
fn a_message_content_carries_its_class_and_slot() {
    let node: VirtualNode = euv_message_content(VirtualNode::Empty);
    assert!(has_class(&node, "c_euv_message_content"));
    assert!(has_attribute(&node, "data-slot"));
    assert!(has_slot(&node, "message-content"));
}

#[test]
fn a_message_footer_carries_its_class_and_slot() {
    let node: VirtualNode = euv_message_footer(VirtualNode::Empty);
    assert!(has_class(&node, "c_euv_message_footer"));
    assert!(has_attribute(&node, "data-slot"));
    assert!(has_slot(&node, "message-footer"));
}
