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

#[test]
fn a_code_block_carries_its_variant_and_size() {
    let node: VirtualNode = euv_code_block(props_node(EuvCodeBlockProps {
        variant: EuvCodeBlockVariant::Destructive,
        size: EuvCodeBlockSize::Sm,
    }));
    assert!(has_class(&node, "c_euv_code_block"));
    assert!(has_class(&node, "c_euv_code_block_destructive"));
    assert!(has_class(&node, "c_euv_code_block_size_sm"));
    assert!(has_attribute(&node, "data-variant"));
    assert!(has_attribute(&node, "data-size"));
}

#[test]
fn the_code_block_header_carries_its_slot() {
    let node: VirtualNode = euv_code_block_header(props_node(EuvCodeBlockHeaderProps {}));
    assert!(has_class(&node, "c_euv_code_block_header"));
    assert!(has_attribute(&node, "data-slot"));
}

#[test]
fn the_code_block_content_reads_from_the_left() {
    let node: VirtualNode = euv_code_block_content(props_node(EuvCodeBlockContentProps {
        wrap: false,
        line_numbers: true,
        start_line: 1,
    }));
    assert!(has_class(&node, "c_euv_code_block_content"));
    assert!(has_attribute(&node, "dir"));
    assert!(has_attribute(&node, "data-numbered"));
}

#[test]
fn a_wrapping_code_block_wraps_its_lines() {
    let node: VirtualNode = euv_code_block_content(props_node(EuvCodeBlockContentProps {
        wrap: true,
        line_numbers: false,
        start_line: 1,
    }));
    assert!(has_class(&node, "c_euv_code_block_content_wrap"));
}

#[test]
fn a_code_block_line_carries_its_variant() {
    let node: VirtualNode = euv_code_block_line(props_node(EuvCodeBlockLineProps {
        variant: EuvCodeBlockLineVariant::Added,
    }));
    assert!(has_class(&node, "c_euv_code_block_line_added"));
    assert!(has_attribute(&node, "data-variant"));
}

#[test]
fn the_code_block_copy_button_is_a_button() {
    let node: VirtualNode = euv_code_block_copy_button(props_node(EuvCodeBlockCopyButtonProps {
        value: "print(1)",
    }));
    assert!(has_class(&node, "c_euv_code_block_copy_button"));
    assert!(has_attribute(&node, "type"));
}
