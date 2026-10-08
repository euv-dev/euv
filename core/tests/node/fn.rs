use super::*;
#[test]
fn tag_element_constructs() {
    let tag: Tag = Tag::Element(Cow::Borrowed("div"));
    match &tag {
        Tag::Element(name) => assert_eq!(name, "div"),
        Tag::Component(_) => panic!("expected Element"),
        Tag::Portal(_) => panic!("expected Element"),
    }
}

#[test]
fn tag_component_constructs() {
    let tag: Tag = Tag::Component(Cow::Borrowed("MyButton"));
    match &tag {
        Tag::Component(name) => assert_eq!(name, "MyButton"),
        Tag::Element(_) => panic!("expected Component"),
        Tag::Portal(_) => panic!("expected Component"),
    }
}

#[test]
fn tag_clone_preserves_variant() {
    let element: Tag = Tag::Element(Cow::Borrowed("span"));
    let cloned_element: Tag = element.clone();
    match &cloned_element {
        Tag::Element(name) => assert_eq!(name, "span"),
        _ => panic!("expected Element"),
    }
    let component: Tag = Tag::Component(Cow::Borrowed("Header"));
    let cloned_component: Tag = component.clone();
    match &cloned_component {
        Tag::Component(name) => assert_eq!(name, "Header"),
        _ => panic!("expected Component"),
    }
}

#[test]
fn tag_equality_same_variant() {
    let a: Tag = Tag::Element(Cow::Borrowed("div"));
    let b: Tag = Tag::Element(Cow::Borrowed("div"));
    assert_eq!(a, b);
}

#[test]
fn tag_equality_different_variants() {
    let element: Tag = Tag::Element(Cow::Borrowed("div"));
    let component: Tag = Tag::Component(Cow::Borrowed("div"));
    assert_ne!(element, component);
}

#[test]
fn tag_equality_different_inner_values() {
    let a: Tag = Tag::Element(Cow::Borrowed("div"));
    let b: Tag = Tag::Element(Cow::Borrowed("span"));
    assert_ne!(a, b);
}

#[test]
fn tag_hash_same_for_equal_tags() {
    let a: Tag = Tag::Element(Cow::Borrowed("div"));
    let b: Tag = Tag::Element(Cow::Borrowed("div"));
    let mut h1: DefaultHasher = DefaultHasher::new();
    let mut h2: DefaultHasher = DefaultHasher::new();
    a.hash(&mut h1);
    b.hash(&mut h2);
    assert_eq!(h1.finish(), h2.finish());
}

#[test]
fn tag_debug_format_works() {
    let tag: Tag = Tag::Element(Cow::Borrowed("div"));
    let formatted: String = format!("{:?}", tag);
    assert!(formatted.contains("Element"));
    assert!(formatted.contains("div"));
}

#[test]
fn virtual_node_empty_constructs() {
    let node: VirtualNode = VirtualNode::Empty;
    assert!(matches!(node, VirtualNode::Empty));
}

#[test]
fn virtual_node_empty_clone_is_identical() {
    let node: VirtualNode = VirtualNode::Empty;
    let cloned: VirtualNode = node.clone();
    assert!(matches!(cloned, VirtualNode::Empty));
}

#[test]
fn virtual_node_empty_debug_format() {
    let node: VirtualNode = VirtualNode::Empty;
    let formatted: String = format!("{:?}", node);
    assert!(formatted.contains("Empty"));
}

#[test]
fn virtual_node_element_with_no_attributes_no_children() {
    let node: VirtualNode = VirtualNode::Element {
        tag: Tag::Element(Cow::Borrowed("div")),
        attributes: Vec::new(),
        children: Vec::new(),
        key: None,
        props: None,
    };
    match node {
        VirtualNode::Element { tag, .. } => match tag {
            Tag::Element(name) => assert_eq!(name, "div"),
            _ => panic!("expected Element tag"),
        },
        _ => panic!("expected Element variant"),
    }
}

#[test]
fn virtual_node_element_with_key() {
    let node: VirtualNode = VirtualNode::Element {
        tag: Tag::Element(Cow::Borrowed("li")),
        attributes: Vec::new(),
        children: Vec::new(),
        key: Some(String::from("item-1")),
        props: None,
    };
    if let VirtualNode::Element { key, .. } = &node {
        assert_eq!(key.as_ref().unwrap(), "item-1");
    } else {
        panic!("expected Element variant");
    }
}

#[test]
fn virtual_node_element_with_nested_children() {
    let inner: VirtualNode = VirtualNode::Element {
        tag: Tag::Element(Cow::Borrowed("span")),
        attributes: Vec::new(),
        children: Vec::new(),
        key: None,
        props: None,
    };
    let outer: VirtualNode = VirtualNode::Element {
        tag: Tag::Element(Cow::Borrowed("div")),
        attributes: Vec::new(),
        children: vec![inner],
        key: None,
        props: None,
    };
    if let VirtualNode::Element { children, .. } = &outer {
        assert_eq!(children.len(), 1);
        matches!(children[0], VirtualNode::Element { .. });
    } else {
        panic!("expected Element variant");
    }
}

