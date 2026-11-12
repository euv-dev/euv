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
fn a_sidebar_header_carries_its_class_and_slot() {
    let node: VirtualNode = euv_sidebar_header(VirtualNode::Empty);
    assert!(has_class(&node, "c_euv_sidebar_header"));
    assert!(has_attribute(&node, "data-slot"));
    assert!(has_slot(&node, "sidebar-header"));
}

#[test]
fn a_sidebar_content_carries_its_class_and_slot() {
    let node: VirtualNode = euv_sidebar_content(VirtualNode::Empty);
    assert!(has_class(&node, "c_euv_sidebar_content"));
    assert!(has_attribute(&node, "data-slot"));
    assert!(has_slot(&node, "sidebar-content"));
}

#[test]
fn a_sidebar_footer_carries_its_class_and_slot() {
    let node: VirtualNode = euv_sidebar_footer(VirtualNode::Empty);
    assert!(has_class(&node, "c_euv_sidebar_footer"));
    assert!(has_attribute(&node, "data-slot"));
    assert!(has_slot(&node, "sidebar-footer"));
}

#[test]
fn a_sidebar_group_label_carries_its_class_and_slot() {
    let node: VirtualNode = euv_sidebar_group_label(VirtualNode::Empty);
    assert!(has_class(&node, "c_euv_sidebar_group_label"));
    assert!(has_attribute(&node, "data-slot"));
    assert!(has_slot(&node, "sidebar-group-label"));
}

#[test]
fn a_sidebar_group_action_carries_its_class_and_slot() {
    let node: VirtualNode = euv_sidebar_group_action(VirtualNode::Empty);
    assert!(has_class(&node, "c_euv_sidebar_group_action"));
    assert!(has_attribute(&node, "data-slot"));
    assert!(has_slot(&node, "sidebar-group-action"));
}

#[test]
fn a_sidebar_group_content_carries_its_class_and_slot() {
    let node: VirtualNode = euv_sidebar_group_content(VirtualNode::Empty);
    assert!(has_class(&node, "c_euv_sidebar_group_content"));
    assert!(has_attribute(&node, "data-slot"));
    assert!(has_slot(&node, "sidebar-group-content"));
}

#[test]
fn a_sidebar_menu_carries_its_class_and_slot() {
    let node: VirtualNode = euv_sidebar_menu(VirtualNode::Empty);
    assert!(has_class(&node, "c_euv_sidebar_menu"));
    assert!(has_attribute(&node, "data-slot"));
    assert!(has_slot(&node, "sidebar-menu"));
}

#[test]
fn a_sidebar_menu_item_carries_its_class_and_slot() {
    let node: VirtualNode = euv_sidebar_menu_item(VirtualNode::Empty);
    assert!(has_class(&node, "c_euv_sidebar_menu_item"));
    assert!(has_attribute(&node, "data-slot"));
    assert!(has_slot(&node, "sidebar-menu-item"));
}

#[test]
fn a_sidebar_menu_button_carries_its_class_and_slot() {
    let node: VirtualNode = euv_sidebar_menu_button(VirtualNode::Empty);
    assert!(has_class(&node, "c_euv_sidebar_menu_button"));
    assert!(has_attribute(&node, "data-slot"));
    assert!(has_slot(&node, "sidebar-menu-button"));
}

#[test]
fn a_sidebar_menu_action_carries_its_class_and_slot() {
    let node: VirtualNode = euv_sidebar_menu_action(VirtualNode::Empty);
    assert!(has_class(&node, "c_euv_sidebar_menu_action"));
    assert!(has_attribute(&node, "data-slot"));
    assert!(has_slot(&node, "sidebar-menu-action"));
}

#[test]
fn a_sidebar_menu_badge_carries_its_class_and_slot() {
    let node: VirtualNode = euv_sidebar_menu_badge(VirtualNode::Empty);
    assert!(has_class(&node, "c_euv_sidebar_menu_badge"));
    assert!(has_attribute(&node, "data-slot"));
    assert!(has_slot(&node, "sidebar-menu-badge"));
}

#[test]
fn a_sidebar_menu_sub_carries_its_class_and_slot() {
    let node: VirtualNode = euv_sidebar_menu_sub(VirtualNode::Empty);
    assert!(has_class(&node, "c_euv_sidebar_menu_sub"));
    assert!(has_attribute(&node, "data-slot"));
    assert!(has_slot(&node, "sidebar-menu-sub"));
}

#[test]
fn a_sidebar_menu_sub_button_carries_its_class_and_slot() {
    let node: VirtualNode = euv_sidebar_menu_sub_button(VirtualNode::Empty);
    assert!(has_class(&node, "c_euv_sidebar_menu_sub_button"));
    assert!(has_attribute(&node, "data-slot"));
    assert!(has_slot(&node, "sidebar-menu-sub-button"));
}

#[test]
fn a_sidebar_menu_sub_item_carries_its_class_and_slot() {
    let node: VirtualNode = euv_sidebar_menu_sub_item(VirtualNode::Empty);
    assert!(has_class(&node, "c_euv_sidebar_menu_sub_item"));
    assert!(has_attribute(&node, "data-slot"));
    assert!(has_slot(&node, "sidebar-menu-sub-item"));
}

#[test]
fn a_sidebar_menu_skeleton_carries_its_class_and_slot() {
    let node: VirtualNode = euv_sidebar_menu_skeleton(VirtualNode::Empty);
    assert!(has_class(&node, "c_euv_sidebar_menu_skeleton"));
    assert!(has_attribute(&node, "data-slot"));
    assert!(has_slot(&node, "sidebar-menu-skeleton"));
}

#[test]
fn a_sidebar_input_carries_its_class_and_slot() {
    let node: VirtualNode = euv_sidebar_input(VirtualNode::Empty);
    assert!(has_class(&node, "c_euv_sidebar_input"));
    assert!(has_attribute(&node, "data-slot"));
    assert!(has_slot(&node, "sidebar-input"));
}

#[test]
fn a_sidebar_inset_carries_its_class_and_slot() {
    let node: VirtualNode = euv_sidebar_inset(VirtualNode::Empty);
    assert!(has_class(&node, "c_euv_sidebar_inset"));
    assert!(has_attribute(&node, "data-slot"));
    assert!(has_slot(&node, "sidebar-inset"));
}

#[test]
fn a_sidebar_rail_carries_its_class_and_slot() {
    let node: VirtualNode = euv_sidebar_rail(VirtualNode::Empty);
    assert!(has_class(&node, "c_euv_sidebar_rail"));
    assert!(has_attribute(&node, "data-slot"));
    assert!(has_slot(&node, "sidebar-rail"));
}

#[test]
fn a_sidebar_separator_carries_its_class_and_slot() {
    let node: VirtualNode = euv_sidebar_separator(VirtualNode::Empty);
    assert!(has_class(&node, "c_euv_sidebar_separator"));
    assert!(has_attribute(&node, "data-slot"));
    assert!(has_slot(&node, "sidebar-separator"));
}

#[test]
fn a_sidebar_trigger_carries_its_class_and_slot() {
    let node: VirtualNode = euv_sidebar_trigger(VirtualNode::Empty);
    assert!(has_class(&node, "c_euv_sidebar_trigger"));
    assert!(has_attribute(&node, "data-slot"));
    assert!(has_slot(&node, "sidebar-trigger"));
}
