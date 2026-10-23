use super::*;

fn props<T>(value: T) -> VirtualNode<T> {
    VirtualNode::Element {
        tag: Tag::Element(Cow::Borrowed("div")),
        attributes: Vec::new(),
        children: Vec::new(),
        key: None,
        props: Some(Box::new(value)),
    }
}

fn item(value: &'static str, label: &'static str) -> EuvMentionItem {
    EuvMentionItem { value, label }
}

fn people() -> Vec<EuvMentionItem> {
    vec![
        item("alice", "Alice"),
        item("albert", "Albert"),
        item("bob", "Bob"),
    ]
}

fn mentions(items: Vec<EuvMentionItem>, text: &str) -> VirtualNode {
    HookContext::with(HookContext::default(), || {
        euv_mentions(props(EuvMentionsProps {
            items,
            value: Signal::create(String::from(text)),
            placeholder: "Write a message",
            label: "Message",
            on_select: None,
        }))
    })
}

fn class_names(node: &VirtualNode, into: &mut Vec<String>) {
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
                class_names(child, into);
            }
        }
        VirtualNode::Fragment(children) => {
            for child in children {
                class_names(child, into);
            }
        }
        _ => {}
    }
}

fn texts_of(node: &VirtualNode, into: &mut Vec<String>) {
    match node {
        VirtualNode::Text(text_node) => {
            into.push(format!("{text_node:?}"));
        }
        VirtualNode::Element { children, .. } => {
            for child in children {
                texts_of(child, into);
            }
        }
        VirtualNode::Fragment(children) => {
            for child in children {
                texts_of(child, into);
            }
        }
        _ => {}
    }
}

fn texts(node: &VirtualNode) -> Vec<String> {
    let mut into: Vec<String> = Vec::new();
    texts_of(node, &mut into);
    into
}

fn carries_classes(entry: &AttributeEntry, names: &[&str]) -> bool {
    if names.len() == 1 {
        let single: AttributeEntry = AttributeEntry::new(
            Cow::Borrowed("class"),
            AttributeValue::Css(Css::new(
                String::from(names[0]),
                String::new(),
                Vec::new(),
                Vec::new(),
            )),
        );
        if entry == &single {
            return true;
        }
    }
    let merged: AttributeEntry = AttributeEntry::new(
        Cow::Borrowed("class"),
        AttributeValue::Text(names.join(" ")),
    );
    entry == &merged
}

fn has_classes(node: &VirtualNode, names: &[&str]) -> bool {
    match node {
        VirtualNode::Element {
            attributes,
            children,
            ..
        } => {
            let own: bool = attributes
                .iter()
                .filter(|entry: &&AttributeEntry| entry.get_name() == "class")
                .any(|entry: &AttributeEntry| carries_classes(entry, names));
            own || children.iter().any(|child: &VirtualNode| has_classes(child, names))
        }
        VirtualNode::Fragment(children) => children.iter().any(|child: &VirtualNode| has_classes(child, names)),
        _ => false,
    }
}

#[test]
fn the_mention_trigger_is_the_fragment_behind_the_last_at() {
    assert_eq!(
        mentions_trigger("hello @ali"),
        Some("ali"),
        "the fragment behind the last at sign is the trigger"
    );
    assert_eq!(
        mentions_trigger("mail me at bob@example.com"),
        Some("example.com"),
        "the last at sign wins, got {:?}",
        mentions_trigger("mail me at bob@example.com")
    );
    assert_eq!(
        mentions_trigger("hello @"),
        None,
        "an empty fragment is not a trigger"
    );
    assert_eq!(
        mentions_trigger("hello @ali there"),
        None,
        "a fragment with a space is not a trigger"
    );
    assert_eq!(
        mentions_trigger("hello"),
        None,
        "text without an at sign holds no trigger"
    );
}

#[test]
fn a_mention_matches_when_its_label_or_value_holds_the_fragment() {
    assert!(
        mentions_matches(&item("alice", "Alice"), "ali"),
        "the label holds the fragment"
    );
    assert!(
        mentions_matches(&item("alice", "Ally"), "ali"),
        "the value holds the fragment even when the label does not"
    );
    assert!(
        mentions_matches(&item("alice", "Alice"), "ALI"),
        "the fragment is lower-cased before it is compared"
    );
    assert!(
        !mentions_matches(&item("bob", "Bob"), "ali"),
        "a mention without the fragment is filtered out"
    );
}

