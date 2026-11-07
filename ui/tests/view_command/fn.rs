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
fn a_command_is_a_frame_that_holds_the_rows() {
    let node: VirtualNode = euv_command(props_node(EuvCommandProps {
        value: Signal::create(String::new()),
        label: "Commands",
    }));
    assert!(has_class(&node, "c_euv_command"));
    assert!(has_attribute(&node, "aria-label"));
}

#[test]
fn the_command_field_is_a_combobox_that_reports_its_text() {
    let node: VirtualNode = euv_command_input(props_node(EuvCommandInputProps {
        value: Signal::create(String::from("re")),
        placeholder: "Type a command",
        disabled: false,
    }));
    assert!(has_class(&node, "c_euv_command_input_wrapper"));
    assert!(has_class(&node, "c_euv_command_input"));
    assert!(has_attribute(&node, "role"));
}

#[test]
fn the_command_rows_live_in_a_listbox() {
    let node: VirtualNode = euv_command_list(props_node(EuvCommandListProps { label: "Results" }));
    assert!(has_class(&node, "c_euv_command_list"));
    assert!(has_attribute(&node, "role"));
}

#[test]
fn a_command_row_is_an_option_that_reports_its_state() {
    let node: VirtualNode = euv_command_item(props_node(EuvCommandItemProps {
        value: "open",
        label: "Open",
        selected: true,
        disabled: false,
        on_select: None,
    }));
    assert!(has_class(&node, "c_euv_command_item_selected"));
    assert!(has_attribute(&node, "role"));
    assert!(has_attribute(&node, "aria-selected"));
}

#[test]
fn a_disabled_command_row_is_marked_inert() {
    let node: VirtualNode = euv_command_item(props_node(EuvCommandItemProps {
        value: "open",
        label: "Open",
        selected: false,
        disabled: true,
        on_select: None,
    }));
    assert!(has_class(&node, "c_euv_command_item_disabled"));
    assert!(has_attribute(&node, "aria-disabled"));
}

#[test]
fn a_command_carries_an_empty_message_a_group_and_a_separator() {
    let empty: VirtualNode = euv_command_empty(props_node(EuvCommandEmptyProps {
        text: "No results.",
    }));
    assert!(has_class(&empty, "c_euv_command_empty"));
    let group: VirtualNode =
        euv_command_group(props_node(EuvCommandGroupProps { heading: "Files" }));
    assert!(has_class(&group, "c_euv_command_group"));
    assert!(has_class(&group, "c_euv_command_group_heading"));
    let separator: VirtualNode =
        euv_command_separator(props_node(EuvCommandSeparatorProps { label: "" }));
    assert!(has_class(&separator, "c_euv_command_separator"));
    assert!(has_attribute(&separator, "role"));
}
