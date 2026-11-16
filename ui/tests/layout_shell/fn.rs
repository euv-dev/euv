use super::*;

fn render_root(props: EuvLayoutProps) -> VirtualNode {
    euv_layout(VirtualNode::Element {
        tag: Tag::Element(Cow::Borrowed("div")),
        attributes: Vec::new(),
        children: Vec::new(),
        key: None,
        props: Some(Box::new(props)),
    })
}

fn render(props: EuvLayoutProps) -> VirtualNode {
    let root: VirtualNode = render_root(props.clone());
    assert_eq!(attribute(&root, "data-slot").as_deref(), Some("layout"));
    let mode: EuvLayoutMode = if props.mobile.get() {
        EuvLayoutMode::Mobile
    } else {
        EuvLayoutMode::Desktop
    };
    let id: String = use_id("euv-layout-test").get_value().get();
    layout_frame(props, mode, id)
}

fn attribute(node: &VirtualNode, name: &str) -> Option<String> {
    match node {
        VirtualNode::Element { attributes, .. } => attributes
            .iter()
            .find(|entry: &&AttributeEntry| entry.get_name() == name)
            .and_then(AttributeEntry::text_value),
        _ => None,
    }
}

fn elements(node: &VirtualNode) -> Vec<VirtualNode> {
    let mut result: Vec<VirtualNode> = Vec::new();
    if matches!(node, VirtualNode::Element { .. }) {
        result.push(node.clone());
    }
    for child in node.get_children() {
        result.extend(elements(child));
    }
    result
}

fn region(node: &VirtualNode, slot: &str) -> VirtualNode {
    elements(node)
        .into_iter()
        .find(|entry: &VirtualNode| attribute(entry, "data-slot").as_deref() == Some(slot))
        .unwrap_or_else(|| panic!("missing {slot}: {node:?}"))
}

fn slot(label: &'static str) -> Option<EuvLayoutSlot> {
    Some(Rc::new(move |mode: EuvLayoutMode| {
        let text: String = format!("{label}:{mode:?}");
        html! { span { "data-test-slot": label text } }
    }))
}

fn fixture(mobile: bool, open: bool) -> EuvLayoutProps {
    EuvLayoutProps {
        mobile: Signal::create(mobile),
        drawer_open: Signal::create(open),
        root_class: Signal::create(String::from("caller-root")),
        slots: EuvLayoutSlots {
            brand: slot("brand"),
            header_navigation: slot("links"),
            selector: slot("selector"),
            theme_control: slot("theme"),
            navigation: slot("navigation"),
            navigation_footer: slot("footer"),
            content: slot("content"),
            extra: slot("extra"),
        },
        menu_label: "菜单",
        close_label: "关闭",
        ..EuvLayoutProps::default()
    }
}

#[test]
fn layout_shell_actual_component_has_shared_header_and_distinct_navigation_main() {
    for mobile in [false, true] {
        let tree: VirtualNode = render(fixture(mobile, false));
        let header: VirtualNode = region(&tree, "layout-header");
        let slots: Vec<String> = elements(&header)
            .iter()
            .filter_map(|entry: &VirtualNode| attribute(entry, "data-test-slot"))
            .collect();
        assert_eq!(slots, ["brand", "links", "selector", "theme"]);
        let nav: VirtualNode = region(&tree, "layout-navigation");
        let main: VirtualNode = region(&tree, "layout-main");
        assert_eq!(nav.try_get_tag_name().as_deref(), Some("nav"));
        assert_eq!(main.try_get_tag_name().as_deref(), Some("main"));
        assert!(format!("{nav:?}").contains("navigation:"));
        assert!(!elements(&nav).iter().any(|entry: &VirtualNode| {
            attribute(entry, "data-test-slot").as_deref() == Some("content")
        }));
        assert!(format!("{main:?}").contains("content:"));
        let footer: VirtualNode = region(&tree, "layout-navigation-footer");
        assert!(!format!("{footer:?}").contains("selector:"));
        assert!(!format!("{footer:?}").contains("theme:"));
        let mode: &str = if mobile { "Mobile" } else { "Desktop" };
        assert!(format!("{main:?}").contains(mode));
    }
}

