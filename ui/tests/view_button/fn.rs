use super::*;

fn button_node(variant: EuvButtonVariant, size: EuvButtonSize) -> VirtualNode<EuvButtonProps> {
    VirtualNode::Element {
        tag: Tag::Element(Cow::Borrowed("div")),
        attributes: Vec::new(),
        children: Vec::new(),
        key: None,
        props: Some(Box::new(EuvButtonProps {
            variant,
            size,
            label: "Press",
            onclick: None,
            disabled: Signal::create(false),
        })),
    }
}

fn render(variant: EuvButtonVariant, size: EuvButtonSize) -> VirtualNode {
    euv_button(button_node(variant, size))
}

fn attribute_names(node: &VirtualNode) -> Vec<String> {
    match node {
        VirtualNode::Element { attributes, .. } => attributes
            .iter()
            .map(|entry: &AttributeEntry| entry.get_name().to_string())
            .collect(),
        _ => Vec::new(),
    }
}

fn class_names(node: &VirtualNode) -> Vec<String> {
    match node {
        VirtualNode::Element { attributes, .. } => {
            let mut out: Vec<String> = Vec::new();
            for entry in attributes.iter() {
                if entry.get_name() == "class" {
                    out.extend(entry.class_names());
                }
            }
            out
        }
        _ => Vec::new(),
    }
}

fn has_class(node: &VirtualNode, name: &str) -> bool {
    class_names(node).iter().any(|entry: &String| entry == name)
}

#[test]
fn a_button_renders_a_button_element() {
    let rendered: VirtualNode = render(EuvButtonVariant::Primary, EuvButtonSize::Default);
    assert!(
        format!("{rendered:?}").contains("Element(\"button\")"),
        "a button renders a real button element, got {rendered:?}"
    );
}

#[test]
fn a_button_never_submits_the_form_it_sits_in() {
    let rendered: VirtualNode = render(EuvButtonVariant::Primary, EuvButtonSize::Default);
    assert!(
        attribute_names(&rendered)
            .iter()
            .any(|name: &String| name == "type"),
        "a button declares its own type so a form is not submitted by accident"
    );
}

#[test]
fn every_registry_variant_reaches_the_class_list() {
    let cases: [(EuvButtonVariant, &str); 7] = [
        (EuvButtonVariant::Primary, "c_euv_button_primary"),
        (EuvButtonVariant::Default, "c_euv_button_default"),
        (EuvButtonVariant::Secondary, "c_euv_button_secondary"),
        (EuvButtonVariant::Destructive, "c_euv_button_destructive"),
        (EuvButtonVariant::Outline, "c_euv_button_outline"),
        (EuvButtonVariant::Ghost, "c_euv_button_ghost"),
        (EuvButtonVariant::Link, "c_euv_button_link"),
    ];
    for (variant, class_name) in cases {
        let rendered: VirtualNode = render(variant, EuvButtonSize::Default);
        assert!(
            has_class(&rendered, class_name),
            "the {variant:?} variant carries {class_name}, got {:?}",
            class_names(&rendered)
        );
    }
}

#[test]
fn the_historical_primary_variant_still_works() {
    let rendered: VirtualNode = render(EuvButtonVariant::Primary, EuvButtonSize::Default);
    assert!(
        has_class(&rendered, "c_euv_button_primary"),
        "the historical primary variant keeps its class, got {:?}",
        class_names(&rendered)
    );
}

#[test]
fn every_registry_size_reaches_the_class_list() {
    let cases: [(EuvButtonSize, &str); 4] = [
        (EuvButtonSize::Default, "c_euv_button_size_default"),
        (EuvButtonSize::Small, "c_euv_button_size_small"),
        (EuvButtonSize::Large, "c_euv_button_size_large"),
        (EuvButtonSize::Icon, "c_euv_button_size_icon"),
    ];
    for (size, class_name) in cases {
        let rendered: VirtualNode = render(EuvButtonVariant::Primary, size);
        assert!(
            has_class(&rendered, class_name),
            "the {size:?} size carries {class_name}, got {:?}",
            class_names(&rendered)
        );
    }
}

#[test]
fn every_button_carries_the_shared_control_class() {
    let rendered: VirtualNode = render(EuvButtonVariant::Ghost, EuvButtonSize::Small);
    assert!(
        has_class(&rendered, "c_euv_button"),
        "the shared control class is on every button, got {:?}",
        class_names(&rendered)
    );
}

#[test]
fn a_disabled_button_says_so() {
    let node: VirtualNode<EuvButtonProps> = VirtualNode::Element {
        tag: Tag::Element(Cow::Borrowed("div")),
        attributes: Vec::new(),
        children: Vec::new(),
        key: None,
        props: Some(Box::new(EuvButtonProps {
            variant: EuvButtonVariant::Primary,
            size: EuvButtonSize::Default,
            label: "Press",
            onclick: None,
            disabled: Signal::create(true),
        })),
    };
    let rendered: VirtualNode = euv_button(node);
    assert!(
        attribute_names(&rendered)
            .iter()
            .any(|name: &String| name == "disabled"),
        "a disabled button says so, got {:?}",
        attribute_names(&rendered)
    );
}

#[test]
fn the_variants_are_distinguishable_from_each_other() {
    let variants: [EuvButtonVariant; 7] = [
        EuvButtonVariant::Primary,
        EuvButtonVariant::Default,
        EuvButtonVariant::Secondary,
        EuvButtonVariant::Destructive,
        EuvButtonVariant::Outline,
        EuvButtonVariant::Ghost,
        EuvButtonVariant::Link,
    ];
    let mut seen: Vec<String> = Vec::new();
    for variant in variants {
        let rendered: VirtualNode = render(variant, EuvButtonSize::Default);
        let classes: Vec<String> = class_names(&rendered);
        let variant_classes: Vec<&String> = classes
            .iter()
            .filter(|class_name: &&String| {
                class_name.starts_with("c_euv_button_") && !class_name.contains("size_")
            })
            .collect();
        assert_eq!(
            variant_classes.len(),
            1,
            "a button carries exactly one variant class, got {classes:?}"
        );
        for class_name in variant_classes {
            assert!(
                !seen.contains(class_name),
                "the variants do not share the class {class_name}"
            );
            seen.push(class_name.clone());
        }
    }
}
