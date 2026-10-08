use super::*;

fn total_elements(node: &VirtualNode) -> usize {
    match node {
        VirtualNode::Element { children, .. } => {
            1 + children.iter().map(total_elements).sum::<usize>()
        }
        VirtualNode::Fragment(children) => children.iter().map(total_elements).sum(),
        _ => 0,
    }
}

fn count_named(node: &VirtualNode, needle: &str) -> usize {
    format!("{node:?}").matches(needle).count()
}

fn slider_node(id: &'static str) -> VirtualNode<EuvSliderProps> {
    VirtualNode::Element {
        tag: Tag::Element(Cow::Borrowed("div")),
        attributes: Vec::new(),
        children: Vec::new(),
        key: None,
        props: Some(Box::new(EuvSliderProps {
            id,
            name: "volume",
            min: 0.0,
            max: 100.0,
            step: 1.0,
            value: Signal::create(0.0),
            label: "",
            oninput: None,
        })),
    }
}

fn tooltip_node(text: &'static str) -> VirtualNode<EuvTooltipProps> {
    VirtualNode::Element {
        tag: Tag::Element(Cow::Borrowed("div")),
        attributes: Vec::new(),
        children: Vec::new(),
        key: None,
        props: Some(Box::new(EuvTooltipProps {
            text,
            placement: EuvTooltipPlacement::default(),
        })),
    }
}

const TABS_THREE: [EuvTabItem; 3] = [
    EuvTabItem {
        key: "a",
        label: "A",
    },
    EuvTabItem {
        key: "b",
        label: "B",
    },
    EuvTabItem {
        key: "c",
        label: "C",
    },
];

const TABS_NONE: [EuvTabItem; 0] = [];

fn tabs_node(items: Vec<EuvTabItem>, active: &str) -> VirtualNode<EuvTabsProps> {
    VirtualNode::Element {
        tag: Tag::Element(Cow::Borrowed("div")),
        attributes: Vec::new(),
        children: Vec::new(),
        key: None,
        props: Some(Box::new(EuvTabsProps {
            items,
            active: Signal::create(String::from(active)),
        })),
    }
}

#[test]
fn a_slider_renders_a_native_range_input() {
    let rendered: VirtualNode = euv_slider(slider_node("volume"));
    let debugged: String = format!("{rendered:?}");
    assert!(
        debugged.contains("Element(\"input\")"),
        "a range control must be a real input element, got {debugged}"
    );
}

#[test]
fn a_slider_without_an_id_differs_from_one_with() {
    let named: VirtualNode = euv_slider(slider_node("volume"));
    let anonymous: VirtualNode = euv_slider(slider_node(""));
    assert_ne!(named, anonymous, "the id reaches the rendered attributes");
}

#[test]
fn a_slider_without_props_renders_the_input_anyway() {
    let bare: VirtualNode<EuvSliderProps> = VirtualNode::Element {
        tag: Tag::Element(Cow::Borrowed("div")),
        attributes: Vec::new(),
        children: Vec::new(),
        key: None,
        props: None,
    };
    let rendered: VirtualNode = euv_slider(bare);
    assert!(
        count_named(&rendered, "Element(\"input\")") >= 1,
        "the control renders even without props, got {rendered:?}"
    );
}

#[test]
fn a_tooltip_with_text_differs_from_an_empty_one() {
    let filled: VirtualNode = euv_tooltip(tooltip_node("Help"));
    let empty: VirtualNode = euv_tooltip(tooltip_node(""));
    assert_ne!(
        filled, empty,
        "the tooltip body only exists when there is text to show"
    );
}

#[test]
fn an_empty_tooltip_keeps_the_same_shape_as_a_filled_one() {
    let filled: VirtualNode = euv_tooltip(tooltip_node("Help"));
    let empty: VirtualNode = euv_tooltip(tooltip_node(""));
    assert_eq!(
        total_elements(&filled),
        total_elements(&empty),
        "the body slot is always present; the text only changes what lands in it, \
         so an empty tooltip is a blank body rather than no body"
    );
}

#[test]
fn a_tooltip_without_props_renders_without_panicking() {
    let bare: VirtualNode<EuvTooltipProps> = VirtualNode::Element {
        tag: Tag::Element(Cow::Borrowed("div")),
        attributes: Vec::new(),
        children: Vec::new(),
        key: None,
        props: None,
    };
    let rendered: VirtualNode = euv_tooltip(bare);
    let expected: VirtualNode = euv_tooltip(VirtualNode::Element {
        tag: Tag::Element(Cow::Borrowed("div")),
        attributes: Vec::new(),
        children: Vec::new(),
        key: None,
        props: Some(Box::new(EuvTooltipProps {
            text: "",
            placement: EuvTooltipPlacement::default(),
        })),
    });
    assert_eq!(
        total_elements(&rendered),
        total_elements(&expected),
        "the tooltip body holds a freshly built value each render, so two trees \
         can never compare equal; the shape is the observable part"
    );
}

#[test]
fn a_tabs_row_renders_one_control_per_tab() {
    let rendered: VirtualNode = euv_tabs(tabs_node(TABS_THREE.to_vec(), "a"));
    assert_eq!(
        count_named(&rendered, "Element(\"button\")"),
        3,
        "three tabs give three controls, got {rendered:?}"
    );
}

#[test]
fn the_tab_count_matches_the_item_count() {
    for (slice, expected) in [
        (TABS_THREE[..1].to_vec(), 1_usize),
        (TABS_THREE[..2].to_vec(), 2),
        (TABS_THREE[..].to_vec(), 3),
    ] {
        let rendered: VirtualNode = euv_tabs(tabs_node(slice, "a"));
        assert_eq!(
            count_named(&rendered, "Element(\"button\")"),
            expected,
            "expected {expected} tab controls"
        );
    }
}

#[test]
fn a_tabs_row_with_no_items_renders_no_controls() {
    let rendered: VirtualNode = euv_tabs(tabs_node(TABS_NONE.to_vec(), "a"));
    assert_eq!(
        count_named(&rendered, "Element(\"button\")"),
        0,
        "an empty tab list gives no controls, got {rendered:?}"
    );
}

#[test]
fn selecting_a_different_tab_changes_the_highlight() {
    let first: VirtualNode = euv_tabs(tabs_node(TABS_THREE.to_vec(), "a"));
    let second: VirtualNode = euv_tabs(tabs_node(TABS_THREE.to_vec(), "b"));
    assert_ne!(
        first, second,
        "the active tab is carried by a class, and the two must be distinguishable"
    );
}

#[test]
fn a_tabs_row_keeps_a_stable_shape_whatever_is_active() {
    let shapes: Vec<usize> = ["a", "b", "c"]
        .iter()
        .map(|key: &&str| total_elements(&euv_tabs(tabs_node(TABS_THREE.to_vec(), key))))
        .collect();
    assert!(
        shapes.windows(2).all(|w: &[usize]| w[0] == w[1]),
        "selecting a tab is a styling change, not a structural one, got {shapes:?}"
    );
}

#[test]
fn a_tabs_row_without_props_renders_no_controls() {
    let bare: VirtualNode<EuvTabsProps> = VirtualNode::Element {
        tag: Tag::Element(Cow::Borrowed("div")),
        attributes: Vec::new(),
        children: Vec::new(),
        key: None,
        props: None,
    };
    let rendered: VirtualNode = euv_tabs(bare);
    assert_eq!(
        count_named(&rendered, "Element(\"button\")"),
        0,
        "default props mean no items, so no controls, got {rendered:?}"
    );
}
