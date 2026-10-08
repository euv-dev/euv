use super::*;

const WITH_ICON: [EuvFeature; 2] = [
    EuvFeature {
        icon: "A",
        title: "Alpha",
        details: "first",
    },
    EuvFeature {
        icon: "B",
        title: "Beta",
        details: "second",
    },
];

const NO_ICON: [EuvFeature; 2] = [
    EuvFeature {
        icon: "",
        title: "Alpha",
        details: "first",
    },
    EuvFeature {
        icon: "",
        title: "Beta",
        details: "second",
    },
];

fn grid(features: &'static [EuvFeature]) -> VirtualNode<EuvFeatureGridProps> {
    VirtualNode::Element {
        tag: Tag::Element(Cow::Borrowed("div")),
        attributes: Vec::new(),
        children: Vec::new(),
        key: None,
        props: Some(Box::new(EuvFeatureGridProps { features })),
    }
}

fn cards(node: &VirtualNode) -> Vec<VirtualNode> {
    match node {
        VirtualNode::Element { children, .. } => children.clone(),
        _ => panic!("expected the grid root element, got {node:?}"),
    }
}

fn total_elements(node: &VirtualNode) -> usize {
    match node {
        VirtualNode::Element { children, .. } => {
            1 + children.iter().map(total_elements).sum::<usize>()
        }
        _ => 0,
    }
}

#[test]
fn an_empty_grid_short_circuits_to_an_empty_text_node() {
    let rendered: VirtualNode = euv_feature_grid(grid(&[]));
    assert!(
        matches!(rendered, VirtualNode::Text(_)),
        "no features means the component renders nothing, got {rendered:?}"
    );
}

#[test]
fn a_grid_renders_one_card_per_feature() {
    let rendered: VirtualNode = euv_feature_grid(grid(&WITH_ICON));
    assert_eq!(cards(&rendered).len(), 2, "two features, two cards");
}

#[test]
fn a_grid_without_icons_renders_the_same_number_of_cards() {
    let rendered: VirtualNode = euv_feature_grid(grid(&NO_ICON));
    assert_eq!(
        cards(&rendered).len(),
        2,
        "an empty icon omits the icon slot, it does not remove the card"
    );
}

#[test]
fn a_feature_with_an_icon_renders_more_elements_than_one_without() {
    let with_icon: VirtualNode = euv_feature_grid(grid(&WITH_ICON));
    let without: VirtualNode = euv_feature_grid(grid(&NO_ICON));
    let with_total: usize = total_elements(&with_icon);
    let without_total: usize = total_elements(&without);
    assert_eq!(
        with_total - without_total,
        2,
        "each of the two features adds exactly one icon element nested \
         inside its header when it has one"
    );
}

#[test]
fn a_single_feature_grid_renders_one_card() {
    let rendered: VirtualNode = euv_feature_grid(grid(&WITH_ICON[..1]));
    assert_eq!(cards(&rendered).len(), 1, "one feature, one card");
}

#[test]
fn two_grids_with_different_titles_render_different_trees() {
    let first: VirtualNode = euv_feature_grid(grid(&WITH_ICON));
    const RENAMED: [EuvFeature; 2] = [
        EuvFeature {
            icon: "A",
            title: "One",
            details: "first",
        },
        EuvFeature {
            icon: "B",
            title: "Two",
            details: "second",
        },
    ];
    let second: VirtualNode = euv_feature_grid(grid(&RENAMED));
    assert_ne!(
        first, second,
        "the title travels into the card key, so the trees differ"
    );
}

#[test]
fn a_grid_without_props_short_circuits_too() {
    let bare: VirtualNode<EuvFeatureGridProps> = VirtualNode::Element {
        tag: Tag::Element(Cow::Borrowed("div")),
        attributes: Vec::new(),
        children: Vec::new(),
        key: None,
        props: None,
    };
    let rendered: VirtualNode = euv_feature_grid(bare);
    assert!(
        matches!(rendered, VirtualNode::Text(_)),
        "default props mean no features, which short-circuits"
    );
}

#[test]
fn the_card_count_matches_the_feature_slice_length() {
    for count in 0_usize..=2 {
        let slice: &'static [EuvFeature] = &WITH_ICON[..count];
        let rendered: VirtualNode = euv_feature_grid(grid(slice));
        if count == 0 {
            assert!(
                matches!(rendered, VirtualNode::Text(_)),
                "an empty slice never reaches the card loop"
            );
        } else {
            assert_eq!(
                cards(&rendered).len(),
                count,
                "{count} features must render {count} cards"
            );
        }
    }
}
