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
fn the_message_scroller_carries_its_slot() {
    let node: VirtualNode = euv_message_scroller(props_node(EuvMessageScrollerProps {}));
    assert!(has_class(&node, "c_euv_message_scroller"));
    assert!(has_attribute(&node, "data-slot"));
}

#[test]
fn the_message_scroller_viewport_carries_its_slot() {
    let node: VirtualNode =
        euv_message_scroller_viewport(props_node(EuvMessageScrollerViewportProps {}));
    assert!(has_class(&node, "c_euv_message_scroller_viewport"));
    assert!(has_attribute(&node, "data-slot"));
}

#[test]
fn the_message_scroller_content_carries_its_slot() {
    let node: VirtualNode =
        euv_message_scroller_content(props_node(EuvMessageScrollerContentProps {}));
    assert!(has_class(&node, "c_euv_message_scroller_content"));
}

#[test]
fn a_message_scroller_item_says_whether_it_is_the_anchor() {
    let node: VirtualNode = euv_message_scroller_item(props_node(EuvMessageScrollerItemProps {
        scroll_anchor: true,
    }));
    assert!(has_class(&node, "c_euv_message_scroller_item"));
    assert!(has_attribute(&node, "data-scroll-anchor"));
}

#[test]
fn the_message_scroller_button_carries_its_direction() {
    let node: VirtualNode =
        euv_message_scroller_button(props_node(EuvMessageScrollerButtonProps {
            direction: EuvMessageScrollerDirection::Start,
        }));
    assert!(has_class(&node, "c_euv_message_scroller_button_start"));
    assert!(has_attribute(&node, "data-direction"));
}
