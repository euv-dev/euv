use super::*;

fn toggle_node(
    pressed: bool,
    variant: EuvToggleVariant,
    size: EuvToggleSize,
    disabled: bool,
) -> VirtualNode<EuvToggleProps> {
    VirtualNode::Element {
        tag: Tag::Element(Cow::Borrowed("div")),
        attributes: Vec::new(),
        children: Vec::new(),
        key: None,
        props: Some(Box::new(EuvToggleProps {
            pressed: Signal::create(pressed),
            variant,
            size,
            label: "Bold",
            disabled: Signal::create(disabled),
            on_change: None,
        })),
    }
}

fn render(
    pressed: bool,
    variant: EuvToggleVariant,
    size: EuvToggleSize,
    disabled: bool,
) -> VirtualNode {
    euv_toggle(toggle_node(pressed, variant, size, disabled))
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
fn a_toggle_renders_a_button() {
    let rendered: VirtualNode = render(
        false,
        EuvToggleVariant::Default,
        EuvToggleSize::Default,
        false,
    );
    assert_eq!(
        tag_name(&rendered),
        "Element(\"button\")",
        "a toggle is a button the browser can press"
    );
}

#[test]
fn a_toggle_reports_its_pressed_state() {
    let pressed: VirtualNode = render(
        true,
        EuvToggleVariant::Default,
        EuvToggleSize::Default,
        false,
    );
    let released: VirtualNode = render(
        false,
        EuvToggleVariant::Default,
        EuvToggleSize::Default,
        false,
    );
    assert!(
        attribute_names(&pressed)
            .iter()
            .any(|name: &String| name == "aria-pressed"),
        "a toggle reports its state to assistive technology"
    );
    assert!(
        has_class(&pressed, "c_euv_toggle_pressed"),
        "a pressed toggle fills with the accent surface"
    );
    assert!(
        !has_class(&released, "c_euv_toggle_pressed"),
        "a released toggle carries no pressed class"
    );
}

#[test]
fn every_variant_reaches_the_class_list() {
    let default: VirtualNode = render(
        false,
        EuvToggleVariant::Default,
        EuvToggleSize::Default,
        false,
    );
    let outline: VirtualNode = render(
        false,
        EuvToggleVariant::Outline,
        EuvToggleSize::Default,
        false,
    );
    assert!(
        !has_class(&default, "c_euv_toggle_outline"),
        "the default toggle has no border"
    );
    assert!(
        has_class(&outline, "c_euv_toggle_outline"),
        "the outline toggle carries a border"
    );
}

#[test]
fn every_size_reaches_the_class_list() {
    let default: VirtualNode = render(
        false,
        EuvToggleVariant::Default,
        EuvToggleSize::Default,
        false,
    );
    let small: VirtualNode = render(
        false,
        EuvToggleVariant::Default,
        EuvToggleSize::Small,
        false,
    );
    let large: VirtualNode = render(
        false,
        EuvToggleVariant::Default,
        EuvToggleSize::Large,
        false,
    );
    assert!(
        has_class(&default, "c_euv_toggle_size_default"),
        "the default size has its own class"
    );
    assert!(
        has_class(&small, "c_euv_toggle_size_small"),
        "the small size has its own class"
    );
    assert!(
        has_class(&large, "c_euv_toggle_size_large"),
        "the large size has its own class"
    );
}

#[test]
fn a_disabled_toggle_is_marked_disabled() {
    let rendered: VirtualNode = render(
        false,
        EuvToggleVariant::Default,
        EuvToggleSize::Default,
        true,
    );
    assert!(
        attribute_names(&rendered)
            .iter()
            .any(|name: &String| name == "disabled"),
        "a disabled toggle says so"
    );
}
