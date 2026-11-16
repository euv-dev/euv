use super::*;

fn tabs_fixture(active: Signal<String>) -> VirtualNode {
    tabs_activation_fixture(active, EuvTabsActivationMode::Automatic)
}

fn tabs_activation_fixture(
    active: Signal<String>,
    activation_mode: EuvTabsActivationMode,
) -> VirtualNode {
    euv_tabs(VirtualNode::Element {
        tag: Tag::Element(Cow::Borrowed("div")),
        attributes: Vec::new(),
        children: vec![html! {
        p {
            "Shared ordinary content"
        }
        }],
        key: None,
        props: Some(Box::new(EuvTabsProps {
            items: vec![
                EuvTabItem {
                    key: "account",
                    label: "Account",
                },
                EuvTabItem {
                    key: "password",
                    label: "Password",
                },
            ],
            active,
            aria_label: "Reference tabs",
            activation_mode,
        })),
    })
}

fn tabs_elements(node: &VirtualNode) -> Vec<VirtualNode> {
    let mut result: Vec<VirtualNode> = Vec::new();
    match node {
        VirtualNode::Element { children, .. } => {
            result.push(node.clone());
            for child in children {
                result.extend(tabs_elements(child));
            }
        }
        VirtualNode::Fragment(children) => {
            for child in children {
                result.extend(tabs_elements(child));
            }
        }
        _ => {}
    }
    result
}

fn tabs_attribute(node: &VirtualNode, name: &str) -> Option<String> {
    match node {
        VirtualNode::Element { attributes, .. } => attributes
            .iter()
            .find(|entry: &&AttributeEntry| entry.get_name() == name)
            .and_then(AttributeEntry::text_value),
        _ => None,
    }
}

fn tabs_role(node: &VirtualNode, role: &str) -> Vec<VirtualNode> {
    tabs_elements(node)
        .into_iter()
        .filter(|element: &VirtualNode| tabs_attribute(element, "role").as_deref() == Some(role))
        .collect()
}

fn tabs_ids(node: &VirtualNode) -> Vec<String> {
    tabs_elements(node)
        .iter()
        .filter_map(|element: &VirtualNode| tabs_attribute(element, "id"))
        .collect()
}

#[test]
fn tabs_controls_reference_one_focusable_panel_named_by_the_selected_trigger() {
    let rendered: VirtualNode = tabs_fixture(Signal::create(String::from("account")));
    let triggers: Vec<VirtualNode> = tabs_role(&rendered, "tab");
    let panels: Vec<VirtualNode> = tabs_role(&rendered, "tabpanel");
    assert_eq!(triggers.len(), 2);
    assert_eq!(panels.len(), 1);
    let panel_id: String = tabs_attribute(&panels[0], "id").expect("panel ID");
    assert!(!panel_id.is_empty());
    assert_eq!(tabs_attribute(&panels[0], "tabindex").as_deref(), Some("0"));
    assert_eq!(
        tabs_attribute(&panels[0], "aria-labelledby"),
        tabs_attribute(&triggers[0], "id")
    );
    for trigger in &triggers {
        assert_eq!(
            tabs_attribute(trigger, "aria-controls"),
            Some(panel_id.clone())
        );
        assert!(tabs_attribute(trigger, "id").is_some_and(|id: String| !id.is_empty()));
    }
    assert_eq!(tabs_ids(&rendered).len(), 3);
    assert_eq!(
        tabs_attribute(&triggers[0], "aria-selected").as_deref(),
        Some("true")
    );
    assert_eq!(
        tabs_attribute(&triggers[1], "aria-selected").as_deref(),
        Some("false")
    );
}

