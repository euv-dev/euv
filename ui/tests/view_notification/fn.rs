use super::*;

fn notification_node(items: Vec<EuvNotificationItem>) -> VirtualNode<EuvNotificationProps> {
    VirtualNode::Element {
        tag: Tag::Element(Cow::Borrowed("div")),
        attributes: Vec::new(),
        children: Vec::new(),
        key: None,
        props: Some(Box::new(EuvNotificationProps { items })),
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
fn an_empty_notification_stack_still_renders_its_container() {
    let rendered: VirtualNode = euv_notification(notification_node(Vec::new()));
    assert_eq!(
        rendered.try_get_tag_name(),
        Some("div".to_string()),
        "the stack container stays mounted so a caller can push notifications in"
    );
    assert!(
        flattened_children(&rendered).is_empty(),
        "an empty stack holds no notifications"
    );
}

#[test]
fn a_notification_stack_renders_one_notification_per_item() {
    let items: Vec<EuvNotificationItem> = vec![
        EuvNotificationItem {
            title: "Build finished",
            text: "The bundle is ready.",
            ..Default::default()
        },
        EuvNotificationItem {
            title: "Upload finished",
            text: "The file is stored.",
            ..Default::default()
        },
    ];
    let rendered: VirtualNode = euv_notification(notification_node(items));
    assert_eq!(
        flattened_children(&rendered).len(),
        2,
        "one notification element is rendered per item"
    );
    let text: String = joined_text(&rendered);
    assert!(
        text.contains("Build finished"),
        "the first title is rendered"
    );
    assert!(
        text.contains("The bundle is ready."),
        "the first body line is rendered"
    );
    assert!(
        text.contains("Upload finished"),
        "the second title is rendered"
    );
}

#[test]
fn a_notification_renders_its_title_and_its_body() {
    let items: Vec<EuvNotificationItem> = vec![EuvNotificationItem {
        title: "Build finished",
        text: "The bundle is ready.",
        ..Default::default()
    }];
    let rendered: VirtualNode = euv_notification(notification_node(items));
    let text: String = joined_text(&rendered);
    assert!(
        text.contains("Build finished") && text.contains("The bundle is ready."),
        "the title and the body line are both rendered, got {text}"
    );
}

#[test]
fn a_plain_notification_renders_no_close_button() {
    let items: Vec<EuvNotificationItem> = vec![EuvNotificationItem {
        title: "Build finished",
        ..Default::default()
    }];
    let rendered: VirtualNode = euv_notification(notification_node(items));
    assert_eq!(
        count_tags(&rendered, "button"),
        0,
        "a notification that is not dismissible renders no close button"
    );
}

#[test]
fn a_dismissible_notification_renders_a_close_button() {
    let items: Vec<EuvNotificationItem> = vec![EuvNotificationItem {
        title: "Build finished",
        dismissible: true,
        ..Default::default()
    }];
    let rendered: VirtualNode = euv_notification(notification_node(items));
    assert_eq!(
        count_tags(&rendered, "button"),
        1,
        "a dismissible notification renders the close button"
    );
}

#[test]
fn a_close_callback_implies_a_dismissible_notification() {
    let items: Vec<EuvNotificationItem> = vec![EuvNotificationItem {
        title: "Build finished",
        on_close: Some(Rc::new(|| {})),
        ..Default::default()
    }];
    let rendered: VirtualNode = euv_notification(notification_node(items));
    assert_eq!(
        count_tags(&rendered, "button"),
        1,
        "supplying a close callback renders the close button on its own"
    );
}

#[test]
fn the_close_callback_fires_when_the_close_is_triggered() {
    let fired: Rc<Cell<u32>> = Rc::new(Cell::new(0));
    let counter: Rc<Cell<u32>> = fired.clone();
    let callback: Rc<dyn Fn()> = Rc::new(move || {
        counter.set(counter.get() + 1);
    });
    let items: Vec<EuvNotificationItem> = vec![EuvNotificationItem {
        title: "Build finished",
        on_close: Some(callback.clone()),
        ..Default::default()
    }];
    let _rendered: VirtualNode = euv_notification(notification_node(items));
    let handler: Option<Rc<dyn Fn(u32)>> =
        notification_callback_handler::<u32>(Some(callback))
        .expect("a notification with a close callback has a handler");
    handler(0);
    assert_eq!(fired.get(), 1, "the close callback fires exactly once");
}

#[test]
fn a_notification_without_a_callback_has_no_handler() {
    let handler: Option<Rc<dyn Fn(u32)>> = notification_callback_handler::<u32>(None);
    assert!(
        handler.is_none(),
        "there is nothing to fire when the caller supplied no callback"
    );
}
