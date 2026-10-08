use super::*;

fn badge_node(outline: bool, text: &'static str) -> VirtualNode<EuvBadgeProps> {
    VirtualNode::Element {
        tag: Tag::Element(Cow::Borrowed("div")),
        attributes: Vec::new(),
        children: Vec::new(),
        key: None,
        props: Some(Box::new(EuvBadgeProps {
            text,
            outline,
            on_click: None,
        })),
    }
}

fn tag_of(node: &VirtualNode) -> String {
    match node {
        VirtualNode::Element { tag, .. } => format!("{tag:?}"),
        _ => panic!("expected an element node"),
    }
}

#[test]
fn a_filled_badge_renders_its_label_as_a_text_child() {
    let rendered: VirtualNode = euv_badge(badge_node(false, "New"));
    assert!(
        matches!(rendered, VirtualNode::Element { .. }),
        "a badge is an element"
    );
    let children: &[VirtualNode] = rendered.get_children();
    assert_eq!(children.len(), 1, "the label is the one nested child");
    match &children[0] {
        VirtualNode::Text(text_node) => assert!(
            format!("{:?}", text_node).contains("New"),
            "the label text is carried through, got {text_node:?}"
        ),
        _ => panic!("the label must be a text node, got {:?}", children[0]),
    }
}

#[test]
fn an_outlined_badge_renders_the_same_shape_as_a_filled_one() {
    let filled: VirtualNode = euv_badge(badge_node(false, "New"));
    let outlined: VirtualNode = euv_badge(badge_node(true, "New"));
    assert_eq!(
        tag_of(&outlined),
        tag_of(&filled),
        "the outline variant only swaps the class list, not the element"
    );
}

#[test]
fn a_badge_without_props_falls_back_to_defaults() {
    let bare: VirtualNode<EuvBadgeProps> = VirtualNode::Element {
        tag: Tag::Element(Cow::Borrowed("div")),
        attributes: Vec::new(),
        children: Vec::new(),
        key: None,
        props: None,
    };
    let rendered: VirtualNode = euv_badge(bare);
    assert!(
        matches!(rendered, VirtualNode::Element { .. }),
        "defaults still render"
    );
}

#[test]
fn a_half_filled_rating_is_fifty_percent() {
    let observed: f64 = rating_fill_percent(2.5, 5.0);
    assert!(
        (observed - 50.0).abs() < 1e-9,
        "half of five is fifty percent, got {observed}"
    );
}

#[test]
fn a_full_rating_is_one_hundred_percent() {
    let observed: f64 = rating_fill_percent(5.0, 5.0);
    assert!(
        (observed - 100.0).abs() < 1e-9,
        "every star lit is one hundred percent, got {observed}"
    );
}

#[test]
fn an_empty_rating_is_zero_percent() {
    let observed: f64 = rating_fill_percent(0.0, 5.0);
    assert_eq!(observed, 0.0, "no stars lit is zero percent");
}

#[test]
fn the_fill_is_clamped_when_the_value_runs_past_the_maximum() {
    let observed: f64 = rating_fill_percent(9.0, 5.0);
    assert_eq!(
        observed, 100.0,
        "a value beyond the maximum must not overflow the bar"
    );
}

#[test]
fn the_fill_is_clamped_when_the_value_is_negative() {
    let observed: f64 = rating_fill_percent(-3.0, 5.0);
    assert_eq!(observed, 0.0, "a negative value must not underflow the bar");
}

#[test]
fn a_zero_maximum_reports_no_fill_rather_than_dividing_by_zero() {
    let observed: f64 = rating_fill_percent(3.0, 0.0);
    assert_eq!(
        observed, 0.0,
        "an unset maximum is a degenerate scale, not an infinite bar"
    );
}

#[test]
fn a_negative_maximum_reports_no_fill() {
    let observed: f64 = rating_fill_percent(3.0, -1.0);
    assert_eq!(observed, 0.0, "a negative scale is treated as degenerate");
}

#[test]
fn a_single_star_scale_reaches_one_hundred_percent() {
    let observed: f64 = rating_fill_percent(1.0, 1.0);
    assert!(
        (observed - 100.0).abs() < 1e-9,
        "a one-star scale fills at the first star, got {observed}"
    );
}

#[test]
fn a_ten_star_scale_halves_at_five() {
    let observed: f64 = rating_fill_percent(5.0, 10.0);
    assert!(
        (observed - 50.0).abs() < 1e-9,
        "five of ten is fifty percent, got {observed}"
    );
}

#[test]
fn a_progress_percent_inside_the_range_is_unchanged() {
    let observed: f64 = progress_percent_clamp(42.0);
    assert!(
        (observed - 42.0).abs() < 1e-9,
        "a value already in range passes through, got {observed}"
    );
}

#[test]
fn a_progress_percent_above_the_maximum_is_clamped() {
    let observed: f64 = progress_percent_clamp(150.0);
    assert_eq!(observed, 100.0, "a bar can never exceed full");
}

#[test]
fn a_progress_percent_below_zero_is_clamped() {
    let observed: f64 = progress_percent_clamp(-20.0);
    assert_eq!(observed, 0.0, "a bar can never run backwards");
}

#[test]
fn a_nan_progress_percent_becomes_zero() {
    let observed: f64 = progress_percent_clamp(f64::NAN);
    assert_eq!(
        observed, 0.0,
        "NaN fails every comparison, so it must be caught before the clamp \
         or it would pass through unchanged and poison the style string"
    );
}

#[test]
fn the_endpoints_of_the_progress_range_survive() {
    let at_zero: f64 = progress_percent_clamp(0.0);
    let at_full: f64 = progress_percent_clamp(100.0);
    assert_eq!(at_zero, 0.0, "the lower endpoint is inclusive");
    assert_eq!(at_full, 100.0, "and so is the upper one");
}

#[test]
fn an_infinite_progress_percent_is_clamped() {
    let positive: f64 = progress_percent_clamp(f64::INFINITY);
    let negative: f64 = progress_percent_clamp(f64::NEG_INFINITY);
    assert_eq!(positive, 100.0, "positive infinity saturates at full");
    assert_eq!(negative, 0.0, "negative infinity saturates at empty");
}

#[test]
fn a_skeleton_line_count_inside_the_range_is_unchanged() {
    let observed: usize = skeleton_line_count(3);
    assert_eq!(observed, 3, "a middling count passes through");
}

#[test]
fn a_skeleton_line_count_below_the_minimum_is_raised() {
    let observed: usize = skeleton_line_count(0);
    assert!(
        observed > 0,
        "a skeleton always draws at least one placeholder line, got {observed}"
    );
}

#[test]
fn a_skeleton_line_count_above_the_maximum_is_capped() {
    let observed: usize = skeleton_line_count(1000);
    assert!(
        observed < 1000,
        "an absurd line count must be capped so a tall skeleton does not \
         blow up the layout, got {observed}"
    );
}

#[test]
fn the_skeleton_line_count_is_monotonic() {
    let low: usize = skeleton_line_count(1);
    let high: usize = skeleton_line_count(2);
    assert!(
        high >= low,
        "asking for more lines must never produce fewer, got {low} then {high}"
    );
}
