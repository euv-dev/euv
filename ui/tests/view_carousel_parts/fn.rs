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
fn a_carousel_content_carries_its_class_and_slot() {
    let node: VirtualNode = euv_carousel_content(VirtualNode::Empty);
    assert!(has_class(&node, "c_euv_carousel_content"));
    assert!(has_attribute(&node, "data-slot"));
    assert!(has_slot(&node, "carousel-content"));
}

#[test]
fn a_carousel_item_carries_its_class_and_slot() {
    let node: VirtualNode = euv_carousel_item(VirtualNode::Empty);
    assert!(has_class(&node, "c_euv_carousel_item"));
    assert!(has_attribute(&node, "data-slot"));
    assert!(has_slot(&node, "carousel-item"));
}

#[test]
fn a_carousel_next_carries_its_class_and_slot() {
    let node: VirtualNode = euv_carousel_next(VirtualNode::Empty);
    assert!(has_class(&node, "c_euv_carousel_next"));
    assert!(has_attribute(&node, "data-slot"));
    assert!(has_slot(&node, "carousel-next"));
    assert!(has_attribute(&node, "type"));
    assert!(has_attribute(&node, "aria-label"));
    assert!(has_attribute(&node, "disabled"));
    assert!(has_attribute(&node, "aria-hidden"));
    assert_eq!(elements(&node).len(), 2);
}

#[test]
fn a_carousel_previous_carries_its_class_and_slot() {
    let node: VirtualNode = euv_carousel_previous(VirtualNode::Empty);
    assert!(has_class(&node, "c_euv_carousel_previous"));
    assert!(has_attribute(&node, "data-slot"));
    assert!(has_slot(&node, "carousel-previous"));
    assert!(has_attribute(&node, "type"));
    assert!(has_attribute(&node, "aria-label"));
    assert!(has_attribute(&node, "disabled"));
    assert!(has_attribute(&node, "aria-hidden"));
    assert_eq!(elements(&node).len(), 2);
}
