use super::*;

fn popconfirm_node(props: EuvPopconfirmProps) -> VirtualNode<EuvPopconfirmProps> {
    VirtualNode::Element {
        tag: Tag::Element(Cow::Borrowed("div")),
        attributes: Vec::new(),
        children: vec![VirtualNode::Element {
            tag: Tag::Element(Cow::Borrowed("button")),
            attributes: Vec::new(),
            children: Vec::new(),
            key: None,
            props: None,
        }],
        key: None,
        props: Some(Box::new(props)),
    }
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

fn bubble_of(rendered: &VirtualNode) -> &VirtualNode {
    flattened_children(rendered)
        .get(1)
        .expect("the bubble follows the trigger inside the wrapper")
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
fn a_popconfirm_keeps_its_bubble_mounted_while_it_is_closed() {
    let props: EuvPopconfirmProps = EuvPopconfirmProps {
        title: "Delete this file?",
        ok_label: "Delete",
        cancel_label: "Cancel",
        ..Default::default()
    };
    let rendered: VirtualNode = euv_popconfirm(popconfirm_node(props));
    assert_eq!(
        flattened_children(&rendered).len(),
        2,
        "the wrapper holds the trigger and the always-mounted bubble"
    );
    let bubble: &VirtualNode = bubble_of(&rendered);
    assert_eq!(
        bubble.try_get_tag_name(),
        Some("div".to_string()),
        "the bubble is still in the tree while it is closed"
    );
    assert!(
        joined_text(bubble).contains("Delete this file?"),
        "the closed bubble already carries the question"
    );
}

#[test]
fn a_popconfirm_renders_the_confirm_and_the_cancel_button() {
    let props: EuvPopconfirmProps = EuvPopconfirmProps {
        title: "Delete this file?",
        ok_label: "Delete",
        cancel_label: "Cancel",
        ..Default::default()
    };
    let rendered: VirtualNode = euv_popconfirm(popconfirm_node(props));
    let bubble: &VirtualNode = bubble_of(&rendered);
    assert_eq!(
        count_tags(bubble, "button"),
        2,
        "both action buttons are rendered"
    );
    let text: String = joined_text(bubble);
    assert!(text.contains("Delete"), "the confirm label is rendered");
    assert!(text.contains("Cancel"), "the cancel label is rendered");
}

#[test]
fn a_popconfirm_without_labels_renders_no_action_button() {
    let props: EuvPopconfirmProps = EuvPopconfirmProps {
        title: "Delete this file?",
        ..Default::default()
    };
    let rendered: VirtualNode = euv_popconfirm(popconfirm_node(props));
    let bubble: &VirtualNode = bubble_of(&rendered);
    assert_eq!(
        count_tags(bubble, "button"),
        0,
        "empty labels render neither the confirm nor the cancel button"
    );
}

#[test]
fn the_confirm_handler_fires_the_callback_and_closes_the_bubble() {
    let fired: Rc<Cell<u32>> = Rc::new(Cell::new(0));
    let counter: Rc<Cell<u32>> = fired.clone();
    let callback: Rc<dyn Fn()> = Rc::new(move || {
        counter.set(counter.get() + 1);
    });
    let open: Signal<bool> = Signal::create(true);
    let handler: Rc<dyn Fn(u32)> = popconfirm_ok_handler(Some(callback), open);
    handler(0);
    assert_eq!(fired.get(), 1, "the confirm callback fires exactly once");
    assert!(!open.get(), "confirming closes the bubble");
}

#[test]
fn the_cancel_handler_fires_the_callback_and_closes_the_bubble() {
    let fired: Rc<Cell<u32>> = Rc::new(Cell::new(0));
    let counter: Rc<Cell<u32>> = fired.clone();
    let callback: Rc<dyn Fn()> = Rc::new(move || {
        counter.set(counter.get() + 1);
    });
    let open: Signal<bool> = Signal::create(true);
    let handler: Rc<dyn Fn(u32)> = popconfirm_cancel_handler(Some(callback), open);
    handler(0);
    assert_eq!(fired.get(), 1, "the cancel callback fires exactly once");
    assert!(!open.get(), "cancelling closes the bubble");
}

#[test]
fn the_toggle_handler_opens_and_closes_the_bubble() {
    let open: Signal<bool> = Signal::create(false);
    let handler: Option<Rc<dyn Fn(u32)>> = popconfirm_toggle_handler::<u32>(open)
        .expect("the trigger always carries a toggle handler");
    handler(0);
    assert!(open.get(), "the first click opens the bubble");
    handler(0);
    assert!(!open.get(), "the second click closes it again");
}

#[test]
fn a_confirmation_without_a_callback_still_closes_the_bubble() {
    let open: Signal<bool> = Signal::create(true);
    let handler: Rc<dyn Fn(u32)> = popconfirm_ok_handler(None, open);
    handler(0);
    assert!(!open.get(), "the bubble closes even without a callback");
}
