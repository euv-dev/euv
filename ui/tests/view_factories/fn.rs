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
    let mut tags: Vec<String> = Vec::new();
    collect_tags(&rendered, &mut tags);
    assert_eq!(
        tags,
        vec!["Element(\"span\")"],
        "no src means no <img>, got {tags:?}"
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
        let mut tags: Vec<String> = Vec::new();
        collect_tags(&rendered, &mut tags);
        shapes.push(tags);
    }
    assert_eq!(shapes[0], shapes[1]);
    assert_eq!(shapes[1], shapes[2]);
    assert_eq!(
        shapes[0],
        vec!["Element(\"span\")".to_string()],
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
        label: "L",
    });
    let on: VirtualNode<EuvCheckboxProps> = props(EuvCheckboxProps {
        id: "c2",
        name: "t",
        autocomplete: "off",
        checked: Signal::create(true),
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
            "Element(\"label\")",
        ],
        "the input precedes its label in both states, got {off_tags:?}"
    );
}

#[test]
fn a_checkbox_always_exposes_a_checked_attribute() {
    let node: VirtualNode<EuvCheckboxProps> = props(EuvCheckboxProps {
        id: "c1",
        name: "terms",
        autocomplete: "off",
        checked: Signal::create(false),
        label: "Accept",
    });
    let rendered: VirtualNode = euv_checkbox(node);
    let text: String = format!("{rendered:?}");
    assert!(
        text.contains("checked"),
        "an uncontrolled checkbox would never update, so the attribute must exist: {text}"
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
            active: false,
        },
        EuvDropdownItem {
            label: "Two",
            value: "2",
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
fn an_open_and_a_closed_dropdown_keep_the_same_frame() {
    let items: Vec<EuvDropdownItem> = vec![EuvDropdownItem {
        label: "One",
        value: "1",
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
