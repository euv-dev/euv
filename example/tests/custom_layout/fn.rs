use super::*;

fn declaration(selector: &str, property: &str) -> Option<&'static str> {
    APP_CUSTOM_LAYOUT_CSS
        .split('}')
        .filter_map(|rule: &str| rule.rsplit_once('{'))
        .filter(|(selectors, _): &(&str, &str)| {
            selectors
                .split(',')
                .any(|value: &str| value.trim() == selector)
        })
        .flat_map(|(_, body): (&str, &str)| body.split(';'))
        .filter_map(|value: &str| value.split_once(':'))
        .find(|(name, _): &(&str, &str)| name.trim() == property)
        .map(|(_, value): (&str, &str)| value.trim())
}

#[test]
fn theme_actions_keep_a_nonshrinking_touch_target_and_intrinsic_glyphs() {
    assert_eq!(
        declaration("#app .c_nav_theme_toggle", "display"),
        Some("flex")
    );
    assert_eq!(
        declaration("#app .c_nav_theme_toggle", "justify-content"),
        Some("space-between")
    );
    for selector in ["#app .c_nav_theme_button", "#app .c_mobile_theme_button"] {
        assert_eq!(declaration(selector, "display"), Some("inline-flex"));
        assert_eq!(declaration(selector, "align-items"), Some("center"));
        assert_eq!(declaration(selector, "justify-content"), Some("center"));
        assert_eq!(declaration(selector, "flex-shrink"), Some("0"));
        assert_eq!(declaration(selector, "min-width"), Some("44px"));
        assert_eq!(declaration(selector, "min-height"), Some("44px"));
    }
    for selector in ["#app .c_theme_icon_sun", "#app .c_theme_icon_moon"] {
        for property in ["width", "height", "font-size"] {
            assert_eq!(
                declaration(selector, property),
                Some("var(--space-xl, 20px)")
            );
        }
        assert_eq!(declaration(selector, "display"), Some("inline-flex"));
        assert_eq!(declaration(selector, "line-height"), Some("1"));
        assert_eq!(declaration(selector, "color"), None);
    }
    assert_eq!(
        declaration("#app .c_theme_icon_sun::before", "content"),
        Some(r#""\2600\FE0E""#)
    );
    assert_eq!(
        declaration("#app .c_theme_icon_moon::before", "content"),
        Some(r#""\263E\FE0E""#)
    );
    for class in [
        c_nav_theme_button(),
        c_theme_icon_sun(),
        c_theme_icon_moon(),
    ] {
        assert!(class.get_style().is_empty());
        assert!(class.get_pseudo_rules().is_empty());
    }
}

fn descendants(node: &VirtualNode) -> Vec<&VirtualNode> {
    let mut result: Vec<&VirtualNode> = vec![node];
    for child in node.get_children() {
        result.extend(descendants(child));
    }
    result
}

fn node_attribute(node: &VirtualNode, name: &str) -> Option<String> {
    match node {
        VirtualNode::Element { attributes, .. } => attributes
            .iter()
            .find(|entry: &&AttributeEntry| entry.get_name() == name)
            .and_then(|entry: &AttributeEntry| entry.text_value()),
        _ => None,
    }
}

fn node_has_class(node: &VirtualNode, class: &str) -> bool {
    match node {
        VirtualNode::Element { attributes, .. } => {
            attributes.iter().any(|attribute: &AttributeEntry| {
                attribute.get_name() == "class"
                    && attribute
                        .class_names()
                        .iter()
                        .any(|name: &String| name == class)
            })
        }
        _ => false,
    }
}

#[test]
fn theme_action_vnodes_are_named_buttons_with_the_matching_decorative_icon() {
    for (theme, icon) in [("light", "c_theme_icon_moon"), ("dark", "c_theme_icon_sun")] {
        let desktop: VirtualNode = desktop_layout(VirtualNode::Element {
            tag: Tag::Element("div".into()),
            attributes: Vec::new(),
            children: Vec::new(),
            key: None,
            props: Some(Box::new(DesktopLayoutProps {
                route_signal: Signal::create("/header-test".to_string()),
                theme_signal: Signal::create(theme.to_string()),
                root_class_signal: Signal::create(String::new()),
                panel_open: Signal::create(false),
            })),
        });
        let mobile: VirtualNode = mobile_layout(VirtualNode::Element {
            tag: Tag::Element("div".into()),
            attributes: Vec::new(),
            children: Vec::new(),
            key: None,
            props: Some(Box::new(MobileLayoutProps {
                route_signal: Signal::create("/header-test".to_string()),
                theme_signal: Signal::create(theme.to_string()),
                root_class_signal: Signal::create(String::new()),
                panel_open: Signal::create(false),
                drawer_open: Signal::create(false),
            })),
        });
        for (node, class) in [
            (&desktop, "c_nav_theme_button"),
            (&mobile, "c_mobile_theme_button"),
        ] {
            let nodes: Vec<&VirtualNode> = descendants(node);
            let button: &VirtualNode = nodes
                .iter()
                .copied()
                .find(|entry: &&VirtualNode| node_has_class(entry, class))
                .expect("theme action");
            assert_eq!(button.try_get_tag_name().as_deref(), Some("button"));
            assert_eq!(
                node_attribute(button, "aria-label").as_deref(),
                Some(APP_THEME_TOGGLE_LABEL)
            );
            let children: Vec<&VirtualNode> = descendants(button);
            let glyph: &VirtualNode = children
                .iter()
                .copied()
                .find(|entry: &&VirtualNode| node_has_class(entry, icon))
                .expect("theme-specific icon");
            assert_eq!(
                node_attribute(glyph, "aria-hidden").as_deref(),
                Some("true")
            );
        }
    }
}

#[test]
fn console_shell_is_out_of_flow_and_closed_layers_are_noninteractive() {
    for (selector, layer) in [
        ("#app .c_vconsole_fab", "9999"),
        ("#app .c_vconsole_overlay", "10000"),
        ("#app .c_vconsole_panel", "10001"),
    ] {
        assert_eq!(declaration(selector, "position"), Some("fixed"));
        assert_eq!(declaration(selector, "z-index"), Some(layer));
    }
    assert_eq!(
        declaration("#app .c_vconsole_fab", "right"),
        Some("var(--edge-gutter, 28px)")
    );
    assert_eq!(
        declaration("#app .c_vconsole_fab", "bottom"),
        Some("calc(var(--edge-gutter-bottom, 20px) + var(--safe-area-inset-bottom, 0px))")
    );
    assert_eq!(declaration("#app .c_vconsole_overlay", "inset"), Some("0"));
    assert_eq!(
        declaration("#app .c_vconsole_panel", "inset"),
        Some("auto 0 0")
    );
    assert_eq!(
        declaration("#app .c_vconsole_panel", "height"),
        Some("calc(76% - var(--safe-area-inset-bottom, 0px))")
    );
    assert_eq!(
        declaration("#app .c_vconsole_panel", "display"),
        Some("flex")
    );
    assert_eq!(
        declaration("#app .c_vconsole_panel", "flex-direction"),
        Some("column")
    );
    assert_eq!(
        declaration("#app .c_vconsole_body", "min-height"),
        Some("0")
    );
    assert_eq!(
        declaration("#app .c_vconsole_body", "overflow-y"),
        Some("auto")
    );
    for selector in ["#app .c_euv_logo_fab", "#app .c_vconsole_close_button"] {
        assert_eq!(declaration(selector, "min-width"), Some("44px"));
        assert_eq!(declaration(selector, "min-height"), Some("44px"));
    }
    for selector in ["#app .c_vconsole_panel", "#app .c_vconsole_overlay"] {
        assert_eq!(declaration(selector, "visibility"), Some("visible"));
        assert_eq!(declaration(selector, "pointer-events"), Some("auto"));
    }
    for selector in [
        "#app .c_vconsole_panel_closed",
        "#app .c_vconsole_overlay_hidden",
    ] {
        assert_eq!(declaration(selector, "visibility"), Some("hidden"));
        assert_eq!(declaration(selector, "pointer-events"), Some("none"));
    }
}

#[test]
fn console_drawer_state_classes_match_the_consumer_visibility_contract() {
    for open in [false, true] {
        let node: VirtualNode = euv_vconsole_drawer(VirtualNode::Element {
            tag: Tag::Element("div".into()),
            attributes: Vec::new(),
            children: Vec::new(),
            key: None,
            props: Some(Box::new(EuvVconsoleDrawerProps {
                panel_open: Signal::create(open),
                console_signal: Signal::create(Vec::new()),
            })),
        });
        let nodes: Vec<&VirtualNode> = descendants(&node);
        for (base, closed) in [
            ("c_vconsole_overlay", "c_vconsole_overlay_hidden"),
            ("c_vconsole_panel", "c_vconsole_panel_closed"),
        ] {
            let layer: &VirtualNode = nodes
                .iter()
                .copied()
                .find(|entry: &&VirtualNode| node_has_class(entry, base))
                .expect("console layer");
            assert_eq!(node_has_class(layer, closed), !open);
        }
    }
}

#[test]
fn business_content_keeps_responsive_grids_and_natural_home_tail() {
    assert_eq!(declaration("#app .c_home_content", "min-width"), Some("0"));
    assert_eq!(declaration("#app .c_home_tail", "position"), None);
    assert_eq!(declaration("#app .c_home_tail", "margin-top"), Some("auto"));
    assert_eq!(
        declaration("#app .c_home_stats", "grid-template-columns"),
        Some("repeat(4, minmax(0, 1fr))")
    );
    assert_eq!(
        declaration("#app .c_home_feature_grid", "display"),
        Some("grid")
    );
    assert!(
        APP_CUSTOM_LAYOUT_CSS
            .contains("#app .c_home_feature_grid { grid-template-columns: minmax(0, 1fr); }")
    );
    assert_eq!(declaration("#app .c_euv_card", "display"), Some("flex"));
    assert_eq!(
        declaration("#app .c_button_controls", "flex-wrap"),
        Some("wrap")
    );
}

#[test]
fn desktop_shell_and_navigation_have_independent_scroll_regions() {
    assert_eq!(declaration("#app .c_app_root", "display"), Some("flex"));
    assert_eq!(declaration("#app .c_app_root", "height"), Some("100%"));
    assert_eq!(
        declaration("#app .c_app_nav", "flex"),
        Some("0 0 var(--nav-width, 248px)")
    );
    for selector in [
        "#app .c_app_main",
        "#app .c_mobile_main",
        "#app .c_nav_items_scroll",
    ] {
        assert_eq!(declaration(selector, "min-height"), Some("0"));
        assert_eq!(declaration(selector, "overflow-y"), Some("auto"));
    }
    assert!(APP_CUSTOM_LAYOUT_CSS.contains("@media (max-width: 767px)"));
    assert!(APP_CUSTOM_LAYOUT_CSS.contains("#app .c_app_nav { display: none; }"));
}

#[test]
fn mobile_drawer_and_overlay_isolate_closed_pointer_and_focus_targets() {
    assert_eq!(
        declaration("#app .c_mobile_app_root", "flex-direction"),
        Some("column")
    );
    assert_eq!(declaration("#app .c_mobile_header", "z-index"), Some("100"));
    assert_eq!(
        declaration("#app .c_mobile_overlay", "z-index"),
        Some("200")
    );
    assert_eq!(
        declaration("#app .c_mobile_nav_drawer", "z-index"),
        Some("201")
    );
    for selector in [
        "#app .c_mobile_overlay_hidden",
        "#app .c_mobile_nav_drawer_closed",
    ] {
        assert_eq!(declaration(selector, "visibility"), Some("hidden"));
        assert_eq!(declaration(selector, "pointer-events"), Some("none"));
    }
    assert_eq!(
        declaration("#app .c_mobile_nav_drawer_closed", "transform"),
        Some("translateX(-100%)")
    );
    assert_eq!(
        declaration("#app .c_mobile_nav_drawer", "pointer-events"),
        Some("auto")
    );
    assert!(
        declaration("#app .c_mobile_header", "height")
            .expect("header height")
            .contains("--euv-mobile-safe-top")
    );
    assert_eq!(
        declaration("#app .c_mobile_nav_drawer", "padding-top"),
        Some("var(--euv-mobile-safe-top, 0px)")
    );
    assert!(!APP_CUSTOM_LAYOUT_CSS.contains("env(safe-area-inset-top)"));
}

#[test]
fn application_styles_only_target_explicit_classes() {
    for rule in APP_CUSTOM_LAYOUT_CSS.split('}') {
        let Some((selectors, _)) = rule.rsplit_once('{') else {
            continue;
        };
        for selector in selectors.split(',') {
            assert!(
                selector.contains(".c_"),
                "unscoped application selector: {selector}"
            );
        }
    }
}

#[test]
fn consumer_geometry_does_not_restore_a_library_skin() {
    for body in APP_CUSTOM_LAYOUT_CSS.split('{').skip(1) {
        for part in body.split('}').next().expect("rule body").split(';') {
            if let Some((property, _)) = part.split_once(':') {
                let name: &str = property.trim();
                assert!(
                    ![
                        "color",
                        "background",
                        "border",
                        "border-radius",
                        "box-shadow",
                        "animation",
                        "transition",
                        "font-family"
                    ]
                    .contains(&name),
                    "unexpected skin property: {name}"
                );
            }
        }
    }
    for class in [
        c_app_root(),
        c_app_nav(),
        c_app_main(),
        c_mobile_header(),
        c_mobile_nav_drawer(),
        c_mobile_overlay(),
        c_page_container(),
    ] {
        assert!(class.get_style().is_empty());
        assert!(class.get_pseudo_rules().is_empty());
    }
    let entry: &str = include_str!("../../src/lib.rs");
    assert!(entry.contains(
        "#[cfg(feature = \"theme-custom\")]\n    Css::inject_css(APP_CUSTOM_LAYOUT_CSS);"
    ));
    assert!(
        entry
            .find("Css::inject_css(APP_CUSTOM_LAYOUT_CSS)")
            .expect("custom injection")
            < entry.find("App::mount(").expect("mount")
    );
}

#[test]
fn portaled_select_is_hidden_when_closed_and_bounded_when_open() {
    assert_eq!(
        declaration(".c_euv_select_panel_closed", "display"),
        Some("none")
    );
    assert_eq!(
        declaration(".c_euv_select_panel_closed", "pointer-events"),
        Some("none")
    );
    assert_eq!(
        declaration(".c_euv_select_panel_open", "z-index"),
        Some("150")
    );
    assert_eq!(
        declaration(".c_euv_select_panel_open", "position"),
        Some("fixed")
    );
    assert_eq!(
        declaration(".c_euv_select_viewport", "overflow-y"),
        Some("auto")
    );
    assert_eq!(
        declaration("#app .c_euv_select_caret svg", "width"),
        Some("var(--space-lg, 16px)")
    );
}
