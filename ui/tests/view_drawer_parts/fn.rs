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
fn the_drawer_trigger_reports_the_state_of_the_drawer() {
    let node: VirtualNode = euv_drawer_trigger(props_node(EuvDrawerTriggerProps {
        open: Signal::create(false),
    }));
    assert!(has_class(&node, "c_euv_drawer_trigger"));
    assert!(has_attribute(&node, "aria-expanded"));
    assert!(has_attribute(&node, "aria-haspopup"));
}

#[test]
fn a_closed_drawer_panel_hides_itself() {
    let node: VirtualNode = euv_drawer_content(props_node(EuvDrawerContentProps {
        open: Signal::create(false),
    }));
    assert!(has_class(&node, "c_euv_drawer_content"));
    assert!(has_class(&node, "c_euv_drawer_content_closed"));
    assert!(has_attribute(&node, "role"));
}

#[test]
fn an_open_drawer_panel_shows_itself() {
    let node: VirtualNode = euv_drawer_content(props_node(EuvDrawerContentProps {
        open: Signal::create(true),
    }));
    assert!(has_class(&node, "c_euv_drawer_content_open"));
}

#[test]
fn the_drawer_header_holds_the_title_and_the_description() {
    let node: VirtualNode = euv_drawer_header(props_node(EuvDrawerHeaderProps {}));
    assert!(has_class(&node, "c_euv_drawer_header"));
    let title: VirtualNode = euv_drawer_title(props_node(EuvDrawerTitleProps {}));
    assert!(has_class(&title, "c_euv_drawer_title"));
    let description: VirtualNode = euv_drawer_description(props_node(EuvDrawerDescriptionProps {}));
    assert!(has_class(&description, "c_euv_drawer_description"));
}

#[test]
fn the_drawer_footer_holds_the_actions() {
    let node: VirtualNode = euv_drawer_footer(props_node(EuvDrawerFooterProps {}));
    assert!(has_class(&node, "c_euv_drawer_footer"));
}

#[test]
fn the_drawer_close_is_a_button() {
    let node: VirtualNode = euv_drawer_close(props_node(EuvDrawerCloseProps {
        open: Signal::create(true),
    }));
    assert!(has_class(&node, "c_euv_drawer_close"));
    assert!(has_attribute(&node, "type"));
}
