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

#[test]
fn a_hover_card_panel_is_hidden_while_the_card_is_closed() {
    let node: VirtualNode = euv_hover_card_content(props_node(EuvHoverCardContentProps {
        open: Signal::create(false),
    }));
    assert!(has_class(&node, "c_euv_hover_card_content_closed"));
    assert!(!has_class(&node, "c_euv_hover_card_content_open"));
}

#[test]
fn a_hover_card_panel_shows_its_face_when_it_is_open() {
    let node: VirtualNode = euv_hover_card_content(props_node(EuvHoverCardContentProps {
        open: Signal::create(true),
    }));
    assert!(has_class(&node, "c_euv_hover_card_content_open"));
    assert!(has_class(&node, "c_euv_hover_card_content"));
}

#[test]
fn a_hover_card_trigger_carries_its_label() {
    let node: VirtualNode = euv_hover_card_trigger(props_node(EuvHoverCardTriggerProps {
        open: Signal::create(false),
        label: "@euv",
    }));
    assert!(has_class(&node, "c_euv_hover_card_trigger"));
}
