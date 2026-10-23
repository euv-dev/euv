use super::*;

fn props_with_children<T>(value: T, children: Vec<VirtualNode>) -> VirtualNode<T> {
    VirtualNode::Element {
        tag: Tag::Element(Cow::Borrowed("div")),
        attributes: Vec::new(),
        children,
        key: None,
        props: Some(Box::new(value)),
    }
}

fn cell(marker: &str) -> VirtualNode {
    VirtualNode::Element {
        tag: Tag::Element(Cow::Borrowed("div")),
        attributes: vec![AttributeEntry::new(
            Cow::Borrowed("class"),
            AttributeValue::Text(marker.to_string()),
        )],
        children: Vec::new(),
        key: None,
        props: None,
    }
}

fn classes_of(node: &VirtualNode, into: &mut Vec<String>) {
    match node {
        VirtualNode::Element {
            attributes,
            children,
            ..
        } => {
            for attribute in attributes {
                if attribute.get_name() == "class" {
                    into.extend(attribute.class_names());
                }
            }
            for child in children {
                classes_of(child, into);
            }
        }
        VirtualNode::Fragment(children) => {
            for child in children {
                classes_of(child, into);
            }
        }
        _ => {}
    }
}

fn styles_of(node: &VirtualNode, into: &mut Vec<String>) {
    match node {
        VirtualNode::Element {
            attributes,
            children,
            ..
        } => {
            for attribute in attributes {
                if attribute.get_name() == "style" {
                    into.extend(attribute.text_value());
                }
            }
            for child in children {
                styles_of(child, into);
            }
        }
        VirtualNode::Fragment(children) => {
            for child in children {
                styles_of(child, into);
            }
        }
        _ => {}
    }
}

fn affix_with(offset: u32) -> VirtualNode {
    euv_affix(props_with_children(
        EuvAffixProps { offset },
        vec![cell("affix-content")],
    ))
}

#[test]
fn an_affix_block_carries_the_affix_class() {
    let rendered: VirtualNode = affix_with(64);
    let mut classes: Vec<String> = Vec::new();
    classes_of(&rendered, &mut classes);
    assert!(
        classes.iter().any(|class: &String| class == "c_euv_affix"),
        "the sticky block must carry the affix class, got {classes:?}"
    );
}

#[test]
fn an_affix_block_sticks_below_its_offset() {
    let rendered: VirtualNode = affix_with(64);
    let mut styles: Vec<String> = Vec::new();
    styles_of(&rendered, &mut styles);
    assert!(
        styles
            .iter()
            .any(|style: &String| style.contains("top: 64px;")),
        "the block must stick sixty-four pixels from the top, got {styles:?}"
    );
}

#[test]
fn an_affix_block_with_no_offset_sticks_to_the_very_top() {
    let rendered: VirtualNode = affix_with(0);
    let mut styles: Vec<String> = Vec::new();
    styles_of(&rendered, &mut styles);
    assert!(
        styles
            .iter()
            .any(|style: &String| style.contains("top: 0px;")),
        "a block with no offset must stick to the top, got {styles:?}"
    );
}

#[test]
fn an_affix_block_keeps_the_callers_content() {
    let rendered: VirtualNode = affix_with(16);
    let mut classes: Vec<String> = Vec::new();
    classes_of(&rendered, &mut classes);
    assert!(
        classes
            .iter()
            .any(|class: &String| class == "affix-content"),
        "the caller's content must survive the affix, got {classes:?}"
    );
}
