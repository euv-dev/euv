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
fn the_context_menu_panel_is_a_menu() {
    let node: VirtualNode = euv_context_menu_content(props_node(EuvContextMenuContentProps {
        open: Signal::create(true),
    }));
    assert!(has_class(&node, "c_euv_context_menu_content"));
    assert!(has_class(&node, "c_euv_context_menu_content_open"));
    assert!(has_attribute(&node, "role"));
}

#[test]
fn a_context_menu_row_is_a_menu_item() {
    let node: VirtualNode = euv_context_menu_item(props_node(EuvContextMenuItemProps {
        value: "copy",
        label: "Copy",
        disabled: false,
        on_select: None,
    }));
    assert!(has_class(&node, "c_euv_context_menu_item"));
    assert!(has_attribute(&node, "role"));
}

#[test]
fn a_context_menu_carries_a_box_row_and_a_dot_row() {
    let checkbox: VirtualNode =
        euv_context_menu_checkbox_item(props_node(EuvContextMenuCheckboxItemProps {
            label: "Show grid",
            checked: Signal::create(true),
            on_select: None,
        }));
    assert!(has_class(&checkbox, "c_euv_context_menu_checkbox_item"));
    assert!(has_attribute(&checkbox, "aria-checked"));
    let radio: VirtualNode =
        euv_context_menu_radio_item(props_node(EuvContextMenuRadioItemProps {
            label: "Small",
            checked: Signal::create(false),
        }));
    assert!(has_class(&radio, "c_euv_context_menu_radio_item"));
    assert!(has_attribute(&radio, "aria-checked"));
}

#[test]
fn a_context_menu_carries_a_label_a_separator_and_a_shortcut() {
    let label: VirtualNode =
        euv_context_menu_label(props_node(EuvContextMenuLabelProps { text: "Actions" }));
    assert!(has_class(&label, "c_euv_context_menu_label"));
    let separator: VirtualNode =
        euv_context_menu_separator(props_node(EuvContextMenuSeparatorProps { label: "" }));
    assert!(has_class(&separator, "c_euv_context_menu_separator"));
    assert!(has_attribute(&separator, "role"));
    let shortcut: VirtualNode =
        euv_context_menu_shortcut(props_node(EuvContextMenuShortcutProps { text: "Ctrl+C" }));
    assert!(has_class(&shortcut, "c_euv_context_menu_shortcut"));
}
