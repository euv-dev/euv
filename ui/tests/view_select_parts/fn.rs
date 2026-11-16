use super::*;

fn elements(node: &VirtualNode) -> Vec<VirtualNode> {
    let mut out: Vec<VirtualNode> = Vec::new();
    collect(node, &mut out);
    out
}

fn collect(node: &VirtualNode, out: &mut Vec<VirtualNode>) {
    match node {
        VirtualNode::Element { children, .. } => {
            out.push(node.clone());
            for child in children.iter() {
                collect(child, out);
            }
        }
        VirtualNode::Fragment(children) => {
            for child in children.iter() {
                collect(child, out);
            }
        }
        _ => {}
    }
}

fn class_names(node: &VirtualNode) -> Vec<String> {
    let mut out: Vec<String> = Vec::new();
    for element in elements(node) {
        if let VirtualNode::Element { attributes, .. } = &element {
            for entry in attributes.iter() {
                if entry.get_name() == "class" {
                    out.extend(entry.class_names());
                }
            }
        }
    }
    out
}

fn has_class(node: &VirtualNode, name: &str) -> bool {
    class_names(node).iter().any(|entry: &String| entry == name)
}

fn has_attribute(node: &VirtualNode, name: &str) -> bool {
    for element in elements(node) {
        if let VirtualNode::Element { attributes, .. } = &element {
            for entry in attributes.iter() {
                if entry.get_name() == name {
                    return true;
                }
            }
        }
    }
    false
}

fn has_slot(node: &VirtualNode, slot: &str) -> bool {
    for element in elements(node) {
        if let VirtualNode::Element { attributes, .. } = &element {
            for entry in attributes.iter() {
                if entry.get_name() == "data-slot" && entry.text_value().as_deref() == Some(slot) {
                    return true;
                }
            }
        }
    }
    false
}

#[test]
fn a_select_scroll_up_button_carries_its_class_and_slot() {
    let node: VirtualNode = euv_select_scroll_up_button(VirtualNode::Empty);
    assert!(has_class(&node, "c_euv_select_scroll_up_button"));
    assert!(has_attribute(&node, "data-slot"));
    assert!(has_slot(&node, "select-scroll-up-button"));
    assert!(has_attribute(&node, "onclick"));
    assert!(has_attribute(&node, "onmousedown"));
}

#[test]
fn a_select_scroll_down_button_carries_its_class_and_slot() {
    let node: VirtualNode = euv_select_scroll_down_button(VirtualNode::Empty);
    assert!(has_class(&node, "c_euv_select_scroll_down_button"));
    assert!(has_attribute(&node, "data-slot"));
    assert!(has_slot(&node, "select-scroll-down-button"));
    assert!(has_attribute(&node, "onclick"));
    assert!(has_attribute(&node, "onmousedown"));
}

#[test]
fn select_scroll_buttons_move_half_a_viewport_in_either_direction() {
    let cases: &[(i32, i32, i32, bool, i32)] = &[
        (0, 200, 1000, false, 100),
        (400, 200, 1000, true, 300),
        (400, 200, 1000, false, 500),
        (0, 1, 1000, false, 1),
    ];
    for &(top, height, total, upward, expected) in cases {
        assert_eq!(
            select_scroll_button_target(top, height, total, upward),
            expected
        );
    }
}

#[test]
fn select_scroll_buttons_clamp_to_the_option_list_boundaries() {
    let cases: &[(i32, i32, i32, bool, i32)] = &[
        (0, 200, 1000, true, 0),
        (790, 200, 1000, false, 800),
        (1000, 200, 1000, true, 700),
        (-100, 200, 1000, false, 100),
        (0, 200, 100, false, 0),
        (i32::MAX, 1, i32::MAX, false, i32::MAX - 1),
    ];
    for &(top, height, total, upward, expected) in cases {
        assert_eq!(
            select_scroll_button_target(top, height, total, upward),
            expected
        );
    }
}

