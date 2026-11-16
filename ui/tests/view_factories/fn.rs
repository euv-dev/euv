use super::*;

#[test]
fn dropdown_root_optional_parent_defaults_to_independent() {
    assert!(EuvDropdownMenuProps::default().parent_open.is_none());
    let open: Signal<bool> = Signal::create(true);
    let cleanup: Box<dyn FnOnce()> = dropdown_menu_parent_binding(open, None);
    cleanup();
    assert!(open.get());
    let cleanup: Box<dyn FnOnce()> = dropdown_menu_parent_binding(open, Some(open));
    cleanup();
    assert!(open.get());
}

#[test]
fn dropdown_surface_parent_closes_menu_descendants_but_menu_never_closes_surface() {
    let surface: Signal<bool> = Signal::create(true);
    let menu: Signal<bool> = Signal::create(true);
    let sub: Signal<bool> = Signal::create(true);
    let cleanup: Box<dyn FnOnce()> = dropdown_menu_parent_binding(menu, Some(surface));
    use_dropdown_menu_sub_binding(menu, sub);
    menu.set(false);
    assert!(surface.get());
    assert!(!sub.get());
    menu.set(true);
    sub.set(true);
    surface.set(false);
    assert!(!menu.get() && !sub.get());
    menu.set(true);
    sub.set(true);
    assert!(!menu.get() && !sub.get());
    surface.set(true);
    assert!(!menu.get() && !sub.get());
    cleanup();
}

#[test]
fn dropdown_surface_parent_initially_closed_guards_orphan_open_and_unsubscribes() {
    let surface: Signal<bool> = Signal::create(false);
    let menu: Signal<bool> = Signal::create(true);
    let cleanup: Box<dyn FnOnce()> = dropdown_menu_parent_binding(menu, Some(surface));
    assert!(!menu.get());
    menu.set(true);
    assert!(!menu.get());
    cleanup();
    menu.set(true);
    assert!(menu.get());
    surface.set(true);
    surface.set(false);
    assert!(menu.get());
}

#[test]
fn dropdown_surface_parent_hook_rebinds_and_cleans_up() {
    let first: Signal<bool> = Signal::create(true);
    let second: Signal<bool> = Signal::create(true);
    let menu: Signal<bool> = Signal::create(true);
    let mut context: HookContext = HookContext::default();
    for parent in [first, first, second] {
        context.reset_index();
        HookContext::with(context.clone(), || {
            use_dropdown_menu_parent_binding(menu, Some(parent))
        });
    }
    menu.set(true);
    first.set(false);
    assert!(menu.get());
    second.set(false);
    assert!(!menu.get());
    second.set(true);
    menu.set(true);
    context.switch_arm(usize::MAX);
    assert!(!menu.get());
    menu.set(true);
    second.set(false);
    assert!(menu.get());
}

#[test]
fn dropdown_portal_selector_waits_for_original_trigger_and_excludes_outer_sheets() {
    let menu: Signal<bool> = Signal::create(false);
    let selector: String = dropdown_portal_target(menu);
    let trigger: String = format!("#euv-dropdown-{}-trigger", menu.get_inner());
    assert!(selector.starts_with("[data-slot=\"sheet-content\"]:has("));
    assert!(selector.contains(&format!(
        ":not(:has([data-slot=\"sheet-content\"] {trigger}))"
    )));
    assert!(selector.contains(&format!(".c_app_root:has({trigger})")));
    let source: &str = include_str!("../../src/component/dropdown/view/fn.rs");
    assert!(!source.contains("append_child"));
    assert!(!source.contains("set_property(\"z-index\""));
}

fn props<T>(value: T) -> VirtualNode<T> {
    VirtualNode::Element {
        tag: Tag::Element(Cow::Borrowed("div")),
        attributes: Vec::new(),
        children: Vec::new(),
        key: None,
        props: Some(Box::new(value)),
    }
}

fn has_class_name(node: &VirtualNode, name: &str) -> bool {
    match node {
        VirtualNode::Element {
            attributes,
            children,
            ..
        } => {
            attributes.iter().any(|entry: &AttributeEntry| {
                entry
                    .class_names()
                    .iter()
                    .any(|class: &String| class == name)
            }) || children
                .iter()
                .any(|child: &VirtualNode| has_class_name(child, name))
        }
        VirtualNode::Fragment(children) => children
            .iter()
            .any(|child: &VirtualNode| has_class_name(child, name)),
        _ => false,
    }
}

fn root_tag(node: &VirtualNode) -> String {
    match node {
        VirtualNode::Element { tag, .. } => format!("{tag:?}"),
        VirtualNode::Text(_) => "Text".to_string(),
        VirtualNode::Fragment(_) => "Fragment".to_string(),
        VirtualNode::Dynamic(_) => "Dynamic".to_string(),
        VirtualNode::Empty => "Empty".to_string(),
    }
}

fn count_elements(node: &VirtualNode) -> usize {
    match node {
        VirtualNode::Element { children, .. } => {
            1 + children.iter().map(count_elements).sum::<usize>()
        }
        VirtualNode::Fragment(children) => children.iter().map(count_elements).sum(),
        _ => 0,
    }
}

fn collect_tags(node: &VirtualNode, into: &mut Vec<String>) {
    match node {
        VirtualNode::Element { tag, children, .. } => {
            into.push(format!("{tag:?}"));
            for child in children {
                collect_tags(child, into);
            }
        }
        VirtualNode::Fragment(children) => {
            for child in children {
                collect_tags(child, into);
            }
        }
        _ => {}
    }
}

fn root_attribute_value(node: &VirtualNode, name: &str) -> Option<String> {
    match node {
        VirtualNode::Element { attributes, .. } => attributes
            .iter()
            .find(|entry: &&AttributeEntry| entry.get_name() == name)
            .and_then(AttributeEntry::text_value),
        _ => None,
    }
}

fn has_attribute_name(node: &VirtualNode, name: &str) -> bool {
    match node {
        VirtualNode::Element {
            attributes,
            children,
            ..
        } => {
            attributes
                .iter()
                .any(|entry: &AttributeEntry| entry.get_name() == name)
                || children
                    .iter()
                    .any(|child: &VirtualNode| has_attribute_name(child, name))
        }
        VirtualNode::Fragment(children) => children
            .iter()
            .any(|child: &VirtualNode| has_attribute_name(child, name)),
        _ => false,
    }
}

fn has_attribute_value(node: &VirtualNode, name: &str, value: &str) -> bool {
    match node {
        VirtualNode::Element {
            attributes,
            children,
            ..
        } => {
            attributes.iter().any(|entry: &AttributeEntry| {
                entry.get_name() == name && entry.text_value().as_deref() == Some(value)
            }) || children
                .iter()
                .any(|child: &VirtualNode| has_attribute_value(child, name, value))
        }
        VirtualNode::Fragment(children) => children
            .iter()
            .any(|child: &VirtualNode| has_attribute_value(child, name, value)),
        _ => false,
    }
}

fn avatar_node(
    src: &'static str,
    initials: &'static str,
    size: EuvAvatarSize,
    square: bool,
) -> VirtualNode<EuvAvatarProps> {
    props(EuvAvatarProps {
        src,
        alt: "avatar",
        initials,
        size,
        square: Signal::create(square),
    })
}

#[test]
fn an_avatar_with_no_src_falls_back_to_its_initials() {
    let rendered: VirtualNode = euv_avatar(avatar_node("", "AB", EuvAvatarSize::Medium, false));
    assert_eq!(
        root_attribute_value(&rendered, "data-slot").as_deref(),
        Some("avatar")
    );
    assert_eq!(
        root_attribute_value(&rendered, "data-size").as_deref(),
        Some("default")
    );
    assert!(has_attribute_value(
        &rendered,
        "data-slot",
        "avatar-fallback"
    ));
    let mut tags: Vec<String> = Vec::new();
    collect_tags(&rendered, &mut tags);
    assert_eq!(
        tags,
        vec!["Element(\"span\")", "Element(\"span\")"],
        "no src means no <img>, and the initials sit in the fallback part, \
         got {tags:?}"
    );
    assert!(
        format!("{rendered:?}").contains("AB"),
        "the initials must be rendered as the text fallback"
    );
}

#[test]
fn an_avatar_with_a_src_renders_an_image_inside_its_span() {
    let rendered: VirtualNode =
        euv_avatar(avatar_node("/a.png", "AB", EuvAvatarSize::Medium, false));
    assert_eq!(
        root_attribute_value(&rendered, "data-slot").as_deref(),
        Some("avatar")
    );
    assert_eq!(
        root_attribute_value(&rendered, "data-size").as_deref(),
        Some("default")
    );
    assert!(has_attribute_value(&rendered, "data-slot", "avatar-image"));
    assert!(has_attribute_name(&rendered, "onerror"));
    let mut tags: Vec<String> = Vec::new();
    collect_tags(&rendered, &mut tags);
    assert_eq!(
        tags,
        vec!["Element(\"span\")", "Element(\"img\")"],
        "a src replaces the initials with an <img>, got {tags:?}"
    );
}

#[test]
fn every_avatar_size_renders_the_same_shape() {
    let mut shapes: Vec<Vec<String>> = Vec::new();
    for size in [
        EuvAvatarSize::Small,
        EuvAvatarSize::Medium,
        EuvAvatarSize::Large,
    ] {
        let rendered: VirtualNode = euv_avatar(avatar_node("", "X", size, false));
        let expected_size: &str = match size {
            EuvAvatarSize::Small => "sm",
            EuvAvatarSize::Medium => "default",
            EuvAvatarSize::Large => "lg",
        };
        assert_eq!(
            root_attribute_value(&rendered, "data-size").as_deref(),
            Some(expected_size)
        );
        let mut tags: Vec<String> = Vec::new();
        collect_tags(&rendered, &mut tags);
        shapes.push(tags);
    }
    assert_eq!(shapes[0], shapes[1]);
    assert_eq!(shapes[1], shapes[2]);
    assert_eq!(
        shapes[0],
        vec![
            "Element(\"span\")".to_string(),
            "Element(\"span\")".to_string()
        ],
        "the size only changes a class, never the structure"
    );
}

#[test]
fn a_switch_renders_one_element_and_respects_its_label() {
    let node: VirtualNode<EuvSwitchProps> = props(EuvSwitchProps {
        id: "s1",
        name: "group",
        checked: Signal::create(false),
        label: "Dark mode",
        disabled: Signal::create(false),
    });
    let rendered: VirtualNode = euv_switch(node);
    assert_ne!(
        root_tag(&rendered),
        "Empty",
        "a switch must render something"
    );
    assert!(format!("{rendered:?}").contains("Dark mode"));
}

#[test]
fn a_disabled_switch_still_renders_its_label() {
    let node: VirtualNode<EuvSwitchProps> = props(EuvSwitchProps {
        id: "s2",
        name: "group",
        checked: Signal::create(true),
        label: "Locked",
        disabled: Signal::create(true),
    });
    let rendered: VirtualNode = euv_switch(node);
    assert!(
        format!("{rendered:?}").contains("Locked"),
        "a disabled control must still be visible, got {rendered:?}"
    );
}

