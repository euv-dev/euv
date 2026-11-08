use super::*;

fn aspect_node(ratio: f32) -> VirtualNode<EuvAspectRatioProps> {
    VirtualNode::Element {
        tag: Tag::Element(Cow::Borrowed("div")),
        attributes: Vec::new(),
        children: Vec::new(),
        key: None,
        props: Some(Box::new(EuvAspectRatioProps { ratio })),
    }
}

fn render(ratio: f32) -> VirtualNode {
    euv_aspect_ratio(aspect_node(ratio))
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
fn an_aspect_ratio_renders_a_container() {
    let rendered: VirtualNode = render(1.7777778);
    assert_eq!(
        tag_name(&rendered),
        "Element(\"div\")",
        "the ratio is carried by a plain container"
    );
}

#[test]
fn an_aspect_ratio_carries_its_ratio_inline() {
    let rendered: VirtualNode = render(1.7777778);
    assert!(
        attribute_names(&rendered)
            .iter()
            .any(|name: &String| name == "style"),
        "the ratio is handed to the box as an inline style"
    );
}

#[test]
fn an_aspect_ratio_names_its_slot() {
    let rendered: VirtualNode = render(1.7777778);
    assert!(
        attribute_names(&rendered)
            .iter()
            .any(|name: &String| name == "data-slot"),
        "the box names its slot so the choice is readable from the dom"
    );
}

#[test]
fn an_aspect_ratio_keeps_its_own_class() {
    let rendered: VirtualNode = render(1.7777778);
    assert!(
        has_class(&rendered, "c_euv_aspect_ratio"),
        "the box carries the aspect-ratio class"
    );
}
