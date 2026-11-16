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

fn has_attribute_value(node: &VirtualNode, name: &str, value: &str) -> bool {
    elements(node)
        .iter()
        .any(|element: &VirtualNode| match element {
            VirtualNode::Element { attributes, .. } => {
                attributes.iter().any(|attribute: &AttributeEntry| {
                    attribute.get_name() == name && attribute.text_value().as_deref() == Some(value)
                })
            }
            _ => false,
        })
}

#[test]
fn a_native_select_carries_the_wrapper_and_the_control() {
    let node: VirtualNode = euv_native_select(props_node(EuvNativeSelectProps {
        size: EuvNativeSelectSize::Default,
        id: "environment-select",
        name: "fruit",
        aria_label: "Fruit".to_string(),
        disabled: false,
        required: false,
        invalid: false,
        value: Signal::create(String::new()),
        onchange: None,
    }));
    assert!(has_class(&node, "c_euv_native_select_wrapper"));
    assert!(has_class(&node, "c_euv_native_select"));
    assert!(has_class(&node, "c_euv_native_select_size_default"));
    if cfg!(feature = "theme-shadcn") {
        assert!(has_class(&node, "c_euv_native_select"));
        assert!(has_class(&node, "c_euv_native_select_size_default"));
    }
    assert!(has_attribute(&node, "data-slot"));
    assert!(has_attribute_value(&node, "id", "environment-select"));
    assert!(has_attribute(&node, "name"));
    assert!(has_attribute_value(&node, "aria-label", "Fruit"));
    assert!(has_attribute_value(&node, "aria-invalid", "false"));
    assert!(has_attribute_value(&node, "disabled", ""));
    assert!(has_attribute_value(&node, "required", ""));
}

#[test]
fn a_small_native_select_carries_the_small_size() {
    let node: VirtualNode = euv_native_select(props_node(EuvNativeSelectProps {
        size: EuvNativeSelectSize::Sm,
        id: "environment-select-small",
        name: "fruit",
        aria_label: "Fruit".to_string(),
        disabled: false,
        required: false,
        invalid: false,
        value: Signal::create(String::new()),
        onchange: None,
    }));
    assert!(has_class(&node, "c_euv_native_select_size_sm"));
    assert!(has_class(&node, "c_euv_native_select_icon"));
}

#[test]
fn a_native_select_option_carries_its_value() {
    let node: VirtualNode = euv_native_select_option(props_node(EuvNativeSelectOptionProps {
        value: "apple",
        label: "Apple",
        disabled: false,
    }));
    assert!(has_class(&node, "c_euv_native_select_option"));
    assert!(has_attribute(&node, "value"));
    assert!(has_attribute_value(&node, "disabled", ""));
}

#[test]
fn a_disabled_native_select_option_is_excluded() {
    let node: VirtualNode = euv_native_select_option(props_node(EuvNativeSelectOptionProps {
        value: "maintenance",
        label: "Maintenance window",
        disabled: true,
    }));
    assert!(has_attribute(&node, "disabled"));
    assert!(has_attribute_value(&node, "disabled", "true"));
}

#[test]
fn a_native_select_group_carries_its_label() {
    let node: VirtualNode = euv_native_select_opt_group(props_node(EuvNativeSelectOptGroupProps {
        label: "Fruit",
        disabled: false,
    }));
    assert!(has_class(&node, "c_euv_native_select_optgroup"));
    assert!(has_attribute(&node, "label"));
    assert!(has_attribute_value(&node, "disabled", ""));
}

#[test]
fn a_disabled_native_select_group_disables_its_options() {
    let node: VirtualNode = euv_native_select_opt_group(props_node(EuvNativeSelectOptGroupProps {
        label: "Provisioned",
        disabled: true,
    }));
    assert!(has_attribute(&node, "disabled"));
    assert!(has_attribute_value(&node, "disabled", "true"));
}

#[test]
fn a_disabled_native_select_dims_its_wrapper() {
    let node: VirtualNode = euv_native_select(props_node(EuvNativeSelectProps {
        size: EuvNativeSelectSize::Default,
        id: "disabled-environment",
        name: "environment",
        aria_label: "Environment".to_string(),
        disabled: true,
        required: false,
        invalid: false,
        value: Signal::create(String::new()),
        onchange: None,
    }));
    assert!(has_class(&node, "c_euv_native_select_wrapper_disabled"));
    assert!(has_attribute(&node, "disabled"));
    assert!(has_attribute_value(&node, "disabled", "true"));
}

#[test]
fn a_native_select_carries_the_value_it_shows() {
    let node: VirtualNode = euv_native_select(props_node(EuvNativeSelectProps {
        size: EuvNativeSelectSize::Default,
        id: "language-select",
        name: "language",
        aria_label: "Language".to_string(),
        disabled: false,
        required: false,
        invalid: false,
        value: Signal::create("zh-CN".to_string()),
        onchange: None,
    }));
    assert!(has_attribute(&node, "value"));
}

#[test]
fn a_native_select_carries_the_chevron_that_marks_it_as_a_picker() {
    let node: VirtualNode = euv_native_select(props_node(EuvNativeSelectProps {
        size: EuvNativeSelectSize::Default,
        id: "language-select-icon",
        name: "language",
        aria_label: "Language".to_string(),
        disabled: false,
        required: false,
        invalid: false,
        value: Signal::create(String::new()),
        onchange: None,
    }));
    assert!(has_class(&node, "c_euv_native_select_icon"));
    assert!(has_attribute(&node, "inner_html"));
}

#[test]
fn an_invalid_required_native_select_exposes_its_form_state() {
    let node: VirtualNode = euv_native_select(props_node(EuvNativeSelectProps {
        size: EuvNativeSelectSize::Default,
        id: "invalid-environment",
        name: "environment",
        aria_label: "Environment".to_string(),
        disabled: false,
        required: true,
        invalid: true,
        value: Signal::create(String::new()),
        onchange: None,
    }));
    assert!(has_attribute(&node, "required"));
    assert!(has_attribute_value(&node, "required", "true"));
    assert!(has_attribute_value(&node, "aria-invalid", "true"));
    assert!(has_class(&node, "c_euv_native_select_invalid"));
}
