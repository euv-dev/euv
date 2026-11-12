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
fn a_menubar_group_carries_its_class_and_slot() {
    let node: VirtualNode = euv_menubar_group(VirtualNode::Empty);
    assert!(has_class(&node, "c_euv_menubar_group"));
    assert!(has_attribute(&node, "data-slot"));
    assert!(has_slot(&node, "menubar-group"));
}

#[test]
fn a_menubar_separator_carries_its_class_and_slot() {
    let node: VirtualNode = euv_menubar_separator(VirtualNode::Empty);
    assert!(has_class(&node, "c_euv_menubar_separator"));
    assert!(has_attribute(&node, "data-slot"));
    assert!(has_slot(&node, "menubar-separator"));
}

#[test]
fn a_menubar_label_carries_its_class_and_slot() {
    let node: VirtualNode = euv_menubar_label(VirtualNode::Empty);
    assert!(has_class(&node, "c_euv_menubar_label"));
    assert!(has_attribute(&node, "data-slot"));
    assert!(has_slot(&node, "menubar-label"));
}

#[test]
fn a_menubar_shortcut_carries_its_class_and_slot() {
    let node: VirtualNode = euv_menubar_shortcut(VirtualNode::Empty);
    assert!(has_class(&node, "c_euv_menubar_shortcut"));
    assert!(has_attribute(&node, "data-slot"));
    assert!(has_slot(&node, "menubar-shortcut"));
}

#[test]
fn a_menubar_checkbox_item_carries_its_class_and_slot() {
    let node: VirtualNode = euv_menubar_checkbox_item(VirtualNode::Empty);
    assert!(has_class(&node, "c_euv_menubar_checkbox_item"));
    assert!(has_attribute(&node, "data-slot"));
    assert!(has_slot(&node, "menubar-checkbox-item"));
    assert!(!has_attribute(&node, "data-disabled"));
}

#[test]
fn a_disabled_menubar_checkbox_emits_only_the_active_disabled_marker() {
    let node: VirtualNode = euv_menubar_checkbox_item(props_node(EuvMenubarCheckboxItemProps {
        label: "Pin",
        checked: Signal::create(false),
        disabled: true,
        on_select: None,
    }));
    assert!(has_attribute(&node, "disabled"));
    assert!(has_attribute(&node, "data-disabled"));
}

#[test]
fn a_menubar_radio_item_carries_its_class_and_slot() {
    let node: VirtualNode = euv_menubar_radio_item(VirtualNode::Empty);
    assert!(has_class(&node, "c_euv_menubar_radio_item"));
    assert!(has_attribute(&node, "data-slot"));
    assert!(has_slot(&node, "menubar-radio-item"));
}

#[test]
fn a_menubar_radio_group_carries_its_class_and_slot() {
    let node: VirtualNode = euv_menubar_radio_group(VirtualNode::Empty);
    assert!(has_class(&node, "c_euv_menubar_radio_group"));
    assert!(has_attribute(&node, "data-slot"));
    assert!(has_slot(&node, "menubar-radio-group"));
}

#[test]
fn a_menubar_sub_carries_its_class_and_slot() {
    let node: VirtualNode = euv_menubar_sub(VirtualNode::Empty);
    assert!(has_class(&node, "c_euv_menubar_sub"));
    assert!(has_attribute(&node, "data-slot"));
    assert!(has_slot(&node, "menubar-sub"));
}

#[test]
fn a_menubar_sub_trigger_carries_its_class_and_slot() {
    let node: VirtualNode = euv_menubar_sub_trigger(VirtualNode::Empty);
    assert!(has_class(&node, "c_euv_menubar_sub_trigger"));
    assert!(has_attribute(&node, "data-slot"));
    assert!(has_slot(&node, "menubar-sub-trigger"));
}

#[test]
fn a_menubar_sub_content_carries_its_class_and_slot() {
    let node: VirtualNode = euv_menubar_sub_content(VirtualNode::Empty);
    assert!(has_class(&node, "c_euv_menubar_sub_content"));
    assert!(has_attribute(&node, "data-slot"));
    assert!(has_slot(&node, "menubar-sub-content"));
}
