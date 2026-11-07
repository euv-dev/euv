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
fn an_alert_dialog_panel_is_a_modal_alert_dialog() {
    let node: VirtualNode = euv_alert_dialog_content(props_node(EuvAlertDialogContentProps {
        open: Signal::create(true),
    }));
    assert!(has_class(&node, "c_euv_alert_dialog_content"));
    assert!(has_class(&node, "c_euv_alert_dialog_content_open"));
    assert!(has_attribute(&node, "role"));
    assert!(has_attribute(&node, "aria-modal"));
}

#[test]
fn an_alert_dialog_carries_two_buttons() {
    let action: VirtualNode =
        euv_alert_dialog_action(props_node(EuvAlertDialogActionProps { label: "Continue" }));
    assert!(has_class(&action, "c_euv_alert_dialog_action"));
    let cancel: VirtualNode =
        euv_alert_dialog_cancel(props_node(EuvAlertDialogCancelProps { label: "Cancel" }));
    assert!(has_class(&cancel, "c_euv_alert_dialog_cancel"));
}

#[test]
fn an_alert_dialog_carries_a_head_a_title_and_a_description() {
    let header: VirtualNode =
        euv_alert_dialog_header(props_node(EuvAlertDialogHeaderProps { label: "Head" }));
    assert!(has_class(&header, "c_euv_alert_dialog_header"));
    let title: VirtualNode = euv_alert_dialog_title(props_node(EuvAlertDialogTitleProps {
        text: "Are you sure?",
    }));
    assert!(has_class(&title, "c_euv_alert_dialog_title"));
    let description: VirtualNode =
        euv_alert_dialog_description(props_node(EuvAlertDialogDescriptionProps {
            text: "This cannot be undone.",
        }));
    assert!(has_class(&description, "c_euv_alert_dialog_description"));
}