#[test]
fn virtual_node_element_clone_deep_copies() {
    let original: VirtualNode = VirtualNode::Element {
        tag: Tag::Element(Cow::Borrowed("div")),
        attributes: vec![AttributeEntry::new(
            Cow::Borrowed("id"),
            AttributeValue::Text(String::from("root")),
        )],
        children: Vec::new(),
        key: None,
        props: None,
    };
    let cloned: VirtualNode = original.clone();
    if let VirtualNode::Element {
        tag, attributes, ..
    } = &cloned
    {
        match tag {
            Tag::Element(name) => assert_eq!(name, "div"),
            _ => panic!("expected Element tag"),
        }
        assert_eq!(attributes.len(), 1);
    } else {
        panic!("expected Element after clone");
    }
}

#[test]
fn virtual_node_fragment_with_empty_children() {
    let node: VirtualNode = VirtualNode::Fragment(Vec::new());
    if let VirtualNode::Fragment(children) = &node {
        assert!(children.is_empty());
    } else {
        panic!("expected Fragment variant");
    }
}

#[test]
fn virtual_node_fragment_with_pure_rust_children() {
    let children: Vec<VirtualNode> = vec![
        VirtualNode::Empty,
        VirtualNode::Element {
            tag: Tag::Element(Cow::Borrowed("br")),
            attributes: Vec::new(),
            children: Vec::new(),
            key: None,
            props: None,
        },
        VirtualNode::Text(TextNode::new(Cow::Owned(String::from("hello")), None)),
    ];
    let fragment: VirtualNode = VirtualNode::Fragment(children);
    if let VirtualNode::Fragment(children) = &fragment {
        assert_eq!(children.len(), 3);
    } else {
        panic!("expected Fragment variant");
    }
}

#[test]
fn virtual_node_fragment_clone_preserves_children() {
    let fragment: VirtualNode = VirtualNode::Fragment(vec![VirtualNode::Empty, VirtualNode::Empty]);
    let cloned: VirtualNode = fragment.clone();
    if let VirtualNode::Fragment(children) = &cloned {
        assert_eq!(children.len(), 2);
    } else {
        panic!("expected Fragment after clone");
    }
}

#[test]
fn text_node_partial_eq_visual_equality() {
    let a: TextNode = TextNode::new(Cow::Owned(String::from("hello")), None);
    let b: TextNode = TextNode::new(Cow::Owned(String::from("hello")), None);
    assert_eq!(a, b);
}

#[test]
fn text_node_partial_eq_different_content() {
    let a: TextNode = TextNode::new(Cow::Owned(String::from("hello")), None);
    let b: TextNode = TextNode::new(Cow::Owned(String::from("world")), None);
    assert_ne!(a, b);
}

#[test]
fn text_node_debug_format_skips_signal_field() {
    let node: TextNode = TextNode::new(Cow::Owned(String::from("hello")), None);
    let formatted: String = format!("{:?}", node);
    assert!(formatted.contains("TextNode"));
}

#[test]
fn virtual_node_empty_debug_format_includes_variant() {
    let node: VirtualNode = VirtualNode::Empty;
    let formatted: String = format!("{:?}", node);
    assert!(formatted.contains("Empty"));
}

#[test]
fn virtual_node_element_debug_format_includes_tag() {
    let node: VirtualNode = VirtualNode::Element {
        tag: Tag::Element(Cow::Borrowed("div")),
        attributes: Vec::new(),
        children: Vec::new(),
        key: None,
        props: None,
    };
    let formatted: String = format!("{:?}", node);
    assert!(formatted.contains("Element"));
    assert!(formatted.contains("div"));
}

#[test]
fn native_virtual_node_empty_construction_does_not_panic() {
    let result: Result<(), String> = catch_unwind(AssertUnwindSafe(|| {
        let _: VirtualNode = VirtualNode::Empty;
    }))
    .map_err(|_| "panic".to_string());
    assert!(result.is_ok());
}

#[test]
fn native_virtual_node_element_construction_does_not_panic() {
    let result: Result<(), String> = catch_unwind(AssertUnwindSafe(|| {
        let _: VirtualNode = VirtualNode::Element {
            tag: Tag::Element(Cow::Borrowed("div")),
            attributes: Vec::new(),
            children: Vec::new(),
            key: None,
            props: None,
        };
    }))
    .map_err(|_| "panic".to_string());
    assert!(result.is_ok());
}

#[test]
fn native_text_node_construction_does_not_panic() {
    let result: Result<(), String> = catch_unwind(AssertUnwindSafe(|| {
        let _: TextNode = TextNode::new(Cow::Owned(String::from("x")), None);
    }))
    .map_err(|_| "panic".to_string());
    assert!(result.is_ok());
}

#[test]
fn native_tag_construction_does_not_panic() {
    let result: Result<(), String> = catch_unwind(AssertUnwindSafe(|| {
        let _: Tag = Tag::Element(Cow::Borrowed("div"));
        let _: Tag = Tag::Component(Cow::Borrowed("Foo"));
    }))
    .map_err(|_| "panic".to_string());
    assert!(result.is_ok());
}

fn as_text_through_the_trait<T: AsReactiveText>(value: &T) -> VirtualNode {
    value.as_reactive_text()
}

#[test]
fn a_type_reachable_through_the_trait_produces_its_reactive_text_node() {
    let signal: Signal<String> = Signal::create(String::from("hello"));
    let built: VirtualNode = as_text_through_the_trait(&signal);

    assert!(
        matches!(built, VirtualNode::Text(_)),
        "the trait is what a generic renderer calls to turn any value into a node, so a \
         caller that only holds a bound has to get the same result as a direct call"
    );
}
