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
fn the_resizable_group_carries_its_direction() {
    let node: VirtualNode = euv_resizable_panel_group(props_node(EuvResizablePanelGroupProps {
        direction: EuvResizableDirection::Vertical,
    }));
    assert!(has_class(&node, "c_euv_resizable_group"));
    assert!(has_class(&node, "c_euv_resizable_group_vertical"));
    assert!(has_attribute(&node, "data-panel-group-direction"));
}

#[test]
fn a_resizable_panel_carries_its_slot() {
    let node: VirtualNode = euv_resizable_panel(props_node(EuvResizablePanelProps {
        size: Signal::create(50.0),
    }));
    assert!(has_class(&node, "c_euv_resizable_panel"));
    assert!(has_attribute(&node, "data-slot"));
}

#[test]
fn the_resizable_handle_is_a_separator() {
    let node: VirtualNode = euv_resizable_handle(props_node(EuvResizableHandleProps {
        size: Signal::create(50.0),
        min: 10.0,
        max: 90.0,
        direction: EuvResizableDirection::Horizontal,
    }));
    assert!(has_class(&node, "c_euv_resizable_handle_horizontal"));
    assert!(has_attribute(&node, "role"));
    assert!(has_attribute(&node, "aria-orientation"));
}
