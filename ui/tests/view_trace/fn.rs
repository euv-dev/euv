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
fn the_trace_carries_its_slot() {
    let node: VirtualNode = euv_trace(props_node(EuvTraceProps {}));
    assert!(has_class(&node, "c_euv_trace"));
    assert!(has_attribute(&node, "data-slot"));
}

#[test]
fn a_failed_trace_line_is_drawn_in_the_destructive_colour() {
    let node: VirtualNode = euv_trace_line(props_node(EuvTraceLineProps {
        status: Some(EuvTraceStatus::Error),
    }));
    assert!(has_class(&node, "c_euv_trace_line_error"));
    assert!(has_attribute(&node, "data-status"));
}

#[test]
fn a_waiting_trace_line_is_dimmed() {
    let node: VirtualNode = euv_trace_line(props_node(EuvTraceLineProps {
        status: Some(EuvTraceStatus::Pending),
    }));
    assert!(has_class(&node, "c_euv_trace_line_pending"));
}

#[test]
fn the_trace_icon_is_decorative() {
    let node: VirtualNode = euv_trace_icon(props_node(EuvTraceIconProps {}));
    assert!(has_class(&node, "c_euv_trace_icon"));
    assert!(has_attribute(&node, "aria-hidden"));
}

#[test]
fn the_trace_collapsible_trigger_reports_the_state_of_the_section() {
    let node: VirtualNode =
        euv_trace_collapsible_trigger(props_node(EuvTraceCollapsibleTriggerProps {
            open: Signal::create(false),
        }));
    assert!(has_class(&node, "c_euv_trace_collapsible_trigger"));
    assert!(has_attribute(&node, "aria-expanded"));
}

#[test]
fn a_closed_trace_section_hides_its_output() {
    let node: VirtualNode =
        euv_trace_collapsible_content(props_node(EuvTraceCollapsibleContentProps {
            open: Signal::create(false),
        }));
    assert!(has_class(&node, "c_euv_trace_collapsible_content_closed"));
}
