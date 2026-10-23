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

fn carousel_with(active: u32) -> VirtualNode {
    euv_carousel(props(EuvCarouselProps {
        slides: vec![
            EuvCarouselSlide {
                title: "First",
                desc: "The first slide",
            },
            EuvCarouselSlide {
                title: "Second",
                desc: "The second slide",
            },
        ],
        active: Signal::create(active),
        on_change: None,
    }))
}

#[test]
fn a_carousel_renders_every_slide() {
    let rendered: VirtualNode = carousel_with(0);
    let mut classes: Vec<String> = Vec::new();
    classes_of(&rendered, &mut classes);
    let slides: usize = classes
        .iter()
        .filter(|class: &&String| *class == "c_euv_carousel_slide")
        .count();
    assert_eq!(slides, 2, "both slides must be rendered, got {classes:?}");
}

#[test]
fn a_carousel_shifts_the_track_by_the_active_slide() {
    let rendered: VirtualNode = carousel_with(1);
    let mut styles: Vec<String> = Vec::new();
    styles_of(&rendered, &mut styles);
    assert!(
        styles
            .iter()
            .any(|style: &String| style.contains("translateX(-100%)")),
        "the track must shift by one slide, got {styles:?}"
    );
}

#[test]
fn a_carousel_renders_a_dot_per_slide() {
    let rendered: VirtualNode = carousel_with(0);
    let mut classes: Vec<String> = Vec::new();
    classes_of(&rendered, &mut classes);
    let dots: usize = classes
        .iter()
        .filter(|class: &&String| {
            *class == "c_euv_carousel_dot" || *class == "c_euv_carousel_dot_active"
        })
        .count();
    assert_eq!(dots, 2, "one dot per slide is expected, got {classes:?}");
}

#[test]
fn exactly_one_dot_is_active() {
    let rendered: VirtualNode = carousel_with(1);
    let mut classes: Vec<String> = Vec::new();
    classes_of(&rendered, &mut classes);
    let active: usize = classes
        .iter()
        .filter(|class: &&String| *class == "c_euv_carousel_dot_active")
        .count();
    assert_eq!(active, 1, "exactly one dot is active, got {classes:?}");
}
