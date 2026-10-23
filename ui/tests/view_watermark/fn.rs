use super::*;

fn props<T>(value: T) -> VirtualNode<T> {
    VirtualNode::Element {
        tag: Tag::Element(Cow::Borrowed("div")),
        attributes: Vec::new(),
        children: Vec::new(),
        key: None,
        props: Some(Box::new(value)),
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

fn watermark_with(text: &'static str) -> VirtualNode {
    euv_watermark(props(EuvWatermarkProps { text }))
}

#[test]
fn a_watermark_renders_its_stamp() {
    let rendered: VirtualNode = watermark_with("DRAFT");
    let rendered_text: String = format!("{rendered:?}");
    assert!(
        rendered_text.contains("DRAFT"),
        "the stamp must be rendered, got {rendered_text}"
    );
}

#[test]
fn a_watermark_renders_the_stamp_layer() {
    let rendered: VirtualNode = watermark_with("DRAFT");
    let mut classes: Vec<String> = Vec::new();
    classes_of(&rendered, &mut classes);
    assert!(
        classes
            .iter()
            .any(|class: &String| class == "c_euv_watermark_layer"),
        "the stamp must sit in its own layer, got {classes:?}"
    );
}

#[test]
fn a_watermark_without_a_stamp_renders_no_layer() {
    let rendered: VirtualNode = watermark_with("");
    let mut classes: Vec<String> = Vec::new();
    classes_of(&rendered, &mut classes);
    assert!(
        !classes
            .iter()
            .any(|class: &String| class == "c_euv_watermark_layer"),
        "an empty watermark must leave no layer behind, got {classes:?}"
    );
}

#[test]
fn a_watermark_keeps_its_content() {
    let rendered: VirtualNode = watermark_with("DRAFT");
    let mut classes: Vec<String> = Vec::new();
    classes_of(&rendered, &mut classes);
    assert!(
        classes
            .iter()
            .any(|class: &String| class == "c_euv_watermark"),
        "the content must be wrapped in the watermark box, got {classes:?}"
    );
}