#[test]
fn tabs_ids_survive_rerender_and_external_selection_without_remounting_the_shared_panel() {
    let mut context: HookContext = HookContext::default();
    let active: Signal<String> = Signal::create(String::from("account"));
    let first: VirtualNode = HookContext::with(context.clone(), || tabs_fixture(active));
    context.reset_index();
    let same: VirtualNode = HookContext::with(context.clone(), || tabs_fixture(active));
    assert_eq!(tabs_ids(&first), tabs_ids(&same));
    active.set(String::from("password"));
    context.reset_index();
    let changed: VirtualNode = HookContext::with(context, || tabs_fixture(active));
    assert_eq!(tabs_ids(&first), tabs_ids(&changed));
    let triggers: Vec<VirtualNode> = tabs_role(&changed, "tab");
    let panels: Vec<VirtualNode> = tabs_role(&changed, "tabpanel");
    assert_eq!(
        tabs_attribute(&panels[0], "aria-labelledby"),
        tabs_attribute(&triggers[1], "id")
    );
    assert_eq!(
        tabs_attribute(&triggers[0], "tabindex").as_deref(),
        Some("-1")
    );
    assert_eq!(
        tabs_attribute(&triggers[1], "tabindex").as_deref(),
        Some("0")
    );
    assert_eq!(
        tabs_attribute(&triggers[1], "aria-selected").as_deref(),
        Some("true")
    );
    assert!(format!("{changed:?}").contains("Shared ordinary content"));
}

#[test]
fn tabs_roots_with_identical_item_keys_have_disjoint_ids() {
    let active: Signal<String> = Signal::create(String::from("account"));
    let first: VirtualNode = HookContext::with(HookContext::default(), || tabs_fixture(active));
    let second: VirtualNode = HookContext::with(HookContext::default(), || tabs_fixture(active));
    let first_ids: HashSet<String> = tabs_ids(&first).into_iter().collect();
    let second_ids: HashSet<String> = tabs_ids(&second).into_iter().collect();
    assert_eq!(first_ids.len(), 3);
    assert_eq!(second_ids.len(), 3);
    assert!(first_ids.is_disjoint(&second_ids));
}

#[test]
fn tabs_keep_native_button_activation_and_keyboard_handlers() {
    let rendered: VirtualNode = tabs_fixture(Signal::create(String::from("account")));
    for trigger in tabs_role(&rendered, "tab") {
        assert_eq!(tabs_attribute(&trigger, "type").as_deref(), Some("button"));
        if let VirtualNode::Element {
            attributes, tag, ..
        } = trigger
        {
            assert_eq!(tag, Tag::Element(Cow::Borrowed("button")));
            for name in ["onclick", "onkeydown"] {
                assert!(
                    attributes
                        .iter()
                        .any(|entry: &AttributeEntry| entry.get_name() == name)
                );
            }
        }
    }
}

#[cfg(not(feature = "theme-custom"))]
fn tabs_style_value(class: &Css, property: &str) -> Option<String> {
    class
        .get_style()
        .split(';')
        .filter_map(|part: &str| part.split_once(':'))
        .filter(|(name, _): &(&str, &str)| name.trim() == property)
        .map(|(_, value): (&str, &str)| value.trim().to_string())
        .next_back()
}

#[cfg(not(feature = "theme-custom"))]
#[test]
fn tabs_shadcn_factories_inherit_before_overriding_selected_fill_and_line_height() {
    assert_eq!(
        tabs_style_value(c_euv_tab_item(), "background").as_deref(),
        Some("transparent")
    );
    if cfg!(feature = "theme-shadcn") {
        assert_eq!(
            tabs_style_value(c_euv_tab_item_active(), "background").as_deref(),
            Some("var(--background)")
        );
        for class in [c_euv_tab_item(), c_euv_tab_item_active()] {
            assert_eq!(
                tabs_style_value(class, "line-height").as_deref(),
                Some("var(--space-xl)")
            );
            assert_eq!(
                tabs_style_value(class, "touch-action").as_deref(),
                Some("manipulation")
            );
            assert_eq!(
                tabs_style_value(class, "height").as_deref(),
                Some("calc(100% - 1px)")
            );
        }
    } else {
        assert_eq!(tabs_style_value(c_euv_tab_item(), "line-height"), None);
        assert_eq!(
            tabs_style_value(c_euv_tab_item_active(), "background").as_deref(),
            Some("var(--primary)")
        );
    }
    assert_eq!(
        tabs_style_value(c_euv_tabs_bar(), "height").as_deref(),
        if cfg!(feature = "theme-shadcn") {
            Some("var(--control-height-sm)")
        } else {
            None
        }
    );
    for (class, name) in [
        (c_euv_tabs(), "c_euv_tabs"),
        (c_euv_tabs_bar(), "c_euv_tabs_bar"),
        (c_euv_tabs_panel(), "c_euv_tabs_panel"),
    ] {
        assert_eq!(class.get_name(), name);
        assert!(!class.get_style().is_empty());
    }
}