#[test]
fn the_panel_opens_while_the_text_ends_in_a_mention_trigger() {
    let opened: VirtualNode = mentions(people(), "hello @al");
    assert!(
        has_classes(
            &opened,
            &["c_euv_mentions_panel", "c_euv_mentions_panel_open"]
        ),
        "a mention trigger opens the panel, got {opened:?}"
    );
    let closed: VirtualNode = mentions(people(), "hello there");
    assert!(
        has_classes(
            &closed,
            &["c_euv_mentions_panel", "c_euv_mentions_panel_closed"]
        ),
        "text without a trigger closes the panel, got {closed:?}"
    );
}

#[test]
fn the_panel_stays_mounted_while_the_field_is_closed() {
    let rendered: VirtualNode = mentions(people(), "hello there");
    let mut classes: Vec<String> = Vec::new();
    class_names(&rendered, &mut classes);
    assert!(
        classes
            .iter()
            .any(|name: &String| name == "c_euv_mentions_input"),
        "the field is a real text input, got {classes:?}"
    );
    assert!(
        !classes
            .iter()
            .any(|name: &String| name == "c_euv_mentions_option"),
        "a closed panel suggests nothing, got {classes:?}"
    );
}

#[test]
fn the_typed_fragment_filters_the_suggestions() {
    let rendered: VirtualNode = mentions(people(), "hello @al");
    let mut classes: Vec<String> = Vec::new();
    class_names(&rendered, &mut classes);
    assert_eq!(
        classes
            .iter()
            .filter(|name: &&String| name.as_str() == "c_euv_mentions_option_active")
            .count(),
        1,
        "the first match carries the active class, got {classes:?}"
    );
    assert_eq!(
        classes
            .iter()
            .filter(|name: &&String| name.as_str() == "c_euv_mentions_option")
            .count(),
        1,
        "the other match keeps the plain class, got {classes:?}"
    );
    let rendered_texts: Vec<String> = texts(&rendered);
    assert!(
        rendered_texts
            .iter()
            .any(|text: &String| text.contains("Alice"))
            && rendered_texts
                .iter()
                .any(|text: &String| text.contains("Albert")),
        "both matching mentions reach the panel, got {rendered:?}"
    );
    assert!(
        !rendered_texts
            .iter()
            .any(|text: &String| text.contains("Bob")),
        "a mention without the fragment is not suggested, got {rendered:?}"
    );
}

#[test]
fn picking_a_mention_writes_its_label_over_the_trigger() {
    let value: Signal<String> = Signal::create(String::from("hello @ali"));
    let open: Signal<bool> = Signal::create(true);
    let picked: Rc<RefCell<Vec<String>>> = Rc::new(RefCell::new(Vec::new()));
    let recorder: Rc<RefCell<Vec<String>>> = picked.clone();
    let on_select: Option<Rc<dyn Fn(&'static str)>> =
        Some(Rc::new(move |reported: &'static str| {
            recorder.borrow_mut().push(String::from(reported));
        }));
    let handler = on_mentions_select(value, open, "alice", "Alice", on_select).unwrap();
    handler(());
    assert_eq!(
        value.get(),
        String::from("hello @Alice"),
        "the mention replaces the trigger fragment, got {:?}",
        value.get()
    );
    assert_eq!(
        picked.borrow().as_slice(),
        &[String::from("alice")],
        "the callback receives the value of the picked mention"
    );
    assert!(!open.get(), "the panel closes on the picked mention");
}

#[test]
fn a_field_without_a_trigger_suggests_nothing() {
    let rendered: VirtualNode = mentions(people(), "hello there");
    let mut classes: Vec<String> = Vec::new();
    class_names(&rendered, &mut classes);
    assert!(
        !classes
            .iter()
            .any(|name: &String| name == "c_euv_mentions_option_active"),
        "there is no query, so there is no suggestion to mark, got {classes:?}"
    );
    assert!(
        !texts(&rendered)
            .iter()
            .any(|text: &String| text.contains("Alice")),
        "no mention reaches a field without a trigger, got {rendered:?}"
    );
}
