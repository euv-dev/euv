use super::*;

#[test]
fn textarea_rows_use_compiled_metadata_without_overriding_positive_caller_rows() {
    let expected_default: i32 = if cfg!(feature = "theme-shadcn") { 2 } else { 3 };
    for rows in [-2, 0, 1, 7] {
        let rendered: VirtualNode = render("Notes", rows, false);
        let VirtualNode::Element { attributes, .. } = rendered else {
            panic!("expected a native textarea");
        };
        let actual: Option<&AttributeEntry> = attributes
            .iter()
            .find(|attribute: &&AttributeEntry| attribute.get_name() == "rows");
        let expected: AttributeEntry = AttributeEntry::new(
            Cow::Borrowed("rows"),
            AttributeValue::Dynamic(if rows > 0 { rows } else { expected_default }.to_string()),
        );
        assert_eq!(actual, Some(&expected));
    }
}

fn textarea_node(
    placeholder: &'static str,
    rows: i32,
    disabled: bool,
) -> VirtualNode<EuvTextareaProps> {
    VirtualNode::Element {
        tag: Tag::Element(Cow::Borrowed("div")),
        attributes: Vec::new(),
        children: Vec::new(),
        key: None,
        props: Some(Box::new(EuvTextareaProps {
            id: "notes",
            name: "notes",
            placeholder,
            value: Signal::create(String::new()),
            rows,
            disabled: Signal::create(disabled),
            oninput: None,
            aria_label: "",
            onkeydown: None,
            class: Css::default(),
        })),
    }
}

fn render(placeholder: &'static str, rows: i32, disabled: bool) -> VirtualNode {
    euv_textarea(textarea_node(placeholder, rows, disabled))
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
fn a_textarea_composes_an_application_class_with_its_design_system_class() {
    let node: VirtualNode<EuvTextareaProps> = VirtualNode::Element {
        tag: Tag::Element(Cow::Borrowed("div")),
        attributes: Vec::new(),
        children: Vec::new(),
        key: None,
        props: Some(Box::new(EuvTextareaProps {
            class: Css {
                name: String::from("commerce-editor"),
                ..Css::default()
            },
            ..EuvTextareaProps::default()
        })),
    };
    let rendered: VirtualNode = euv_textarea(node);
    assert!(has_class(&rendered, "commerce-editor"));
    assert!(has_class(&rendered, "c_euv_textarea"));
}

#[test]
fn a_textarea_renders_a_textarea_element() {
    let rendered: VirtualNode = render("Type here", 4, false);
    assert_eq!(
        tag_name(&rendered),
        "Element(\"textarea\")",
        "a textarea is a textarea element the browser gives its own keyboard"
    );
}

#[test]
fn a_textarea_carries_its_placeholder_and_rows() {
    let rendered: VirtualNode = render("Type here", 4, false);
    let names: Vec<String> = attribute_names(&rendered);
    assert!(
        names.iter().any(|name: &String| name == "placeholder"),
        "the field shows its placeholder"
    );
    assert!(
        names.iter().any(|name: &String| name == "rows"),
        "the field carries the height it was given"
    );
}

#[test]
fn a_textarea_uses_the_control_type_scale() {
    let rendered: VirtualNode = render("Type here", 4, false);
    assert!(
        has_class(&rendered, "c_euv_textarea"),
        "the field is drawn with the textarea class"
    );
    if cfg!(feature = "theme-shadcn") {
        assert!(has_class(&rendered, "c_euv_textarea"));
    }
}

#[test]
fn a_disabled_textarea_says_so() {
    let rendered: VirtualNode = render("Type here", 4, true);
    assert!(
        attribute_names(&rendered)
            .iter()
            .any(|name: &String| name == "disabled"),
        "a disabled field says so"
    );
}
