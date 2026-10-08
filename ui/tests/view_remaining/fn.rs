use super::*;

fn total(node: &VirtualNode) -> usize {
    match node {
        VirtualNode::Element { children, .. } => 1 + children.iter().map(total).sum::<usize>(),
        VirtualNode::Fragment(children) => children.iter().map(total).sum(),
        _ => 0,
    }
}

fn modal_node(title: &'static str) -> VirtualNode<EuvModalProps> {
    VirtualNode::Element {
        tag: Tag::Element(Cow::Borrowed("div")),
        attributes: Vec::new(),
        children: Vec::new(),
        key: None,
        props: Some(Box::new(EuvModalProps {
            title,
            onclick: None,
        })),
    }
}

fn skeleton_node(lines: usize) -> VirtualNode<EuvSkeletonProps> {
    VirtualNode::Element {
        tag: Tag::Element(Cow::Borrowed("div")),
        attributes: Vec::new(),
        children: Vec::new(),
        key: None,
        props: Some(Box::new(EuvSkeletonProps {
            lines,
            width: "100%",
            height: "1rem",
            rounded: Signal::create(false),
        })),
    }
}

fn progress_node(percent: f64, active: bool) -> VirtualNode<EuvProgressProps> {
    VirtualNode::Element {
        tag: Tag::Element(Cow::Borrowed("div")),
        attributes: Vec::new(),
        children: Vec::new(),
        key: None,
        props: Some(Box::new(EuvProgressProps {
            percent: Signal::create(percent),
            label: "Loading",
            active: Signal::create(active),
        })),
    }
}

fn rating_node(value: f64, max: f64) -> VirtualNode<EuvRatingProps> {
    VirtualNode::Element {
        tag: Tag::Element(Cow::Borrowed("div")),
        attributes: Vec::new(),
        children: Vec::new(),
        key: None,
        props: Some(Box::new(EuvRatingProps {
            value: Signal::create(value),
            max: Signal::create(max),
            readonly: Signal::create(true),
            size: EuvRatingSize::default(),
        })),
    }
}

#[test]
fn a_modal_renders_a_dialog_shell() {
    let rendered: VirtualNode = euv_modal(modal_node("T"));
    assert!(
        total(&rendered) > 0,
        "a modal always renders, got {rendered:?}"
    );
    assert_eq!(
        total(&rendered),
        6,
        "a backdrop, a panel and the title plus body slots, got {rendered:?}"
    );
}

#[test]
fn a_modal_with_a_title_differs_from_one_without() {
    let titled: VirtualNode = euv_modal(modal_node("T"));
    let plain: VirtualNode = euv_modal(modal_node(""));
    assert_ne!(
        titled, plain,
        "the title reaches the rendered tree, so the two must be distinguishable"
    );
}

#[test]
fn a_modal_without_props_renders_without_panicking() {
    let bare: VirtualNode<EuvModalProps> = VirtualNode::Element {
        tag: Tag::Element(Cow::Borrowed("div")),
        attributes: Vec::new(),
        children: Vec::new(),
        key: None,
        props: None,
    };
    let rendered: VirtualNode = euv_modal(bare);
    assert!(
        total(&rendered) > 0,
        "the modal frame renders, got {rendered:?}"
    );
}

#[test]
fn a_skeleton_draws_one_placeholder_per_line() {
    let one: VirtualNode = euv_skeleton(skeleton_node(1));
    let three: VirtualNode = euv_skeleton(skeleton_node(3));
    assert_eq!(
        total(&one),
        2,
        "one line is the wrapper plus its placeholder"
    );
    assert_eq!(
        total(&three),
        4,
        "three lines are the wrapper plus three placeholders"
    );
}

#[test]
fn the_skeleton_element_count_follows_lines_plus_one() {
    for lines in 1_usize..=6 {
        let rendered: VirtualNode = euv_skeleton(skeleton_node(lines));
        assert_eq!(
            total(&rendered),
            lines + 1,
            "{lines} lines must render {lines} placeholders inside one wrapper"
        );
    }
}