#[test]
fn select_scroll_buttons_do_not_move_an_unmeasured_viewport() {
    assert_eq!(select_scroll_button_target(0, 0, 1000, false), 0);
    assert_eq!(select_scroll_button_target(400, 0, 1000, true), 400);
    assert_eq!(select_scroll_button_target(200, -2, 1000, false), 200);
    assert_eq!(select_scroll_button_target(200, 0, -1, false), 0);
}

#[test]
fn dropdown_geometry_reads_selected_class_metadata_without_a_browser() {
    let policy: DropdownGeometryPolicy = dropdown_geometry_policy(c_euv_dropdown_menu());
    let content: bool = cfg!(all(feature = "theme-shadcn", not(feature = "theme-custom")));
    assert_eq!(policy.get_align_start(), content);
    assert_eq!(policy.get_anchor_trigger(), content);
    assert_eq!(policy.get_content_width(), content);
    assert_eq!(policy.get_gap(), if content { 4.0 } else { 8.0 });
    assert_eq!(
        dropdown_popup_position(
            (100.5, 180.75, 40.25, 60.5),
            (50.25, 80.0),
            (400.0, 300.0),
            policy.get_gap()
        ),
        dropdown_popup_position_aligned(
            (100.5, 180.75, 40.25, 60.5),
            (50.25, 80.0),
            (400.0, 300.0),
            policy.get_gap(),
            content
        )
    );
}

#[test]
fn dropdown_empty_class_keeps_default_geometry() {
    let policy: DropdownGeometryPolicy = dropdown_geometry_policy(&Css::default());
    assert!(!policy.get_align_start());
    assert!(!policy.get_anchor_trigger());
    assert!(!policy.get_content_width());
    assert_eq!(policy.get_gap(), 8.0);
}

#[test]
fn dropdown_geometry_metadata_is_semantic_not_a_feature_identity() {
    let css: Css = Css::new(
        String::from("arbitrary-menu"),
        String::from(
            "--euv-dropdown-align:start;--euv-dropdown-anchor:root;--euv-dropdown-width-mode:anchor;--euv-dropdown-gap:2.5;",
        ),
        Vec::new(),
        Vec::new(),
    );
    let policy: DropdownGeometryPolicy = dropdown_geometry_policy(&css);
    assert!(policy.get_align_start());
    assert!(!policy.get_anchor_trigger());
    assert!(!policy.get_content_width());
    assert_eq!(policy.get_gap(), 2.5);
}

#[test]
fn dropdown_alignment_flip_and_clamp_preserve_fractional_coordinates() {
    for align_start in [false, true] {
        let (left, top, available): (f64, f64, f64) = dropdown_popup_position_aligned(
            (100.5, 180.75, 240.25, 260.5),
            (50.25, 80.0),
            (400.0, 300.0),
            4.0,
            align_start,
        );
        assert_eq!(left, if align_start { 100.5 } else { 130.5 });
        assert_eq!(top, 156.25);
        assert_eq!(available, 228.25);
        let (left, top, available): (f64, f64, f64) = dropdown_popup_position_aligned(
            (-20.5, 500.75, 40.25, 60.5),
            (500.0, 1000.0),
            (320.0, 300.0),
            8.0,
            align_start,
        );
        assert_eq!(left, 8.0);
        assert_eq!(top, 68.5);
        assert_eq!(available, 223.5);
    }
}

#[test]
fn dropdown_width_policy_preserves_anchor_scale_and_submenu_override() {
    assert_eq!(
        dropdown_popup_width(120.75, 304.0, 1.25, false, false),
        Some(96.6)
    );
    assert_eq!(
        dropdown_popup_width(400.0, 304.0, 1.25, false, false),
        Some(243.2)
    );
    assert_eq!(dropdown_popup_width(120.75, 304.0, 1.25, true, false), None);
    assert_eq!(dropdown_popup_width(120.75, 304.0, 1.25, false, true), None);
    assert_eq!(dropdown_popup_width(120.75, 304.0, 1.25, true, true), None);
}
