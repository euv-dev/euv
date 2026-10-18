use super::*;

fn loading_node(overlay: bool) -> VirtualNode<EuvLoadingProps> {
    VirtualNode::Element {
        tag: Tag::Element(Cow::Borrowed("div")),
        attributes: Vec::new(),
        children: Vec::new(),
        key: None,
        props: Some(Box::new(EuvLoadingProps {
            title: "Loading",
            subtitle: "please wait",
            overlay,
            background: "",
        })),
    }
}

fn stat_node(hint: &'static str, icon: &'static str) -> VirtualNode<EuvStatProps> {
    VirtualNode::Element {
        tag: Tag::Element(Cow::Borrowed("div")),
        attributes: Vec::new(),
        children: Vec::new(),
        key: None,
        props: Some(Box::new(EuvStatProps {
            label: "Label",
            value: "42",
            hint,
            icon,
        })),
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
fn a_plain_loading_renders_a_non_empty_tree() {
    let rendered: VirtualNode = euv_loading(loading_node(false));
    assert!(
        total_elements(&rendered) > 0,
        "a loader always renders, got {rendered:?}"
    );
}

#[test]
fn an_overlay_loading_differs_from_a_plain_one() {
    let overlay: VirtualNode = euv_loading(loading_node(true));
    let plain: VirtualNode = euv_loading(loading_node(false));
    assert_ne!(
        overlay, plain,
        "the overlay flag is carried by the class attached to the shell"
    );
}

#[test]
fn the_overlay_flag_does_not_change_the_element_shape() {
    let overlay: VirtualNode = euv_loading(loading_node(true));
    let plain: VirtualNode = euv_loading(loading_node(false));
    assert_eq!(
        total_elements(&overlay),
        total_elements(&plain),
        "the overlay is a styling concern, not a structural one"
    );
}

#[test]
fn the_spinner_draws_three_quarters_of_the_ring_not_one() {
    let style: String = c_spinner().get_style().to_string();
    for side in ["border-top:", "border-right:", "border-bottom:"] {
        assert!(
            style.contains(side),
            "the accent arc must reach {side}, got {style:?} — a single-sided \
             arc reads as a short stub rather than a ring in motion"
        );
    }
    assert!(
        !style.contains("border-left:"),
        "the trailing quarter must stay muted, got {style:?}"
    );
}

#[test]
fn the_sidebar_guide_aligns_with_the_parent_label_text_at_every_depth() {
    let children: String = c_euv_sidebar_children().get_style().to_string();
    let parent: String = c_euv_sidebar_group_title().get_style().to_string();
    let guide_margin: String = token_of(&children, "margin-left");
    let parent_padding: String = shorthand_token(&parent, "padding");
    assert_eq!(
        guide_margin, parent_padding,
        "the guide must sit one parent padding-token left of the parent label, \
         got margin {guide_margin:?} against padding {parent_padding:?} — \
         children {children:?}, parent {parent:?}"
    );
    assert!(
        token_of(&children, "padding-left") != parent_padding,
        "the guide-to-child gap is not the same distance as the parent label's \
         own inset, so matching them would move every child row as well as the \
         guide, got {children:?}"
    );
}

#[test]
fn the_sidebar_active_row_keeps_its_pull_back_off_the_container_margin() {
    let children: String = c_euv_sidebar_children().get_style().to_string();
    let active: String = c_euv_sidebar_link_active_flush().get_style().to_string();
    assert!(
        !active.contains("margin-left: var("),
        "the active fill's own negative margin is the guide distance, not the \
         container's indent — reading the margin token would stretch it to \
         13px and push the fill past the guide, got {active:?}"
    );
    assert!(
        active.contains("padding-left: calc(var(--space-md)"),
        "the pull-back is paid back as left padding so the label keeps its x, got \
         {active:?}"
    );
    assert!(
        children.contains("border-left: 1px dashed")
            && children.contains("padding-left: var(--space-sm);"),
        "the guide the active row pulls back to is the container's dashed \
         border, offset from its rows by the unchanged `space-sm`, got \
         {children:?}"
    );
}

#[test]
fn a_loading_without_props_falls_back_to_defaults() {
    let bare: VirtualNode<EuvLoadingProps> = VirtualNode::Element {
        tag: Tag::Element(Cow::Borrowed("div")),
        attributes: Vec::new(),
        children: Vec::new(),
        key: None,
        props: None,
    };
    let rendered: VirtualNode = euv_loading(bare);
    let expected: VirtualNode = euv_loading(VirtualNode::Element {
        tag: Tag::Element(Cow::Borrowed("div")),
        attributes: Vec::new(),
        children: Vec::new(),
        key: None,
        props: Some(Box::new(EuvLoadingProps {
            title: "",
            subtitle: "",
            overlay: false,
            background: "",
        })),
    });
    assert_eq!(
        rendered, expected,
        "a missing props slot renders the plain loader"
    );
}

#[test]
fn a_bare_stat_renders_a_non_empty_tree() {
    let rendered: VirtualNode = euv_stat(stat_node("", ""));
    assert!(
        total_elements(&rendered) > 0,
        "a stat always renders, got {rendered:?}"
    );
}

#[test]
fn a_stat_with_a_hint_differs_from_a_bare_one() {
    let with_hint: VirtualNode = euv_stat(stat_node("extra", ""));
    let bare: VirtualNode = euv_stat(stat_node("", ""));
    assert_ne!(
        with_hint, bare,
        "a non-empty hint adds its own slot to the stat"
    );
}

#[test]
fn a_stat_with_an_icon_differs_from_a_bare_one() {
    let with_icon: VirtualNode = euv_stat(stat_node("", "I"));
    let bare: VirtualNode = euv_stat(stat_node("", ""));
    assert_ne!(
        with_icon, bare,
        "a non-empty icon adds its own slot to the stat"
    );
}

#[test]
fn a_stat_with_both_hint_and_icon_renders_more_than_either_alone() {
    let both: VirtualNode = euv_stat(stat_node("extra", "I"));
    let hint_only: VirtualNode = euv_stat(stat_node("extra", ""));
    let icon_only: VirtualNode = euv_stat(stat_node("", "I"));
    assert!(
        total_elements(&both) > total_elements(&hint_only)
            && total_elements(&both) > total_elements(&icon_only),
        "hint and icon are independent slots, got {} vs {} and {}",
        total_elements(&both),
        total_elements(&hint_only),
        total_elements(&icon_only)
    );
}

#[test]
fn a_stat_without_props_renders_the_same_shape_as_default_props() {
    let bare: VirtualNode<EuvStatProps> = VirtualNode::Element {
        tag: Tag::Element(Cow::Borrowed("div")),
        attributes: Vec::new(),
        children: Vec::new(),
        key: None,
        props: None,
    };
    let rendered: VirtualNode = euv_stat(bare);
    let expected: VirtualNode = euv_stat(VirtualNode::Element {
        tag: Tag::Element(Cow::Borrowed("div")),
        attributes: Vec::new(),
        children: Vec::new(),
        key: None,
        props: Some(Box::new(EuvStatProps {
            label: "",
            value: "",
            hint: "",
            icon: "",
        })),
    });
    assert_eq!(
        total_elements(&rendered),
        total_elements(&expected),
        "label and value render through fresh signals each time, so two renders \
         can never compare equal; the element count is the observable part"
    );
}

#[test]
fn a_stat_always_renders_its_label_and_value_slots() {
    let rendered: VirtualNode = euv_stat(stat_node("", ""));
    assert_eq!(
        element_count(&rendered, "Element(\"span\")"),
        2,
        "label and value are the two unconditional slots, got {rendered:?}"
    );
}

fn element_count(node: &VirtualNode, needle: &str) -> usize {
    format!("{node:?}").matches(needle).count()
}

fn token_of(style: &str, property: &str) -> String {
    declaration(style, property)
        .split("var(--")
        .nth(1)
        .map(|rest: &str| rest[..rest.find(')').unwrap_or(rest.len())].to_string())
        .unwrap_or_default()
}

fn shorthand_token(style: &str, property: &str) -> String {
    let horizontal: &str = declaration(style, property)
        .rsplit(' ')
        .find(|part: &&str| part.starts_with("var(--"))
        .unwrap_or_default();
    token_of(&format!("padding-left: {horizontal};"), "padding-left")
}

fn declaration<'a>(style: &'a str, property: &str) -> &'a str {
    let found: String = format!("{property}: ");
    let start: usize = style.find(&found).expect("the property is declared");
    let rest: &str = &style[start + found.len()..];
    rest[..rest.find(';').unwrap_or(rest.len())].trim()
}
