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
fn a_empty_header_carries_its_class_and_slot() {
    let node: VirtualNode = euv_empty_header(VirtualNode::Empty);
    assert!(has_class(&node, "c_euv_empty_header"));
    assert!(has_attribute(&node, "data-slot"));
    assert!(has_slot(&node, "empty-header"));
}

#[test]
fn a_empty_media_carries_its_class_and_slot() {
    let node: VirtualNode = euv_empty_media(VirtualNode::Empty);
    assert!(has_class(&node, "c_euv_empty_media"));
    assert!(has_attribute(&node, "data-slot"));
    assert!(has_slot(&node, "empty-media"));
}

#[test]
fn a_empty_title_carries_its_class_and_slot() {
    let node: VirtualNode = euv_empty_title(VirtualNode::Empty);
    assert!(has_class(&node, "c_euv_empty_title"));
    assert!(has_attribute(&node, "data-slot"));
    assert!(has_slot(&node, "empty-title"));
}

#[test]
fn a_empty_description_carries_its_class_and_slot() {
    let node: VirtualNode = euv_empty_description(VirtualNode::Empty);
    assert!(has_class(&node, "c_euv_empty_description"));
    assert!(has_attribute(&node, "data-slot"));
    assert!(has_slot(&node, "empty-description"));
}

#[test]
fn a_empty_content_carries_its_class_and_slot() {
    let node: VirtualNode = euv_empty_content(VirtualNode::Empty);
    assert!(has_class(&node, "c_euv_empty_content"));
    assert!(has_attribute(&node, "data-slot"));
    assert!(has_slot(&node, "empty-content"));
}