#[cfg(not(feature = "theme-custom"))]
#[test]
fn tabs_shadcn_paint_keeps_transparent_borders_and_theme_scoped_dark_overrides() {
    let inactive: &Css = c_euv_tab_item();
    let selected: &Css = c_euv_tab_item_active();
    if cfg!(feature = "theme-shadcn") {
        assert_eq!(
            tabs_style_value(inactive, "color").as_deref(),
            Some("color-mix(in srgb, var(--foreground) 60%, transparent)")
        );
        assert_eq!(
            tabs_style_value(inactive, "border-color").as_deref(),
            Some("transparent")
        );
        assert_eq!(
            tabs_style_value(selected, "box-shadow").as_deref(),
            Some("var(--shadow-sm)")
        );
        let inactive_dark: String = format!("{:?}", inactive.get_pseudo_rules());
        let selected_dark: String = format!("{:?}", selected.get_pseudo_rules());
        assert!(
            inactive_dark.contains(":is(.c_theme_dark button)"),
            "{inactive_dark}"
        );
        assert!(inactive_dark.contains("var(--muted-foreground)"));
        assert!(inactive_dark.contains(":is(.c_theme_dark button):hover"));
        assert!(selected_dark.contains("color-mix(in srgb, var(--input) 30%, transparent)"));
        assert!(selected_dark.contains("border-color: var(--input)"));
    } else {
        assert!(!format!("{:?}", c_euv_tab_item().get_pseudo_rules()).contains("c_theme_dark"));
    }
}

#[test]
fn tabs_render_only_the_feature_selected_trigger_strategy() {
    let rendered: VirtualNode = tabs_fixture(Signal::create(String::from("account")));
    let triggers: Vec<VirtualNode> = tabs_role(&rendered, "tab");
    let classes: Vec<String> = triggers
        .iter()
        .map(|node: &VirtualNode| {
            if let VirtualNode::Element { attributes, .. } = node {
                attributes
                    .iter()
                    .filter(|entry: &&AttributeEntry| entry.get_name() == "class")
                    .flat_map(AttributeEntry::class_names)
                    .collect::<BTreeSet<String>>()
                    .into_iter()
                    .collect::<Vec<String>>()
                    .join(" ")
            } else {
                String::new()
            }
        })
        .collect();
    assert_eq!(classes[0], "c_euv_tab_item c_euv_tab_item_active");
    assert_eq!(classes[1], "c_euv_tab_item");
    if cfg!(feature = "theme-custom") {
        assert!(c_euv_tab_item().get_style().is_empty());
        assert!(c_euv_tab_item_active().get_style().is_empty());
    }
}

#[test]
fn tabs_activation_defaults_to_automatic() {
    assert_eq!(
        EuvTabsActivationMode::default(),
        EuvTabsActivationMode::Automatic
    );
    assert_eq!(
        EuvTabsProps::default().activation_mode,
        EuvTabsActivationMode::Automatic
    );
}

