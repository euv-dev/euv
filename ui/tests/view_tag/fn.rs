use super::*;

fn tag_node(variant: EuvTagVariant, color: EuvTagColor) -> VirtualNode<EuvTagProps> {
    VirtualNode::Element {
        tag: Tag::Element(Cow::Borrowed("div")),
        attributes: Vec::new(),
        children: Vec::new(),
        key: None,
        props: Some(Box::new(EuvTagProps {
            color,
            variant,
            text: "Label",
            on_click: None,
        })),
    }
}

fn render(variant: EuvTagVariant, color: EuvTagColor) -> VirtualNode {
    euv_tag(tag_node(variant, color))
}

fn all_four() -> [(EuvTagVariant, EuvTagColor, VirtualNode); 4] {
    [
        (
            EuvTagVariant::Solid,
            EuvTagColor::Black,
            render(EuvTagVariant::Solid, EuvTagColor::Black),
        ),
        (
            EuvTagVariant::Solid,
            EuvTagColor::White,
            render(EuvTagVariant::Solid, EuvTagColor::White),
        ),
        (
            EuvTagVariant::Outline,
            EuvTagColor::Black,
            render(EuvTagVariant::Outline, EuvTagColor::Black),
        ),
        (
            EuvTagVariant::Outline,
            EuvTagColor::White,
            render(EuvTagVariant::Outline, EuvTagColor::White),
        ),
    ]
}

#[test]
fn every_variant_and_colour_combination_renders_a_span() {
    for (variant, color, node) in all_four() {
        match &node {
            VirtualNode::Element { tag, .. } => assert_eq!(
                format!("{tag:?}"),
                "Element(\"span\")",
                "{variant:?}/{color:?} must still render a span"
            ),
            _ => panic!("{variant:?}/{color:?} must render an element"),
        }
    }
}

#[test]
fn the_two_colours_of_one_variant_render_differently() {
    let solid_black: VirtualNode = render(EuvTagVariant::Solid, EuvTagColor::Black);
    let solid_white: VirtualNode = render(EuvTagVariant::Solid, EuvTagColor::White);
    assert_ne!(
        solid_black, solid_white,
        "the colour is carried by the class, so the trees must differ"
    );
}

#[test]
fn the_two_variants_of_one_colour_render_differently() {
    let black_solid: VirtualNode = render(EuvTagVariant::Solid, EuvTagColor::Black);
    let black_outline: VirtualNode = render(EuvTagVariant::Outline, EuvTagColor::Black);
    assert_ne!(
        black_solid, black_outline,
        "the variant is carried by the class, so the trees must differ"
    );
}

#[test]
fn all_four_combinations_are_mutually_distinct() {
    let combos: Vec<(EuvTagVariant, EuvTagColor, VirtualNode)> = all_four().to_vec();
    for (index, (variant_a, color_a, node_a)) in combos.iter().enumerate() {
        for (variant_b, color_b, node_b) in combos.iter().skip(index + 1) {
            assert_ne!(
                node_a, node_b,
                "{variant_a:?}/{color_a:?} and {variant_b:?}/{color_b:?} must not \
                 collapse onto the same markup"
            );
        }
    }
}

#[test]
fn the_variant_and_colour_axes_are_independent() {
    let solid_black: VirtualNode = render(EuvTagVariant::Solid, EuvTagColor::Black);
    let outline_white: VirtualNode = render(EuvTagVariant::Outline, EuvTagColor::White);
    assert_ne!(
        solid_black, outline_white,
        "the diagonal pair differs from the origin, so neither axis dominates"
    );
}

#[test]
fn a_tag_without_props_falls_back_to_defaults() {
    let bare: VirtualNode<EuvTagProps> = VirtualNode::Element {
        tag: Tag::Element(Cow::Borrowed("div")),
        attributes: Vec::new(),
        children: Vec::new(),
        key: None,
        props: None,
    };
    let rendered: VirtualNode = euv_tag(bare);
    let expected: VirtualNode = euv_tag(VirtualNode::Element {
        tag: Tag::Element(Cow::Borrowed("div")),
        attributes: Vec::new(),
        children: Vec::new(),
        key: None,
        props: Some(Box::new(EuvTagProps {
            color: EuvTagColor::default(),
            variant: EuvTagVariant::default(),
            text: "",
            on_click: None,
        })),
    });
    assert_eq!(
        rendered, expected,
        "a missing props slot yields the default variant, colour and an empty label"
    );
}

#[test]
fn the_default_variant_is_solid() {
    let observed: EuvTagVariant = EuvTagVariant::default();
    assert_eq!(
        format!("{observed:?}"),
        "Solid",
        "the derive default is a solid tag"
    );
}

#[test]
fn the_default_colour_is_black() {
    let observed: EuvTagColor = EuvTagColor::default();
    assert_eq!(
        format!("{observed:?}"),
        "Black",
        "the derive default is the dark tag"
    );
}
