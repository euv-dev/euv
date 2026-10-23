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

fn text_with(variant: EuvTextVariant) -> VirtualNode {
    euv_text(props(EuvTextProps {
        text: "Some text",
        variant,
    }))
}

#[test]
fn body_text_carries_the_body_class() {
    let rendered: VirtualNode = text_with(EuvTextVariant::Body);
    let mut classes: Vec<String> = Vec::new();
    classes_of(&rendered, &mut classes);
    assert!(
        classes.iter().any(|class: &String| class == "c_euv_text"),
        "body text must carry the body class, got {classes:?}"
    );
}

#[test]
fn the_monospace_role_carries_its_own_class() {
    let rendered: VirtualNode = text_with(EuvTextVariant::Mono);
    let mut classes: Vec<String> = Vec::new();
    classes_of(&rendered, &mut classes);
    assert!(
        classes
            .iter()
            .any(|class: &String| class == "c_euv_text_mono"),
        "monospace text must carry the mono class, got {classes:?}"
    );
}

#[test]
fn the_deletion_role_carries_its_own_class() {
    let rendered: VirtualNode = text_with(EuvTextVariant::Deletion);
    let mut classes: Vec<String> = Vec::new();
    classes_of(&rendered, &mut classes);
    assert!(
        classes
            .iter()
            .any(|class: &String| class == "c_euv_text_deletion"),
        "deleted text must carry the deletion class, got {classes:?}"
    );
}

#[test]
fn every_role_renders_exactly_one_element() {
    let roles: Vec<EuvTextVariant> = vec![
        EuvTextVariant::Body,
        EuvTextVariant::Title,
        EuvTextVariant::TitleLg,
        EuvTextVariant::TitleSm,
        EuvTextVariant::Paragraph,
        EuvTextVariant::Secondary,
        EuvTextVariant::Mono,
        EuvTextVariant::Deletion,
        EuvTextVariant::Underline,
    ];
    for role in roles {
        let rendered: VirtualNode = text_with(role);
        let mut classes: Vec<String> = Vec::new();
        classes_of(&rendered, &mut classes);
        assert_eq!(
            classes.len(),
            1,
            "a run of text must render exactly one classed element, got {classes:?}"
        );
    }
}

#[test]
fn a_run_of_text_carries_the_string_it_was_given() {
    let rendered: VirtualNode = euv_text(props(EuvTextProps {
        text: "Hello",
        variant: EuvTextVariant::Paragraph,
    }));
    let rendered_text: String = format!("{rendered:?}");
    assert!(
        rendered_text.contains("Hello"),
        "the text must be rendered, got {rendered_text}"
    );
}
