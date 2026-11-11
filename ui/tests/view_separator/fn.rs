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
fn a_decorative_separator_carries_no_role() {
    let node: VirtualNode = euv_separator(props_node(EuvSeparatorProps {
        orientation: EuvSeparatorOrientation::Horizontal,
        decorative: true,
    }));
    assert!(has_class(&node, "c_euv_separator"));
    assert!(has_class(&node, "c_euv_separator_horizontal"));
    assert!(has_attribute(&node, "data-slot"));
    assert!(has_attribute(&node, "data-orientation"));
    if cfg!(feature = "theme-shadcn") {
        assert!(has_class(&node, "c_euv_separator_shadcn"));
    }
}

#[test]
fn a_meaningful_separator_carries_the_separator_role() {
    let node: VirtualNode = euv_separator(props_node(EuvSeparatorProps {
        orientation: EuvSeparatorOrientation::Vertical,
        decorative: false,
    }));
    assert!(has_class(&node, "c_euv_separator_vertical"));
    assert!(has_attribute(&node, "role"));
    assert!(has_attribute(&node, "aria-orientation"));
    assert!(has_attribute(&node, "data-slot"));
    assert!(has_attribute(&node, "data-orientation"));
}
