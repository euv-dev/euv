use super::*;

const TOC_TWO: [EuvTocItem; 2] = [
    EuvTocItem {
        level: 2,
        text: "One",
        href: "/one",
    },
    EuvTocItem {
        level: 3,
        text: "Two",
        href: "/two",
    },
];

const TOC_THREE: [EuvTocItem; 3] = [
    EuvTocItem {
        level: 2,
        text: "One",
        href: "/one",
    },
    EuvTocItem {
        level: 3,
        text: "Two",
        href: "/two",
    },
    EuvTocItem {
        level: 4,
        text: "Three",
        href: "/three",
    },
];

const TOC_NONE: [EuvTocItem; 0] = [];

fn toc_node(items: &'static [EuvTocItem]) -> VirtualNode<EuvTocProps> {
    VirtualNode::Element {
        tag: Tag::Element(Cow::Borrowed("div")),
        attributes: Vec::new(),
        children: Vec::new(),
        key: None,
        props: Some(Box::new(EuvTocProps {
            title: "Contents",
            items,
        })),
    }
}

fn count_anchors(node: &VirtualNode) -> usize {
    format!("{node:?}").matches("Element(\"a\")").count()
}

#[test]
fn an_empty_toc_short_circuits_to_an_empty_text_node() {
    let rendered: VirtualNode = euv_toc(toc_node(&TOC_NONE));
    assert!(
        matches!(rendered, VirtualNode::Text(_)),
        "no entries means the component renders nothing, got {rendered:?}"
    );
}

#[test]
fn a_two_item_toc_renders_two_anchors() {
    let rendered: VirtualNode = euv_toc(toc_node(&TOC_TWO));
    assert_eq!(
        count_anchors(&rendered),
        2,
        "one anchor per entry, got {rendered:?}"
    );
}

#[test]
fn a_three_item_toc_renders_three_anchors() {
    let rendered: VirtualNode = euv_toc(toc_node(&TOC_THREE));
    assert_eq!(
        count_anchors(&rendered),
        3,
        "one anchor per entry, got {rendered:?}"
    );
}

#[test]
fn a_one_item_toc_renders_one_anchor() {
    let rendered: VirtualNode = euv_toc(toc_node(&TOC_TWO[..1]));
    assert_eq!(
        count_anchors(&rendered),
        1,
        "a single entry gives a single link"
    );
}

#[test]
fn the_anchor_count_tracks_the_entry_count() {
    for (slice, expected) in [
        (&TOC_NONE[..], 0_usize),
        (&TOC_TWO[..1], 1),
        (&TOC_TWO[..], 2),
        (&TOC_THREE[..], 3),
    ] {
        let rendered: VirtualNode = euv_toc(toc_node(slice));
        assert_eq!(
            count_anchors(&rendered),
            expected,
            "{expected} entries should produce {expected} anchors"
        );
    }
}

#[test]
fn each_anchor_carries_its_href_as_the_key() {
    let rendered: VirtualNode = euv_toc(toc_node(&TOC_TWO));
    let debugged: String = format!("{rendered:?}");
    assert!(
        debugged.contains("/one") && debugged.contains("/two"),
        "both hrefs must reach the anchors, got {debugged}"
    );
}

#[test]
fn a_toc_with_entries_differs_from_an_empty_one() {
    let with: VirtualNode = euv_toc(toc_node(&TOC_TWO));
    let without: VirtualNode = euv_toc(toc_node(&TOC_NONE));
    assert_ne!(
        with, without,
        "entries turn the empty text node into a full element tree"
    );
}

#[test]
fn changing_the_href_changes_the_rendered_tree() {
    const RELABELLED: [EuvTocItem; 2] = [
        EuvTocItem {
            level: 2,
            text: "One",
            href: "/uno",
        },
        EuvTocItem {
            level: 3,
            text: "Two",
            href: "/dos",
        },
    ];
    let original: VirtualNode = euv_toc(toc_node(&TOC_TWO));
    let relabelled: VirtualNode = euv_toc(toc_node(&RELABELLED));
    assert_eq!(
        count_anchors(&original),
        count_anchors(&relabelled),
        "only the hrefs differ, so the shape matches"
    );
    assert_ne!(
        original, relabelled,
        "yet the trees differ, so the href really is carried"
    );
}

#[test]
fn a_toc_without_props_falls_back_to_defaults() {
    let bare: VirtualNode<EuvTocProps> = VirtualNode::Element {
        tag: Tag::Element(Cow::Borrowed("div")),
        attributes: Vec::new(),
        children: Vec::new(),
        key: None,
        props: None,
    };
    let rendered: VirtualNode = euv_toc(bare);
    let expected: VirtualNode = euv_toc(VirtualNode::Element {
        tag: Tag::Element(Cow::Borrowed("div")),
        attributes: Vec::new(),
        children: Vec::new(),
        key: None,
        props: Some(Box::new(EuvTocProps {
            title: "",
            items: &TOC_NONE,
        })),
    });
    assert_eq!(
        rendered, expected,
        "a missing props slot renders exactly the same thing as empty default props"
    );
}

#[test]
fn a_toc_renders_a_title_slot_ahead_of_its_entries() {
    let rendered: VirtualNode = euv_toc(toc_node(&TOC_TWO));
    let debugged: String = format!("{rendered:?}");
    let title_slot: usize = debugged.find("Dynamic").unwrap_or(usize::MAX);
    let first_anchor: usize = debugged.find("Element(\"a\")").unwrap_or(0);
    assert_ne!(
        title_slot,
        usize::MAX,
        "the title is rendered, its text just lives inside a Dynamic node"
    );
    assert!(
        title_slot < first_anchor,
        "and the title slot comes before the first entry, got {debugged}"
    );
}

#[test]
fn a_toc_flattens_its_entries_rather_than_nesting_them() {
    let rendered: VirtualNode = euv_toc(toc_node(&TOC_TWO));
    assert!(
        !format!("{rendered:?}").contains("Fragment"),
        "the entries are produced by a for loop, so they land as \
         siblings rather than behind a Fragment wrapper"
    );
}