#[test]
fn an_empty_label_still_yields_a_control() {
    let node: VirtualNode<EuvSwitchProps> = props(EuvSwitchProps {
        id: "s3",
        name: "",
        checked: Signal::create(false),
        label: "",
        disabled: Signal::create(false),
    });
    let rendered: VirtualNode = euv_switch(node);
    assert_ne!(root_tag(&rendered), "Empty");
}

#[test]
fn use_euv_switch_state_reflects_the_signal_it_wraps() {
    let checked: Signal<bool> = Signal::create(false);
    let state: EuvSwitchState = use_euv_switch_state(checked);
    assert!(
        !state.get_checked().get(),
        "the state starts on the signal's value"
    );
    state.toggle();
    assert!(
        state.get_checked().get(),
        "toggling through the state must reach the signal"
    );
    assert!(checked.get());
}

#[test]
fn a_checkbox_renders_its_own_label() {
    let node: VirtualNode<EuvCheckboxProps> = props(EuvCheckboxProps {
        id: "c1",
        name: "terms",
        autocomplete: "off",
        checked: Signal::create(false),
        disabled: Signal::create(false),
        label: "Accept",
    });
    let rendered: VirtualNode = euv_checkbox(node);
    assert!(
        format!("{rendered:?}").contains("Accept"),
        "got {rendered:?}"
    );
    assert!(count_elements(&rendered) >= 1);
}

#[test]
fn a_square_avatar_has_the_same_shape_as_a_round_one() {
    let round: VirtualNode = euv_avatar(avatar_node("", "X", EuvAvatarSize::Medium, false));
    let square: VirtualNode = euv_avatar(avatar_node("", "X", EuvAvatarSize::Medium, true));
    let mut round_tags: Vec<String> = Vec::new();
    let mut square_tags: Vec<String> = Vec::new();
    collect_tags(&round, &mut round_tags);
    collect_tags(&square, &mut square_tags);
    assert_eq!(
        round_tags, square_tags,
        "the square modifier is a class, not a structural change, so the tree \
         shape must be identical; only the class value differs"
    );
}

#[test]
fn an_input_wraps_its_input_in_a_container() {
    let node: VirtualNode<EuvInputProps> = props(EuvInputProps {
        id: "i1",
        name: "email",
        label: "Email",
        input_type: "email",
        placeholder: "you@example.com",
        value: Signal::create(String::new()),
        autocomplete: "email",
        oninput: None,
        class: Css::default(),
        aria_label: "",
        readonly: false,
        disabled: Signal::create(false),
        wrapper_class: Css::default(),
        required: false,
        max_length: 0,
        step: "",
        aria_describedby: "",
    });
    let rendered: VirtualNode = euv_input(node);
    let mut tags: Vec<String> = Vec::new();
    collect_tags(&rendered, &mut tags);
    assert_eq!(
        tags,
        vec!["Element(\"div\")", "Element(\"input\")"],
        "an input is a container plus the <input> itself, got {tags:?}"
    );
}

#[test]
fn an_input_always_carries_the_attributes_the_dom_needs() {
    let node: VirtualNode<EuvInputProps> = props(EuvInputProps {
        id: "i1",
        name: "email",
        label: "Email",
        input_type: "email",
        placeholder: "you@example.com",
        value: Signal::create(String::new()),
        autocomplete: "email",
        oninput: None,
        class: Css::default(),
        aria_label: "",
        readonly: false,
        disabled: Signal::create(false),
        wrapper_class: Css::default(),
        required: false,
        max_length: 0,
        step: "",
        aria_describedby: "",
    });
    let rendered: VirtualNode = euv_input(node);
    let mut names: Vec<String> = Vec::new();
    if let VirtualNode::Element { children, .. } = &rendered {
        for child in children {
            if let VirtualNode::Element { attributes, .. } = child {
                for attribute in attributes {
                    names.push(attribute.get_name().to_string());
                }
            }
        }
    }
    for expected in [
        "id",
        "name",
        "type",
        "placeholder",
        "value",
        "oninput",
        "data-slot",
        "class",
    ] {
        assert!(
            names.iter().any(|name: &String| name == expected),
            "the <input> must carry a {expected} attribute, got {names:?}"
        );
    }
}

#[test]
fn a_field_pairs_its_label_with_its_input() {
    let node: VirtualNode<EuvFieldProps> = props(EuvFieldProps {
        id: "f1",
        name: "nick",
        label: "Nickname",
        input_type: "text",
        placeholder: "pick one",
        autocomplete: "off",
        value: Signal::create(String::new()),
        error: None,
        oninput: None,
    });
    let rendered: VirtualNode = euv_field(node);
    let mut tags: Vec<String> = Vec::new();
    collect_tags(&rendered, &mut tags);
    assert_eq!(
        tags,
        vec![
            "Element(\"div\")",
            "Element(\"label\")",
            "Element(\"input\")",
        ],
        "a field is a labelled input, and the label must come first so a click \
         focuses it, got {tags:?}"
    );
    assert!(
        format!("{rendered:?}").contains("Nickname"),
        "the label text is a real text node, so it is visible: {rendered:?}"
    );
}

#[test]
fn a_checked_checkbox_has_the_same_shape_as_an_unchecked_one() {
    let off: VirtualNode<EuvCheckboxProps> = props(EuvCheckboxProps {
        id: "c2",
        name: "t",
        autocomplete: "off",
        checked: Signal::create(false),
        disabled: Signal::create(false),
        label: "L",
    });
    let on: VirtualNode<EuvCheckboxProps> = props(EuvCheckboxProps {
        id: "c2",
        name: "t",
        autocomplete: "off",
        checked: Signal::create(true),
        disabled: Signal::create(false),
        label: "L",
    });
    let mut off_tags: Vec<String> = Vec::new();
    let mut on_tags: Vec<String> = Vec::new();
    collect_tags(&euv_checkbox(off), &mut off_tags);
    collect_tags(&euv_checkbox(on), &mut on_tags);
    assert_eq!(
        off_tags, on_tags,
        "checkedness rides on an attribute value, not on structure, so both \
         states must produce the same tree shape"
    );
    assert_eq!(
        off_tags,
        vec![
            "Element(\"div\")",
            "Element(\"input\")",
            "Element(\"span\")",
            "Element(\"label\")",
        ],
        "the input and mounted indicator precede the label in both states, got {off_tags:?}"
    );
}

#[test]
fn a_checkbox_always_exposes_a_checked_attribute() {
    let node: VirtualNode<EuvCheckboxProps> = props(EuvCheckboxProps {
        id: "c1",
        name: "terms",
        autocomplete: "off",
        checked: Signal::create(false),
        disabled: Signal::create(false),
        label: "Accept",
    });
    let rendered: VirtualNode = euv_checkbox(node);
    let text: String = format!("{rendered:?}");
    assert!(
        text.contains("checked"),
        "an uncontrolled checkbox would never update, so the attribute must exist: {text}"
    );
    assert!(
        text.contains("disabled"),
        "a checkbox must expose its disabled state: {text}"
    );
    assert!(
        text.contains("onchange"),
        "a read-only checkbox is useless, got {text}"
    );
}

#[test]
fn a_hero_with_no_actions_still_renders_its_frame() {
    let node: VirtualNode<EuvHeroProps> = props(EuvHeroProps {
        title: "Welcome",
        subtitle: "to euv",
        actions: &[],
    });
    let rendered: VirtualNode = euv_hero(node);
    let mut tags: Vec<String> = Vec::new();
    collect_tags(&rendered, &mut tags);
    assert!(
        tags.contains(&"Element(\"h1\")".to_string()),
        "the title is an <h1> however it is populated, got {tags:?}"
    );
    assert_eq!(
        count_elements(&rendered),
        4,
        "an actionless hero is a frame, a title and a body, got {tags:?}"
    );
}

#[test]
fn a_hero_with_one_action_and_one_without_agree_on_their_frame() {
    static ONE_ACTION: [EuvHeroAction; 1] = [EuvHeroAction {
        text: "Go",
        link: "/go",
        primary: true,
    }];
    let bare: VirtualNode = euv_hero(props(EuvHeroProps {
        title: "W",
        subtitle: "S",
        actions: &[],
    }));
    let with: VirtualNode = euv_hero(props(EuvHeroProps {
        title: "W",
        subtitle: "S",
        actions: &ONE_ACTION,
    }));
    let mut bare_tags: Vec<String> = Vec::new();
    let mut with_tags: Vec<String> = Vec::new();
    collect_tags(&bare, &mut bare_tags);
    collect_tags(&with, &mut with_tags);
    assert_eq!(
        bare_tags, with_tags,
        "an action is spliced into a dynamic slot, so the statically visible          frame is identical in both cases"
    );
}

static NAV_ITEMS: [EuvNavbarItem; 2] = [
    EuvNavbarItem {
        text: "Home",
        link: "/",
    },
    EuvNavbarItem {
        text: "Docs",
        link: "/docs",
    },
];

static SIDEBAR_ITEMS: [EuvSidebarItem; 2] = [
    EuvSidebarItem {
        text: "Group",
        link: None,
        children: &[
            EuvSidebarItem {
                text: "Child",
                link: Some("/child"),
                children: &[],
            },
            EuvSidebarItem {
                text: "Other",
                link: Some("/other"),
                children: &[],
            },
        ],
    },
    EuvSidebarItem {
        text: "Solo",
        link: Some("/solo"),
        children: &[],
    },
];

fn navbar_link_node(item: EuvNavbarItem) -> VirtualNode<EuvNavbarLinkProps> {
    props(EuvNavbarLinkProps {
        route_signal: Signal::create(String::from("/")),
        item,
    })
}

fn sidebar_item_node(item: EuvSidebarItem) -> VirtualNode<EuvSidebarItemProps> {
    props(EuvSidebarItemProps {
        route_signal: Signal::create(String::from("/")),
        collapsed: Signal::create(Vec::new()),
        item,
        prefix: String::from("/app"),
        on_navigate: None,
    })
}

#[test]
fn a_navbar_link_renders_one_anchor() {
    let rendered: VirtualNode = euv_navbar_link(navbar_link_node(NAV_ITEMS[0]));
    let mut tags: Vec<String> = Vec::new();
    collect_tags(&rendered, &mut tags);
    assert_eq!(
        tags,
        vec!["Element(\"a\")".to_string()],
        "a nav link is an anchor, got {tags:?}"
    );
}

#[test]
fn a_navbar_link_carries_href_and_click_handlers() {
    let rendered: VirtualNode = euv_navbar_link(navbar_link_node(NAV_ITEMS[1]));
    let text: String = format!("{rendered:?}");
    for attribute in ["href", "onclick", "class"] {
        assert!(
            text.contains(attribute),
            "a link without {attribute} cannot navigate or be styled: {text}"
        );
    }
}

#[test]
fn a_navbar_renders_one_link_per_item() {
    let node: VirtualNode<EuvNavbarProps> = props(EuvNavbarProps {
        route_signal: Signal::create(String::from("/")),
        brand_logo: "/logo.svg",
        brand_title: "euv",
        brand_href: "/",
        items: &NAV_ITEMS,
        drawer_open: None,
    });
    let rendered: VirtualNode = euv_navbar(node);
    let mut tags: Vec<String> = Vec::new();
    collect_tags(&rendered, &mut tags);
    let links: usize = tags
        .iter()
        .filter(|tag: &&String| tag.contains("\"a\""))
        .count();
    assert_eq!(
        links, 3,
        "two nav items plus the brand link are all anchors, got {tags:?}"
    );
    assert_eq!(
        tags.first().map(String::as_str),
        Some("Element(\"nav\")"),
        "a navbar is a <nav> landmark, got {tags:?}"
    );
    assert_eq!(
        tags.get(1).map(String::as_str),
        Some("Element(\"a\")"),
        "the brand comes first inside the nav, got {tags:?}"
    );
}

