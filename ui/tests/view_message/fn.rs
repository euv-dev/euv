use super::*;

fn message_node(items: Vec<EuvMessageItem>) -> VirtualNode<EuvMessageProps> {
    VirtualNode::Element {
        tag: Tag::Element(Cow::Borrowed("div")),
        attributes: Vec::new(),
        children: Vec::new(),
        key: None,
        props: Some(Box::new(EuvMessageProps { items })),
    }
}

fn count_tags(node: &VirtualNode, tag: &str) -> usize {
    let mut count: usize = 0;
    if node.try_get_tag_name().as_deref() == Some(tag) {
        count += 1;
    }
    for child in node.get_children() {
        count += count_tags(child, tag);
    }
    count
}

fn flattened_children<'a>(node: &'a VirtualNode) -> Vec<&'a VirtualNode> {
    let mut out: Vec<&'a VirtualNode> = Vec::new();
    for child in node.get_children() {
        match child {
            VirtualNode::Fragment(inner) => out.extend(inner.iter()),
            VirtualNode::Empty => {}
            _ => out.push(child),
        }
    }
    out
}

fn joined_text(node: &VirtualNode) -> String {
    let mut parts: Vec<String> = Vec::new();
    collect_text(node, &mut parts);
    parts.join(" ")
}

fn collect_text(node: &VirtualNode, parts: &mut Vec<String>) {
    match node {
        VirtualNode::Text(text) => parts.push(format!("{text:?}")),
        _ => {
            for child in node.get_children() {
                collect_text(child, parts);
            }
        }
    }
}

#[test]
fn an_empty_message_stack_still_renders_its_container() {
    let rendered: VirtualNode = euv_message(message_node(Vec::new()));
    assert_eq!(
        rendered.try_get_tag_name(),
        Some("div".to_string()),
        "the stack container stays mounted so a caller can push messages in"
    );
    assert!(
        flattened_children(&rendered).is_empty(),
        "an empty stack holds no messages"
    );
}

#[test]
fn a_message_stack_renders_one_message_per_item() {
    let items: Vec<EuvMessageItem> = vec![
        EuvMessageItem {
            text: "Saved",
            ..Default::default()
        },
        EuvMessageItem {
            text: "Deleted",
            ..Default::default()
        },
    ];
    let rendered: VirtualNode = euv_message(message_node(items));
    assert_eq!(
        flattened_children(&rendered).len(),
        2,
        "one message element is rendered per item"
    );
    let text: String = joined_text(&rendered);
    assert!(text.contains("Saved"), "the first message text is rendered");
    assert!(
        text.contains("Deleted"),
        "the second message text is rendered"
    );
}

#[test]
fn a_message_with_an_action_label_renders_an_action_button() {
    let items: Vec<EuvMessageItem> = vec![EuvMessageItem {
        text: "Saved",
        action_label: "Undo",
        ..Default::default()
    }];
    let rendered: VirtualNode = euv_message(message_node(items));
    assert_eq!(
        count_tags(&rendered, "button"),
        1,
        "the action label adds the only button of the message"
    );
    assert!(
        joined_text(&rendered).contains("Undo"),
        "the action button carries the action label"
    );
}

#[test]
fn a_message_without_an_action_label_renders_no_action_button() {
    let items: Vec<EuvMessageItem> = vec![EuvMessageItem {
        text: "Saved",
        ..Default::default()
    }];
    let rendered: VirtualNode = euv_message(message_node(items));
    assert_eq!(
        count_tags(&rendered, "button"),
        0,
        "an empty action label renders no action button"
    );
}

#[test]
fn a_dismissible_message_renders_a_close_button() {
    let items: Vec<EuvMessageItem> = vec![EuvMessageItem {
        text: "Saved",
        dismissible: true,
        ..Default::default()
    }];
    let rendered: VirtualNode = euv_message(message_node(items));
    assert_eq!(
        count_tags(&rendered, "button"),
        1,
        "a dismissible message renders the close button"
    );
}

#[test]
fn a_close_callback_implies_a_dismissible_message() {
    let items: Vec<EuvMessageItem> = vec![EuvMessageItem {
        text: "Saved",
        on_close: Some(Rc::new(|| {})),
        ..Default::default()
    }];
    let rendered: VirtualNode = euv_message(message_node(items));
    assert_eq!(
        count_tags(&rendered, "button"),
        1,
        "supplying a close callback renders the close button on its own"
    );
}

#[test]
fn the_action_callback_fires_when_the_action_is_triggered() {
    let fired: Rc<Cell<u32>> = Rc::new(Cell::new(0));
    let counter: Rc<Cell<u32>> = fired.clone();
    let callback: Rc<dyn Fn()> = Rc::new(move || {
        counter.set(counter.get() + 1);
    });
    let items: Vec<EuvMessageItem> = vec![EuvMessageItem {
        text: "Saved",
        action_label: "Undo",
        on_action: Some(callback.clone()),
        ..Default::default()
    }];
    let _rendered: VirtualNode = euv_message(message_node(items));
    let handler: Option<Rc<dyn Fn(u32)>> = message_callback_handler::<u32>(Some(callback))
        .expect("a message with an action callback has a handler");
    handler(0);
    assert_eq!(fired.get(), 1, "the action callback fires exactly once");
}

#[test]
fn a_message_without_a_callback_has_no_handler() {
    let handler: Option<Rc<dyn Fn(u32)>> = message_callback_handler::<u32>(None);
    assert!(
        handler.is_none(),
        "there is nothing to fire when the caller supplied no callback"
    );
}
