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

fn attribute_value(node: &VirtualNode, name: &str) -> Option<String> {
    match node {
        VirtualNode::Element { attributes, .. } => attributes
            .iter()
            .find(|entry: &&AttributeEntry| entry.get_name() == name)
            .and_then(AttributeEntry::text_value),
        _ => None,
    }
}

#[test]
fn the_alert_dialog_root_reports_its_open_state() {
    let closed: VirtualNode = euv_alert_dialog(props_node(EuvAlertDialogProps {
        open: Signal::create(false),
    }));
    let open: VirtualNode = euv_alert_dialog(props_node(EuvAlertDialogProps {
        open: Signal::create(true),
    }));
    assert_eq!(
        attribute_value(&closed, "data-state").as_deref(),
        Some("closed")
    );
    assert_eq!(
        attribute_value(&open, "data-state").as_deref(),
        Some("open")
    );
}

#[test]
fn an_alert_dialog_panel_is_a_modal_alert_dialog() {
    let node: VirtualNode = euv_alert_dialog_content(props_node(EuvAlertDialogContentProps {
        open: Signal::create(true),
        id: "alert-content",
        title_id: "alert-title",
        description_id: "alert-description",
    }));
    assert!(has_class(&node, "c_euv_alert_dialog_content"));
    assert!(has_class(&node, "c_euv_alert_dialog_content_open"));
    assert_eq!(
        attribute_value(&node, "aria-labelledby").as_deref(),
        Some("alert-title")
    );
    assert_eq!(
        attribute_value(&node, "aria-describedby").as_deref(),
        Some("alert-description")
    );
    assert!(has_attribute(&node, "role"));
    assert!(has_attribute(&node, "aria-modal"));
    assert!(has_attribute(&node, "tabindex"));
    assert!(has_attribute(&node, "onkeydown"));
    assert_eq!(
        attribute_value(&node, "id").as_deref(),
        Some("alert-content")
    );
}

#[test]
fn an_alert_dialog_trigger_controls_its_panel_and_names_its_popup() {
    let node: VirtualNode = euv_alert_dialog_trigger(props_node(EuvAlertDialogTriggerProps {
        open: Signal::create(false),
        content_id: "alert-content",
        label: "Open confirmation",
    }));
    assert_eq!(
        attribute_value(&node, "aria-haspopup").as_deref(),
        Some("dialog")
    );
    assert_eq!(
        attribute_value(&node, "aria-controls").as_deref(),
        Some("alert-content")
    );
    assert!(has_attribute(&node, "onclick"));
}

#[test]
fn an_alert_dialog_carries_two_dismissable_buttons() {
    let open: Signal<bool> = Signal::create(true);
    let action: VirtualNode = euv_alert_dialog_action(props_node(EuvAlertDialogActionProps {
        label: "Continue",
        variant: Some(EuvButtonVariant::Destructive),
        open,
        onclick: None,
    }));
    assert!(has_class(&action, "c_euv_button_destructive"));
    assert_eq!(
        attribute_value(&action, "data-slot").as_deref(),
        Some("alert-dialog-action")
    );
    assert_eq!(
        attribute_value(&action, "data-size").as_deref(),
        Some("default")
    );
    assert!(has_attribute(&action, "onclick"));
    let cancel: VirtualNode = euv_alert_dialog_cancel(props_node(EuvAlertDialogCancelProps {
        label: "Cancel",
        open,
        onclick: None,
    }));
    assert!(has_class(&cancel, "c_euv_button_outline"));
    assert_eq!(
        attribute_value(&cancel, "data-slot").as_deref(),
        Some("alert-dialog-cancel")
    );
    assert_eq!(
        attribute_value(&cancel, "data-size").as_deref(),
        Some("default")
    );
    assert!(has_attribute(&cancel, "onclick"));
}

#[test]
fn an_alert_dialog_carries_a_head_a_title_and_a_description() {
    let header: VirtualNode =
        euv_alert_dialog_header(props_node(EuvAlertDialogHeaderProps { label: "Head" }));
    assert!(has_class(&header, "c_euv_alert_dialog_header"));
    let title: VirtualNode = euv_alert_dialog_title(props_node(EuvAlertDialogTitleProps {
        id: "alert-title",
        text: "Are you sure?",
    }));
    assert!(has_class(&title, "c_euv_alert_dialog_title"));
    assert_eq!(
        attribute_value(&title, "id").as_deref(),
        Some("alert-title")
    );
    let description: VirtualNode =
        euv_alert_dialog_description(props_node(EuvAlertDialogDescriptionProps {
            id: "alert-description",
            text: "This cannot be undone.",
        }));
    assert!(has_class(&description, "c_euv_alert_dialog_description"));
    assert_eq!(
        attribute_value(&description, "id").as_deref(),
        Some("alert-description")
    );
}