#[test]
fn an_open_drawer_changes_the_navbar_shape() {
    let closed: VirtualNode = euv_navbar(props(EuvNavbarProps {
        route_signal: Signal::create(String::from("/")),
        brand_logo: "/logo.svg",
        brand_title: "euv",
        brand_href: "/",
        items: &NAV_ITEMS,
        drawer_open: Some(Signal::create(false)),
    }));
    let open: VirtualNode = euv_navbar(props(EuvNavbarProps {
        route_signal: Signal::create(String::from("/")),
        brand_logo: "/logo.svg",
        brand_title: "euv",
        brand_href: "/",
        items: &NAV_ITEMS,
        drawer_open: Some(Signal::create(true)),
    }));
    assert_eq!(
        count_elements(&closed),
        count_elements(&open),
        "drawer open state rides on an attribute and a class, so the statically \
         visible tree is the same in both states"
    );
}

#[test]
fn a_sidebar_group_expands_into_its_children() {
    let rendered: VirtualNode = euv_sidebar_item(sidebar_item_node(SIDEBAR_ITEMS[0]));
    let mut tags: Vec<String> = Vec::new();
    collect_tags(&rendered, &mut tags);
    let links: usize = tags
        .iter()
        .filter(|tag: &&String| tag.contains("\"a\""))
        .count();
    assert_eq!(
        links, 2,
        "a group with two children must emit one anchor per child, got {tags:?}"
    );
}

#[test]
fn a_sidebar_leaf_with_a_link_emits_a_single_anchor() {
    let rendered: VirtualNode = euv_sidebar_item(sidebar_item_node(SIDEBAR_ITEMS[1]));
    let mut tags: Vec<String> = Vec::new();
    collect_tags(&rendered, &mut tags);
    let links: usize = tags
        .iter()
        .filter(|tag: &&String| tag.contains("\"a\""))
        .count();
    assert_eq!(
        links, 1,
        "a childless item with a link is one anchor, got {tags:?}"
    );
}

#[test]
fn collapsing_a_sidebar_group_does_not_reshape_the_tree() {
    let expanded: VirtualNode = euv_sidebar_item(sidebar_item_node(SIDEBAR_ITEMS[0]));
    let mut collapsed: VirtualNode<EuvSidebarItemProps> = props(EuvSidebarItemProps {
        route_signal: Signal::create(String::from("/")),
        collapsed: Signal::create(vec![String::from("/app")]),
        item: SIDEBAR_ITEMS[0],
        prefix: String::from("/app"),
        on_navigate: None,
    });
    let mut expanded_tags: Vec<String> = Vec::new();
    let mut collapsed_tags: Vec<String> = Vec::new();
    collect_tags(&expanded, &mut expanded_tags);
    collect_tags(&euv_sidebar_item(collapsed.clone()), &mut collapsed_tags);
    assert_eq!(
        expanded_tags, collapsed_tags,
        "collapse state is a class on the same nodes, not a different tree"
    );
    collapsed = sidebar_item_node(SIDEBAR_ITEMS[0]);
    let _: VirtualNode<EuvSidebarItemProps> = collapsed;
}

#[test]
fn a_sidebar_renders_its_whole_item_tree() {
    let node: VirtualNode<EuvSidebarProps> = props(EuvSidebarProps {
        route_signal: Signal::create(String::from("/")),
        collapsed: Signal::create(Vec::new()),
        items: &SIDEBAR_ITEMS,
        prefix: String::from("/app"),
        on_navigate: None,
    });
    let rendered: VirtualNode = euv_sidebar(node);
    let mut tags: Vec<String> = Vec::new();
    collect_tags(&rendered, &mut tags);
    let links: usize = tags
        .iter()
        .filter(|tag: &&String| tag.contains("\"a\""))
        .count();
    assert_eq!(links, 3, "two children plus one solo link, got {tags:?}");
}

#[test]
fn the_vconsole_panel_renders_even_when_closed() {
    let closed: VirtualNode = euv_vconsole_panel(props(EuvVconsolePanelProps {
        panel_open: Signal::create(false),
    }));
    let open: VirtualNode = euv_vconsole_panel(props(EuvVconsolePanelProps {
        panel_open: Signal::create(true),
    }));
    assert_ne!(
        root_tag(&closed),
        "Empty",
        "a panel that vanishes is not a panel"
    );
    assert_eq!(
        count_elements(&closed),
        count_elements(&open),
        "open state rides on a class, so both states render the same nodes"
    );
}

#[test]
fn the_vconsole_fab_carries_its_entry_count_signal() {
    let rendered: VirtualNode = euv_vconsole_fab(props(EuvVconsoleFabProps {
        panel_open: Signal::create(false),
        console_signal: Signal::create(Vec::new()),
    }));
    assert_ne!(
        root_tag(&rendered),
        "Empty",
        "the fab is how the panel is reached"
    );
}

#[test]
fn the_vconsole_drawer_renders_its_filter_buttons() {
    let rendered: VirtualNode = euv_vconsole_drawer(props(EuvVconsoleDrawerProps {
        console_signal: Signal::create(Vec::new()),
        panel_open: Signal::create(true),
    }));
    let mut tags: Vec<String> = Vec::new();
    collect_tags(&rendered, &mut tags);
    assert!(
        tags.len() >= 4,
        "an All/Log/Warn/Error filter row alone is four buttons, got {tags:?}"
    );
}

#[test]
fn the_vconsole_drawer_renders_one_row_per_entry() {
    let empty: VirtualNode = euv_vconsole_drawer(props(EuvVconsoleDrawerProps {
        console_signal: Signal::create(Vec::new()),
        panel_open: Signal::create(true),
    }));
    let entries: Vec<ConsoleEntry> = vec![
        ConsoleEntry {
            level: LogLevel::Log,
            message: String::from("one"),
        },
        ConsoleEntry {
            level: LogLevel::Error,
            message: String::from("two"),
        },
    ];
    let filled: VirtualNode = euv_vconsole_drawer(props(EuvVconsoleDrawerProps {
        console_signal: Signal::create(entries),
        panel_open: Signal::create(true),
    }));
    assert!(
        count_elements(&filled) > count_elements(&empty),
        "two entries must add rows: {} vs {}",
        count_elements(&filled),
        count_elements(&empty)
    );
}

#[test]
fn a_vconsole_drawer_with_one_entry_beats_one_with_two() {
    let one: Vec<ConsoleEntry> = vec![ConsoleEntry {
        level: LogLevel::Warn,
        message: String::from("a"),
    }];
    let two: Vec<ConsoleEntry> = vec![
        ConsoleEntry {
            level: LogLevel::Warn,
            message: String::from("a"),
        },
        ConsoleEntry {
            level: LogLevel::Error,
            message: String::from("b"),
        },
    ];
    let single: VirtualNode = euv_vconsole_drawer(props(EuvVconsoleDrawerProps {
        console_signal: Signal::create(one),
        panel_open: Signal::create(true),
    }));
    let double: VirtualNode = euv_vconsole_drawer(props(EuvVconsoleDrawerProps {
        console_signal: Signal::create(two),
        panel_open: Signal::create(true),
    }));
    assert!(
        count_elements(&double) > count_elements(&single),
        "each entry must add a row: {} vs {}",
        count_elements(&double),
        count_elements(&single)
    );
}

static MD_RULE: [EuvMdBlock; 1] = [EuvMdBlock::Rule];

#[test]
fn the_markdown_table_sheet_keeps_the_table_in_table_layout() {
    let sheet: &str = euv_md_css();
    let anchored: String = sheet
        .split(".md-body table{width:")
        .nth(1)
        .map(|rest: &str| format!("width:{rest}"))
        .unwrap_or_default();
    let table_rule: &str = anchored.split('}').next().unwrap_or_default();
    assert!(
        !table_rule.contains("display:block"),
        "display:block on the table drops it out of table layout, so it \
         shrink-wraps its content instead of filling the column — the \
         reported \"the table does not expand to full width\". \
         Rule was {table_rule:?}"
    );
    assert!(
        !table_rule.contains("overflow-x"),
        "overflow-x on a display:table box is ignored by the browser, so \
         the table can never scroll; the wrapper carries it instead. \
         Rule was {table_rule:?}"
    );
    assert!(
        table_rule.contains("width:100%"),
        "without width:100% the table shrink-wraps its content instead of \
         filling the prose column. Rule was {table_rule:?}"
    );
}

#[test]
fn the_markdown_table_sheet_keeps_every_row_on_one_line() {
    let sheet: &str = euv_md_css();
    assert!(
        sheet.contains("white-space:nowrap"),
        "cells wrap as soon as the content does not fit, so every row grows \
         tall and the table reads as broken; cells must stay on one line \
         and the wrapper scrolls horizontally instead"
    );
}

#[test]
fn a_calendar_renders_one_cell_per_day_plus_the_weekday_header() {
    let node: VirtualNode<EuvCalendarProps> = props(EuvCalendarProps {
        title: "2026-10",
        weekdays: vec![
            EuvCalendarWeekday::Mon,
            EuvCalendarWeekday::Tue,
            EuvCalendarWeekday::Wed,
            EuvCalendarWeekday::Thu,
            EuvCalendarWeekday::Fri,
            EuvCalendarWeekday::Sat,
            EuvCalendarWeekday::Sun,
        ],
        on_day_click: None,
        days: vec![
            EuvCalendarDay {
                day: 1,
                muted: false,
                today: false,
            },
            EuvCalendarDay {
                day: 2,
                muted: false,
                today: false,
            },
            EuvCalendarDay {
                day: 3,
                muted: true,
                today: true,
            },
        ],
        selected: Signal::create(0),
    });
    let rendered: VirtualNode = euv_calendar(node);
    assert_ne!(root_tag(&rendered), "Empty");
    let empty: VirtualNode = euv_calendar(props(EuvCalendarProps {
        title: "2026-10",
        weekdays: Vec::new(),
        days: Vec::new(),
        selected: Signal::create(0),
        on_day_click: None,
    }));
    assert!(
        count_elements(&rendered) > count_elements(&empty),
        "seven weekday headers plus three days must beat an empty calendar: {} vs {}",
        count_elements(&rendered),
        count_elements(&empty)
    );
}

#[test]
fn the_calendar_day_click_handler_is_always_present() {
    let selected: Signal<u32> = Signal::create(0);
    assert!(
        on_calendar_day_click(selected, 12).is_some(),
        "a calendar whose days cannot be clicked is a read-only grid"
    );
}

#[test]
fn a_dropdown_renders_one_node_per_item() {
    let items: Vec<EuvDropdownItem> = vec![
        EuvDropdownItem {
            label: "One",
            value: "1",
            disabled: false,
            active: false,
        },
        EuvDropdownItem {
            label: "Two",
            value: "2",
            disabled: false,
            active: true,
        },
    ];
    let full: VirtualNode = euv_dropdown(props(EuvDropdownProps {
        open: Signal::create(true),
        items: items.clone(),
        on_select: None,
    }));
    let bare: VirtualNode = euv_dropdown(props(EuvDropdownProps {
        open: Signal::create(true),
        items: Vec::new(),
        on_select: None,
    }));
    assert!(
        count_elements(&full) > count_elements(&bare),
        "two items must add nodes: {} vs {}",
        count_elements(&full),
        count_elements(&bare)
    );
}

