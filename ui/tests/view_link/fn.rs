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

fn link_with(href: &'static str, disabled: bool) -> VirtualNode {
    euv_link(props(EuvLinkProps {
        text: "Docs",
        href,
        disabled,
        on_click: None,
    }))
}

#[test]
fn a_link_with_a_destination_is_an_anchor() {
    let rendered: VirtualNode = link_with("https://example.com", false);
    let mut tags: Vec<String> = Vec::new();
    tags_of(&rendered, &mut tags);
    assert!(
        tags.iter().any(|tag: &String| tag == "Element(\"a\")"),
        "a link with a destination must be an anchor, got {tags:?}"
    );
}

#[test]
fn a_link_without_a_destination_is_not_an_anchor() {
    let rendered: VirtualNode = link_with("", false);
    let mut tags: Vec<String> = Vec::new();
    tags_of(&rendered, &mut tags);
    assert!(
        !tags.iter().any(|tag: &String| tag == "Element(\"a\")"),
        "a link without a destination must not pretend to be navigable, got {tags:?}"
    );
}

#[test]
fn an_anchor_carries_its_destination() {
    let rendered: VirtualNode = link_with("https://example.com", false);
    let mut hrefs: Vec<String> = Vec::new();
    attributes_of(&rendered, "href", &mut hrefs);
    assert!(
        hrefs
            .iter()
            .any(|href: &String| href == "https://example.com"),
        "the anchor must carry its destination, got {hrefs:?}"
    );
}

#[test]
fn a_disabled_link_carries_the_disabled_class() {
    let rendered: VirtualNode = link_with("https://example.com", true);
    let mut classes: Vec<String> = Vec::new();
    classes_of(&rendered, &mut classes);
    assert!(
        classes
            .iter()
            .any(|class: &String| class == "c_euv_link_disabled"),
        "a disabled link must carry the disabled class, got {classes:?}"
    );
}

#[test]
fn an_enabled_link_carries_the_link_class() {
    let rendered: VirtualNode = link_with("https://example.com", false);
    let mut classes: Vec<String> = Vec::new();
    classes_of(&rendered, &mut classes);
    assert!(
        classes.iter().any(|class: &String| class == "c_euv_link"),
        "an enabled link must carry the link class, got {classes:?}"
    );
}
