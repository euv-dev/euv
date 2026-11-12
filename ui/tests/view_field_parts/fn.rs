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
fn a_field_set_carries_its_class_and_slot() {
    let node: VirtualNode = euv_field_set(VirtualNode::Empty);
    assert!(has_class(&node, "c_euv_field_set"));
    assert!(has_attribute(&node, "data-slot"));
    assert!(has_slot(&node, "field-set"));
}

#[test]
fn a_field_legend_carries_its_class_and_slot() {
    let node: VirtualNode = euv_field_legend(VirtualNode::Empty);
    assert!(has_class(&node, "c_euv_field_legend"));
    assert!(has_attribute(&node, "data-slot"));
    assert!(has_slot(&node, "field-legend"));
}

#[test]
fn a_field_group_carries_its_class_and_slot() {
    let node: VirtualNode = euv_field_group(VirtualNode::Empty);
    assert!(has_class(&node, "c_euv_field_group"));
    assert!(has_attribute(&node, "data-slot"));
    assert!(has_slot(&node, "field-group"));
}

#[test]
fn a_field_content_carries_its_class_and_slot() {
    let node: VirtualNode = euv_field_content(VirtualNode::Empty);
    assert!(has_class(&node, "c_euv_field_content"));
    assert!(has_attribute(&node, "data-slot"));
    assert!(has_slot(&node, "field-content"));
}

#[test]
fn a_field_label_carries_its_class_and_slot() {
    let node: VirtualNode = euv_field_label(VirtualNode::Empty);
    assert!(has_class(&node, "c_euv_field_label"));
    assert!(has_attribute(&node, "data-slot"));
    assert!(has_slot(&node, "field-label"));
}

#[test]
fn a_field_title_carries_its_class_and_slot() {
    let node: VirtualNode = euv_field_title(VirtualNode::Empty);
    assert!(has_class(&node, "c_euv_field_title"));
    assert!(has_attribute(&node, "data-slot"));
    assert!(has_slot(&node, "field-label"));
}

#[test]
fn a_field_description_carries_its_class_and_slot() {
    let node: VirtualNode = euv_field_description(VirtualNode::Empty);
    assert!(has_class(&node, "c_euv_field_description"));
    assert!(has_attribute(&node, "data-slot"));
    assert!(has_slot(&node, "field-description"));
}

#[test]
fn a_field_error_carries_its_class_and_slot() {
    let node: VirtualNode = euv_field_error(VirtualNode::Empty);
    assert!(has_class(&node, "c_euv_field_error"));
    assert!(has_attribute(&node, "data-slot"));
    assert!(has_slot(&node, "field-error"));
}

#[test]
fn a_field_separator_carries_its_class_and_slot() {
    let node: VirtualNode = euv_field_separator(VirtualNode::Empty);
    assert!(has_class(&node, "c_euv_field_separator"));
    assert!(has_attribute(&node, "data-slot"));
    assert!(has_slot(&node, "field-separator"));
}
