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

fn has_attribute(node: &VirtualNode, name: &str) -> bool {
    for element in elements(node) {
        if let VirtualNode::Element { attributes, .. } = &element {
            for entry in attributes.iter() {
                if entry.get_name() == name {
                    return true;
                }
            }
        }
    }
    false
}

fn has_slot(node: &VirtualNode, slot: &str) -> bool {
    for element in elements(node) {
        if let VirtualNode::Element { attributes, .. } = &element {
            for entry in attributes.iter() {
                if entry.get_name() == "data-slot" && entry.text_value().as_deref() == Some(slot) {
                    return true;
                }
            }
        }
    }
    false
}

#[test]
fn a_menubar_group_carries_its_class_and_slot() {
    let node: VirtualNode = euv_menubar_group(VirtualNode::Empty);
    assert!(has_class(&node, "c_euv_menubar_group"));
    assert!(has_attribute(&node, "data-slot"));
    assert!(has_slot(&node, "menubar-group"));
}

#[test]
fn a_menubar_separator_carries_its_class_and_slot() {
    let node: VirtualNode = euv_menubar_separator(VirtualNode::Empty);
    assert!(has_class(&node, "c_euv_menubar_separator"));
    assert!(has_attribute(&node, "data-slot"));
    assert!(has_slot(&node, "menubar-separator"));
}

#[test]
fn a_menubar_label_carries_its_class_and_slot() {
    let node: VirtualNode = euv_menubar_label(VirtualNode::Empty);
    assert!(has_class(&node, "c_euv_menubar_label"));
    assert!(has_attribute(&node, "data-slot"));
    assert!(has_slot(&node, "menubar-label"));
}

#[test]
fn a_menubar_shortcut_carries_its_class_and_slot() {
    let node: VirtualNode = euv_menubar_shortcut(VirtualNode::Empty);
    assert!(has_class(&node, "c_euv_menubar_shortcut"));
    assert!(has_attribute(&node, "data-slot"));
    assert!(has_slot(&node, "menubar-shortcut"));
}

#[test]
fn a_menubar_checkbox_item_carries_its_class_and_slot() {
    let node: VirtualNode = euv_menubar_checkbox_item(VirtualNode::Empty);
    assert!(has_class(&node, "c_euv_menubar_checkbox_item"));
    assert!(has_attribute(&node, "data-slot"));
    assert!(has_slot(&node, "menubar-checkbox-item"));
    assert!(!has_attribute(&node, "data-disabled"));
}

#[test]
fn a_disabled_menubar_checkbox_emits_only_the_active_disabled_marker() {
    let node: VirtualNode = euv_menubar_checkbox_item(props_node(EuvMenubarCheckboxItemProps {
        label: "Pin",
        checked: Signal::create(false),
        disabled: true,
        on_select: None,
    }));
    assert!(has_attribute(&node, "disabled"));
    assert!(has_attribute(&node, "data-disabled"));
}

#[test]
fn a_menubar_radio_item_carries_its_class_and_slot() {
    let node: VirtualNode = euv_menubar_radio_item(VirtualNode::Empty);
    assert!(has_class(&node, "c_euv_menubar_radio_item"));
    assert!(has_attribute(&node, "data-slot"));
    assert!(has_slot(&node, "menubar-radio-item"));
}

#[test]
fn a_menubar_radio_group_carries_its_class_and_slot() {
    let node: VirtualNode = euv_menubar_radio_group(VirtualNode::Empty);
    assert!(has_class(&node, "c_euv_menubar_radio_group"));
    assert!(has_attribute(&node, "data-slot"));
    assert!(has_slot(&node, "menubar-radio-group"));
}

#[test]
fn a_menubar_sub_carries_its_class_and_slot() {
    let node: VirtualNode = euv_menubar_sub(VirtualNode::Empty);
    assert!(has_class(&node, "c_euv_menubar_sub"));
    assert!(has_attribute(&node, "data-slot"));
    assert!(has_slot(&node, "menubar-sub"));
}

#[test]
fn a_menubar_sub_trigger_carries_its_class_and_slot() {
    let node: VirtualNode = euv_menubar_sub_trigger(VirtualNode::Empty);
    assert!(has_class(&node, "c_euv_menubar_sub_trigger"));
    assert!(has_attribute(&node, "data-slot"));
    assert!(has_slot(&node, "menubar-sub-trigger"));
}

#[test]
fn a_menubar_sub_content_carries_its_class_and_slot() {
    let node: VirtualNode = euv_menubar_sub_content(VirtualNode::Empty);
    assert!(has_class(&node, "c_euv_menubar_sub_content"));
    assert!(has_attribute(&node, "data-slot"));
    assert!(has_slot(&node, "menubar-sub-content"));
}

fn menubar_attribute(node: &VirtualNode, name: &str) -> Option<String> {
    if let VirtualNode::Element { attributes, .. } = node {
        return attributes
            .iter()
            .find(|attribute: &&AttributeEntry| attribute.get_name() == name)
            .and_then(|attribute: &AttributeEntry| attribute.text_value());
    }
    None
}