#[test]
fn a_skeleton_without_props_falls_back_to_a_default_line_count() {
    let bare: VirtualNode<EuvSkeletonProps> = VirtualNode::Element {
        tag: Tag::Element(Cow::Borrowed("div")),
        attributes: Vec::new(),
        children: Vec::new(),
        key: None,
        props: None,
    };
    let rendered: VirtualNode = euv_skeleton(bare);
    let expected: VirtualNode = euv_skeleton(VirtualNode::Element {
        tag: Tag::Element(Cow::Borrowed("div")),
        attributes: Vec::new(),
        children: Vec::new(),
        key: None,
        props: Some(Box::new(EuvSkeletonProps {
            lines: skeleton_line_count(0),
            width: "",
            height: "",
            rounded: Signal::create(false),
        })),
    });
    assert_eq!(
        total(&rendered),
        total(&expected),
        "the default line count is clamped by skeleton_line_count either way"
    );
}

#[test]
fn a_progress_bar_keeps_a_stable_shape_across_percentages() {
    let shapes: Vec<usize> = [0.0_f64, 50.0, 100.0]
        .iter()
        .map(|p: &f64| total(&euv_progress(progress_node(*p, true))))
        .collect();
    assert!(
        shapes.windows(2).all(|w: &[usize]| w[0] == w[1]),
        "the fill width is an inline style, not an extra node, got {shapes:?}"
    );
}

#[test]
fn two_percentages_still_produce_distinguishable_trees() {
    let half: VirtualNode = euv_progress(progress_node(50.0, true));
    let full: VirtualNode = euv_progress(progress_node(100.0, true));
    assert_ne!(
        half, full,
        "the inline width style must differ even though the node count does not"
    );
}

#[test]
fn the_active_flag_changes_the_progress_tree() {
    let running: VirtualNode = euv_progress(progress_node(50.0, true));
    let idle: VirtualNode = euv_progress(progress_node(50.0, false));
    assert_ne!(
        running, idle,
        "an indeterminate bar is styled differently from a determinate one"
    );
    assert_eq!(
        total(&running),
        total(&idle),
        "while using the same number of nodes"
    );
}

#[test]
fn a_progress_bar_without_props_renders_its_frame() {
    let bare: VirtualNode<EuvProgressProps> = VirtualNode::Element {
        tag: Tag::Element(Cow::Borrowed("div")),
        attributes: Vec::new(),
        children: Vec::new(),
        key: None,
        props: None,
    };
    let rendered: VirtualNode = euv_progress(bare);
    assert!(
        total(&rendered) > 0,
        "the bar frame renders, got {rendered:?}"
    );
}

#[test]
fn a_rating_keeps_one_node_per_star() {
    let three: VirtualNode = euv_rating(rating_node(3.0, 5.0));
    let five: VirtualNode = euv_rating(rating_node(5.0, 5.0));
    assert_eq!(
        total(&three),
        total(&five),
        "the fill level is a style, so both ratings use the same nodes"
    );
}

#[test]
fn the_star_count_follows_the_maximum_not_the_value() {
    let three_of_five: VirtualNode = euv_rating(rating_node(3.0, 5.0));
    let five_of_five: VirtualNode = euv_rating(rating_node(5.0, 5.0));
    let one_of_three: VirtualNode = euv_rating(rating_node(1.0, 3.0));
    assert_eq!(
        total(&three_of_five),
        total(&five_of_five),
        "3.0 and 5.0 share a max of 5, so they draw the same number of stars"
    );
    assert!(
        total(&one_of_three) < total(&three_of_five),
        "a three-star scale draws fewer stars than a five-star one, got {} vs {}",
        total(&one_of_three),
        total(&three_of_five)
    );
}

#[test]
fn two_fill_levels_produce_distinguishable_trees() {
    let three: VirtualNode = euv_rating(rating_node(3.0, 5.0));
    let five: VirtualNode = euv_rating(rating_node(5.0, 5.0));
    assert_ne!(
        three, five,
        "the filled width must differ between a partial and a full rating"
    );
}

#[test]
fn a_rating_without_props_renders_its_frame() {
    let bare: VirtualNode<EuvRatingProps> = VirtualNode::Element {
        tag: Tag::Element(Cow::Borrowed("div")),
        attributes: Vec::new(),
        children: Vec::new(),
        key: None,
        props: None,
    };
    let rendered: VirtualNode = euv_rating(bare);
    assert!(
        total(&rendered) > 0,
        "the rating frame renders, got {rendered:?}"
    );
}
