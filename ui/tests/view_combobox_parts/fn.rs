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
fn a_combobox_chips_carries_its_class_and_slot() {
    let node: VirtualNode = euv_combobox_chips(VirtualNode::Empty);
    assert!(has_class(&node, "c_euv_combobox_chips"));
    assert!(has_attribute(&node, "data-slot"));
    assert!(has_slot(&node, "combobox-chips"));
}

#[test]
fn a_combobox_chip_carries_its_class_and_slot() {
    let node: VirtualNode = euv_combobox_chip(VirtualNode::Empty);
    assert!(has_class(&node, "c_euv_combobox_chip"));
    assert!(has_attribute(&node, "data-slot"));
    assert!(has_slot(&node, "combobox-chip"));
}

#[test]
fn a_combobox_chip_list_carries_its_class_and_slot() {
    let node: VirtualNode = euv_combobox_chip_list(VirtualNode::Empty);
    assert!(has_class(&node, "c_euv_combobox_chip_list"));
    assert!(has_attribute(&node, "data-slot"));
    assert!(has_slot(&node, "combobox-chip-list"));
}

#[test]
fn a_combobox_chips_input_carries_its_class_and_slot() {
    let node: VirtualNode = euv_combobox_chips_input(VirtualNode::Empty);
    assert!(has_class(&node, "c_euv_combobox_chips_input"));
    assert!(has_attribute(&node, "data-slot"));
    assert!(has_slot(&node, "combobox-chip-input"));
}

#[test]
fn a_combobox_value_carries_its_class_and_slot() {
    let node: VirtualNode = euv_combobox_value(VirtualNode::Empty);
    assert!(has_class(&node, "c_euv_combobox_value"));
    assert!(has_attribute(&node, "data-slot"));
    assert!(has_slot(&node, "combobox-value"));
}
