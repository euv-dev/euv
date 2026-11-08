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
fn a_chart_carries_its_identifier() {
    let node: VirtualNode = euv_chart(props_node(EuvChartProps {
        id: "chart-sales",
        config: Vec::new(),
    }));
    assert!(has_class(&node, "c_euv_chart"));
    assert!(has_attribute(&node, "data-chart"));
}

#[test]
fn the_chart_tooltip_carries_its_slot() {
    let node: VirtualNode = euv_chart_tooltip(VirtualNode::Empty);
    assert!(has_class(&node, "c_euv_chart_tooltip"));
    assert!(has_attribute(&node, "data-slot"));
}

#[test]
fn a_chart_legend_item_carries_its_slot() {
    let node: VirtualNode = euv_chart_legend_item(props_node(EuvChartLegendItemProps {
        label: "Desktop",
        color: "var(--primary)",
    }));
    assert!(has_class(&node, "c_euv_chart_legend_item"));
    assert!(has_attribute(&node, "data-slot"));
}