#[test]
fn dropdown_shadcn_rows_do_not_reuse_the_monochrome_locale_style() {
    let rendered: VirtualNode = euv_dropdown(props(EuvDropdownProps {
        open: Signal::create(true),
        items: vec![
            EuvDropdownItem {
                label: "One",
                value: "one",
                disabled: false,
                active: false,
            },
            EuvDropdownItem {
                label: "Two",
                value: "two",
                disabled: false,
                active: true,
            },
        ],
        on_select: None,
    }));
    assert!(has_class_name(&rendered, "c_euv_dropdown_menu"));
    assert!(has_class_name(&rendered, "c_euv_dropdown_item_active"));
    assert!(has_class_name(&rendered, "c_euv_dropdown_item"));
    assert!(!has_class_name(&rendered, "c_euv_dropdown_menu_item"));
}

#[test]
fn a_dropdown_menu_exposes_state_and_roving_menu_item_semantics() {
    let rendered: VirtualNode = euv_dropdown(props(EuvDropdownProps {
        open: Signal::create(true),
        items: vec![
            EuvDropdownItem {
                label: "One",
                value: "1",
                disabled: false,
                active: false,
            },
            EuvDropdownItem {
                label: "Two",
                value: "2",
                disabled: false,
                active: false,
            },
        ],
        on_select: None,
    }));
    assert!(has_attribute_value(&rendered, "role", "menu"));
    assert!(has_attribute_value(
        &rendered,
        "aria-label",
        "Dropdown menu"
    ));
    assert!(has_attribute_value(&rendered, "data-state", "open"));
    assert!(has_attribute_value(&rendered, "role", "menuitem"));
    assert!(has_attribute_value(&rendered, "tabindex", "-1"));
    assert!(has_attribute_value(&rendered, "aria-disabled", "false"));
    assert!(has_attribute_name(&rendered, "onkeydown"));
}

#[test]
fn a_dropdown_trigger_exposes_open_state_and_keyboard_entry() {
    let trigger: VirtualNode = euv_dropdown_trigger(props(EuvDropdownTriggerProps {
        open: Signal::create(false),
    }));
    assert!(has_attribute_value(&trigger, "aria-haspopup", "menu"));
    assert!(has_attribute_value(&trigger, "aria-expanded", "false"));
    assert!(has_attribute_name(&trigger, "onkeydown"));
}

#[test]
fn a_dropdown_root_binds_the_lifecycle_managed_capture_listener() {
    let node: VirtualNode = euv_dropdown(props(EuvDropdownProps::default()));
    assert!(has_attribute_name(&node, "ref"));
    let view: &str = include_str!("../../src/component/dropdown/view/fn.rs");
    assert!(view.contains("use_dropdown_outside(open, outside_target.clone())"));
    assert!(view.contains("dropdown_content_for_root(&root, open)"));
    assert!(view.contains("App::use_cleanup("));
    assert!(view.contains("add_event_listener_with_callback_and_bool"));
    assert!(view.contains("remove_event_listener_with_callback_and_bool"));
    assert!(view.contains("if !open.get() || !root.is_connected()"));
    assert!(view.contains("cancel_animation_frame"));
    assert!(view.contains("open.unsubscribe(subscription)"));
    assert!(view.contains("mounted.set(false)"));
}

#[test]
fn dropdown_ids_associate_each_menu_with_its_plain_text_button() {
    let open: Signal<bool> = Signal::create(false);
    let menu: VirtualNode = euv_dropdown(props(EuvDropdownProps {
        open,
        ..EuvDropdownProps::default()
    }));
    let mut trigger_props: VirtualNode<EuvDropdownTriggerProps> =
        props(EuvDropdownTriggerProps { open });
    if let VirtualNode::Element { children, .. } = &mut trigger_props {
        children.push(VirtualNode::Text(TextNode::new(
            Cow::Borrowed("Open actions"),
            None,
        )));
    }
    let trigger: VirtualNode = euv_dropdown_trigger(trigger_props);
    let trigger_id: String = root_attribute_value(&trigger, "id").unwrap();
    let menu_id: String = root_attribute_value(&trigger, "aria-controls").unwrap();
    assert!(has_attribute_value(&menu, "id", &menu_id));
    assert!(has_attribute_value(&menu, "aria-labelledby", &trigger_id));
    assert_eq!(count_elements(&trigger), 1);
    assert!(root_tag(&trigger).contains("button"));
    assert!(
        root_attribute_value(&trigger, "class")
            .unwrap()
            .contains("c_euv_button")
    );
    let other: VirtualNode = euv_dropdown_trigger(props(EuvDropdownTriggerProps::default()));
    assert_ne!(root_attribute_value(&other, "id"), Some(trigger_id));
}

#[test]
fn an_open_and_a_closed_dropdown_keep_the_same_frame() {
    let items: Vec<EuvDropdownItem> = vec![EuvDropdownItem {
        label: "One",
        value: "1",
        disabled: false,
        active: false,
    }];
    let open: VirtualNode = euv_dropdown(props(EuvDropdownProps {
        open: Signal::create(true),
        items: items.clone(),
        on_select: None,
    }));
    let closed: VirtualNode = euv_dropdown(props(EuvDropdownProps {
        open: Signal::create(false),
        items,
        on_select: None,
    }));
    assert_eq!(
        count_elements(&open),
        count_elements(&closed),
        "openness rides on a class, so both states must render the same nodes"
    );
    assert!(has_attribute_value(&open, "data-state", "open"));
    assert!(has_attribute_value(&closed, "data-state", "closed"));
}

#[test]
fn dropdown_typeahead_cycles_buffers_and_leaves_missing_matches_untouched() {
    let labels: Vec<String> = vec![
        "Create document".into(),
        "Duplicate document".into(),
        "Archive document".into(),
        "Archive copy".into(),
    ];
    assert_eq!(dropdown_next_index(&labels, Some(0), "a"), Some(2));
    assert_eq!(dropdown_next_index(&labels, Some(2), "a"), Some(3));
    assert_eq!(dropdown_next_index(&labels, Some(3), "a"), Some(2));
    assert_eq!(dropdown_next_index(&labels, Some(2), "archive c"), Some(3));
    assert_eq!(dropdown_next_index(&labels, Some(2), "ARCHIVE D"), Some(2));
    assert_eq!(dropdown_next_index(&labels, Some(2), "missing"), None);
    assert_eq!(dropdown_search_prefix("a", "A", 20.0), "a");
    assert_eq!(dropdown_search_prefix("ar", "c", 499.0), "arc");
    assert_eq!(dropdown_search_prefix("ar", "d", 500.0), "d");
    assert_eq!(dropdown_search_prefix("", "D", 0.0), "d");
}

#[test]
fn dropdown_navigation_wraps_and_handles_empty_enabled_rows() {
    let labels: Vec<String> = vec!["One".into(), "Three".into()];
    assert_eq!(dropdown_next_index(&labels, None, "ArrowDown"), Some(0));
    assert_eq!(dropdown_next_index(&labels, None, "ArrowUp"), Some(1));
    assert_eq!(dropdown_next_index(&labels, Some(0), "ArrowUp"), Some(1));
    assert_eq!(dropdown_next_index(&labels, Some(1), "ArrowDown"), Some(0));
    assert_eq!(dropdown_next_index(&labels, Some(0), "End"), Some(1));
    assert_eq!(dropdown_next_index(&labels, Some(1), "Home"), Some(0));
    for key in ["ArrowDown", "ArrowUp", "Home", "End", "a"] {
        assert_eq!(dropdown_next_index(&[], None, key), None);
    }
}

#[test]
fn dropdown_disabled_rows_are_native_disabled_and_have_no_selection_handler() {
    let item: EuvDropdownItem = EuvDropdownItem::new("Disabled", "disabled", false, true);
    assert!(item.get_disabled());
    let row: VirtualNode = euv_dropdown(props(EuvDropdownProps {
        open: Signal::create(true),
        items: vec![item],
        on_select: None,
    }));
    assert!(has_attribute_name(&row, "disabled"));
    assert!(has_attribute_value(&row, "disabled", "true"));
    assert!(has_attribute_value(&row, "aria-disabled", "true"));
    assert!(has_class_name(&row, "c_euv_dropdown_menu_item_disabled"));
    let standalone: VirtualNode = euv_dropdown_menu_item(props(EuvDropdownMenuItemProps {
        disabled: true,
        ..EuvDropdownMenuItemProps::default()
    }));
    assert!(has_attribute_name(&standalone, "disabled"));
    assert!(has_attribute_value(&standalone, "disabled", "true"));
    assert!(has_attribute_value(&standalone, "aria-disabled", "true"));
    assert!(has_class_name(&standalone, "c_euv_dropdown_menu_item"));
    assert!(!EuvDropdownItem::default().get_disabled());
}

#[test]
fn dropdown_composable_defaults_are_closed_and_actions_close_on_selection() {
    assert!(!EuvDropdownMenuProps::default().open.get());
    assert!(!EuvDropdownMenuContentProps::default().open.get());
    let action: EuvDropdownMenuItemProps = EuvDropdownMenuItemProps::default();
    assert!(action.close_on_click);
    assert!(!action.open.get());
    assert!(!action.disabled);
    assert!(action.on_select.is_none());
    assert!(!EuvDropdownMenuCheckboxItemProps::default().close_on_click);
}

