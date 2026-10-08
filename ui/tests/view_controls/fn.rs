use super::*;

fn total_elements(node: &VirtualNode) -> usize {
    match node {
        VirtualNode::Element { children, .. } => {
            1 + children.iter().map(total_elements).sum::<usize>()
        }
        _ => 0,
    }
}

fn info_node(label: &'static str) -> VirtualNode<EuvInfoProps> {
    VirtualNode::Element {
        tag: Tag::Element(Cow::Borrowed("div")),
        attributes: Vec::new(),
        children: Vec::new(),
        key: None,
        props: Some(Box::new(EuvInfoProps { label })),
    }
}

fn logo_node(variant: LogoButtonVariant) -> VirtualNode<EuvLogoProps> {
    VirtualNode::Element {
        tag: Tag::Element(Cow::Borrowed("div")),
        attributes: Vec::new(),
        children: Vec::new(),
        key: None,
        props: Some(Box::new(EuvLogoProps {
            variant,
            on_click: None,
        })),
    }
}

fn button_node(variant: EuvButtonVariant, disabled: bool) -> VirtualNode<EuvButtonProps> {
    VirtualNode::Element {
        tag: Tag::Element(Cow::Borrowed("div")),
        attributes: Vec::new(),
        children: Vec::new(),
        key: None,
        props: Some(Box::new(EuvButtonProps {
            variant,
            label: "Press",
            onclick: None,
            disabled: Signal::create(disabled),
        })),
    }
}

fn logo_variants() -> [LogoButtonVariant; 2] {
    [LogoButtonVariant::Nav, LogoButtonVariant::Fab]
}

fn button_variants() -> [EuvButtonVariant; 2] {
    [EuvButtonVariant::Primary, EuvButtonVariant::Outline]
}

#[test]
fn an_info_row_renders_two_label_slots() {
    let rendered: VirtualNode = euv_info(info_node("Hello"));
    assert_eq!(
        format!("{rendered:?}").matches("Element(\"span\")").count(),
        2,
        "an info row pairs a term and its value, got {rendered:?}"
    );
}

#[test]
fn an_info_row_carries_its_label_as_text() {
    let rendered: VirtualNode = euv_info(info_node("Hello"));
    assert!(
        format!("{rendered:?}").contains("Hello"),
        "a static label reaches the tree as plain text, got {rendered:?}"
    );
}

#[test]
fn a_labelled_info_row_differs_from_an_unlabelled_one() {
    let with: VirtualNode = euv_info(info_node("Hello"));
    let without: VirtualNode = euv_info(info_node(""));
    assert_ne!(
        with, without,
        "an empty label and a filled one must be distinguishable"
    );
    assert_eq!(
        total_elements(&with),
        total_elements(&without),
        "the label fills the existing slots rather than adding more"
    );
}

#[test]
fn an_info_row_without_props_still_renders_its_frame() {
    let bare: VirtualNode<EuvInfoProps> = VirtualNode::Element {
        tag: Tag::Element(Cow::Borrowed("div")),
        attributes: Vec::new(),
        children: Vec::new(),
        key: None,
        props: None,
    };
    let rendered: VirtualNode = euv_info(bare);
    assert!(
        total_elements(&rendered) > 0,
        "the row frame renders even with no label, got {rendered:?}"
    );
}

#[test]
fn every_logo_variant_renders_a_single_span() {
    for variant in logo_variants() {
        let rendered: VirtualNode = euv_logo(logo_node(variant));
        assert_eq!(
            total_elements(&rendered),
            1,
            "{variant:?} must be a single element, got {rendered:?}"
        );
    }
}

#[test]
fn the_two_logo_variants_render_differently() {
    let nav: VirtualNode = euv_logo(logo_node(LogoButtonVariant::Nav));
    let fab: VirtualNode = euv_logo(logo_node(LogoButtonVariant::Fab));
    assert_ne!(
        nav, fab,
        "the sidebar logo and the floating action logo must not look alike"
    );
}

#[test]
fn a_logo_without_a_handler_can_be_compared_across_renders() {
    let first: VirtualNode = euv_logo(logo_node(LogoButtonVariant::Nav));
    let second: VirtualNode = euv_logo(logo_node(LogoButtonVariant::Nav));
    assert_eq!(
        first, second,
        "with no click handler attached there is no closure identity to \
         differ, so two identical renders compare equal"
    );
}

#[test]
fn the_default_logo_variant_is_the_navigation_one() {
    let observed: LogoButtonVariant = LogoButtonVariant::default();
    assert_eq!(
        observed,
        LogoButtonVariant::Nav,
        "the sidebar logo is the default"
    );
}

#[test]
fn every_button_variant_renders_a_single_button_element() {
    for variant in button_variants() {
        let rendered: VirtualNode = euv_button(button_node(variant, false));
        assert_eq!(
            total_elements(&rendered),
            1,
            "{variant:?} must be a single element, got {rendered:?}"
        );
        assert!(
            format!("{rendered:?}").contains("Element(\"button\")"),
            "{variant:?} must use a real button element for accessibility"
        );
    }
}

#[test]
fn the_two_button_variants_render_differently() {
    let primary: VirtualNode = euv_button(button_node(EuvButtonVariant::Primary, false));
    let outline: VirtualNode = euv_button(button_node(EuvButtonVariant::Outline, false));
    assert_ne!(
        primary, outline,
        "a filled button and an outlined one must be distinguishable"
    );
}

#[test]
fn a_disabled_button_differs_from_an_enabled_one() {
    let enabled: VirtualNode = euv_button(button_node(EuvButtonVariant::Primary, false));
    let disabled: VirtualNode = euv_button(button_node(EuvButtonVariant::Primary, true));
    assert_ne!(
        enabled, disabled,
        "the disabled state is carried into the rendered attribute set"
    );
    assert_eq!(
        total_elements(&enabled),
        total_elements(&disabled),
        "without changing how many elements the button uses"
    );
}

#[test]
fn a_button_carries_its_label_as_text() {
    let rendered: VirtualNode = euv_button(button_node(EuvButtonVariant::Primary, false));
    assert!(
        format!("{rendered:?}").contains("Press"),
        "a static label reaches the tree as plain text, got {rendered:?}"
    );
}

#[test]
fn a_handler_less_button_can_be_compared_across_renders() {
    let first: VirtualNode = euv_button(button_node(EuvButtonVariant::Primary, false));
    let second: VirtualNode = euv_button(button_node(EuvButtonVariant::Primary, false));
    assert_eq!(
        first, second,
        "with onclick left as None the button holds no closure, so two \
         identical renders compare equal and the whole button is assertable"
    );
}
