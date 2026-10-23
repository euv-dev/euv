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

fn anchor_with(active: &str) -> VirtualNode {
    euv_anchor(props(EuvAnchorProps {
        items: vec![
            EuvAnchorItem {
                value: "intro",
                label: "Intro",
            },
            EuvAnchorItem {
                value: "usage",
                label: "Usage",
            },
        ],
        title: "On this page",
        active: Signal::create(String::from(active)),
        on_select: None,
    }))
}

#[test]
fn an_anchor_renders_one_entry_per_item() {
    let rendered: VirtualNode = anchor_with("intro");
    let mut classes: Vec<String> = Vec::new();
    classes_of(&rendered, &mut classes);
    let links: usize = classes
        .iter()
        .filter(|class: &&String| {
            *class == "c_euv_anchor_link" || *class == "c_euv_anchor_link_active"
        })
        .count();
    assert_eq!(links, 2, "both entries must be rendered, got {classes:?}");
}

#[test]
fn the_active_entry_carries_the_active_class() {
    let rendered: VirtualNode = anchor_with("usage");
    let mut classes: Vec<String> = Vec::new();
    classes_of(&rendered, &mut classes);
    let active: usize = classes
        .iter()
        .filter(|class: &&String| *class == "c_euv_anchor_link_active")
        .count();
    assert_eq!(
        active, 1,
        "exactly one entry must be active, got {classes:?}"
    );
}

#[test]
fn an_anchor_without_an_active_entry_leaves_none_active() {
    let rendered: VirtualNode = anchor_with("missing");
    let mut classes: Vec<String> = Vec::new();
    classes_of(&rendered, &mut classes);
    let active: usize = classes
        .iter()
        .filter(|class: &&String| *class == "c_euv_anchor_link_active")
        .count();
    assert_eq!(
        active, 0,
        "an entry that matches nothing must not be active, got {classes:?}"
    );
}

#[test]
fn an_anchor_renders_its_title() {
    let rendered: VirtualNode = anchor_with("intro");
    let rendered_text: String = format!("{rendered:?}");
    assert!(
        rendered_text.contains("On this page"),
        "the title must be rendered, got {rendered_text}"
    );
}