#[test]
fn tabs_manual_keeps_selected_panel_relationships_and_external_roving_entrance() {
    let mut context: HookContext = HookContext::default();
    let active: Signal<String> = Signal::create(String::from("account"));
    let first: VirtualNode = HookContext::with(context.clone(), || {
        tabs_activation_fixture(active, EuvTabsActivationMode::Manual)
    });
    let first_ids: Vec<String> = tabs_ids(&first);
    for selected in ["password", "account"] {
        active.set(String::from(selected));
        context.reset_index();
        let changed: VirtualNode = HookContext::with(context.clone(), || {
            tabs_activation_fixture(active, EuvTabsActivationMode::Manual)
        });
        assert_eq!(first_ids, tabs_ids(&changed));
        let triggers: Vec<VirtualNode> = tabs_role(&changed, "tab");
        let panels: Vec<VirtualNode> = tabs_role(&changed, "tabpanel");
        let index: usize = usize::from(selected == "password");
        assert_eq!(
            tabs_attribute(&panels[0], "aria-labelledby"),
            tabs_attribute(&triggers[index], "id")
        );
        assert_eq!(tabs_attribute(&panels[0], "tabindex").as_deref(), Some("0"));
        for (position, trigger) in triggers.iter().enumerate() {
            assert_eq!(
                tabs_attribute(trigger, "aria-selected").as_deref(),
                Some(if position == index { "true" } else { "false" })
            );
            assert_eq!(
                tabs_attribute(trigger, "tabindex").as_deref(),
                Some(if position == index { "0" } else { "-1" })
            );
        }
    }
}

#[test]
fn tabs_manual_missing_selection_has_one_usable_roving_entrance_without_selecting() {
    let active: Signal<String> = Signal::create(String::from("missing"));
    let rendered: VirtualNode = tabs_activation_fixture(active, EuvTabsActivationMode::Manual);
    let triggers: Vec<VirtualNode> = tabs_role(&rendered, "tab");
    assert_eq!(active.get(), "missing");
    assert_eq!(
        tabs_attribute(&triggers[0], "tabindex").as_deref(),
        Some("0")
    );
    assert_eq!(
        tabs_attribute(&triggers[1], "tabindex").as_deref(),
        Some("-1")
    );
    for trigger in triggers {
        assert_eq!(
            tabs_attribute(&trigger, "aria-selected").as_deref(),
            Some("false")
        );
    }
}

#[test]
fn tabs_manual_focus_and_key_phases_do_not_implicitly_select_or_native_double_click() {
    let source: &str = include_str!("../../src/component/tabs/view/fn.rs");
    let focus_handler: &str = source
        .split("fn track_tab_focus(")
        .nth(1)
        .expect("focus handler")
        .split("fn prevent_tab_activation_default(")
        .next()
        .expect("focus body");
    assert!(focus_handler.contains("focus.set((active.get(), key.to_string()))"));
    assert!(!focus_handler.contains("active.set("));
    let key_handler: &str = source
        .split("fn select_tab_by_keyboard(")
        .nth(1)
        .expect("keyboard handler")
        .split("pub(super) fn tab_index_for_key(")
        .next()
        .expect("keyboard body");
    let activation: &str = key_handler
        .split("if activation_mode == EuvTabsActivationMode::Manual")
        .nth(1)
        .expect("manual activation")
        .split("let rtl:")
        .next()
        .expect("activation body");
    assert!(activation.contains("TAB_KEY_ENTER | TAB_KEY_SPACE"));
    assert!(activation.contains("keyboard_event.prevent_default()"));
    assert!(activation.contains("if !keyboard_event.repeat()"));
    assert_eq!(activation.matches("current_tab.click()").count(), 1);
    assert!(!activation.contains("active.set("));
    assert!(key_handler.contains("if activation_mode == EuvTabsActivationMode::Automatic"));
    let keyup: &str = source
        .split("fn prevent_tab_activation_default(")
        .nth(1)
        .expect("key-up handler");
    assert!(keyup.contains("keyboard_event.prevent_default()"));
    assert!(!keyup.contains(".click()"));
    assert!(!keyup.contains("active.set("));
}
