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

fn empty_with(icon: &'static str, desc: &'static str, action: &'static str) -> VirtualNode {
    euv_empty(props(EuvEmptyProps {
        icon,
        title: "Nothing here",
        desc,
        action_label: action,
        on_action: None,
    }))
}

#[test]
fn an_empty_state_renders_its_headline() {
    let rendered: VirtualNode = empty_with("", "", "");
    let rendered_text: String = format!("{rendered:?}");
    assert!(
        rendered_text.contains("Nothing here"),
        "the headline must be rendered, got {rendered_text}"
    );
}

#[test]
fn an_empty_state_without_an_icon_leaves_no_icon_row() {
    let rendered: VirtualNode = empty_with("", "", "");
    let mut classes: Vec<String> = Vec::new();
    classes_of(&rendered, &mut classes);
    assert!(
        !classes
            .iter()
            .any(|class: &String| class == "c_euv_empty_icon"),
        "an empty icon must not leave an empty row behind, got {classes:?}"
    );
}

#[test]
fn an_empty_state_with_an_icon_renders_the_icon_row() {
    let rendered: VirtualNode = empty_with("📭", "", "");
    let mut classes: Vec<String> = Vec::new();
    classes_of(&rendered, &mut classes);
    assert!(
        classes
            .iter()
            .any(|class: &String| class == "c_euv_empty_icon"),
        "an icon must be rendered, got {classes:?}"
    );
}

#[test]
fn an_empty_state_with_a_description_renders_it() {
    let rendered: VirtualNode = empty_with("", "Add a record to see it here", "");
    let rendered_text: String = format!("{rendered:?}");
    assert!(
        rendered_text.contains("Add a record to see it here"),
        "the description must be rendered, got {rendered_text}"
    );
}

#[test]
fn an_empty_state_without_an_action_renders_no_button() {
    let rendered: VirtualNode = empty_with("", "", "");
    let mut tags: Vec<String> = Vec::new();
    collect_tags(&rendered, &mut tags);
    assert!(
        !tags.iter().any(|tag: &String| tag == "Element(\"button\")"),
        "an empty state without an action must not render a button, got {tags:?}"
    );
}

#[test]
fn an_empty_state_with_an_action_renders_a_button() {
    let rendered: VirtualNode = empty_with("", "", "Add record");
    let mut tags: Vec<String> = Vec::new();
    collect_tags(&rendered, &mut tags);
    assert!(
        tags.iter().any(|tag: &String| tag == "Element(\"button\")"),
        "an action must render a button, got {tags:?}"
    );
}

fn collect_tags(node: &VirtualNode, into: &mut Vec<String>) {
    match node {
        VirtualNode::Element { tag, children, .. } => {
            into.push(format!("{tag:?}"));
            for child in children {
                collect_tags(child, into);
            }
        }
        VirtualNode::Fragment(children) => {
            for child in children {
                collect_tags(child, into);
            }
        }
        _ => {}
    }
}
