use super::*;

fn item_node(_index: usize) -> VirtualNode {
    VirtualNode::Element {
        tag: Tag::Element(Cow::Borrowed("div")),
        attributes: Vec::new(),
        children: Vec::new(),
        key: None,
        props: None,
    }
}

fn list_node(config: EuvVirtualListConfig) -> VirtualNode<EuvVirtualListProps> {
    VirtualNode::Element {
        tag: Tag::Element(Cow::Borrowed("div")),
        attributes: Vec::new(),
        children: Vec::new(),
        key: None,
        props: Some(Box::new(EuvVirtualListProps {
            config,
            item_renderer: Rc::new(|index: usize| item_node(index)),
            on_scroll: None,
            on_visible_range_change: None,
        })),
    }
}

fn config(total_count: usize, item_height: i32, overscan_count: usize) -> EuvVirtualListConfig {
    EuvVirtualListConfig {
        id: String::from("rows"),
        total_count,
        item_height,
        overscan_count,
    }
}

fn count_items(node: &VirtualNode) -> usize {
    match node {
        VirtualNode::Element {
            attributes,
            children,
            ..
        } => {
            let is_item: usize = usize::from(attributes.is_empty());
            is_item + children.iter().map(count_items).sum::<usize>()
        }
        VirtualNode::Fragment(children) => children.iter().map(count_items).sum::<usize>(),
        _ => 0,
    }
}

fn root_attributes(node: &VirtualNode) -> String {
    match node {
        VirtualNode::Element { attributes, .. } => format!("{attributes:?}"),
        _ => String::new(),
    }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
#[cfg_attr(
    not(target_arch = "wasm32"),
    ignore = "reads its viewport through a signal the native build never ticks"
)]
fn a_long_list_is_windowed_instead_of_rendering_every_item() {
    let rendered: VirtualNode = euv_virtual_list(list_node(config(5000, 24, 2)));
    let observed: usize = count_items(&rendered);
    assert!(
        observed > 0,
        "the first viewport still has to render something, otherwise the list is blank"
    );
    assert!(
        observed < 5000,
        "a 5000 item list must stay windowed, but every item was rendered"
    );
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
#[cfg_attr(
    not(target_arch = "wasm32"),
    ignore = "reads its viewport through a signal the native build never ticks"
)]
fn a_list_shorter_than_one_viewport_renders_all_of_its_items() {
    let rendered: VirtualNode = euv_virtual_list(list_node(config(3, 24, 2)));
    let observed: usize = count_items(&rendered);
    assert_eq!(
        observed, 3,
        "windowing must never hide an item that the viewport can actually show"
    );
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
#[cfg_attr(
    not(target_arch = "wasm32"),
    ignore = "reads its viewport through a signal the native build never ticks"
)]
fn an_unmeasured_viewport_falls_back_to_a_fixed_window_plus_overscan() {
    let plain: usize = count_items(&euv_virtual_list(list_node(config(5000, 10, 0))));
    let padded: usize = count_items(&euv_virtual_list(list_node(config(5000, 10, 3))));
    assert_eq!(
        plain, 20,
        "before any measurement there is no viewport, so the list must fall back to a fixed window"
    );
    assert_eq!(
        padded, 23,
        "the fallback window is widened by the overscan on the trailing edge"
    );
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
#[cfg_attr(
    not(target_arch = "wasm32"),
    ignore = "reads its viewport through a signal the native build never ticks"
)]
fn the_fallback_window_is_never_wider_than_the_list_itself() {
    let observed: usize = count_items(&euv_virtual_list(list_node(config(6, 10, 3))));
    assert_eq!(
        observed, 6,
        "overscan must not invent rows past the end of the list"
    );
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
#[cfg_attr(
    not(target_arch = "wasm32"),
    ignore = "reads its viewport through a signal the native build never ticks"
)]
fn an_empty_list_renders_no_rows_at_all() {
    let observed: usize = count_items(&euv_virtual_list(list_node(config(0, 24, 4))));
    assert_eq!(
        observed, 0,
        "a list with no items must not render placeholder rows"
    );
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
#[cfg_attr(
    not(target_arch = "wasm32"),
    ignore = "reads its viewport through a signal the native build never ticks"
)]
fn the_container_is_wired_for_id_lookup_ref_capture_and_scrolling() {
    let rendered: VirtualNode = euv_virtual_list(list_node(config(500, 24, 2)));
    let names: String = root_attributes(&rendered);
    for expected in ["name: \"id\"", "name: \"ref\"", "name: \"onscroll\""] {
        assert!(
            names.contains(expected),
            "the container must expose {expected} to the rest of the component, got {names}"
        );
    }
}
