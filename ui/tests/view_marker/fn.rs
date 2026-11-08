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
fn a_marker_carries_its_slot_and_variant() {
    let node: VirtualNode = euv_marker(props_node(EuvMarkerProps {
        variant: EuvMarkerVariant::Default,
    }));
    assert!(has_class(&node, "c_euv_marker"));
    assert!(has_class(&node, "c_euv_marker_default"));
    assert!(has_attribute(&node, "data-slot"));
    assert!(has_attribute(&node, "data-variant"));
}

#[test]
fn a_separator_marker_does_not_take_the_row() {
    let node: VirtualNode = euv_marker(props_node(EuvMarkerProps {
        variant: EuvMarkerVariant::Separator,
    }));
    assert!(has_class(&node, "c_euv_marker_separator"));
}

#[test]
fn a_border_marker_draws_the_rule_above_the_content() {
    let node: VirtualNode = euv_marker(props_node(EuvMarkerProps {
        variant: EuvMarkerVariant::Border,
    }));
    assert!(has_class(&node, "c_euv_marker_border"));
}

#[test]
fn the_marker_icon_is_decorative() {
    let node: VirtualNode = euv_marker_icon(VirtualNode::Empty);
    assert!(has_class(&node, "c_euv_marker_icon"));
    assert!(has_attribute(&node, "aria-hidden"));
}
