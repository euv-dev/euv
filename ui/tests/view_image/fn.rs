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

fn attributes_of(node: &VirtualNode, name: &str, into: &mut Vec<String>) {
    match node {
        VirtualNode::Element {
            attributes,
            children,
            ..
        } => {
            for attribute in attributes {
                if attribute.get_name() == name {
                    into.extend(attribute.text_value());
                }
            }
            for child in children {
                attributes_of(child, name, into);
            }
        }
        VirtualNode::Fragment(children) => {
            for child in children {
                attributes_of(child, name, into);
            }
        }
        _ => {}
    }
}

fn tags_of(node: &VirtualNode, into: &mut Vec<String>) {
    match node {
        VirtualNode::Element { tag, children, .. } => {
            into.push(format!("{tag:?}"));
            for child in children {
                tags_of(child, into);
            }
        }
        VirtualNode::Fragment(children) => {
            for child in children {
                tags_of(child, into);
            }
        }
        _ => {}
    }
}

fn image_with(src: &'static str, placeholder: &'static str) -> VirtualNode {
    euv_image(props(EuvImageProps {
        src,
        alt: "A picture",
        placeholder,
        width: None,
        height: None,
    }))
}

#[test]
fn an_image_renders_an_img_element() {
    let rendered: VirtualNode = image_with("cat.png", "");
    let mut tags: Vec<String> = Vec::new();
    tags_of(&rendered, &mut tags);
    assert!(
        tags.iter().any(|tag: &String| tag == "Element(\"img\")"),
        "an image must render an img element, got {tags:?}"
    );
}

#[test]
fn an_image_carries_its_source_and_alternative_text() {
    let rendered: VirtualNode = image_with("cat.png", "");
    let mut sources: Vec<String> = Vec::new();
    attributes_of(&rendered, "src", &mut sources);
    assert!(
        sources.iter().any(|source: &String| source == "cat.png"),
        "the image must carry its source, got {sources:?}"
    );
    let mut alts: Vec<String> = Vec::new();
    attributes_of(&rendered, "alt", &mut alts);
    assert!(
        alts.iter().any(|alt: &String| alt == "A picture"),
        "the image must carry its alternative text, got {alts:?}"
    );
}

#[test]
fn an_image_is_loaded_lazily() {
    let rendered: VirtualNode = image_with("cat.png", "");
    let mut loading: Vec<String> = Vec::new();
    attributes_of(&rendered, "loading", &mut loading);
    assert!(
        loading.iter().any(|value: &String| value == "lazy"),
        "an image must load lazily, got {loading:?}"
    );
}

#[test]
fn an_image_with_a_placeholder_shows_it_while_loading() {
    let rendered: VirtualNode = image_with("cat.png", "Loading…");
    let rendered_text: String = format!("{rendered:?}");
    assert!(
        rendered_text.contains("Loading…"),
        "the placeholder must be shown while the picture loads, got {rendered_text}"
    );
}

#[test]
fn an_image_without_a_placeholder_shows_no_text() {
    let rendered: VirtualNode = image_with("cat.png", "");
    let rendered_text: String = format!("{rendered:?}");
    assert!(
        !rendered_text.contains("TextNode"),
        "an image without a placeholder must carry no text node, got {rendered_text}"
    );
}
