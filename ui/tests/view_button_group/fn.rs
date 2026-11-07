use super::*;

fn group_node(orientation: EuvButtonGroupOrientation) -> VirtualNode<EuvButtonGroupProps> {
    VirtualNode::Element {
        tag: Tag::Element(Cow::Borrowed("div")),
        attributes: Vec::new(),
        children: Vec::new(),
        key: None,
        props: Some(Box::new(EuvButtonGroupProps { orientation })),
    }
}

fn render(orientation: EuvButtonGroupOrientation) -> VirtualNode {
    euv_button_group(group_node(orientation))
}

fn text_node(text: &'static str) -> VirtualNode<EuvButtonGroupTextProps> {
    VirtualNode::Element {
        tag: Tag::Element(Cow::Borrowed("div")),
        attributes: Vec::new(),
        children: Vec::new(),
        key: None,
        props: Some(Box::new(EuvButtonGroupTextProps { text })),
    }
}

fn separator_node(
    orientation: EuvButtonGroupSeparatorOrientation,
) -> VirtualNode<EuvButtonGroupSeparatorProps> {
    VirtualNode::Element {
        tag: Tag::Element(Cow::Borrowed("div")),
        attributes: Vec::new(),
        children: Vec::new(),
        key: None,
        props: Some(Box::new(EuvButtonGroupSeparatorProps { orientation })),
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
fn a_button_group_is_a_group() {
    let rendered: VirtualNode = render(EuvButtonGroupOrientation::Horizontal);
    assert!(
        attribute_names(&rendered)
            .iter()
            .any(|name: &String| name == "role"),
        "the group announces itself as a group"
    );
    assert!(
        attribute_names(&rendered)
            .iter()
            .any(|name: &String| name == "data-orientation"),
        "the group names the direction it was given"
    );
}

#[test]
fn a_horizontal_group_keeps_its_own_class() {
    let rendered: VirtualNode = render(EuvButtonGroupOrientation::Horizontal);
    assert!(
        has_class(&rendered, "c_euv_button_group_horizontal"),
        "a row of joined buttons has its own class"
    );
    assert!(
        !has_class(&rendered, "c_euv_button_group_vertical"),
        "a row of joined buttons is not a column"
    );
}

#[test]
fn a_vertical_group_keeps_its_own_class() {
    let rendered: VirtualNode = render(EuvButtonGroupOrientation::Vertical);
    assert!(
        has_class(&rendered, "c_euv_button_group_vertical"),
        "a column of joined buttons has its own class"
    );
    assert!(
        !has_class(&rendered, "c_euv_button_group_horizontal"),
        "a column of joined buttons is not a row"
    );
}

#[test]
fn the_group_caption_keeps_its_own_class() {
    let rendered: VirtualNode = euv_button_group_text(text_node("Zoom"));
    assert!(
        has_class(&rendered, "c_euv_button_group_text"),
        "the caption inside a group has its own class"
    );
}

#[test]
fn every_separator_orientation_reaches_the_class_list() {
    let vertical: VirtualNode =
        euv_button_group_separator(separator_node(EuvButtonGroupSeparatorOrientation::Vertical));
    let horizontal: VirtualNode = euv_button_group_separator(separator_node(
        EuvButtonGroupSeparatorOrientation::Horizontal,
    ));
    assert!(
        has_class(&vertical, "c_euv_button_group_separator_vertical"),
        "a vertical rule between the buttons of a row"
    );
    assert!(
        has_class(&horizontal, "c_euv_button_group_separator_horizontal"),
        "a horizontal rule between the buttons of a column"
    );
}