#[test]
fn dropdown_composable_content_shares_trigger_ids_and_retains_its_mixed_rows() {
    let open: Signal<bool> = Signal::create(false);
    let checked: Signal<bool> = Signal::create(false);
    let trigger: VirtualNode = euv_dropdown_trigger(props(EuvDropdownTriggerProps { open }));
    let trigger_id: String = root_attribute_value(&trigger, "id").unwrap();
    let content_id: String = root_attribute_value(&trigger, "aria-controls").unwrap();
    for is_open in [false, true] {
        open.set(is_open);
        let mut content_props: VirtualNode<EuvDropdownMenuContentProps> =
            props(EuvDropdownMenuContentProps { open });
        if let VirtualNode::Element { children, .. } = &mut content_props {
            children.push(euv_dropdown_menu_checkbox_item(props(
                EuvDropdownMenuCheckboxItemProps {
                    open,
                    checked,
                    label: "Toolbar",
                    on_checked_change: Some(Rc::new(|_: bool| panic!("render must not activate"))),
                    ..EuvDropdownMenuCheckboxItemProps::default()
                },
            )));
            children.push(euv_dropdown_menu_item(props(EuvDropdownMenuItemProps {
                open,
                label: "Apply",
                shortcut: "Ctrl+A",
                on_select: Some(Rc::new(|_: &'static str| panic!("render must not select"))),
                ..EuvDropdownMenuItemProps::default()
            })));
        }
        let content: VirtualNode = euv_dropdown_menu_content(content_props);
        let VirtualNode::Element {
            tag: Tag::Portal(target),
            children,
            ..
        } = &content
        else {
            panic!("dropdown content must escape clipping ancestors through a portal");
        };
        assert_eq!(target.as_ref(), dropdown_portal_target(open));
        let popup: &VirtualNode = children.first().unwrap();
        assert_eq!(root_attribute_value(popup, "id"), Some(content_id.clone()));
        assert_eq!(
            root_attribute_value(popup, "aria-labelledby"),
            Some(trigger_id.clone())
        );
        assert_eq!(root_attribute_value(popup, "role").as_deref(), Some("menu"));
        assert_eq!(
            root_attribute_value(popup, "data-state").as_deref(),
            Some(if is_open { "open" } else { "closed" })
        );
        assert!(has_attribute_name(&content, "onkeydown"));
        assert!(has_attribute_value(&content, "role", "menuitemcheckbox"));
        assert!(has_attribute_value(&content, "role", "menuitem"));
        assert!(has_attribute_value(&content, "data-label", "Toolbar"));
        assert!(has_attribute_value(&content, "data-label", "Apply"));
        assert!(has_class_name(&content, "c_euv_dropdown_menu"));
        assert!(has_attribute_name(popup, "ref"));
        assert_eq!(count_elements(&content), 7);
        let mut root_props: VirtualNode<EuvDropdownMenuProps> = props(EuvDropdownMenuProps {
            open,
            ..EuvDropdownMenuProps::default()
        });
        if let VirtualNode::Element { children, .. } = &mut root_props {
            children.push(trigger.clone());
            children.push(content);
        }
        let root: VirtualNode = euv_dropdown_menu(root_props);
        assert_eq!(
            root_attribute_value(&root, "data-slot").as_deref(),
            Some("dropdown-menu")
        );
        assert!(has_attribute_name(&root, "ref"));
        assert!(has_attribute_value(&root, "id", &trigger_id));
        assert!(has_attribute_value(&root, "id", &content_id));
        assert_eq!(count_elements(&root), 9);
        assert_eq!(open.get(), is_open);
        assert!(!checked.get());
    }
    let sibling: VirtualNode =
        euv_dropdown_menu_content(props(EuvDropdownMenuContentProps::default()));
    assert!(!has_attribute_value(&sibling, "id", &content_id));
}

#[test]
fn dropdown_popup_placement_preserves_gaps_flips_and_clamps_both_edges() {
    for gap in [4.0, 8.0] {
        let (_, top, available): (f64, f64, f64) = dropdown_popup_position(
            (40.0, 140.0, 50.0, 86.0),
            (128.0, 120.0),
            (320.0, 568.0),
            gap,
        );
        assert_eq!(top, 86.0 + gap);
        assert_eq!(available, 560.0 - top);
        let (_, top, available): (f64, f64, f64) = dropdown_popup_position(
            (40.0, 140.0, 400.0, 436.0),
            (128.0, 200.0),
            (320.0, 568.0),
            gap,
        );
        assert_eq!(top, 400.0 - gap - 200.0);
        assert_eq!(available, 400.0 - gap - 8.0);
    }
    for anchor in [(-40.0, 10.0, 50.0, 86.0), (300.0, 350.0, 50.0, 86.0)] {
        let (left, _, _): (f64, f64, f64) =
            dropdown_popup_position(anchor, (128.0, 120.0), (320.0, 568.0), 4.0);
        assert_eq!(left, if anchor.0 < 0.0 { 8.0 } else { 184.0 });
    }
}

#[test]
fn dropdown_popup_placement_scrolls_the_larger_side_without_shrinking_rows() {
    let (_, top, available): (f64, f64, f64) = dropdown_popup_position(
        (20.0, 120.0, 300.0, 336.0),
        (128.0, 1000.0),
        (320.0, 568.0),
        8.0,
    );
    assert_eq!((top, available), (8.0, 284.0));
    let (_, top, available): (f64, f64, f64) = dropdown_popup_position(
        (20.0, 120.0, 150.0, 186.0),
        (128.0, 1000.0),
        (320.0, 568.0),
        8.0,
    );
    assert_eq!((top, available), (194.0, 366.0));
    for anchor in [(0.0, 20.0, -80.0, -44.0), (0.0, 20.0, 600.0, 636.0)] {
        let (left, top, available): (f64, f64, f64) =
            dropdown_popup_position(anchor, (304.0, 1000.0), (320.0, 568.0), 8.0);
        assert_eq!(left, 8.0);
        assert!(top >= 8.0 && top + available <= 560.0);
    }
}

#[test]
fn dropdown_both_public_apis_share_portal_and_managed_positioning() {
    let legacy: VirtualNode = euv_dropdown(props(EuvDropdownProps::default()));
    let mut tags: Vec<String> = Vec::new();
    collect_tags(&legacy, &mut tags);
    assert!(
        tags.iter()
            .any(|tag: &String| tag.contains("Portal") && tag.contains("c_app_root"))
    );
    let view: &str = include_str!("../../src/component/dropdown/view/fn.rs");
    let styles: &str = include_str!("../../src/style/class/overlay/fn.rs");
    assert!(view.contains("use_dropdown_position(open, panel_ref.clone())"));
    let compact: String = view.split_whitespace().collect();
    assert!(compact.contains("frame_mounted.get()&&open.get()"));
    assert!(view.contains("options.set_prevent_scroll(true)"));
    assert!(view.contains("content.set_scroll_top("));
    assert!(styles.contains("overscroll-behavior: \"contain\""));
}

#[test]
fn dropdown_mixed_rows_expose_clean_labels_and_are_outside_the_tab_sequence() {
    let action: VirtualNode = euv_dropdown_menu_item(props(EuvDropdownMenuItemProps {
        label: "Apply",
        shortcut: "Ctrl+A",
        ..EuvDropdownMenuItemProps::default()
    }));
    let checkbox: VirtualNode =
        euv_dropdown_menu_checkbox_item(props(EuvDropdownMenuCheckboxItemProps {
            label: "Toolbar",
            checked: Signal::create(true),
            ..EuvDropdownMenuCheckboxItemProps::default()
        }));
    let radio: VirtualNode = euv_dropdown_menu_radio_item(props(EuvDropdownMenuRadioItemProps {
        label: "Grid",
        value: "grid",
        selection: use_dropdown_menu_radio_group(None, "grid", None),
        ..EuvDropdownMenuRadioItemProps::default()
    }));
    for (row, label) in [(&action, "Apply"), (&checkbox, "Toolbar"), (&radio, "Grid")] {
        assert_eq!(root_attribute_value(row, "tabindex").as_deref(), Some("-1"));
        assert_eq!(
            root_attribute_value(row, "data-label").as_deref(),
            Some(label)
        );
    }
    assert!(has_attribute_value(&checkbox, "aria-hidden", "true"));
    assert!(has_class_name(
        &checkbox,
        "c_euv_dropdown_menu_checkbox_indicator"
    ));
    assert!(has_class_name(&action, "c_euv_dropdown_menu_shortcut"));
}

#[test]
fn the_upload_handlers_are_all_present() {
    let drag: Signal<bool> = Signal::create(false);
    assert!(on_upload_files_change(None).is_some());
    assert!(on_upload_drag_over(drag).is_some());
    assert!(
        on_upload_drag_leave(drag).is_some(),
        "without a drag-leave handler the drop zone stays highlighted forever"
    );
}

#[test]
fn an_upload_renders_its_drop_zone() {
    let rendered: VirtualNode = euv_upload(props(EuvUploadProps {
        accept: "image/*",
        multiple: Signal::create(false),
        files: Signal::create(Vec::new()),
        drag_active: Signal::create(false),
        on_files: None,
        id: "",
        label: "",
        disabled: Signal::create(false),
        input_type: "file",
        aria_label: "Upload files",
        picker_label: "Choose files",
        onchange: None,
    }));
    assert_ne!(
        root_tag(&rendered),
        "Empty",
        "an upload must render a drop target"
    );
}

#[test]
fn upload_files_appear_as_the_signal_grows() {
    let base: VirtualNode = euv_upload(props(EuvUploadProps {
        accept: "",
        multiple: Signal::create(true),
        files: Signal::create(Vec::new()),
        drag_active: Signal::create(false),
        on_files: None,
        id: "",
        label: "",
        disabled: Signal::create(false),
        input_type: "file",
        aria_label: "Upload files",
        picker_label: "Choose files",
        onchange: None,
    }));
    let with_file: VirtualNode = euv_upload(props(EuvUploadProps {
        accept: "",
        multiple: Signal::create(true),
        files: Signal::create(vec![EuvUploadFile {
            name: "a.png",
            size: "10 B",
            status: EuvUploadStatus::Done,
        }]),
        drag_active: Signal::create(false),
        on_files: None,
        id: "",
        label: "",
        disabled: Signal::create(false),
        input_type: "file",
        aria_label: "Upload files",
        picker_label: "Choose files",
        onchange: None,
    }));
    assert!(
        count_elements(&with_file) >= count_elements(&base),
        "a listed file must not shrink the tree: {} vs {}",
        count_elements(&with_file),
        count_elements(&base)
    );
}

#[test]
fn a_debug_tree_renders_its_label() {
    let rendered: VirtualNode = euv_debug(props(EuvDebugProps {
        label: "count",
        value: None,
        expanded: false,
    }));
    assert_ne!(root_tag(&rendered), "Empty");
}

#[test]
fn the_popover_toggle_and_radio_handlers_are_present() {
    assert!(on_popover_toggle(Signal::create(false)).is_some());
    assert!(on_radio_change(Signal::create(String::new()), "a").is_some());
}

#[test]
fn a_markdown_document_is_wrapped_in_an_article() {
    let rendered: VirtualNode = euv_markdown(props(EuvMarkdownProps { blocks: &MD_RULE }));
    let mut tags: Vec<String> = Vec::new();
    collect_tags(&rendered, &mut tags);
    assert_eq!(
        tags.first().map(String::as_str),
        Some("Element(\"article\")"),
        "markdown content belongs in an <article> landmark, got {tags:?}"
    );
}

#[test]
fn a_markdown_document_with_no_blocks_still_renders_its_wrapper() {
    let empty: VirtualNode = euv_markdown(props(EuvMarkdownProps { blocks: &[] }));
    let mut tags: Vec<String> = Vec::new();
    collect_tags(&empty, &mut tags);
    assert_eq!(
        tags.first().map(String::as_str),
        Some("Element(\"article\")"),
        "an empty document is still a document, got {tags:?}"
    );
}

#[test]
fn a_markdown_block_list_splices_its_blocks_into_a_fragment() {
    let one: VirtualNode = euv_markdown_blocks(&MD_RULE);
    let two: VirtualNode = euv_markdown_blocks(&[EuvMdBlock::Rule, EuvMdBlock::Rule]);
    assert!(
        count_elements(&two) >= count_elements(&one),
        "each block must occupy its own slot in the fragment: {} vs {}",
        count_elements(&two),
        count_elements(&one)
    );
    assert!(
        matches!(one, VirtualNode::Fragment(_)),
        "a list of blocks is a fragment, not a wrapper element, got {:?}",
        root_tag(&one)
    );
}

fn nav_item_node() -> VirtualNode<EuvNavItemProps> {
    props(EuvNavItemProps {
        route_signal: Signal::create(String::from("/")),
        icon: "/i.svg",
        label: "Home",
        target: "/",
        on_click: None,
        class: None,
    })
}

fn mobile_nav_item_node() -> VirtualNode<EuvMobileNavItemProps> {
    props(EuvMobileNavItemProps {
        route_signal: Signal::create(String::from("/")),
        drawer_open: Signal::create(false),
        icon: "/i.svg",
        label: "Home",
        target: "/",
        on_navigate: None,
    })
}

fn nav_items_node(count: usize) -> VirtualNode<EuvNavItemsProps> {
    static CONFIGS: [EuvNavItemConfig; 3] = [
        EuvNavItemConfig {
            icon: "/a.svg",
            label: "A",
            target: "/a",
        },
        EuvNavItemConfig {
            icon: "/b.svg",
            label: "B",
            target: "/b",
        },
        EuvNavItemConfig {
            icon: "/c.svg",
            label: "C",
            target: "/c",
        },
    ];
    props(EuvNavItemsProps {
        route_signal: Signal::create(String::from("/")),
        items: CONFIGS[..count].to_vec(),
        drawer_open: Some(Signal::create(false)),
        on_item_click: None,
    })
}

#[test]
fn a_nav_item_renders_a_containable_element() {
    let rendered: VirtualNode = euv_nav_item(nav_item_node());
    assert_ne!(
        root_tag(&rendered),
        "Empty",
        "a nav item must render something"
    );
}

#[test]
fn a_mobile_nav_item_renders_the_same_shell_as_a_desktop_one() {
    let desktop: VirtualNode = euv_nav_item(nav_item_node());
    let mobile: VirtualNode = euv_mobile_nav_item(mobile_nav_item_node());
    let mut desktop_tags: Vec<String> = Vec::new();
    let mut mobile_tags: Vec<String> = Vec::new();
    collect_tags(&desktop, &mut desktop_tags);
    collect_tags(&mobile, &mut mobile_tags);
    assert_eq!(
        desktop_tags.len(),
        mobile_tags.len(),
        "the two nav variants describe the same control, so their trees must \
         match in size: {desktop_tags:?} vs {mobile_tags:?}"
    );
}

#[test]
fn a_nav_item_with_an_explicit_class_differs_from_a_bare_one() {
    let bare: VirtualNode = euv_nav_item(nav_item_node());
    let styled: VirtualNode = euv_nav_item(props(EuvNavItemProps {
        route_signal: Signal::create(String::from("/")),
        icon: "/i.svg",
        label: "Home",
        target: "/",
        on_click: None,
        class: Some(Css::default()),
    }));
    assert_eq!(
        count_elements(&bare),
        count_elements(&styled),
        "an extra class must not change the structure, only the class attribute"
    );
}

#[test]
fn a_nav_items_list_renders_one_entry_per_config() {
    let one: VirtualNode = euv_nav_items(nav_items_node(1));
    let three: VirtualNode = euv_nav_items(nav_items_node(3));
    assert!(
        count_elements(&three) > count_elements(&one),
        "three configs must beat one: {} vs {}",
        count_elements(&three),
        count_elements(&one)
    );
}

#[test]
fn an_empty_nav_items_list_renders_a_container_nothing_in() {
    let empty: VirtualNode = euv_nav_items(nav_items_node(0));
    let full: VirtualNode = euv_nav_items(nav_items_node(3));
    assert!(
        count_elements(&full) > count_elements(&empty),
        "an empty list must not vanish — the container is what the layout styles"
    );
}

#[test]
fn a_hero_action_is_an_anchor_carrying_its_href() {
    let action: EuvHeroAction = EuvHeroAction {
        text: "Get started",
        link: "/start",
        primary: true,
    };
    let rendered: VirtualNode = euv_hero_action(props(EuvHeroActionProps { action }));
    let mut tags: Vec<String> = Vec::new();
    collect_tags(&rendered, &mut tags);
    assert_eq!(
        tags.first().map(String::as_str),
        Some("Element(\"a\")"),
        "a hero action navigates, so it is an anchor, got {tags:?}"
    );
    let text: String = format!("{rendered:?}");
    for attribute in ["href", "onclick", "class"] {
        assert!(
            text.contains(attribute),
            "a hero action without {attribute} cannot navigate or be styled: {text}"
        );
    }
}

#[test]
fn a_secondary_hero_action_shares_the_anchor_shape() {
    let primary: VirtualNode = euv_hero_action(props(EuvHeroActionProps {
        action: EuvHeroAction {
            text: "Go",
            link: "/go",
            primary: true,
        },
    }));
    let secondary: VirtualNode = euv_hero_action(props(EuvHeroActionProps {
        action: EuvHeroAction {
            text: "Later",
            link: "/later",
            primary: false,
        },
    }));
    let mut primary_tags: Vec<String> = Vec::new();
    let mut secondary_tags: Vec<String> = Vec::new();
    collect_tags(&primary, &mut primary_tags);
    collect_tags(&secondary, &mut secondary_tags);
    assert_eq!(
        primary_tags, secondary_tags,
        "primary only swaps a class, so both actions are the same anchor"
    );
}

fn route(path: &'static str) -> EuvRouteConfig {
    EuvRouteConfig {
        path,
        component: Rc::new(|| VirtualNode::Empty),
    }
}

fn marked_route(path: &'static str) -> EuvRouteConfig {
    EuvRouteConfig {
        path,
        component: Rc::new(|| VirtualNode::Element {
            tag: Tag::Element(Cow::Borrowed("section")),
            attributes: Vec::new(),
            children: Vec::new(),
            key: None,
            props: None,
        }),
    }
}

#[test]
fn a_page_router_always_wraps_its_children_in_a_container() {
    let rendered: VirtualNode = euv_page_router(props(EuvPageRouterProps {
        route_signal: Signal::create(String::from("/")),
        fallback: None,
    }));
    let mut tags: Vec<String> = Vec::new();
    collect_tags(&rendered, &mut tags);
    assert_eq!(
        tags.first().map(String::as_str),
        Some("Element(\"div\")"),
        "the router exists to wrap whatever the route matched, got {tags:?}"
    );
}

#[test]
fn a_page_router_keeps_the_children_it_was_given() {
    let with_child: VirtualNode<EuvPageRouterProps> = VirtualNode::Element {
        tag: Tag::Element(Cow::Borrowed("div")),
        attributes: Vec::new(),
        children: vec![VirtualNode::Element {
            tag: Tag::Element(Cow::Borrowed("section")),
            attributes: Vec::new(),
            children: Vec::new(),
            key: None,
            props: None,
        }],
        key: None,
        props: Some(Box::new(EuvPageRouterProps {
            route_signal: Signal::create(String::from("/")),
            fallback: None,
        })),
    };
    let mut tags: Vec<String> = Vec::new();
    collect_tags(&euv_page_router(with_child), &mut tags);
    assert_eq!(
        tags,
        vec![
            "Element(\"div\")".to_string(),
            "Element(\"section\")".to_string()
        ],
        "the wrapper must pass its children through untouched, got {tags:?}"
    );
}

#[test]
fn a_matching_route_renders_its_component() {
    let rendered: VirtualNode = euv_routes(props(EuvRoutesProps {
        route_signal: Signal::create(String::from("/a")),
        routes: vec![marked_route("/a"), marked_route("/b")],
        fallback: None,
    }));
    let mut tags: Vec<String> = Vec::new();
    collect_tags(&rendered, &mut tags);
    assert_eq!(
        tags,
        vec!["Element(\"section\")".to_string()],
        "the matched route's component is what must appear, got {tags:?}"
    );
}

#[test]
fn an_unmatched_route_renders_nothing_when_there_is_no_fallback() {
    let rendered: VirtualNode = euv_routes(props(EuvRoutesProps {
        route_signal: Signal::create(String::from("/zzz")),
        routes: vec![marked_route("/a"), marked_route("/b")],
        fallback: None,
    }));
    assert_eq!(
        root_tag(&rendered),
        "Empty",
        "a route table that matches nothing must not invent a page"
    );
}

#[test]
fn an_unmatched_route_falls_back_when_one_is_supplied() {
    let rendered: VirtualNode = euv_routes(props(EuvRoutesProps {
        route_signal: Signal::create(String::from("/zzz")),
        routes: vec![marked_route("/a")],
        fallback: Some(Rc::new(|| VirtualNode::Element {
            tag: Tag::Element(Cow::Borrowed("main")),
            attributes: Vec::new(),
            children: Vec::new(),
            key: None,
            props: None,
        })),
    }));
    let mut tags: Vec<String> = Vec::new();
    collect_tags(&rendered, &mut tags);
    assert_eq!(
        tags,
        vec!["Element(\"main\")".to_string()],
        "the fallback replaces the page when no route matches, got {tags:?}"
    );
}

#[test]
fn a_matched_route_wins_over_the_fallback() {
    let rendered: VirtualNode = euv_routes(props(EuvRoutesProps {
        route_signal: Signal::create(String::from("/a")),
        routes: vec![marked_route("/a")],
        fallback: Some(Rc::new(|| VirtualNode::Element {
            tag: Tag::Element(Cow::Borrowed("main")),
            attributes: Vec::new(),
            children: Vec::new(),
            key: None,
            props: None,
        })),
    }));
    let mut tags: Vec<String> = Vec::new();
    collect_tags(&rendered, &mut tags);
    assert_eq!(
        tags,
        vec!["Element(\"section\")".to_string()],
        "the fallback is for misses only, never a shadow, got {tags:?}"
    );
}

#[test]
fn a_route_config_holds_the_path_and_component_it_was_built_with() {
    let config: EuvRouteConfig = route("/docs");
    assert_eq!(
        config.path, "/docs",
        "the path is matched against the route signal"
    );
    let rendered: VirtualNode = (config.component)();
    assert_eq!(
        root_tag(&rendered),
        "Empty",
        "the component closure must be callable and return a node"
    );
}

#[test]
fn a_virtual_list_config_holds_its_counts() {
    let config: EuvVirtualListConfig = EuvVirtualListConfig {
        id: String::from("feed"),
        total_count: 1000,
        item_height: 48,
        overscan_count: 4,
    };
    assert_eq!(config.id, "feed");
    assert_eq!(config.total_count, 1000);
    assert_eq!(config.item_height, 48);
    assert_eq!(config.overscan_count, 4);
}

#[test]
fn dropdown_checkbox_defaults_are_unchecked_enabled_and_keep_the_menu_open() {
    let settings: EuvDropdownMenuCheckboxItemProps = EuvDropdownMenuCheckboxItemProps::default();
    assert_eq!(settings.label, "");
    assert!(!settings.checked.get());
    assert!(!settings.disabled);
    assert!(!settings.open.get());
    assert!(!settings.close_on_click);
    assert!(settings.on_checked_change.is_none());
}

#[test]
fn dropdown_checkbox_is_a_native_button_with_checkbox_menu_semantics() {
    let node: VirtualNode =
        euv_dropdown_menu_checkbox_item(props(EuvDropdownMenuCheckboxItemProps {
            label: "Show grid",
            ..EuvDropdownMenuCheckboxItemProps::default()
        }));
    assert_eq!(root_tag(&node), "Element(\"button\")");
    assert_eq!(
        root_attribute_value(&node, "type").as_deref(),
        Some("button")
    );
    assert_eq!(
        root_attribute_value(&node, "role").as_deref(),
        Some("menuitemcheckbox")
    );
    assert_eq!(
        root_attribute_value(&node, "data-slot").as_deref(),
        Some("dropdown-menu-checkbox-item")
    );
    assert_eq!(
        root_attribute_value(&node, "aria-checked").as_deref(),
        Some("false")
    );
    assert_eq!(
        root_attribute_value(&node, "aria-disabled").as_deref(),
        Some("false")
    );
    assert!(has_attribute_name(&node, "onclick"));
    assert!(has_class_name(&node, "c_euv_dropdown_menu_checkbox_item"));
    assert_eq!(root_attribute_value(&node, "onclick"), None);
}

#[test]
fn dropdown_checkbox_disabled_rows_have_native_disabled_text_and_no_click_handler() {
    for initial_checked in [false, true] {
        for close_on_click in [false, true] {
            let checked: Signal<bool> = Signal::create(initial_checked);
            let open: Signal<bool> = Signal::create(true);
            let node: VirtualNode =
                euv_dropdown_menu_checkbox_item(props(EuvDropdownMenuCheckboxItemProps {
                    label: "Locked grid",
                    checked,
                    open,
                    disabled: true,
                    close_on_click,
                    on_checked_change: Some(Rc::new(|_: bool| panic!("disabled callback"))),
                }));
            assert_eq!(root_tag(&node), "Element(\"button\")");
            assert_eq!(
                root_attribute_value(&node, "disabled").as_deref(),
                Some("true")
            );
            assert_eq!(
                root_attribute_value(&node, "aria-disabled").as_deref(),
                Some("true")
            );
            assert_eq!(root_attribute_value(&node, "onclick").as_deref(), Some(""));
            assert_eq!(checked.get(), initial_checked);
            assert!(open.get());
        }
    }
}

#[test]
fn dropdown_checkbox_render_reads_the_shared_signal_without_mutation_or_callback() {
    for close_on_click in [false, true] {
        let checked: Signal<bool> = Signal::create(false);
        let open: Signal<bool> = Signal::create(true);
        let settings: EuvDropdownMenuCheckboxItemProps = EuvDropdownMenuCheckboxItemProps {
            checked,
            open,
            close_on_click,
            on_checked_change: Some(Rc::new(|_: bool| panic!("render callback"))),
            ..EuvDropdownMenuCheckboxItemProps::default()
        };
        for value in [false, true, false] {
            checked.set(value);
            let node: VirtualNode = euv_dropdown_menu_checkbox_item(props(settings.clone()));
            assert_eq!(
                root_attribute_value(&node, "aria-checked").as_deref(),
                Some(if value { "true" } else { "false" })
            );
            assert!(has_attribute_name(&node, "onclick"));
            assert_eq!(checked.get(), value);
            assert!(open.get());
        }
    }
}

fn dropdown_radio_attribute(
    settings: &EuvDropdownMenuRadioItemProps,
    name: &str,
) -> Option<String> {
    let node: VirtualNode = euv_dropdown_menu_radio_item(props(settings.clone()));
    root_attribute_value(&node, name)
}

#[test]
fn dropdown_disabled_radio_uses_the_shared_disabled_row_style() {
    let node: VirtualNode = euv_dropdown_menu_radio_item(props(EuvDropdownMenuRadioItemProps {
        disabled: true,
        ..EuvDropdownMenuRadioItemProps::default()
    }));
    assert!(has_class_name(&node, "c_euv_dropdown_menu_item_disabled"));
}

#[test]
fn dropdown_radio_group_excludes_siblings_preserves_other_groups_and_reports_changes_once() {
    let changes: Rc<RefCell<Vec<String>>> = Rc::new(RefCell::new(Vec::new()));
    let recorded: Rc<RefCell<Vec<String>>> = changes.clone();
    let selection: EuvDropdownMenuRadioGroupState = use_dropdown_menu_radio_group(
        None,
        "grid",
        Some(Rc::new(move |value: String| {
            recorded.borrow_mut().push(value)
        })),
    );
    let other: EuvDropdownMenuRadioGroupState = use_dropdown_menu_radio_group(None, "other", None);
    let first: EuvDropdownMenuRadioItemProps = EuvDropdownMenuRadioItemProps {
        value: "grid",
        selection: selection.clone(),
        open: Signal::create(true),
        ..EuvDropdownMenuRadioItemProps::default()
    };
    let second: EuvDropdownMenuRadioItemProps = EuvDropdownMenuRadioItemProps {
        value: "list",
        ..first.clone()
    };
    assert_eq!(
        dropdown_radio_attribute(&first, "aria-checked").as_deref(),
        Some("true")
    );
    assert_eq!(
        dropdown_radio_attribute(&second, "aria-checked").as_deref(),
        Some("false")
    );
    assert!(!activate_dropdown_menu_radio(&second));
    assert!(!activate_dropdown_menu_radio(&second));
    assert_eq!(selection.value.get(), "list");
    assert_eq!(other.value.get(), "other");
    assert_eq!(changes.borrow().as_slice(), &["list"]);
    assert_eq!(
        dropdown_radio_attribute(&first, "aria-checked").as_deref(),
        Some("false")
    );
    assert_eq!(
        dropdown_radio_attribute(&second, "aria-checked").as_deref(),
        Some("true")
    );
    assert!(first.open.get());
    first.open.set(false);
    first.open.set(true);
    assert_eq!(
        dropdown_radio_attribute(&second, "aria-checked").as_deref(),
        Some("true")
    );
}

#[test]
fn dropdown_radio_disabled_activation_neither_changes_value_nor_closes_the_menu() {
    let props: EuvDropdownMenuRadioItemProps = EuvDropdownMenuRadioItemProps {
        value: "disabled",
        disabled: true,
        close_on_click: true,
        open: Signal::create(true),
        selection: use_dropdown_menu_radio_group(
            None,
            "grid",
            Some(Rc::new(|_: String| panic!("disabled callback"))),
        ),
        ..EuvDropdownMenuRadioItemProps::default()
    };
    assert!(!activate_dropdown_menu_radio(&props));
    assert_eq!(props.selection.value.get(), "grid");
    assert!(props.open.get());
    assert_eq!(
        dropdown_radio_attribute(&props, "aria-disabled").as_deref(),
        Some("true")
    );
    assert_eq!(
        dropdown_radio_attribute(&props, "tabindex").as_deref(),
        Some("-1")
    );
}

#[test]
fn dropdown_radio_opt_in_close_also_closes_an_already_selected_choice_without_callback() {
    let props: EuvDropdownMenuRadioItemProps = EuvDropdownMenuRadioItemProps {
        value: "grid",
        close_on_click: true,
        open: Signal::create(true),
        selection: use_dropdown_menu_radio_group(
            None,
            "grid",
            Some(Rc::new(|_: String| panic!("unchanged callback"))),
        ),
        ..EuvDropdownMenuRadioItemProps::default()
    };
    assert!(!EuvDropdownMenuRadioItemProps::default().close_on_click);
    assert!(activate_dropdown_menu_radio(&props));
    assert!(!props.open.get());
    assert_eq!(props.selection.value.get(), "grid");
}

#[test]
fn dropdown_controlled_radio_requests_changes_but_waits_for_the_owner_to_update_its_signal() {
    let value: Signal<String> = Signal::create("grid".to_owned());
    let changes: Rc<RefCell<Vec<String>>> = Rc::new(RefCell::new(Vec::new()));
    let recorded: Rc<RefCell<Vec<String>>> = changes.clone();
    let selection: EuvDropdownMenuRadioGroupState = use_dropdown_menu_radio_group(
        Some(value),
        "ignored",
        Some(Rc::new(move |next: String| {
            recorded.borrow_mut().push(next)
        })),
    );
    let props: EuvDropdownMenuRadioItemProps = EuvDropdownMenuRadioItemProps {
        value: "list",
        selection,
        ..EuvDropdownMenuRadioItemProps::default()
    };
    assert!(!activate_dropdown_menu_radio(&props));
    assert_eq!(value.get(), "grid");
    assert_eq!(changes.borrow().as_slice(), &["list"]);
    value.set("list".to_owned());
    assert_eq!(
        dropdown_radio_attribute(&props, "aria-checked").as_deref(),
        Some("true")
    );
    assert!(!activate_dropdown_menu_radio(&props));
    assert_eq!(changes.borrow().len(), 1);
}

#[test]
fn dropdown_radio_defaults_and_native_text_attributes_match_the_selection_model() {
    let defaults: EuvDropdownMenuRadioGroupState = EuvDropdownMenuRadioGroupState::default();
    assert!(defaults.value.get().is_empty());
    assert!(!defaults.controlled);
    assert!(defaults.on_value_change.is_none());
    let default_item: EuvDropdownMenuRadioItemProps = EuvDropdownMenuRadioItemProps::default();
    assert!(!default_item.disabled);
    assert!(!default_item.close_on_click);
    assert!(!default_item.open.get());
    let selection: EuvDropdownMenuRadioGroupState =
        use_dropdown_menu_radio_group(None, "grid", None);
    let group: VirtualNode = euv_dropdown_menu_radio_group(props(EuvDropdownMenuRadioGroupProps {
        label: "View",
        selection: selection.clone(),
    }));
    assert_eq!(
        root_attribute_value(&group, "role").as_deref(),
        Some("group")
    );
    assert_eq!(
        root_attribute_value(&group, "data-value").as_deref(),
        Some("grid")
    );
    for disabled in [false, true] {
        for value in ["grid", "list"] {
            let row: VirtualNode =
                euv_dropdown_menu_radio_item(props(EuvDropdownMenuRadioItemProps {
                    label: "View",
                    value,
                    selection: selection.clone(),
                    disabled,
                    ..EuvDropdownMenuRadioItemProps::default()
                }));
            assert_eq!(root_tag(&row), "Element(\"button\")");
            assert_eq!(
                root_attribute_value(&row, "role").as_deref(),
                Some("menuitemradio")
            );
            assert_eq!(
                root_attribute_value(&row, "type").as_deref(),
                Some("button")
            );
            assert_eq!(
                root_attribute_value(&row, "disabled").as_deref(),
                Some(if disabled { "true" } else { "false" })
            );
            assert_eq!(
                root_attribute_value(&row, "aria-disabled").as_deref(),
                Some(if disabled { "true" } else { "false" })
            );
            assert_eq!(
                root_attribute_value(&row, "aria-checked").as_deref(),
                Some(if value == "grid" { "true" } else { "false" })
            );
            assert!(has_class_name(&row, "c_euv_dropdown_menu_radio_item"));
            assert!(has_class_name(
                &row,
                "c_euv_dropdown_menu_checkbox_indicator"
            ));
            assert!(has_attribute_value(&row, "aria-hidden", "true"));
            if disabled {
                assert_eq!(root_attribute_value(&row, "onclick").as_deref(), Some(""));
            }
            let source: &str = include_str!("../../src/component/dropdown/view/fn.rs");
            assert!(source.contains("if selection.value.get() == value {"));
            assert!(source.contains("circle {"));
            assert!(source.contains(r#"fill: "currentColor""#));
        }
    }
}

#[test]
fn dropdown_radio_activation_reads_fresh_signals_and_updates_before_callback() {
    let value: Signal<String> = Signal::create("grid".to_owned());
    let calls: Signal<u32> = Signal::create(0);
    let settings: EuvDropdownMenuRadioItemProps = EuvDropdownMenuRadioItemProps {
        value: "list",
        selection: EuvDropdownMenuRadioGroupState {
            value,
            controlled: false,
            on_value_change: Some(Rc::new(move |next: String| {
                assert_eq!(value.get(), next);
                calls.set(calls.get() + 1);
            })),
        },
        ..EuvDropdownMenuRadioItemProps::default()
    };
    assert!(!activate_dropdown_menu_radio(&settings));
    assert!(!activate_dropdown_menu_radio(&settings));
    assert_eq!(calls.get(), 1);
    value.set("grid".to_owned());
    assert!(!activate_dropdown_menu_radio(&settings));
    assert_eq!(calls.get(), 2);
    assert_eq!(value.get(), "list");
}

#[test]
fn dropdown_submenu_pairs_stable_ids_visibility_and_native_disabled_triggers() {
    let open: Signal<bool> = Signal::create(false);
    let trigger: VirtualNode =
        euv_dropdown_menu_sub_trigger(props(EuvDropdownMenuSubTriggerProps {
            label: "Share document",
            open,
            disabled: false,
            ..EuvDropdownMenuSubTriggerProps::default()
        }));
    let trigger_id: String = root_attribute_value(&trigger, "id").unwrap();
    let content_id: String = root_attribute_value(&trigger, "aria-controls").unwrap();
    assert!(has_attribute_value(
        &trigger,
        "data-slot",
        "dropdown-menu-sub-caret"
    ));
    assert!(has_attribute_value(&trigger, "aria-hidden", "true"));
    assert!(crate::DROPDOWN_SUB_CARET_MARKUP.contains("xmlns=\"http://www.w3.org/2000/svg\""));
    assert!(crate::DROPDOWN_SUB_CARET_MARKUP.contains("focusable=\"false\""));
    assert_eq!(root_tag(&trigger), "Element(\"button\")");
    assert!(has_attribute_value(
        &trigger,
        "data-label",
        "Share document"
    ));
    assert!(has_attribute_value(&trigger, "tabindex", "-1"));
    assert!(has_attribute_value(&trigger, "aria-expanded", "false"));
    assert!(has_attribute_name(&trigger, "onkeydown"));
    for is_open in [false, true] {
        open.set(is_open);
        let panel: VirtualNode =
            euv_dropdown_menu_sub_content(props(EuvDropdownMenuSubContentProps { open }));
        assert!(has_attribute_value(&panel, "id", &content_id));
        assert!(has_attribute_value(&panel, "aria-labelledby", &trigger_id));
        assert!(has_attribute_value(
            &panel,
            "data-slot",
            "dropdown-menu-sub-content"
        ));
        assert!(has_attribute_value(&panel, "role", "menu"));
        assert!(has_class_name(&panel, "c_euv_dropdown_sub_panel"));
        assert!(has_class_name(
            &panel,
            if is_open {
                "c_euv_dropdown_menu_open"
            } else {
                "c_euv_dropdown_menu_closed"
            }
        ));
        assert!(has_attribute_name(&panel, "onkeydown"));
    }
    let disabled: VirtualNode =
        euv_dropdown_menu_sub_trigger(props(EuvDropdownMenuSubTriggerProps {
            label: "Unavailable",
            disabled: true,
            ..EuvDropdownMenuSubTriggerProps::default()
        }));
    assert!(has_attribute_value(&disabled, "disabled", "true"));
    assert!(has_attribute_value(&disabled, "aria-disabled", "true"));
    assert_eq!(
        root_attribute_value(&disabled, "onclick").as_deref(),
        Some("")
    );
    let other: VirtualNode =
        euv_dropdown_menu_sub_trigger(props(EuvDropdownMenuSubTriggerProps::default()));
    assert_ne!(root_attribute_value(&other, "id"), Some(trigger_id));
}

#[test]
fn dropdown_submenu_parent_close_recursively_resets_only_its_branch() {
    let root: Signal<bool> = Signal::create(true);
    let first: Signal<bool> = Signal::create(false);
    let second: Signal<bool> = Signal::create(false);
    let sibling: Signal<bool> = Signal::create(false);
    use_dropdown_menu_sub_binding(root, first);
    use_dropdown_menu_sub_binding(first, second);
    use_dropdown_menu_sub_binding(root, sibling);
    first.set(true);
    second.set(true);
    sibling.set(true);
    second.set(false);
    assert!(first.get() && root.get() && sibling.get());
    second.set(true);
    first.set(false);
    assert!(!second.get());
    assert!(root.get() && sibling.get());
    root.set(false);
    assert!(!first.get() && !second.get() && !sibling.get());
    second.set(true);
    assert!(!second.get());
    root.set(true);
    assert!(!first.get() && !second.get() && !sibling.get());
}

#[test]
fn dropdown_submenu_keys_follow_local_direction_not_collision_side() {
    assert_eq!(dropdown_direction_keys(false), ("ArrowRight", "ArrowLeft"));
    assert_eq!(dropdown_direction_keys(true), ("ArrowLeft", "ArrowRight"));
    let labels: Vec<String> = vec![
        "Email link".into(),
        "Copy share link".into(),
        "More destinations".into(),
    ];
    assert_eq!(dropdown_next_index(&labels, Some(0), "End"), Some(2));
    assert_eq!(dropdown_next_index(&labels, Some(2), "Home"), Some(0));
    assert_eq!(dropdown_next_index(&labels, Some(0), "c"), Some(1));
}

#[test]
fn dropdown_submenu_position_flips_clamps_and_limits_tall_panels() {
    assert_eq!(
        dropdown_sub_position((100.0, 220.0, 40.0), (140.0, 120.0), (800.0, 600.0), false),
        (220.0, 40.0, 552.0)
    );
    assert_eq!(
        dropdown_sub_position((650.0, 780.0, 560.0), (140.0, 120.0), (800.0, 600.0), false),
        (510.0, 472.0, 120.0)
    );
    assert_eq!(
        dropdown_sub_position((220.0, 340.0, 40.0), (140.0, 120.0), (800.0, 600.0), true),
        (80.0, 40.0, 552.0)
    );
    assert_eq!(
        dropdown_sub_position((8.0, 120.0, 40.0), (140.0, 120.0), (800.0, 600.0), true),
        (120.0, 40.0, 552.0)
    );
    assert_eq!(
        dropdown_sub_position((100.0, 220.0, 400.0), (140.0, 900.0), (800.0, 600.0), false),
        (220.0, 8.0, 584.0)
    );
    assert_eq!(
        dropdown_sub_position((0.0, 10.0, -50.0), (140.0, 120.0), (20.0, 20.0), false),
        (8.0, 8.0, 4.0)
    );
}

#[test]
fn dropdown_hover_defaults_and_configured_native_triggers_preserve_activation() {
    let defaults: EuvDropdownMenuSubTriggerProps = EuvDropdownMenuSubTriggerProps::default();
    assert!(defaults.open_on_hover);
    assert!(!defaults.disabled);
    assert_eq!((defaults.delay_ms, defaults.close_delay_ms), (100, 0));
    for (enabled, disabled, delay, close_delay) in [
        (false, false, 100, 0),
        (true, false, 300, 150),
        (true, true, 100, 0),
    ] {
        let trigger: VirtualNode =
            euv_dropdown_menu_sub_trigger(props(EuvDropdownMenuSubTriggerProps {
                open_on_hover: enabled,
                disabled,
                delay_ms: delay,
                close_delay_ms: close_delay,
                label: "Hover settings",
                ..EuvDropdownMenuSubTriggerProps::default()
            }));
        assert!(has_attribute_value(
            &trigger,
            "data-label",
            "Hover settings"
        ));
        assert!(has_attribute_name(&trigger, "onkeydown"));
        assert!(has_attribute_name(&trigger, "onclick"));
        assert!(has_attribute_value(&trigger, "aria-expanded", "false"));
    }
}

#[test]
fn dropdown_hover_eligibility_rejects_touch_disabled_hover_off_and_closed_parent() {
    assert!(dropdown_hover_eligible("mouse", true, false, true, true));
    assert!(dropdown_hover_eligible("pen", true, false, true, true));
    for pointer in ["touch", "", "unknown"] {
        assert!(!dropdown_hover_eligible(pointer, true, false, true, true));
    }
    for (hover, disabled, enabled, parent) in [
        (false, false, true, true),
        (true, true, true, true),
        (true, false, false, true),
        (true, false, true, false),
    ] {
        assert!(!dropdown_hover_eligible(
            "mouse", hover, disabled, enabled, parent
        ));
    }
}

#[test]
fn dropdown_hover_intents_cancel_early_entry_and_retain_descendants_and_bridges() {
    assert_eq!(
        dropdown_hover_intent(false, true, false, false, true),
        Some(true)
    );
    assert_eq!(
        dropdown_hover_intent(false, false, false, false, true),
        None
    );
    assert_eq!(
        dropdown_hover_intent(false, true, false, false, false),
        None
    );
    assert_eq!(dropdown_hover_intent(true, true, false, false, true), None);
    assert_eq!(dropdown_hover_intent(true, false, true, false, true), None);
    assert_eq!(dropdown_hover_intent(true, false, false, true, true), None);
    assert_eq!(
        dropdown_hover_intent(true, false, false, false, true),
        Some(false)
    );
    assert_eq!(
        dropdown_hover_intent(true, false, false, false, false),
        Some(false)
    );
}

#[test]
fn dropdown_hover_bridge_tracks_actual_side_flips_and_clamped_edges_without_leaking() {
    let row: (f64, f64, f64, f64) = (100.0, 100.0, 200.0, 130.0);
    for (panel, exit, inside, outside) in [
        (
            (208.0, 80.0, 350.0, 300.0),
            (190.0, 115.0),
            (204.0, 160.0),
            (204.0, 10.0),
        ),
        (
            (-50.0, 80.0, 92.0, 300.0),
            (110.0, 115.0),
            (96.0, 160.0),
            (96.0, 10.0),
        ),
        (
            (208.0, 8.0, 350.0, 108.0),
            (190.0, 115.0),
            (204.0, 90.0),
            (204.0, 140.0),
        ),
    ] {
        assert!(dropdown_hover_bridge(inside, exit, row, panel));
        assert!(!dropdown_hover_bridge(outside, exit, row, panel));
        assert!(!dropdown_hover_bridge((500.0, 150.0), exit, row, panel));
        assert!(!dropdown_hover_bridge((0.0, 500.0), exit, row, panel));
    }
    assert!(!dropdown_hover_bridge(
        (150.0, 115.0),
        (150.0, 115.0),
        row,
        (200.0, 0.0, 200.0, 0.0)
    ));
}

#[test]
fn dropdown_hover_focused_subtrigger_exit_closes_only_its_own_open_branch() {
    for rtl in [false, true] {
        assert!(dropdown_sub_trigger_exit(true, "Escape", rtl));
        assert!(!dropdown_sub_trigger_exit(false, "Escape", rtl));
        assert!(dropdown_sub_trigger_exit(
            true,
            dropdown_direction_keys(rtl).1,
            rtl
        ));
        assert!(!dropdown_sub_trigger_exit(
            true,
            dropdown_direction_keys(rtl).0,
            rtl
        ));
        assert!(!dropdown_sub_trigger_exit(true, "Tab", rtl));
    }
}
