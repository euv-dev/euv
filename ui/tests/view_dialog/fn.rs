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
fn a_dialog_panel_is_a_modal_dialog() {
    let node: VirtualNode = euv_dialog(props_node(EuvDialogProps {
        open: Signal::create(true),
        title: "Edit",
        description: "Change the details.",
    }));
    assert!(has_class(&node, "c_euv_dialog"));
    assert!(has_class(&node, "c_euv_dialog_open"));
    assert!(has_attribute(&node, "role"));
    assert!(has_attribute(&node, "aria-modal"));
}

#[test]
fn a_closed_dialog_hides_its_panel() {
    let node: VirtualNode = euv_dialog(props_node(EuvDialogProps {
        open: Signal::create(false),
        title: "Edit",
        description: "",
    }));
    assert!(has_class(&node, "c_euv_dialog_closed"));
    assert!(has_class(&node, "c_euv_dialog_overlay_closed"));
}

#[test]
fn a_dialog_carries_a_header_a_title_a_description_and_a_footer() {
    let header: VirtualNode = euv_dialog_header(props_node(EuvDialogHeaderProps { label: "Head" }));
    assert!(has_class(&header, "c_euv_dialog_header"));
    let title: VirtualNode = euv_dialog_title(props_node(EuvDialogTitleProps { text: "Edit" }));
    assert!(has_class(&title, "c_euv_dialog_title"));
    let description: VirtualNode =
        euv_dialog_description(props_node(EuvDialogDescriptionProps { text: "Body" }));
    assert!(has_class(&description, "c_euv_dialog_description"));
    let footer: VirtualNode = euv_dialog_footer(props_node(EuvDialogFooterProps { label: "Foot" }));
    assert!(has_class(&footer, "c_euv_dialog_footer"));
    let close: VirtualNode = euv_dialog_close(props_node(EuvDialogCloseProps { label: "Close" }));
    assert!(has_class(&close, "c_euv_dialog_close"));
}
