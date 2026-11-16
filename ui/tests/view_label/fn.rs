use super::*;

fn label_node(
    html_for: &'static str,
    text: &'static str,
    disabled: bool,
) -> VirtualNode<EuvLabelProps> {
    VirtualNode::Element {
        tag: Tag::Element(Cow::Borrowed("div")),
        attributes: Vec::new(),
        children: Vec::new(),
        key: None,
        props: Some(Box::new(EuvLabelProps {
            html_for,
            text,
            disabled: Signal::create(disabled),
            class: Css::default(),
        })),
    }
}

fn render(html_for: &'static str, text: &'static str, disabled: bool) -> VirtualNode {
    euv_label(label_node(html_for, text, disabled))
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
fn a_label_composes_an_application_class_with_its_design_system_class() {
    let node: VirtualNode<EuvLabelProps> = VirtualNode::Element {
        tag: Tag::Element(Cow::Borrowed("div")),
        attributes: Vec::new(),
        children: Vec::new(),
        key: None,
        props: Some(Box::new(EuvLabelProps {
            html_for: "email",
            text: "Email",
            disabled: Signal::create(false),
            class: Css {
                name: String::from("commerce-field-label"),
                ..Css::default()
            },
        })),
    };
    let rendered: VirtualNode = euv_label(node);
    assert!(has_class(&rendered, "commerce-field-label"));
    assert!(has_class(&rendered, "c_euv_label"));
}

#[test]
fn a_label_renders_a_label_element() {
    let rendered: VirtualNode = render("email", "Email", false);
    assert_eq!(
        tag_name(&rendered),
        "Element(\"label\")",
        "a label is a label element the browser associates with its control"
    );
}

#[test]
fn a_label_points_at_the_control_it_names() {
    let rendered: VirtualNode = render("email", "Email", false);
    assert!(
        attribute_names(&rendered)
            .iter()
            .any(|name: &String| name == "for"),
        "the label carries the for attribute that names its control"
    );
}

#[test]
fn an_enabled_label_keeps_the_full_ink() {
    let rendered: VirtualNode = render("email", "Email", false);
    assert!(
        has_class(&rendered, "c_euv_label"),
        "an enabled label is drawn in the full ink"
    );
    assert!(
        !has_class(&rendered, "c_euv_label_disabled"),
        "an enabled label carries no disabled class"
    );
}

#[test]
fn a_disabled_label_is_dimmed() {
    let rendered: VirtualNode = render("email", "Email", true);
    assert!(
        has_class(&rendered, "c_euv_label_disabled"),
        "a label whose control is disabled is dimmed"
    );
}
