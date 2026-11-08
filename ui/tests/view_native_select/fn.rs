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
fn a_native_select_carries_the_wrapper_and_the_control() {
    let node: VirtualNode = euv_native_select(props_node(EuvNativeSelectProps {
        size: EuvNativeSelectSize::Default,
        name: "fruit",
        disabled: false,
    }));
    assert!(has_class(&node, "c_euv_native_select_wrapper"));
    assert!(has_class(&node, "c_euv_native_select"));
    assert!(has_class(&node, "c_euv_native_select_size_default"));
    assert!(has_attribute(&node, "data-slot"));
    assert!(has_attribute(&node, "name"));
}

#[test]
fn a_small_native_select_carries_the_small_size() {
    let node: VirtualNode = euv_native_select(props_node(EuvNativeSelectProps {
        size: EuvNativeSelectSize::Sm,
        name: "fruit",
        disabled: false,
    }));
    assert!(has_class(&node, "c_euv_native_select_size_sm"));
    assert!(has_class(&node, "c_euv_native_select_icon_sm"));
}

#[test]
fn a_native_select_option_carries_its_value() {
    let node: VirtualNode = euv_native_select_option(props_node(EuvNativeSelectOptionProps {
        value: "apple",
        label: "Apple",
    }));
    assert!(has_class(&node, "c_euv_native_select_option"));
    assert!(has_attribute(&node, "value"));
}

#[test]
fn a_native_select_group_carries_its_label() {
    let node: VirtualNode =
        euv_native_select_opt_group(props_node(EuvNativeSelectOptGroupProps { label: "Fruit" }));
    assert!(has_class(&node, "c_euv_native_select_optgroup"));
    assert!(has_attribute(&node, "label"));
}
