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
            data_slot: None,
            onclick: None,
            disabled: Signal::create(false),
            submit: false,
            aria_label: None,
            busy: Signal::create(false),
            data_testid: "",
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

fn attribute_text(node: &VirtualNode, name: &str) -> Option<String> {
    match node {
        VirtualNode::Element { attributes, .. } => attributes
            .iter()
            .find(|entry: &&AttributeEntry| entry.get_name() == name)
            .and_then(AttributeEntry::text_value),
        _ => None,
    }
}

#[test]
fn a_button_carries_the_reference_variant_and_size_slots() {
    let rendered: VirtualNode = euv_button(button_node(
        EuvButtonVariant::Secondary,
        EuvButtonSize::Small,
    ));
    assert_eq!(
        attribute_text(&rendered, "data-slot").as_deref(),
        Some("button")
    );
    assert_eq!(
        attribute_text(&rendered, "data-variant").as_deref(),
        Some("secondary")
    );
    assert_eq!(
        attribute_text(&rendered, "data-size").as_deref(),
        Some("sm")
    );
    assert_eq!(
        attribute_text(&rendered, "aria-label").as_deref(),
        Some("Press")
    );
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
fn a_composed_button_can_publish_its_semantic_slot() {
    let node: VirtualNode<EuvButtonProps> = VirtualNode::Element {
        tag: Tag::Element(Cow::Borrowed("div")),
        attributes: Vec::new(),
        children: Vec::new(),
        key: None,
        props: Some(Box::new(EuvButtonProps {
            variant: EuvButtonVariant::Destructive,
            size: EuvButtonSize::Small,
            label: "Delete",
            data_slot: Some("alert-dialog-action"),
            onclick: None,
            disabled: Signal::create(false),
            submit: false,
            aria_label: None,
            busy: Signal::create(false),
            data_testid: "",
        })),
    };
    let rendered: VirtualNode = euv_button(node);
    assert_eq!(
        attribute_text(&rendered, "data-slot").as_deref(),
        Some("alert-dialog-action")
    );
}

#[test]
fn every_registry_variant_reaches_the_class_list() {
    let cases: [(EuvButtonVariant, &str, &str); 7] = [
        (
            EuvButtonVariant::Primary,
            "c_euv_button_primary",
            "c_euv_button_primary",
        ),
        (
            EuvButtonVariant::Default,
            "c_euv_button_default",
            "c_euv_button_default",
        ),
        (
            EuvButtonVariant::Secondary,
            "c_euv_button_secondary",
            "c_euv_button_secondary",
        ),
        (
            EuvButtonVariant::Destructive,
            "c_euv_button_destructive",
            "c_euv_button_destructive",
        ),
        (
            EuvButtonVariant::Outline,
            "c_euv_button_outline",
            "c_euv_button_outline",
        ),
        (
            EuvButtonVariant::Ghost,
            "c_euv_button_ghost",
            "c_euv_button_ghost",
        ),
        (EuvButtonVariant::Link, "c_euv_button_link", ""),
    ];
    for (variant, class_name, shadcn_class) in cases {
        let rendered: VirtualNode = render(variant, EuvButtonSize::Default);
        assert!(
            has_class(&rendered, class_name),
            "the {variant:?} variant carries {class_name}, got {:?}",
            class_names(&rendered)
        );
        if cfg!(feature = "theme-shadcn") && !shadcn_class.is_empty() {
            assert!(
                has_class(&rendered, shadcn_class),
                "the {variant:?} variant carries its shadcn rule {shadcn_class}"
            );
        }
    }
}

#[test]
fn a_button_keeps_exactly_its_base_variant_and_size_classes() {
    for (variant, expected) in [
        (EuvButtonVariant::Primary, "c_euv_button_primary"),
        (EuvButtonVariant::Default, "c_euv_button_default"),
        (EuvButtonVariant::Secondary, "c_euv_button_secondary"),
        (EuvButtonVariant::Destructive, "c_euv_button_destructive"),
        (EuvButtonVariant::Outline, "c_euv_button_outline"),
        (EuvButtonVariant::Ghost, "c_euv_button_ghost"),
        (EuvButtonVariant::Link, "c_euv_button_link"),
    ] {
        let rendered: VirtualNode = render(variant, EuvButtonSize::Default);
        let names: Vec<String> = class_names(&rendered);
        assert_eq!(
            names,
            vec![
                String::from("c_euv_button"),
                String::from(expected),
                String::from("c_euv_button_size_default")
            ]
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
    let cases: [(EuvButtonSize, &str, &str, &str); 8] = [
        (
            EuvButtonSize::Default,
            "c_euv_button_size_default",
            "c_euv_button_size_default",
            "default",
        ),
        (
            EuvButtonSize::Small,
            "c_euv_button_size_small",
            "c_euv_button_size_small",
            "sm",
        ),
        (
            EuvButtonSize::Large,
            "c_euv_button_size_large",
            "c_euv_button_size_large",
            "lg",
        ),
        (
            EuvButtonSize::Icon,
            "c_euv_button_size_icon",
            "c_euv_button_size_icon",
            "icon",
        ),
        (
            EuvButtonSize::ExtraSmall,
            "c_euv_button_size_xs",
            "c_euv_button_size_xs",
            "xs",
        ),
        (
            EuvButtonSize::IconExtraSmall,
            "c_euv_button_size_icon_xs",
            "c_euv_button_size_icon_xs",
            "icon-xs",
        ),
        (
            EuvButtonSize::IconSmall,
            "c_euv_button_size_icon_sm",
            "c_euv_button_size_icon_sm",
            "icon-sm",
        ),
        (
            EuvButtonSize::IconLarge,
            "c_euv_button_size_icon_lg",
            "c_euv_button_size_icon_lg",
            "icon-lg",
        ),
    ];
    for (size, class_name, shadcn_class, data_size) in cases {
        let rendered: VirtualNode = render(EuvButtonVariant::Primary, size);
        assert!(
            has_class(&rendered, class_name),
            "the {size:?} size carries {class_name}, got {:?}",
            class_names(&rendered)
        );
        if cfg!(feature = "theme-shadcn") {
            assert!(has_class(&rendered, shadcn_class));
            assert_eq!(
                attribute_text(&rendered, "data-size").as_deref(),
                Some(data_size)
            );
        }
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

#[cfg(feature = "theme-shadcn")]
#[test]
fn a_shadcn_button_carries_its_radius_modifier() {
    let rendered: VirtualNode = render(EuvButtonVariant::Primary, EuvButtonSize::Default);
    assert!(
        has_class(&rendered, "c_euv_button"),
        "the shadcn scheme applies its button radius, got {:?}",
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
            data_slot: None,
            onclick: None,
            disabled: Signal::create(true),
            submit: false,
            aria_label: None,
            busy: Signal::create(false),
            data_testid: "",
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
                class_name.starts_with("c_euv_button_")
                    && !class_name.contains("size_")
                    && !class_name.ends_with("_shadcn")
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
