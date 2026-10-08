use super::*;

fn alert_node(variant: AlertVariant) -> VirtualNode<EuvAlertProps> {
    VirtualNode::Element {
        tag: Tag::Element(Cow::Borrowed("div")),
        attributes: Vec::new(),
        children: Vec::new(),
        key: None,
        props: Some(Box::new(EuvAlertProps { variant })),
    }
}

fn tag_name(node: &VirtualNode) -> String {
    match node {
        VirtualNode::Element { tag, .. } => format!("{tag:?}"),
        _ => panic!("expected an element node"),
    }
}

fn class_attribute_count(node: &VirtualNode) -> usize {
    match node {
        VirtualNode::Element { attributes, .. } => attributes.len(),
        _ => panic!("expected an element node"),
    }
}

#[test]
fn an_error_alert_renders_a_div_shell() {
    let rendered: VirtualNode = euv_alert(alert_node(AlertVariant::Error));
    assert_eq!(
        tag_name(&rendered),
        "Element(\"div\")",
        "the alert shell is a div"
    );
    assert_eq!(
        class_attribute_count(&rendered),
        1,
        "and it carries exactly one attribute, the class"
    );
}

#[test]
fn a_success_alert_renders_the_same_shape_as_an_error_one() {
    let error: VirtualNode = euv_alert(alert_node(AlertVariant::Error));
    let success: VirtualNode = euv_alert(alert_node(AlertVariant::Success));
    assert_eq!(
        tag_name(&success),
        tag_name(&error),
        "the variant only swaps the class, not the element"
    );
    assert_eq!(
        class_attribute_count(&success),
        class_attribute_count(&error),
        "and not the attribute count either"
    );
}

#[test]
fn the_two_alert_variants_are_distinguishable() {
    let error: VirtualNode = euv_alert(alert_node(AlertVariant::Error));
    let success: VirtualNode = euv_alert(alert_node(AlertVariant::Success));
    assert_ne!(
        error, success,
        "the two trees compare unequal, so the variant really is carried \
         by the class attached to the shell"
    );
}

#[test]
fn an_alert_wraps_the_children_it_was_given() {
    let mut node: VirtualNode<EuvAlertProps> = alert_node(AlertVariant::Error);
    if let VirtualNode::Element { children, .. } = &mut node {
        *children = vec![VirtualNode::from(String::from("boom"))];
    } else {
        panic!("the helper always builds an element");
    }
    let rendered: VirtualNode = euv_alert(node);
    let carried: Vec<&VirtualNode> = rendered.get_children().iter().collect();
    assert_eq!(
        carried.len(),
        1,
        "the body must be nested inside the shell, got {carried:?}"
    );
    assert!(
        format!("{:?}", carried[0]).contains("boom"),
        "and the body text must survive the wrapping, got {:?}",
        carried[0]
    );
}

#[test]
fn a_childless_alert_still_renders_its_shell() {
    let rendered: VirtualNode = euv_alert(alert_node(AlertVariant::Success));
    assert_eq!(
        tag_name(&rendered),
        "Element(\"div\")",
        "the shell is unconditional"
    );
}

#[test]
fn an_alert_without_props_falls_back_to_the_default_variant() {
    let bare: VirtualNode<EuvAlertProps> = VirtualNode::Element {
        tag: Tag::Element(Cow::Borrowed("div")),
        attributes: Vec::new(),
        children: Vec::new(),
        key: None,
        props: None,
    };
    let rendered: VirtualNode = euv_alert(bare);
    let with_default: VirtualNode = euv_alert(alert_node(AlertVariant::default()));
    assert_eq!(
        rendered, with_default,
        "a missing props slot yields the default variant rather than a panic"
    );
}

#[test]
fn a_card_renders_its_title_above_its_body() {
    let node: VirtualNode<EuvCardProps> = VirtualNode::Element {
        tag: Tag::Element(Cow::Borrowed("div")),
        attributes: Vec::new(),
        children: vec![VirtualNode::from(String::from("body"))],
        key: None,
        props: Some(Box::new(EuvCardProps { title: "Hello" })),
    };
    let rendered: VirtualNode = euv_card(node);
    match &rendered {
        VirtualNode::Element { children, .. } => {
            assert_eq!(children.len(), 2, "a card is a heading plus a body slot");
        }
        _ => panic!("a card renders an element"),
    }
}

#[test]
fn a_card_always_renders_a_heading_even_when_childless() {
    let node: VirtualNode<EuvCardProps> = VirtualNode::Element {
        tag: Tag::Element(Cow::Borrowed("div")),
        attributes: Vec::new(),
        children: Vec::new(),
        key: None,
        props: Some(Box::new(EuvCardProps { title: "Empty" })),
    };
    let rendered: VirtualNode = euv_card(node);
    match &rendered {
        VirtualNode::Element { children, .. } => {
            assert_eq!(
                children.len(),
                2,
                "the heading is unconditional and the empty body slice still \
                 becomes an Empty placeholder, so a bodyless card keeps both slots"
            );
            assert!(
                matches!(children[1], VirtualNode::Empty),
                "and the body slot holds the Empty placeholder, got {:?}",
                children[1]
            );
        }
        _ => panic!("a card renders an element"),
    }
}

#[test]
fn two_cards_differing_only_in_title_still_render_the_same_shell() {
    let make: fn(&'static str) -> VirtualNode = |title: &'static str| -> VirtualNode {
        euv_card(VirtualNode::Element {
            tag: Tag::Element(Cow::Borrowed("div")),
            attributes: Vec::new(),
            children: Vec::new(),
            key: None,
            props: Some(Box::new(EuvCardProps { title })),
        })
    };
    let one: VirtualNode = make("One");
    let two: VirtualNode = make("Two");
    match (&one, &two) {
        (VirtualNode::Element { children: a, .. }, VirtualNode::Element { children: b, .. }) => {
            assert_eq!(
                a.len(),
                b.len(),
                "only the title differs, so the shape matches"
            );
        }
        _ => panic!("both cards render elements"),
    }
    assert_ne!(
        one, two,
        "yet the trees still compare unequal, so the title reaches the heading"
    );
}
