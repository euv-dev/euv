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

fn back_top_with(visible: bool) -> VirtualNode {
    euv_back_top(props(EuvBackTopProps {
        visible: Signal::create(visible),
        label: "↑",
        on_click: None,
    }))
}

#[test]
fn a_visible_back_top_carries_the_visible_class() {
    let rendered: VirtualNode = back_top_with(true);
    let mut classes: Vec<String> = Vec::new();
    classes_of(&rendered, &mut classes);
    assert!(
        classes
            .iter()
            .any(|class: &String| class == "c_euv_back_top_visible"),
        "a showing button must carry the visible class, got {classes:?}"
    );
}

#[test]
fn a_hidden_back_top_carries_the_hidden_class() {
    let rendered: VirtualNode = back_top_with(false);
    let mut classes: Vec<String> = Vec::new();
    classes_of(&rendered, &mut classes);
    assert!(
        classes
            .iter()
            .any(|class: &String| class == "c_euv_back_top_hidden"),
        "a hidden button must carry the hidden class, got {classes:?}"
    );
}

#[test]
fn a_back_top_stays_mounted_while_it_is_hidden() {
    let rendered: VirtualNode = back_top_with(false);
    let mut tags: Vec<String> = Vec::new();
    tags_of(&rendered, &mut tags);
    assert!(
        tags.iter().any(|tag: &String| tag == "Element(\"button\")"),
        "a hidden button must stay mounted, got {tags:?}"
    );
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
