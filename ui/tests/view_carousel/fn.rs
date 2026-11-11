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

fn attribute_value_count(node: &VirtualNode, name: &str, value: &str) -> usize {
    match node {
        VirtualNode::Element {
            attributes,
            children,
            ..
        } => {
            let own: usize = attributes
                .iter()
                .filter(|attribute: &&AttributeEntry| {
                    attribute.get_name() == name && attribute.text_value().as_deref() == Some(value)
                })
                .count();
            own + children
                .iter()
                .map(|child: &VirtualNode| attribute_value_count(child, name, value))
                .sum::<usize>()
        }
        VirtualNode::Fragment(children) => children
            .iter()
            .map(|child: &VirtualNode| attribute_value_count(child, name, value))
            .sum(),
        _ => 0,
    }
}

fn attribute_name_count(node: &VirtualNode, name: &str) -> usize {
    match node {
        VirtualNode::Element {
            attributes,
            children,
            ..
        } => {
            attributes
                .iter()
                .filter(|attribute: &&AttributeEntry| attribute.get_name() == name)
                .count()
                + children
                    .iter()
                    .map(|child: &VirtualNode| attribute_name_count(child, name))
                    .sum::<usize>()
        }
        VirtualNode::Fragment(children) => children
            .iter()
            .map(|child: &VirtualNode| attribute_name_count(child, name))
            .sum(),
        _ => 0,
    }
}

fn slot_attribute_value_count(node: &VirtualNode, slot: &str, name: &str, value: &str) -> usize {
    match node {
        VirtualNode::Element {
            attributes,
            children,
            ..
        } => {
            let is_slot: bool = attributes.iter().any(|attribute: &AttributeEntry| {
                attribute.get_name() == "data-slot"
                    && attribute.text_value().as_deref() == Some(slot)
            });
            let own: usize = if is_slot {
                attributes
                    .iter()
                    .filter(|attribute: &&AttributeEntry| {
                        attribute.get_name() == name
                            && attribute.text_value().as_deref() == Some(value)
                    })
                    .count()
            } else {
                0
            };
            own + children
                .iter()
                .map(|child: &VirtualNode| slot_attribute_value_count(child, slot, name, value))
                .sum::<usize>()
        }
        VirtualNode::Fragment(children) => children
            .iter()
            .map(|child: &VirtualNode| slot_attribute_value_count(child, slot, name, value))
            .sum(),
        _ => 0,
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

#[test]
fn a_carousel_exposes_labeled_slides_and_navigation_controls() {
    let rendered: VirtualNode = carousel_with(0);

    assert_eq!(attribute_value_count(&rendered, "role", "region"), 1);
    assert_eq!(
        attribute_value_count(&rendered, "aria-roledescription", "carousel"),
        1
    );
    assert_eq!(
        attribute_value_count(&rendered, "aria-label", "Carousel"),
        1
    );
    assert_eq!(attribute_value_count(&rendered, "aria-live", "polite"), 1);
    assert_eq!(attribute_value_count(&rendered, "aria-atomic", "true"), 1);
    assert_eq!(attribute_value_count(&rendered, "role", "group"), 3);
    assert_eq!(
        attribute_value_count(&rendered, "aria-roledescription", "slide"),
        2
    );
    assert_eq!(
        attribute_value_count(&rendered, "aria-label", "Slide 1 of 2"),
        1
    );
    assert_eq!(
        attribute_value_count(&rendered, "aria-label", "Slide 2 of 2"),
        1
    );
    assert_eq!(
        slot_attribute_value_count(&rendered, "carousel-item", "aria-hidden", "false"),
        1
    );
    assert_eq!(
        slot_attribute_value_count(&rendered, "carousel-item", "aria-hidden", "true"),
        1
    );
    assert_eq!(
        attribute_value_count(&rendered, "aria-label", "Previous slide"),
        1
    );
    assert_eq!(
        attribute_value_count(&rendered, "aria-label", "Next slide"),
        1
    );
    assert_eq!(attribute_value_count(&rendered, "aria-pressed", "true"), 1);
    assert_eq!(attribute_value_count(&rendered, "aria-pressed", "false"), 1);
    assert_eq!(attribute_name_count(&rendered, "onkeydown"), 1);
    assert_eq!(attribute_name_count(&rendered, "ontouchstart"), 1);
    assert_eq!(attribute_name_count(&rendered, "ontouchend"), 1);
    assert_eq!(attribute_name_count(&rendered, "ontouchcancel"), 1);
}

#[test]
fn carousel_navigation_and_slide_visibility_follow_the_active_index() {
    let rendered: VirtualNode = carousel_with(1);

    assert_eq!(
        slot_attribute_value_count(&rendered, "carousel-item", "aria-hidden", "false"),
        1
    );
    assert_eq!(
        slot_attribute_value_count(&rendered, "carousel-item", "aria-hidden", "true"),
        1
    );
    assert_eq!(attribute_value_count(&rendered, "aria-pressed", "true"), 1);
    assert!(attribute_value_count(&rendered, "aria-label", "Previous slide") > 0);
    assert!(attribute_value_count(&rendered, "aria-label", "Next slide") > 0);
}
