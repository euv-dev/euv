use super::*;

fn code_block_node(lang: &'static str, code: &'static str) -> VirtualNode<DocsCodeBlockProps> {
    VirtualNode::Element {
        tag: Tag::Element(Cow::Borrowed("div")),
        attributes: Vec::new(),
        children: Vec::new(),
        key: None,
        props: Some(Box::new(DocsCodeBlockProps { lang, code })),
    }
}

fn render_code_block(lang: &'static str, code: &'static str) -> VirtualNode {
    HookContext::with(HookContext::current(), || {
        docs_code_block(code_block_node(lang, code))
    })
}

fn own_classes(node: &VirtualNode) -> Vec<String> {
    match node {
        VirtualNode::Element { attributes, .. } => {
            let mut found: Vec<String> = Vec::new();
            for entry in attributes.iter() {
                if entry.get_name() == "class" {
                    found.extend(entry.class_names());
                }
            }
            found
        }
        _ => Vec::new(),
    }
}

fn has_class(node: &VirtualNode, name: &str) -> bool {
    own_classes(node).iter().any(|entry: &String| entry == name)
}

fn tree_has_class(node: &VirtualNode, name: &str) -> bool {
    if has_class(node, name) {
        return true;
    }
    children_of(node)
        .iter()
        .any(|child: &&VirtualNode| tree_has_class(child, name))
}

fn children_of(node: &VirtualNode) -> Vec<&VirtualNode> {
    match node {
        VirtualNode::Element { children, .. } => children.iter().collect(),
        VirtualNode::Fragment(children) => children.iter().collect(),
        _ => Vec::new(),
    }
}

fn find_by_class<'a>(node: &'a VirtualNode, name: &str) -> Option<&'a VirtualNode> {
    for child in children_of(node) {
        if has_class(child, name) {
            return Some(child);
        }
        if let Some(found) = find_by_class(child, name) {
            return Some(found);
        }
    }
    None
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

fn child_count(node: &VirtualNode) -> usize {
    match node {
        VirtualNode::Element { children, .. } => children.len(),
        VirtualNode::Fragment(children) => children.len(),
        _ => 0,
    }
}

#[test]
fn the_code_block_renders_its_frame() {
    let rendered: VirtualNode = render_code_block("rust", "fn main() {}");
    assert!(tree_has_class(&rendered, "c_docs_code"));
}

#[test]
fn the_code_block_renders_a_toolbar() {
    let rendered: VirtualNode = render_code_block("rust", "fn main() {}");
    assert!(tree_has_class(&rendered, "c_docs_code_header"));
}

#[test]
fn the_toolbar_names_the_language() {
    let rendered: VirtualNode = render_code_block("rust", "fn main() {}");
    let label: Option<&VirtualNode> = find_by_class(&rendered, "c_docs_code_lang");
    assert!(label.is_some());
    assert!(child_count(label.unwrap_or(&rendered)) > 0);
}

#[test]
fn the_code_block_renders_a_copy_button() {
    let rendered: VirtualNode = render_code_block("rust", "fn main() {}");
    assert!(tree_has_class(&rendered, "c_docs_code_copy"));
}

#[test]
fn the_copy_button_is_a_button_element() {
    let rendered: VirtualNode = render_code_block("rust", "fn main() {}");
    let button: Option<&VirtualNode> = find_by_class(&rendered, "c_docs_code_copy");
    let button: &VirtualNode = button.unwrap_or(&rendered);
    assert!(format!("{button:?}").contains("Element(\"button\")"));
}

#[test]
fn the_copy_button_carries_a_click_handler() {
    let rendered: VirtualNode = render_code_block("rust", "fn main() {}");
    let button: Option<&VirtualNode> = find_by_class(&rendered, "c_docs_code_copy");
    let button: &VirtualNode = button.unwrap_or(&rendered);
    assert!(
        attribute_names(button)
            .iter()
            .any(|name: &String| name == "onclick")
    );
}

#[test]
fn the_copy_button_names_its_action() {
    let rendered: VirtualNode = render_code_block("rust", "fn main() {}");
    let button: Option<&VirtualNode> = find_by_class(&rendered, "c_docs_code_copy");
    let button: &VirtualNode = button.unwrap_or(&rendered);
    assert!(
        attribute_names(button)
            .iter()
            .any(|name: &String| name == "aria-label")
    );
}

#[test]
fn the_copy_button_carries_its_label() {
    let rendered: VirtualNode = render_code_block("rust", "fn main() {}");
    let button: Option<&VirtualNode> = find_by_class(&rendered, "c_docs_code_copy");
    let button: &VirtualNode = button.unwrap_or(&rendered);
    assert!(child_count(button) > 0);
}

#[test]
fn the_code_block_renders_the_code_in_a_pre() {
    let rendered: VirtualNode = render_code_block("rust", "fn main() {}");
    assert!(format!("{rendered:?}").contains("Element(\"pre\")"));
}

#[test]
fn the_code_block_renders_the_code_in_a_code_element() {
    let rendered: VirtualNode = render_code_block("rust", "fn main() {}");
    assert!(format!("{rendered:?}").contains("Element(\"code\")"));
}

#[test]
fn the_code_block_carries_the_highlighted_markup() {
    let rendered: VirtualNode = render_code_block("rust", "fn main() {}");
    assert!(format!("{rendered:?}").contains("inner_html"));
}

#[test]
fn a_code_block_of_an_unknown_language_still_renders() {
    let rendered: VirtualNode = render_code_block("brainfuck", "+[-]");
    assert!(tree_has_class(&rendered, "c_docs_code"));
}
