use super::*;

fn group_node(disabled: bool) -> VirtualNode<EuvInputGroupProps> {
    VirtualNode::Element {
        tag: Tag::Element(Cow::Borrowed("div")),
        attributes: Vec::new(),
        children: Vec::new(),
        key: None,
        props: Some(Box::new(EuvInputGroupProps {
            disabled: Signal::create(disabled),
        })),
    }
}

fn addon_node(align: EuvInputGroupAlign) -> VirtualNode<EuvInputGroupAddonProps> {
    VirtualNode::Element {
        tag: Tag::Element(Cow::Borrowed("div")),
        attributes: Vec::new(),
        children: Vec::new(),
        key: None,
        props: Some(Box::new(EuvInputGroupAddonProps { align })),
    }
}

fn button_node(size: EuvInputGroupButtonSize) -> VirtualNode<EuvInputGroupButtonProps> {
    VirtualNode::Element {
        tag: Tag::Element(Cow::Borrowed("div")),
        attributes: Vec::new(),
        children: Vec::new(),
        key: None,
        props: Some(Box::new(EuvInputGroupButtonProps {
            size,
            label: "Go",
            onclick: None,
        })),
    }
}

fn input_node() -> VirtualNode<EuvInputGroupInputProps> {
    VirtualNode::Element {
        tag: Tag::Element(Cow::Borrowed("div")),
        attributes: Vec::new(),
        children: Vec::new(),
        key: None,
        props: Some(Box::new(EuvInputGroupInputProps {
            id: "query",
            placeholder: "Search",
            value: Signal::create(String::new()),
            disabled: Signal::create(false),
            oninput: None,
        })),
    }
}

fn textarea_node() -> VirtualNode<EuvInputGroupTextareaProps> {
    VirtualNode::Element {
        tag: Tag::Element(Cow::Borrowed("div")),
        attributes: Vec::new(),
        children: Vec::new(),
        key: None,
        props: Some(Box::new(EuvInputGroupTextareaProps {
            id: "body",
            placeholder: "Write",
            value: Signal::create(String::new()),
            rows: 3,
            disabled: Signal::create(false),
            oninput: None,
        })),
    }
}

fn tag_name(node: &VirtualNode) -> String {
    match node {
        VirtualNode::Element { tag, .. } => format!("{tag:?}"),
        _ => panic!("unexpected node variant: {node:?}"),
    }
}

fn walk(node: &VirtualNode, out: &mut Vec<VirtualNode>) {
    match node {
        VirtualNode::Element { children, .. } => {
            out.push(node.clone());
            for child in children.iter() {
                walk(child, out);
            }
        }
        VirtualNode::Fragment(children) => {
            for child in children.iter() {
                walk(child, out);
            }
        }
        _ => {}
    }
}

fn elements(node: &VirtualNode) -> Vec<VirtualNode> {
    let mut out: Vec<VirtualNode> = Vec::new();
    walk(node, &mut out);
    out
}

fn attribute_names(node: &VirtualNode) -> Vec<String> {
    match node {
        VirtualNode::Element { attributes, .. } => attributes
            .iter()
            .map(|entry: &AttributeEntry| entry.get_name().to_string())
            .collect(),
        _ => Vec::new(),
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

#[test]
fn an_input_group_is_a_group() {
    let rendered: VirtualNode = euv_input_group(group_node(false));
    assert!(
        attribute_names(&rendered)
            .iter()
            .any(|name: &String| name == "role"),
        "the frame announces itself as a group"
    );
    assert!(
        has_class(&rendered, "c_euv_input_group"),
        "the frame carries the input-group class"
    );
}

#[test]
fn a_disabled_input_group_is_dimmed() {
    let rendered: VirtualNode = euv_input_group(group_node(true));
    assert!(
        has_class(&rendered, "c_euv_input_group_disabled"),
        "a disabled group is dimmed"
    );
    assert!(
        !has_class(&rendered, "c_euv_input_group"),
        "a disabled group does not keep the enabled class"
    );
}

#[test]
fn every_addon_alignment_reaches_the_class_list() {
    let start: VirtualNode = euv_input_group_addon(addon_node(EuvInputGroupAlign::InlineStart));
    let end: VirtualNode = euv_input_group_addon(addon_node(EuvInputGroupAlign::InlineEnd));
    let top: VirtualNode = euv_input_group_addon(addon_node(EuvInputGroupAlign::BlockStart));
    let bottom: VirtualNode = euv_input_group_addon(addon_node(EuvInputGroupAlign::BlockEnd));
    assert!(
        has_class(&start, "c_euv_input_group_addon_inline_start"),
        "the addon before the field has its own class"
    );
    assert!(
        has_class(&end, "c_euv_input_group_addon_inline_end"),
        "the addon after the field has its own class"
    );
    assert!(
        has_class(&top, "c_euv_input_group_addon_block_start"),
        "the addon above the field has its own class"
    );
    assert!(
        has_class(&bottom, "c_euv_input_group_addon_block_end"),
        "the addon below the field has its own class"
    );
}

#[test]
fn every_button_size_reaches_the_class_list() {
    let xs: VirtualNode = euv_input_group_button(button_node(EuvInputGroupButtonSize::Xs));
    let sm: VirtualNode = euv_input_group_button(button_node(EuvInputGroupButtonSize::Sm));
    let icon_xs: VirtualNode = euv_input_group_button(button_node(EuvInputGroupButtonSize::IconXs));
    let icon_sm: VirtualNode = euv_input_group_button(button_node(EuvInputGroupButtonSize::IconSm));
    assert!(
        has_class(&xs, "c_euv_input_group_button_xs"),
        "the extra-small button has its own class"
    );
    assert!(
        has_class(&sm, "c_euv_input_group_button_sm"),
        "the small button has its own class"
    );
    assert!(
        has_class(&icon_xs, "c_euv_input_group_button_icon_xs"),
        "the extra-small icon button has its own class"
    );
    assert!(
        has_class(&icon_sm, "c_euv_input_group_button_icon_sm"),
        "the small icon button has its own class"
    );
}

#[test]
fn a_group_button_never_submits_a_form() {
    let rendered: VirtualNode = euv_input_group_button(button_node(EuvInputGroupButtonSize::Xs));
    assert_eq!(
        tag_name(&rendered),
        "Element(\"button\")",
        "the button inside a group is a button"
    );
    assert!(
        attribute_names(&rendered)
            .iter()
            .any(|name: &String| name == "type"),
        "the button carries its type so it never submits the form around it"
    );
}

#[test]
fn the_group_control_is_a_field() {
    let rendered: VirtualNode = euv_input_group_input(input_node());
    assert_eq!(
        tag_name(&rendered),
        "Element(\"input\")",
        "the single-line control of a group is an input"
    );
    assert!(
        has_class(&rendered, "c_euv_input_group_input"),
        "the control carries its own class"
    );
}

#[test]
fn the_group_multiline_control_is_a_textarea() {
    let rendered: VirtualNode = euv_input_group_textarea(textarea_node());
    assert_eq!(
        tag_name(&rendered),
        "Element(\"textarea\")",
        "the multi-line control of a group is a textarea"
    );
    assert!(
        has_class(&rendered, "c_euv_input_group_textarea"),
        "the multi-line control carries its own class"
    );
}
