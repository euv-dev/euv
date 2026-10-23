use super::*;

fn entry(label: &'static str, value: &'static str) -> EuvDescriptionsItem {
    EuvDescriptionsItem { label, value }
}

fn descriptions_node(
    items: Vec<EuvDescriptionsItem>,
    columns: usize,
) -> VirtualNode<EuvDescriptionsProps> {
    VirtualNode::Element {
        tag: Tag::Element(Cow::Borrowed("div")),
        attributes: Vec::new(),
        children: Vec::new(),
        key: None,
        props: Some(Box::new(EuvDescriptionsProps { items, columns })),
    }
}

fn child_count(node: &VirtualNode) -> usize {
    match node {
        VirtualNode::Element { children, .. } => children.iter().map(count_one).sum(),
        VirtualNode::Fragment(children) => children.iter().map(count_one).sum(),
        _ => panic!("expected an element or fragment node, got {node:?}"),
    }
}

fn count_one(node: &VirtualNode) -> usize {
    match node {
        VirtualNode::Fragment(children) => children.iter().map(count_one).sum(),
        VirtualNode::Empty => 0,
        _ => 1,
    }
}

fn class_lists(node: &VirtualNode, into: &mut Vec<Vec<String>>) {
    match node {
        VirtualNode::Element {
            attributes,
            children,
            ..
        } => {
            for attribute in attributes {
                if attribute.get_name() == "class" {
                    into.push(attribute.class_names());
                }
            }
            for child in children {
                class_lists(child, into);
            }
        }
        VirtualNode::Fragment(children) => {
            for child in children {
                class_lists(child, into);
            }
        }
        _ => {}
    }
}

fn style_of(node: &VirtualNode) -> Option<String> {
    match node {
        VirtualNode::Element {
            attributes,
            children,
            ..
        } => {
            for attribute in attributes {
                if attribute.get_name() == "style" {
                    return attribute.text_value();
                }
            }
            for child in children {
                if let Some(style) = style_of(child) {
                    return Some(style);
                }
            }
            None
        }
        VirtualNode::Fragment(children) => {
            for child in children {
                if let Some(style) = style_of(child) {
                    return Some(style);
                }
            }
            None
        }
        _ => None,
    }
}

fn carries_text(node: &VirtualNode, needle: &str) -> bool {
    format!("{node:?}").contains(needle)
}

#[test]
fn a_descriptions_grid_renders_one_cell_per_item() {
    let node: VirtualNode<EuvDescriptionsProps> = descriptions_node(
        vec![
            entry("Name", "Ada"),
            entry("Role", "Engineer"),
            entry("Team", "Platform"),
        ],
        2,
    );
    let rendered: VirtualNode = euv_descriptions(node);
    assert_eq!(
        child_count(&rendered),
        3,
        "one cell is rendered per entry, got {rendered:?}"
    );
}

#[test]
fn every_cell_carries_the_item_label_and_value_classes() {
    let node: VirtualNode<EuvDescriptionsProps> = descriptions_node(vec![entry("Name", "Ada")], 1);
    let rendered: VirtualNode = euv_descriptions(node);
    let mut lists: Vec<Vec<String>> = Vec::new();
    class_lists(&rendered, &mut lists);
    assert_eq!(
        lists,
        vec![
            vec![String::from("c_euv_descriptions")],
            vec![String::from("c_euv_descriptions_item")],
            vec![String::from("c_euv_descriptions_label")],
            vec![String::from("c_euv_descriptions_value")],
        ],
        "the grid, the cell, the label and the value are all classed, got {lists:?}"
    );
}

#[test]
fn every_cell_carries_its_label_and_value_text() {
    let node: VirtualNode<EuvDescriptionsProps> = descriptions_node(vec![entry("Name", "Ada")], 1);
    let rendered: VirtualNode = euv_descriptions(node);
    assert!(
        carries_text(&rendered, "Name"),
        "the cell label must reach the tree, got {rendered:?}"
    );
    assert!(
        carries_text(&rendered, "Ada"),
        "the cell value must reach the tree, got {rendered:?}"
    );
}

#[test]
fn the_column_count_is_written_onto_the_grid_as_a_custom_property() {
    let node: VirtualNode<EuvDescriptionsProps> = descriptions_node(vec![entry("Name", "Ada")], 3);
    let rendered: VirtualNode = euv_descriptions(node);
    assert_eq!(
        style_of(&rendered),
        Some(String::from("--descriptions-columns: 3")),
        "the grid rule reads the column count from a custom property, got {rendered:?}"
    );
}

#[test]
fn a_zero_column_count_falls_back_to_a_single_column() {
    let node: VirtualNode<EuvDescriptionsProps> = descriptions_node(vec![entry("Name", "Ada")], 0);
    let rendered: VirtualNode = euv_descriptions(node);
    assert_eq!(
        style_of(&rendered),
        Some(String::from("--descriptions-columns: 1")),
        "a zero repeat count is not a valid grid track, so it must fall back to one, got {rendered:?}"
    );
}

#[test]
fn the_column_count_changes_the_grid_without_changing_its_cells() {
    let single: VirtualNode = euv_descriptions(descriptions_node(vec![entry("Name", "Ada")], 1));
    let double: VirtualNode = euv_descriptions(descriptions_node(vec![entry("Name", "Ada")], 2));
    assert_eq!(
        child_count(&single),
        child_count(&double),
        "the column count is a layout knob, not a structural one"
    );
    assert_ne!(
        style_of(&single),
        style_of(&double),
        "and the two column counts must not ask for the same grid"
    );
}

#[test]
fn an_empty_descriptions_renders_an_empty_grid() {
    let node: VirtualNode<EuvDescriptionsProps> = descriptions_node(Vec::new(), 2);
    let rendered: VirtualNode = euv_descriptions(node);
    assert_eq!(
        child_count(&rendered),
        0,
        "no entries means no cells, but the grid container is still there, got {rendered:?}"
    );
    let mut lists: Vec<Vec<String>> = Vec::new();
    class_lists(&rendered, &mut lists);
    assert_eq!(
        lists,
        vec![vec![String::from("c_euv_descriptions")]],
        "the empty grid container is still rendered, got {lists:?}"
    );
}

#[test]
fn a_descriptions_without_props_falls_back_to_defaults() {
    let bare: VirtualNode<EuvDescriptionsProps> = VirtualNode::Element {
        tag: Tag::Element(Cow::Borrowed("div")),
        attributes: Vec::new(),
        children: Vec::new(),
        key: None,
        props: None,
    };
    let rendered: VirtualNode = euv_descriptions(bare);
    assert_eq!(
        child_count(&rendered),
        0,
        "the default props carry no entries, got {rendered:?}"
    );
    assert_eq!(
        style_of(&rendered),
        Some(String::from("--descriptions-columns: 1")),
        "and the default column count is the single-column fallback"
    );
}
