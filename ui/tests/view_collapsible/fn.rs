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
fn a_collapsible_carries_the_state_of_the_section() {
    let node: VirtualNode = euv_collapsible(props_node(EuvCollapsibleProps {
        open: Signal::create(false),
    }));
    assert!(has_class(&node, "c_euv_collapsible"));
    assert!(has_attribute(&node, "data-slot"));
    assert!(has_attribute(&node, "data-state"));
}

#[test]
fn the_collapsible_trigger_reports_the_state_of_the_section() {
    let node: VirtualNode = euv_collapsible_trigger(props_node(EuvCollapsibleTriggerProps {
        open: Signal::create(true),
    }));
    assert!(has_class(&node, "c_euv_collapsible_trigger"));
    assert!(has_attribute(&node, "aria-expanded"));
}

#[test]
fn a_closed_collapsible_hides_its_content() {
    let node: VirtualNode = euv_collapsible_content(props_node(EuvCollapsibleContentProps {
        open: Signal::create(false),
    }));
    assert!(has_class(&node, "c_euv_collapsible_content"));
    assert!(has_class(&node, "c_euv_collapsible_content_closed"));
}

#[test]
fn an_open_collapsible_shows_its_content() {
    let node: VirtualNode = euv_collapsible_content(props_node(EuvCollapsibleContentProps {
        open: Signal::create(true),
    }));
    assert!(has_class(&node, "c_euv_collapsible_content_open"));
}
