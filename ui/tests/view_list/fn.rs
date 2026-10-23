use super::*;

fn item(title: &'static str, desc: &'static str, meta: &'static str) -> EuvListItem {
    EuvListItem { title, desc, meta }
}

fn list_node(items: Vec<EuvListItem>, empty_text: &'static str) -> VirtualNode<EuvListProps> {
    VirtualNode::Element {
        tag: Tag::Element(Cow::Borrowed("div")),
        attributes: Vec::new(),
        children: Vec::new(),
        key: None,
        props: Some(Box::new(EuvListProps { items, empty_text })),
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

fn total(node: &VirtualNode) -> usize {
    match node {
        VirtualNode::Element { children, .. } => 1 + children.iter().map(total).sum::<usize>(),
        VirtualNode::Fragment(children) => children.iter().map(total).sum(),
        _ => 0,
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

fn carries_text(node: &VirtualNode, needle: &str) -> bool {
    format!("{node:?}").contains(needle)
}

#[test]
fn a_list_renders_one_row_per_item() {
    let node: VirtualNode<EuvListProps> = list_node(
        vec![
            item("First", "", ""),
            item("Second", "", ""),
            item("Third", "", ""),
        ],
        "",
    );
    let rendered: VirtualNode = euv_list(node);
    assert_eq!(
        child_count(&rendered),
        3,
        "one row is rendered per item, got {rendered:?}"
    );
}

#[test]
fn every_row_carries_the_item_body_and_title_classes() {
    let node: VirtualNode<EuvListProps> = list_node(vec![item("Heading", "", "")], "");
    let rendered: VirtualNode = euv_list(node);
    let mut lists: Vec<Vec<String>> = Vec::new();
    class_lists(&rendered, &mut lists);
    assert_eq!(
        lists,
        vec![
            vec![String::from("c_euv_list")],
            vec![String::from("c_euv_list_item")],
            vec![String::from("c_euv_list_item_body")],
            vec![String::from("c_euv_list_item_title")],
        ],
        "the list, the row, its body column and its title are all classed, got {lists:?}"
    );
}

#[test]
fn a_row_with_a_description_renders_the_description_line() {
    let node: VirtualNode<EuvListProps> = list_node(vec![item("Heading", "Some detail", "")], "");
    let rendered: VirtualNode = euv_list(node);
    let mut lists: Vec<Vec<String>> = Vec::new();
    class_lists(&rendered, &mut lists);
    assert!(
        lists
            .iter()
            .any(|list: &Vec<String>| list.contains(&String::from("c_euv_list_item_desc"))),
        "a non-empty description must render the description line, got {lists:?}"
    );
    assert!(
        carries_text(&rendered, "Some detail"),
        "the description text must reach the tree, got {rendered:?}"
    );
}

#[test]
fn a_row_without_a_description_omits_the_description_line() {
    let terse: VirtualNode = euv_list(list_node(vec![item("Heading", "", "")], ""));
    let described: VirtualNode = euv_list(list_node(vec![item("Heading", "Some detail", "")], ""));
    let mut terse_lists: Vec<Vec<String>> = Vec::new();
    class_lists(&terse, &mut terse_lists);
    assert!(
        !terse_lists
            .iter()
            .any(|list: &Vec<String>| list.contains(&String::from("c_euv_list_item_desc"))),
        "an empty description must leave no empty line behind, got {terse_lists:?}"
    );
    assert_eq!(
        total(&terse) + 1,
        total(&described),
        "and a described row is exactly one element taller than a terse one, got {} then {}",
        total(&terse),
        total(&described)
    );
}

#[test]
fn a_row_with_meta_text_renders_the_trailing_meta() {
    let node: VirtualNode<EuvListProps> = list_node(vec![item("Heading", "", "2h ago")], "");
    let rendered: VirtualNode = euv_list(node);
    let mut lists: Vec<Vec<String>> = Vec::new();
    class_lists(&rendered, &mut lists);
    assert!(
        lists
            .iter()
            .any(|list: &Vec<String>| list.contains(&String::from("c_euv_list_item_meta"))),
        "a non-empty meta must render the trailing meta block, got {lists:?}"
    );
    assert!(
        carries_text(&rendered, "2h ago"),
        "the meta text must reach the tree, got {rendered:?}"
    );
}

#[test]
fn a_row_without_meta_text_omits_the_meta_block() {
    let terse: VirtualNode = euv_list(list_node(vec![item("Heading", "", "")], ""));
    let with_meta: VirtualNode = euv_list(list_node(vec![item("Heading", "", "2h ago")], ""));
    let mut terse_lists: Vec<Vec<String>> = Vec::new();
    class_lists(&terse, &mut terse_lists);
    assert!(
        !terse_lists
            .iter()
            .any(|list: &Vec<String>| list.contains(&String::from("c_euv_list_item_meta"))),
        "an empty meta must leave no empty block at the trailing edge, got {terse_lists:?}"
    );
    assert_eq!(
        total(&terse) + 1,
        total(&with_meta),
        "and a row with a meta is exactly one element taller, got {} then {}",
        total(&terse),
        total(&with_meta)
    );
}

#[test]
fn an_empty_list_with_empty_text_renders_the_empty_state() {
    let node: VirtualNode<EuvListProps> = list_node(Vec::new(), "Nothing here yet");
    let rendered: VirtualNode = euv_list(node);
    assert_eq!(
        child_count(&rendered),
        1,
        "the empty state is the only row of an empty list, got {rendered:?}"
    );
    let mut lists: Vec<Vec<String>> = Vec::new();
    class_lists(&rendered, &mut lists);
    assert!(
        lists
            .iter()
            .any(|list: &Vec<String>| list.contains(&String::from("c_euv_list_empty"))),
        "the empty state must carry its class, got {lists:?}"
    );
    assert!(
        carries_text(&rendered, "Nothing here yet"),
        "the empty state must carry its text, got {rendered:?}"
    );
}

#[test]
fn an_empty_list_without_empty_text_renders_no_empty_state() {
    let node: VirtualNode<EuvListProps> = list_node(Vec::new(), "");
    let rendered: VirtualNode = euv_list(node);
    assert_eq!(
        child_count(&rendered),
        0,
        "an empty empty-text leaves the list blank, got {rendered:?}"
    );
    let mut lists: Vec<Vec<String>> = Vec::new();
    class_lists(&rendered, &mut lists);
    assert!(
        !lists
            .iter()
            .any(|list: &Vec<String>| list.contains(&String::from("c_euv_list_empty"))),
        "and no empty-state row is rendered, got {lists:?}"
    );
}

#[test]
fn a_list_without_props_falls_back_to_defaults() {
    let bare: VirtualNode<EuvListProps> = VirtualNode::Element {
        tag: Tag::Element(Cow::Borrowed("div")),
        attributes: Vec::new(),
        children: Vec::new(),
        key: None,
        props: None,
    };
    let rendered: VirtualNode = euv_list(bare);
    assert_eq!(
        child_count(&rendered),
        0,
        "the default props carry no items and no empty text, got {rendered:?}"
    );
}