#[test]
fn menubar_native_disabled_values_are_text_for_every_interactive_part() {
    for disabled in [false, true] {
        let nodes: Vec<VirtualNode> = vec![
            euv_menubar_trigger(props_node(EuvMenubarTriggerProps {
                disabled,
                ..Default::default()
            })),
            euv_menubar_item(props_node(EuvMenubarItemProps {
                disabled,
                ..Default::default()
            })),
            euv_menubar_checkbox_item(props_node(EuvMenubarCheckboxItemProps {
                disabled,
                ..Default::default()
            })),
            euv_menubar_radio_item(props_node(EuvMenubarRadioItemProps {
                disabled,
                ..Default::default()
            })),
            euv_menubar_sub_trigger(props_node(EuvMenubarSubTriggerProps {
                disabled,
                ..Default::default()
            })),
        ];
        for node in nodes {
            let expected: &str = if disabled { "true" } else { "false" };
            assert!(
                matches!(menubar_attribute(&node, "disabled"), Some(value) if value == expected)
            );
            if disabled {
                assert!(
                    matches!(menubar_attribute(&node, "onclick"), Some(value) if value.is_empty())
                );
            }
        }
    }
}

#[test]
fn menubar_disabled_triggers_have_no_keyboard_activation_handler() {
    for node in [
        euv_menubar_trigger(props_node(EuvMenubarTriggerProps {
            disabled: true,
            ..Default::default()
        })),
        euv_menubar_sub_trigger(props_node(EuvMenubarSubTriggerProps {
            disabled: true,
            ..Default::default()
        })),
    ] {
        assert!(matches!(menubar_attribute(&node, "onkeydown"), Some(value) if value.is_empty()));
    }
}

#[test]
fn menubar_initial_vnodes_do_not_assume_index_zero_or_unique_indices() {
    for index in [0, 0, 99] {
        let node: VirtualNode = euv_menubar_trigger(props_node(EuvMenubarTriggerProps {
            index,
            ..Default::default()
        }));
        assert!(matches!(menubar_attribute(&node, "tabindex"), Some(value) if value == "-1"));
    }
}

#[test]
fn menubar_leaf_selection_uses_only_native_click_not_a_second_keyboard_handler() {
    for node in [
        euv_menubar_item(VirtualNode::Empty),
        euv_menubar_checkbox_item(VirtualNode::Empty),
        euv_menubar_radio_item(VirtualNode::Empty),
    ] {
        assert!(has_attribute(&node, "onclick"));
        assert_eq!(menubar_attribute(&node, "onclick"), None);
        assert!(!has_attribute(&node, "onkeydown"));
    }
}

#[test]
fn menubar_roving_policy_skips_disabled_first_and_has_no_all_disabled_stop() {
    assert_eq!(
        menubar_enabled_tab_stop(&[true, false, false], None),
        Some(1)
    );
    assert_eq!(menubar_enabled_tab_stop(&[true, true], Some(0)), None);
    assert_eq!(menubar_enabled_tab_stop(&[], None), None);
}

#[test]
fn menubar_roving_policy_preserves_enabled_entry_and_repairs_disabled_entry() {
    assert_eq!(
        menubar_enabled_tab_stop(&[true, false, false], Some(2)),
        Some(2)
    );
    assert_eq!(
        menubar_enabled_tab_stop(&[true, true, false], Some(1)),
        Some(2)
    );
    assert_eq!(
        menubar_enabled_tab_stop(&[true, false, false], Some(2)),
        Some(2)
    );
    assert_eq!(
        menubar_enabled_tab_stop(&[true, false, false], None),
        Some(1)
    );
    assert_eq!(
        menubar_enabled_tab_stop(&[true, false, false], Some(99)),
        Some(1)
    );
}

#[test]
fn menubar_navigation_is_enabled_only_and_logical_in_rtl() {
    let disabled: &[bool] = &[true, false, false, false];
    assert_eq!(
        menubar_next_enabled_trigger(disabled, 1, "ArrowRight", false),
        Some(2)
    );
    assert_eq!(
        menubar_next_enabled_trigger(disabled, 1, "ArrowRight", true),
        Some(3)
    );
    assert_eq!(
        menubar_next_enabled_trigger(disabled, 1, "ArrowLeft", false),
        Some(3)
    );
    assert_eq!(
        menubar_next_enabled_trigger(disabled, 1, "ArrowLeft", true),
        Some(2)
    );
    for rtl in [false, true] {
        assert_eq!(
            menubar_next_enabled_trigger(&[true, false, false, true], 2, "Home", rtl),
            Some(1)
        );
        assert_eq!(
            menubar_next_enabled_trigger(&[true, false, false, true], 1, "End", rtl),
            Some(2)
        );
        assert_eq!(
            menubar_next_enabled_trigger(&[true, true], 0, "ArrowRight", rtl),
            None
        );
    }
}

#[test]
fn menubar_roving_policy_selects_at_most_one_enabled_dom_position() {
    for mask in 0..16 {
        let disabled: Vec<bool> = (0..4)
            .map(|index: usize| mask & (1 << index) != 0)
            .collect();
        for preferred in [None, Some(0), Some(1), Some(2), Some(3), Some(99)] {
            let selected: Option<usize> = menubar_enabled_tab_stop(&disabled, preferred);
            let stops: Vec<usize> = (0..4)
                .filter(|index: &usize| selected == Some(*index))
                .collect();
            assert_eq!(
                stops.len(),
                usize::from(disabled.iter().any(|disabled: &bool| !disabled))
            );
            if let Some(index) = selected {
                assert!(!disabled[index]);
            }
        }
    }
}