#[test]
fn layout_shell_closed_drawer_remains_mounted_and_uniquely_controlled() {
    let first: VirtualNode =
        HookContext::with(HookContext::default(), || render(fixture(true, false)));
    let second: VirtualNode =
        HookContext::with(HookContext::default(), || render(fixture(true, true)));
    let first_drawer: VirtualNode = region(&first, "layout-drawer");
    let second_drawer: VirtualNode = region(&second, "layout-drawer");
    assert_eq!(
        attribute(&first_drawer, "aria-hidden").as_deref(),
        Some("true")
    );
    assert_eq!(
        attribute(&second_drawer, "aria-hidden").as_deref(),
        Some("false")
    );
    assert_ne!(
        attribute(&first_drawer, "id"),
        attribute(&second_drawer, "id")
    );
    assert_eq!(
        attribute(&region(&first, "layout-menu"), "aria-controls"),
        attribute(&first_drawer, "id")
    );
    assert_eq!(
        attribute(&region(&first, "layout-menu"), "aria-expanded").as_deref(),
        Some("false")
    );
    if let VirtualNode::Element { attributes, .. } = first_drawer {
        for name in ["hidden", "inert"] {
            assert!(
                attributes
                    .iter()
                    .any(|entry: &AttributeEntry| entry.get_name() == name)
            );
        }
    }
}

#[test]
fn layout_shell_close_is_idempotent_and_does_not_close_other_instances() {
    let first: Signal<bool> = Signal::create(true);
    let second: Signal<bool> = Signal::create(true);
    let closed: Cell<usize> = Cell::new(0);
    close_layout_drawer(first, || closed.set(closed.get() + 1));
    assert!(!first.get());
    assert!(second.get());
    close_layout_drawer(first, || closed.set(closed.get() + 1));
    assert!(!first.get());
    assert!(second.get());
    assert_eq!(closed.get(), 1);
}

#[test]
fn layout_shell_has_no_builtin_styles_or_classes() {
    let tree: VirtualNode = render(EuvLayoutProps::default());
    for element in elements(&tree) {
        if let VirtualNode::Element { attributes, .. } = element {
            assert!(
                !attributes
                    .iter()
                    .any(|entry: &AttributeEntry| entry.get_name() == "style")
            );
            assert!(
                attributes
                    .iter()
                    .flat_map(AttributeEntry::class_names)
                    .all(|name: String| name.is_empty() || name.starts_with("layout-"))
            );
        }
    }
    let source: &str = include_str!("../../src/component/layout/view/fn.rs");
    for forbidden in ["Css::", "class!", "inject_css", "style:", "cfg!(", "#[cfg"] {
        assert!(!source.contains(forbidden), "unexpected {forbidden}");
    }
}

#[test]
fn layout_shell_caller_signals_are_independent_and_root_class_updates_on_rerender() {
    let first: EuvLayoutProps = fixture(false, false);
    let second: EuvLayoutProps = fixture(true, false);
    let root: Signal<String> = first.root_class;
    let initial: VirtualNode = render_root(first.clone());
    assert!(format!("{initial:?}").contains("data-slot"));
    root.set(String::from("caller-dark"));
    let tree: VirtualNode = render_root(first.clone());
    first.drawer_open.set(true);
    assert!(!second.drawer_open.get());
    assert!(first.drawer_open.get());
    if let VirtualNode::Element { attributes, .. } = tree {
        let classes: Vec<String> = attributes
            .iter()
            .flat_map(AttributeEntry::class_names)
            .collect();
        assert!(
            classes.contains(&String::from("caller-dark")),
            "{classes:?}"
        );
    }
}
