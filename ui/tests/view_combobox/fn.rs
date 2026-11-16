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
fn a_combobox_trigger_reports_whether_the_list_is_open() {
    let node: VirtualNode = euv_combobox_trigger(props_node(EuvComboboxTriggerProps {
        open: Signal::create(true),
        label: "Pick one",
        disabled: false,
    }));
    assert!(has_class(&node, "c_euv_combobox_trigger"));
    if cfg!(feature = "theme-shadcn") {
        assert!(has_class(&node, "c_euv_combobox_trigger"));
    }
    assert!(has_attribute(&node, "aria-haspopup"));
    assert!(has_attribute(&node, "aria-expanded"));
    assert!(!has_attribute(&node, "role"));
}

#[test]
fn a_combobox_input_reports_its_open_state_and_geometry() {
    let node: VirtualNode = euv_combobox_input(props_node(EuvComboboxInputProps {
        open: Signal::create(false),
        value: Signal::create(String::new()),
        placeholder: "Type to search",
        disabled: false,
        aria_label: "Search framework",
        aria_controls: "framework-options",
    }));
    assert!(has_class(&node, "c_euv_combobox_input"));
    assert!(has_attribute(&node, "aria-expanded"));
    assert!(has_attribute(&node, "aria-label"));
    assert!(has_attribute(&node, "aria-controls"));
    assert!(has_attribute(&node, "aria-autocomplete"));
    assert!(has_attribute(&node, "onkeydown"));
    if cfg!(feature = "theme-shadcn") {
        assert!(has_class(&node, "c_euv_combobox_input"));
    }
}

#[test]
fn the_combobox_panel_is_hidden_while_the_list_is_closed() {
    let node: VirtualNode = euv_combobox_content(props_node(EuvComboboxContentProps {
        open: Signal::create(false),
    }));
    assert!(has_class(&node, "c_euv_combobox_content_closed"));
    assert!(!has_class(&node, "c_euv_combobox_content_open"));
}

#[test]
fn the_combobox_rows_live_in_a_listbox() {
    let list: VirtualNode = euv_combobox_list(props_node(EuvComboboxListProps {
        id: "framework-options",
        label: "Options",
    }));
    assert!(has_class(&list, "c_euv_combobox_list"));
    assert!(has_attribute(&list, "role"));
    let item: VirtualNode = euv_combobox_item(props_node(EuvComboboxItemProps {
        open: Signal::create(true),
        value: "one",
        label: "One",
        selected: true,
        disabled: false,
        on_select: None,
    }));
    assert!(has_class(&item, "c_euv_combobox_item_selected"));
    assert!(has_attribute(&item, "aria-selected"));
    assert!(has_attribute(&item, "data-value"));
    assert!(has_attribute(&item, "tabindex"));
    assert!(has_attribute(&item, "onkeydown"));
}

#[test]
fn a_combobox_carries_a_label_an_empty_message_and_a_separator() {
    let label: VirtualNode =
        euv_combobox_label(props_node(EuvComboboxLabelProps { text: "Group" }));
    assert!(has_class(&label, "c_euv_combobox_label"));
    let empty: VirtualNode = euv_combobox_empty(props_node(EuvComboboxEmptyProps {
        text: "Nothing found.",
    }));
    assert!(has_class(&empty, "c_euv_combobox_empty"));
    let separator: VirtualNode =
        euv_combobox_separator(props_node(EuvComboboxSeparatorProps { label: "" }));
    assert!(has_class(&separator, "c_euv_combobox_separator"));
    assert!(has_attribute(&separator, "role"));
}
