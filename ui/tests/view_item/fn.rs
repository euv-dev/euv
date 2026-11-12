use super::*;

fn item_node(variant: EuvItemVariant, size: EuvItemSize) -> VirtualNode<EuvItemProps> {
    VirtualNode::Element {
        tag: Tag::Element(Cow::Borrowed("div")),
        attributes: Vec::new(),
        children: Vec::new(),
        key: None,
        props: Some(Box::new(EuvItemProps { variant, size })),
    }
}

fn media_node(variant: EuvItemMediaVariant) -> VirtualNode<EuvItemMediaProps> {
    VirtualNode::Element {
        tag: Tag::Element(Cow::Borrowed("div")),
        attributes: Vec::new(),
        children: Vec::new(),
        key: None,
        props: Some(Box::new(EuvItemMediaProps { variant })),
    }
}

fn title_node(text: &'static str) -> VirtualNode<EuvItemTitleProps> {
    VirtualNode::Element {
        tag: Tag::Element(Cow::Borrowed("div")),
        attributes: Vec::new(),
        children: Vec::new(),
        key: None,
        props: Some(Box::new(EuvItemTitleProps { text })),
    }
}

fn description_node(text: &'static str) -> VirtualNode<EuvItemDescriptionProps> {
    VirtualNode::Element {
        tag: Tag::Element(Cow::Borrowed("div")),
        attributes: Vec::new(),
        children: Vec::new(),
        key: None,
        props: Some(Box::new(EuvItemDescriptionProps { text })),
    }
}

fn walk(node: &VirtualNode, out: &mut Vec<VirtualNode>) {
    match node {
        VirtualNode::Element { children, .. } => {
            out.push(node.clone());
            for child in children.iter() {
                walk(child, out);
            }
        }
        VirtualNode::Fragment(children) => {
            for child in children.iter() {
                walk(child, out);
            }
        }
        _ => {}
    }
}

fn elements(node: &VirtualNode) -> Vec<VirtualNode> {
    let mut out: Vec<VirtualNode> = Vec::new();
    walk(node, &mut out);
    out
}

fn attribute_names(node: &VirtualNode) -> Vec<String> {
    match node {
        VirtualNode::Element { attributes, .. } => attributes
            .iter()
            .map(|entry: &AttributeEntry| entry.get_name().to_string())
            .collect(),
        _ => Vec::new(),
    }
}

fn attribute_value(node: &VirtualNode, name: &str) -> Option<String> {
    match node {
        VirtualNode::Element { attributes, .. } => attributes
            .iter()
            .find(|entry: &&AttributeEntry| entry.get_name() == name)
            .and_then(AttributeEntry::text_value),
        _ => None,
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

#[test]
fn an_item_group_is_a_list() {
    let rendered: VirtualNode =
        euv_item_group(item_node(EuvItemVariant::Default, EuvItemSize::Default));
    assert!(
        attribute_names(&rendered)
            .iter()
            .any(|name: &String| name == "role"),
        "the group announces itself as a list"
    );
    assert!(
        has_class(&rendered, "c_euv_item_group"),
        "the group carries the item-group class"
    );
    if cfg!(feature = "theme-shadcn") {
        assert!(has_class(&rendered, "c_euv_item_group_shadcn"));
    }
}

#[test]
fn every_item_variant_reaches_the_class_list() {
    let default: VirtualNode = euv_item(item_node(EuvItemVariant::Default, EuvItemSize::Default));
    let outline: VirtualNode = euv_item(item_node(EuvItemVariant::Outline, EuvItemSize::Default));
    let muted: VirtualNode = euv_item(item_node(EuvItemVariant::Muted, EuvItemSize::Default));
    assert!(
        has_class(&default, "c_euv_item_default"),
        "the transparent row has its own class"
    );
    assert!(
        has_class(&outline, "c_euv_item_outline"),
        "the bordered row has its own class"
    );
    assert!(
        has_class(&muted, "c_euv_item_muted"),
        "the muted row has its own class"
    );
}

#[test]
fn every_item_size_reaches_the_class_list() {
    let default: VirtualNode = euv_item(item_node(EuvItemVariant::Default, EuvItemSize::Default));
    let small: VirtualNode = euv_item(item_node(EuvItemVariant::Default, EuvItemSize::Sm));
    let extra_small: VirtualNode = euv_item(item_node(EuvItemVariant::Default, EuvItemSize::Xs));
    for (node, class_name, data_size) in [
        (&default, "c_euv_item_size_default", ITEM_SIZE_DEFAULT),
        (&small, "c_euv_item_size_sm", ITEM_SIZE_SM),
        (&extra_small, "c_euv_item_size_xs", ITEM_SIZE_XS),
    ] {
        assert!(
            has_class(node, class_name),
            "the {data_size} row has its own class"
        );
        assert_eq!(
            attribute_value(node, "data-size").as_deref(),
            Some(data_size)
        );
    }
    if cfg!(feature = "theme-shadcn") {
        for (node, class_name) in [
            (&default, "c_euv_item_size_default_shadcn"),
            (&small, "c_euv_item_size_sm_shadcn"),
            (&extra_small, "c_euv_item_size_xs_shadcn"),
        ] {
            assert!(has_class(node, class_name));
        }
    }
}

#[test]
fn an_item_names_its_slot_and_its_state() {
    let rendered: VirtualNode = euv_item(item_node(EuvItemVariant::Outline, EuvItemSize::Sm));
    let names: Vec<String> = attribute_names(&rendered);
    assert!(
        names.iter().any(|name: &String| name == "data-slot"),
        "the row names its slot"
    );
    assert!(
        names.iter().any(|name: &String| name == "data-variant"),
        "the row names its variant"
    );
    assert!(
        names.iter().any(|name: &String| name == "data-size"),
        "the row names its density"
    );
}

#[test]
fn every_media_variant_reaches_the_class_list() {
    let default: VirtualNode = euv_item_media(media_node(EuvItemMediaVariant::Default));
    let icon: VirtualNode = euv_item_media(media_node(EuvItemMediaVariant::Icon));
    let image: VirtualNode = euv_item_media(media_node(EuvItemMediaVariant::Image));
    assert!(
        has_class(&default, "c_euv_item_media_default"),
        "the bare media has its own class"
    );
    assert!(
        has_class(&icon, "c_euv_item_media_icon"),
        "the icon tile has its own class"
    );
    assert!(
        has_class(&image, "c_euv_item_media_image"),
        "the picture frame has its own class"
    );
}

#[test]
fn the_title_and_the_description_keep_their_own_classes() {
    let title: VirtualNode = euv_item_title(title_node("Release notes"));
    let description: VirtualNode = euv_item_description(description_node("What changed"));
    assert!(
        has_class(&title, "c_euv_item_title"),
        "the title carries its own class"
    );
    assert!(
        has_class(&description, "c_euv_item_description"),
        "the description carries its own class"
    );
}

#[test]
fn the_content_and_the_actions_keep_their_own_classes() {
    let content: VirtualNode =
        euv_item_content(item_node(EuvItemVariant::Default, EuvItemSize::Default));
    let actions: VirtualNode =
        euv_item_actions(item_node(EuvItemVariant::Default, EuvItemSize::Default));
    assert!(
        has_class(&content, "c_euv_item_content"),
        "the text column carries its own class"
    );
    assert!(
        has_class(&actions, "c_euv_item_actions"),
        "the actions carry their own class"
    );
}
