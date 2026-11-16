use super::*;

fn props_node<T: Clone + 'static>(props: T) -> VirtualNode<T> {
    VirtualNode::Element {
        tag: Tag::Element(Cow::Borrowed("div")),
        attributes: Vec::new(),
        children: Vec::new(),
        key: None,
        props: Some(Box::new(props)),
    }
}

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

fn attribute_names(node: &VirtualNode) -> Vec<String> {
    let mut out: Vec<String> = Vec::new();
    for element in elements(node) {
        if let VirtualNode::Element { attributes, .. } = &element {
            for entry in attributes.iter() {
                out.push(entry.get_name().to_string());
            }
        }
    }
    out
}

fn has_class(node: &VirtualNode, name: &str) -> bool {
    class_names(node).iter().any(|entry: &String| entry == name)
}

fn has_attribute(node: &VirtualNode, name: &str) -> bool {
    attribute_names(node)
        .iter()
        .any(|entry: &String| entry == name)
}

#[test]
fn vertical_geometry_matches_the_official_proportional_thumb() {
    let (length, offset, range): (f64, f64, f64) =
        scroll_area_geometry(286.0, 1912.0, 286.0, 2.0, 180.0);
    assert!((length - 42.48117154811715).abs() < 0.001);
    assert!((offset - (284.0 - length) * 180.0 / 1626.0).abs() < 0.001);
    assert_eq!(range, 1626.0);
}

#[test]
fn short_frames_use_the_official_sixteen_pixel_minimum_thumb() {
    let (length, offset, range): (f64, f64, f64) =
        scroll_area_geometry(78.0, 1912.0, 78.0, 2.0, 0.0);
    assert_eq!(length, 16.0);
    assert_eq!(offset, 0.0);
    assert_eq!(range, 1834.0);
}

#[test]
fn no_overflow_or_zero_track_has_no_thumb_travel() {
    assert_eq!(
        scroll_area_geometry(286.0, 100.0, 286.0, 2.0, 100.0),
        (284.0, 0.0, 0.0)
    );
    assert_eq!(
        scroll_area_geometry(286.0, 1912.0, 0.0, 2.0, 0.0),
        (0.0, 0.0, 1626.0)
    );
    assert_eq!(
        scroll_area_geometry(0.0, 0.0, 0.0, 2.0, 0.0),
        (0.0, 0.0, 0.0)
    );
}

#[test]
fn pointer_drag_maps_inner_track_travel_and_clamps_at_both_edges() {
    let (length, _, range): (f64, f64, f64) = scroll_area_geometry(286.0, 1912.0, 286.0, 2.0, 0.0);
    let travel: f64 = 284.0 - length;
    assert!((scroll_area_drag_offset(60.0, travel, range) - 403.45).abs() < 1.0);
    assert_eq!(scroll_area_drag_offset(-50.0, travel, range), 0.0);
    assert_eq!(scroll_area_drag_offset(1000.0, travel, range), 1626.0);
    assert_eq!(scroll_area_drag_offset(10.0, 0.0, range), 0.0);
}

#[test]
fn keyboard_edges_pages_and_arrows_are_bounded() {
    assert_eq!(
        scroll_area_key_offset("Home", 180.0, 286.0, 1626.0),
        Some(0.0)
    );
    assert_eq!(
        scroll_area_key_offset("End", 180.0, 286.0, 1626.0),
        Some(1626.0)
    );
    assert_eq!(
        scroll_area_key_offset("PageDown", 180.0, 286.0, 1626.0),
        Some(466.0)
    );
    assert_eq!(
        scroll_area_key_offset("PageUp", 180.0, 286.0, 1626.0),
        Some(0.0)
    );
    assert_eq!(
        scroll_area_key_offset("ArrowDown", 180.0, 286.0, 1626.0),
        Some(220.0)
    );
    assert_eq!(
        scroll_area_key_offset("ArrowUp", 10.0, 286.0, 1626.0),
        Some(0.0)
    );
    assert_eq!(scroll_area_key_offset("Enter", 180.0, 286.0, 1626.0), None);
}

#[test]
fn automatically_composed_parts_have_stable_slots_and_a_focusable_viewport() {
    let node: VirtualNode = euv_scroll_area(props_node(EuvScrollAreaProps { label: "Output" }));
    assert_eq!(
        attribute_names(&node)
            .iter()
            .filter(|name: &&String| name.as_str() == "data-slot")
            .count(),
        4
    );
    assert!(has_attribute(&node, "tabindex"));
    assert!(has_attribute(&node, "aria-hidden"));
    assert!(has_attribute(&node, "ref"));
}

#[test]
fn public_viewport_and_bar_keep_their_existing_part_classes() {
    let viewport: VirtualNode = euv_scroll_area_viewport(props_node(EuvScrollAreaProps::default()));
    let bar: VirtualNode = euv_scroll_bar(props_node(EuvScrollAreaProps::default()));
    assert!(has_class(&viewport, "c_euv_scroll_area_viewport"));
    assert!(has_class(&bar, "c_euv_scroll_bar"));
    assert!(has_class(&bar, "c_euv_scroll_bar_thumb"));
}

#[test]
fn a_scroll_area_is_a_frame_around_a_scrolling_viewport() {
    let node: VirtualNode = euv_scroll_area(props_node(EuvScrollAreaProps { label: "Output" }));
    assert!(has_class(&node, "c_euv_scroll_area"));
    assert!(has_class(&node, "c_euv_scroll_area_viewport"));
    assert!(has_class(&node, "c_euv_scroll_bar"));
    assert!(has_class(&node, "c_euv_scroll_bar_thumb"));
}

#[test]
fn a_scroll_area_is_announced_as_a_region() {
    let node: VirtualNode = euv_scroll_area(props_node(EuvScrollAreaProps { label: "Output" }));
    assert!(has_attribute(&node, "role"));
    assert!(has_attribute(&node, "aria-label"));
}
