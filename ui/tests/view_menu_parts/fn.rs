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
fn a_dropdown_menu_item_is_a_menu_item_command() {
    let node: VirtualNode = euv_dropdown_menu_item(props_node(EuvDropdownMenuItemProps {
        label: "Copy link",
        shortcut: "Ctrl+C",
        value: "copy",
        disabled: false,
        on_select: None,
    }));
    assert!(has_attribute(&node, "role"));
    assert!(has_class(&node, "c_euv_dropdown_menu_item"));
    assert!(has_class(&node, "c_euv_dropdown_menu_shortcut"));
    assert!(has_attribute(&node, "aria-disabled"));
}

#[test]
fn a_disabled_dropdown_menu_item_is_marked_inert() {
    let node: VirtualNode = euv_dropdown_menu_item(props_node(EuvDropdownMenuItemProps {
        label: "Copy link",
        shortcut: "",
        value: "copy",
        disabled: true,
        on_select: None,
    }));
    assert!(has_class(&node, "c_euv_dropdown_menu_item_disabled"));
    assert!(!has_class(&node, "c_euv_dropdown_menu_shortcut"));
}

#[test]
fn a_dropdown_menu_carries_a_label_and_a_separator() {
    let label: VirtualNode =
        euv_dropdown_menu_label(props_node(EuvDropdownMenuLabelProps { text: "Actions" }));
    assert!(has_class(&label, "c_euv_dropdown_menu_label"));
    let separator: VirtualNode =
        euv_dropdown_menu_separator(props_node(EuvDropdownMenuSeparatorProps { label: "" }));
    assert!(has_class(&separator, "c_euv_dropdown_menu_separator"));
    assert!(has_attribute(&separator, "role"));
}

#[test]
fn a_radio_group_is_a_radiogroup_of_radio_rows() {
    let group: VirtualNode = euv_radio_group(props_node(EuvRadioGroupRootProps { label: "Size" }));
    assert!(has_class(&group, "c_euv_radio_group_root"));
    assert!(has_attribute(&group, "role"));
    let item: VirtualNode = euv_radio_group_item(props_node(EuvRadioGroupItemProps {
        label: "Large",
        value: "large",
        checked: Signal::create(true),
        on_select: None,
    }));
    assert!(has_class(&item, "c_euv_radio_group_item_checked"));
    assert!(has_attribute(&item, "aria-checked"));
}

#[test]
fn an_unchecked_radio_row_carries_the_unchecked_class() {
    let item: VirtualNode = euv_radio_group_item(props_node(EuvRadioGroupItemProps {
        label: "Small",
        value: "small",
        checked: Signal::create(false),
        on_select: None,
    }));
    assert!(has_class(&item, "c_euv_radio_group_item"));
    assert!(!has_class(&item, "c_euv_radio_group_item_checked"));
}

#[test]
fn a_toast_is_a_live_status_carrying_its_parts() {
    let node: VirtualNode = euv_toast(props_node(EuvToastProps {
        variant: EuvToastVariant::Default,
        title: "Saved",
        description: "The document is saved.",
        action_label: "Undo",
        dismissible: true,
        on_action: None,
        on_close: None,
    }));
    assert!(has_class(&node, "c_euv_toast"));
    assert!(has_class(&node, "c_euv_toast_title"));
    assert!(has_class(&node, "c_euv_toast_description"));
    assert!(has_class(&node, "c_euv_toast_action"));
    assert!(has_class(&node, "c_euv_toast_close"));
    assert!(has_attribute(&node, "aria-live"));
}

#[test]
fn a_destructive_toast_paints_the_destructive_surface() {
    let node: VirtualNode = euv_toast(props_node(EuvToastProps {
        variant: EuvToastVariant::Destructive,
        title: "Failed",
        description: "The upload did not finish.",
        action_label: "",
        dismissible: false,
        on_action: None,
        on_close: None,
    }));
    assert!(has_class(&node, "c_euv_toast_destructive"));
    assert!(!has_class(&node, "c_euv_toast_close"));
}

#[test]
fn a_toast_title_and_description_can_be_used_on_their_own() {
    let title: VirtualNode = euv_toast_title(props_node(EuvToastTitleProps { text: "Saved" }));
    assert!(has_class(&title, "c_euv_toast_title"));
    let description: VirtualNode =
        euv_toast_description(props_node(EuvToastDescriptionProps { text: "All good." }));
    assert!(has_class(&description, "c_euv_toast_description"));
    let action: VirtualNode = euv_toast_action(props_node(EuvToastActionProps {
        label: "Undo",
        on_click: None,
    }));
    assert!(has_class(&action, "c_euv_toast_action"));
}

#[test]
fn a_menubar_is_a_bar_of_menus() {
    let bar: VirtualNode = euv_menubar(props_node(EuvMenubarProps { label: "Main" }));
    assert!(has_class(&bar, "c_euv_menubar"));
    assert!(has_attribute(&bar, "role"));
}

#[test]
fn a_menubar_trigger_reports_the_state_of_its_menu() {
    let trigger: VirtualNode = euv_menubar_trigger(props_node(EuvMenubarTriggerProps {
        label: "File",
        open: Signal::create(true),
    }));
    assert!(has_class(&trigger, "c_euv_menubar_trigger_open"));
    assert!(has_attribute(&trigger, "aria-haspopup"));
    assert!(has_attribute(&trigger, "aria-expanded"));
}

#[test]
fn a_menubar_content_holds_menu_rows() {
    let content: VirtualNode = euv_menubar_content(props_node(EuvMenubarContentProps {
        open: Signal::create(false),
    }));
    assert!(has_class(&content, "c_euv_menubar_content_closed"));
    assert!(has_attribute(&content, "role"));
    let item: VirtualNode = euv_menubar_item(props_node(EuvMenubarItemProps {
        label: "New",
        value: "new",
        on_select: None,
    }));
    assert!(has_class(&item, "c_euv_menubar_item"));
    assert!(has_attribute(&item, "role"));
}
