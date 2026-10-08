use super::*;

fn item(title: &'static str) -> EuvTimelineItem {
    EuvTimelineItem {
        title,
        description: "d",
        time: "t",
    }
}

fn timeline_node(items: Vec<EuvTimelineItem>) -> VirtualNode<EuvTimelineProps> {
    VirtualNode::Element {
        tag: Tag::Element(Cow::Borrowed("div")),
        attributes: Vec::new(),
        children: Vec::new(),
        key: None,
        props: Some(Box::new(EuvTimelineProps { items })),
    }
}

fn entries(node: &VirtualNode) -> Vec<VirtualNode> {
    match node {
        VirtualNode::Element { children, .. } => match children.first() {
            Some(VirtualNode::Fragment(inner)) => inner.clone(),
            Some(other) => vec![other.clone()],
            None => Vec::new(),
        },
        _ => panic!("expected the timeline root element, got {node:?}"),
    }
}

fn element_children(node: &VirtualNode) -> usize {
    match node {
        VirtualNode::Element { children, .. } => children
            .iter()
            .filter(|child: &&VirtualNode| matches!(child, VirtualNode::Element { .. }))
            .count(),
        _ => 0,
    }
}

#[test]
fn an_empty_timeline_still_renders_its_container() {
    let node: VirtualNode<EuvTimelineProps> = timeline_node(Vec::new());
    let rendered: VirtualNode = euv_timeline(node);
    let collected: Vec<VirtualNode> = entries(&rendered);
    assert_eq!(
        collected.len(),
        1,
        "the entry vector is empty but still occupies one slot, as Empty"
    );
    assert!(
        matches!(collected[0], VirtualNode::Empty),
        "and that slot holds the Empty placeholder, got {:?}",
        collected[0]
    );
}

#[test]
fn a_one_item_timeline_renders_exactly_one_entry() {
    let node: VirtualNode<EuvTimelineProps> = timeline_node(vec![item("Only")]);
    let rendered: VirtualNode = euv_timeline(node);
    assert_eq!(entries(&rendered).len(), 1, "one item is one entry");
}

#[test]
fn a_three_item_timeline_renders_three_entries() {
    let node: VirtualNode<EuvTimelineProps> = timeline_node(vec![item("A"), item("B"), item("C")]);
    let rendered: VirtualNode = euv_timeline(node);
    assert_eq!(entries(&rendered).len(), 3, "one entry per item, in order");
}

#[test]
fn the_entry_count_tracks_the_item_count() {
    for count in 1_usize..=5 {
        let items: Vec<EuvTimelineItem> = (0..count).map(|_| item("X")).collect();
        let node: VirtualNode<EuvTimelineProps> = timeline_node(items);
        let rendered: VirtualNode = euv_timeline(node);
        assert_eq!(
            entries(&rendered).len(),
            count,
            "{count} items must render {count} entries"
        );
    }
}

#[test]
fn every_entry_but_the_last_carries_a_connector() {
    let node: VirtualNode<EuvTimelineProps> =
        timeline_node(vec![item("A"), item("B"), item("C"), item("D")]);
    let rendered: VirtualNode = euv_timeline(node);
    let counts: Vec<usize> = entries(&rendered).iter().map(element_children).collect();
    assert_eq!(counts, vec![3, 3, 3, 2], "the last entry ends the line");
}

#[test]
fn a_single_entry_has_no_connector() {
    let node: VirtualNode<EuvTimelineProps> = timeline_node(vec![item("Only")]);
    let rendered: VirtualNode = euv_timeline(node);
    let counts: Vec<usize> = entries(&rendered).iter().map(element_children).collect();
    assert_eq!(
        counts,
        vec![2],
        "a lone item is both first and last, so nothing connects to anything"
    );
}

#[test]
fn a_two_item_timeline_connects_only_the_first() {
    let node: VirtualNode<EuvTimelineProps> = timeline_node(vec![item("A"), item("B")]);
    let rendered: VirtualNode = euv_timeline(node);
    let counts: Vec<usize> = entries(&rendered).iter().map(element_children).collect();
    assert_eq!(
        counts,
        vec![3, 2],
        "exactly one connector between two entries"
    );
}

#[test]
fn a_timeline_without_props_renders_an_empty_container() {
    let bare: VirtualNode<EuvTimelineProps> = VirtualNode::Element {
        tag: Tag::Element(Cow::Borrowed("div")),
        attributes: Vec::new(),
        children: Vec::new(),
        key: None,
        props: None,
    };
    let rendered: VirtualNode = euv_timeline(bare);
    assert!(
        matches!(entries(&rendered).first(), Some(VirtualNode::Empty)),
        "default props mean no items, leaving the Empty placeholder slot"
    );
}

#[test]
fn two_timelines_with_different_titles_render_different_trees() {
    let first: VirtualNode = euv_timeline(timeline_node(vec![item("FIRST-TITLE")]));
    let second: VirtualNode = euv_timeline(timeline_node(vec![item("SECOND-TITLE")]));
    assert_ne!(
        first, second,
        "the title travels into the entry key, so the trees differ"
    );
}

#[test]
fn reordering_the_items_changes_the_rendered_tree() {
    let forward: VirtualNode = euv_timeline(timeline_node(vec![item("A"), item("B")]));
    let backward: VirtualNode = euv_timeline(timeline_node(vec![item("B"), item("A")]));
    assert_eq!(
        element_children(&forward),
        element_children(&backward),
        "swapping two same-shaped items keeps the element count"
    );
    assert_ne!(
        forward, backward,
        "yet the trees differ, so the order really is carried"
    );
}
