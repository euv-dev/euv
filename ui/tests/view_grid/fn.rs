use super::*;

fn props_with_children<T>(value: T, children: Vec<VirtualNode>) -> VirtualNode<T> {
    VirtualNode::Element {
        tag: Tag::Element(Cow::Borrowed("div")),
        attributes: Vec::new(),
        children,
        key: None,
        props: Some(Box::new(value)),
    }
}

fn cell(marker: &str) -> VirtualNode {
    VirtualNode::Element {
        tag: Tag::Element(Cow::Borrowed("div")),
        attributes: vec![AttributeEntry::new(
            Cow::Borrowed("class"),
            AttributeValue::Text(marker.to_string()),
        )],
        children: Vec::new(),
        key: None,
        props: None,
    }
}

fn classes_of(node: &VirtualNode, into: &mut Vec<String>) {
    match node {
        VirtualNode::Element {
            attributes,
            children,
            ..
        } => {
            for attribute in attributes {
                if attribute.get_name() == "class" {
                    into.extend(attribute.class_names());
                }
            }
            for child in children {
                classes_of(child, into);
            }
        }
        VirtualNode::Fragment(children) => {
            for child in children {
                classes_of(child, into);
            }
        }
        _ => {}
    }
}

fn styles_of(node: &VirtualNode, into: &mut Vec<String>) {
    match node {
        VirtualNode::Element {
            attributes,
            children,
            ..
        } => {
            for attribute in attributes {
                if attribute.get_name() == "style" {
                    into.extend(attribute.text_value());
                }
            }
            for child in children {
                styles_of(child, into);
            }
        }
        VirtualNode::Fragment(children) => {
            for child in children {
                styles_of(child, into);
            }
        }
        _ => {}
    }
}

fn grid_with(cols: Vec<EuvGridCol>, gutter: bool, cell_count: usize) -> VirtualNode {
    let children: Vec<VirtualNode> = (0..cell_count)
        .map(|index: usize| cell(&format!("cell-{index}")))
        .collect();
    euv_grid(props_with_children(EuvGridProps { cols, gutter }, children))
}

#[test]
fn a_grid_renders_one_column_per_child() {
    let rendered: VirtualNode = grid_with(
        vec![
            EuvGridCol { span: 6, offset: 0 },
            EuvGridCol { span: 6, offset: 0 },
        ],
        false,
        2,
    );
    let mut classes: Vec<String> = Vec::new();
    classes_of(&rendered, &mut classes);
    let columns: usize = classes
        .iter()
        .filter(|class: &&String| *class == "c_euv_grid_col")
        .count();
    assert_eq!(
        columns, 2,
        "one column per child is expected, got {classes:?}"
    );
}

#[test]
fn a_grid_row_without_a_gutter_carries_the_plain_grid_class() {
    let rendered: VirtualNode = grid_with(Vec::new(), false, 1);
    let mut classes: Vec<String> = Vec::new();
    classes_of(&rendered, &mut classes);
    assert!(
        classes.iter().any(|class: &String| class == "c_euv_grid"),
        "a row without a gap must carry the plain grid class, got {classes:?}"
    );
    assert!(
        !classes
            .iter()
            .any(|class: &String| class == "c_euv_grid_gutter"),
        "a row without a gap must not carry the gutter class, got {classes:?}"
    );
}

#[test]
fn a_gutter_grid_row_carries_the_gutter_class() {
    let rendered: VirtualNode = grid_with(Vec::new(), true, 1);
    let mut classes: Vec<String> = Vec::new();
    classes_of(&rendered, &mut classes);
    assert!(
        classes
            .iter()
            .any(|class: &String| class == "c_euv_grid_gutter"),
        "a row with a gap must carry the gutter class, got {classes:?}"
    );
}

#[test]
fn a_column_takes_its_share_of_the_twenty_four_column_row() {
    let rendered: VirtualNode = grid_with(vec![EuvGridCol { span: 6, offset: 0 }], false, 1);
    let mut styles: Vec<String> = Vec::new();
    styles_of(&rendered, &mut styles);
    assert!(
        styles
            .iter()
            .any(|style: &String| style.contains("flex: 0 0 25%;")),
        "a span of six of twenty-four is a quarter of the row, got {styles:?}"
    );
}

#[test]
fn a_zero_span_column_still_takes_a_share_of_the_row() {
    let rendered: VirtualNode = grid_with(vec![EuvGridCol { span: 0, offset: 0 }], false, 1);
    let mut styles: Vec<String> = Vec::new();
    styles_of(&rendered, &mut styles);
    assert!(
        styles
            .iter()
            .any(|style: &String| style.contains("flex: 0 0 4%;")),
        "a zero span is clamped to one column of the row, got {styles:?}"
    );
}

#[test]
fn an_offset_column_is_pushed_to_the_right() {
    let rendered: VirtualNode = grid_with(vec![EuvGridCol { span: 6, offset: 6 }], false, 1);
    let mut classes: Vec<String> = Vec::new();
    classes_of(&rendered, &mut classes);
    assert!(
        classes
            .iter()
            .any(|class: &String| class == "c_euv_grid_col_offset"),
        "an offset column must carry the offset class, got {classes:?}"
    );
    let mut styles: Vec<String> = Vec::new();
    styles_of(&rendered, &mut styles);
    assert!(
        styles
            .iter()
            .any(|style: &String| style.contains("margin-left: 25%;")),
        "an offset of six of twenty-four is a quarter of the row, got {styles:?}"
    );
}

#[test]
fn a_grid_keeps_the_callers_content_in_its_columns() {
    let rendered: VirtualNode = grid_with(
        vec![
            EuvGridCol {
                span: 12,
                offset: 0,
            },
            EuvGridCol {
                span: 12,
                offset: 0,
            },
        ],
        false,
        2,
    );
    let mut classes: Vec<String> = Vec::new();
    classes_of(&rendered, &mut classes);
    let cells: usize = classes
        .iter()
        .filter(|class: &&String| class.starts_with("cell-"))
        .count();
    assert_eq!(
        cells, 2,
        "both cells must survive the grid, got {classes:?}"
    );
}
