use super::*;

fn spinner_node(label: &'static str, show_label: bool) -> VirtualNode<EuvSpinnerProps> {
    VirtualNode::Element {
        tag: Tag::Element(Cow::Borrowed("div")),
        attributes: Vec::new(),
        children: Vec::new(),
        key: None,
        props: Some(Box::new(EuvSpinnerProps {
            label,
            show_label: Signal::create(show_label),
        })),
    }
}

fn render(label: &'static str, show_label: bool) -> VirtualNode {
    euv_spinner(spinner_node(label, show_label))
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
fn a_spinner_carries_the_busy_role() {
    let rendered: VirtualNode = render("Loading", false);
    assert!(
        attribute_names(&rendered)
            .iter()
            .any(|name: &String| name == "role"),
        "the spinner announces itself as a status"
    );
}

#[test]
fn a_spinner_names_itself_for_assistive_technology() {
    let rendered: VirtualNode = render("Loading", false);
    assert!(
        attribute_names(&rendered)
            .iter()
            .any(|name: &String| name == "aria-label"),
        "the spinner carries an accessible name"
    );
}

#[test]
fn the_spinner_glyph_carries_its_own_class() {
    let rendered: VirtualNode = render("Loading", false);
    assert!(
        has_class(&rendered, "c_euv_spinner"),
        "the spinning glyph has its own class"
    );
}

#[test]
fn the_label_is_shown_only_when_asked_for() {
    let shown: VirtualNode = render("Loading", true);
    let hidden: VirtualNode = render("Loading", false);
    assert!(
        has_class(&shown, "c_euv_spinner_label"),
        "the visible label appears when the caller asks for it"
    );
    assert!(
        has_class(&hidden, "c_euv_spinner_label_hidden"),
        "the visible label stays away when the caller does not ask for it"
    );
}
