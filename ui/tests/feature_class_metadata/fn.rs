use super::*;

#[cfg(feature = "theme-shadcn")]
#[test]
fn selected_select_rules_keep_attribute_conditions_and_real_descendant_selectors() {
    for (css, expected) in [
        (
            c_euv_native_select(),
            concat!(
                ":disabled{pointer-events: none; cursor: not-allowed; }",
                ":focus-visible{border-color: var(--ring); }",
                ":is([aria-invalid=true]){border-color: var(--destructive); }",
                ":is(.c_theme_dark .c_euv_native_select){background-color: color-mix(in srgb, var(--input) 30%, transparent); }",
                ":is(.c_theme_dark .c_euv_native_select):hover{background-color: color-mix(in srgb, var(--input) 50%, transparent); }",
            ),
        ),
        (
            c_euv_select_trigger(),
            concat!(
                ":hover{border-color: var(--ring); background: var(--muted); }",
                ":focus{outline: none; border-color: var(--ring); }",
                ":focus-visible{border-color: var(--ring); }",
                ":disabled{cursor: not-allowed; opacity: 0.5; }",
                ":is([data-size=sm]){height: var(--control-height-sm); min-height: var(--control-height-sm); padding-block: var(--space-xs); }",
                ":is(.c_theme_dark .c_euv_select_trigger){background-color: color-mix(in srgb, var(--input) 30%, transparent); }",
                ":is(.c_theme_dark .c_euv_select_trigger):hover{background-color: color-mix(in srgb, var(--input) 50%, transparent); }",
            ),
        ),
        (
            c_euv_select_option_indicator(),
            ":is(.c_euv_select_option[aria-selected=\"false\"] .c_euv_select_option_indicator){visibility: hidden; }",
        ),
    ] {
        assert_eq!(
            css.get_pseudo_rules(),
            &Css::parse_pseudo_rules(expected.replace(CSS_BLOCK_OPEN, " { ")),
            "{}: full ordered native selector and declaration syntax",
            css.get_name()
        );
    }
}

#[cfg(feature = "theme-shadcn")]
#[test]
fn mounted_tabs_and_checkbox_states_keep_ordered_state_paint_above_the_base() {
    assert_eq!(
        class_property(c_euv_tab_item(), "line-height").as_deref(),
        Some("var(--space-xl)")
    );
    assert_eq!(
        class_property(c_euv_tab_item_active(), "background").as_deref(),
        Some("var(--background)")
    );
    assert_eq!(
        class_property(c_euv_tab_item_active(), "box-shadow").as_deref(),
        Some("var(--shadow-sm)")
    );
    let expected: Vec<PseudoRule> = Css::parse_pseudo_rules(concat!(
        ":hover{background: var(--muted); color: var(--primary); }",
        ":hover{background: transparent; color: var(--foreground); }",
        ":is(.c_theme_dark button){color: var(--muted-foreground); }",
        ":is(.c_theme_dark button):hover{color: var(--foreground); }",
        ":hover{background: var(--background); color: var(--foreground); }",
        ":is(.c_theme_dark button){background: color-mix(in srgb, var(--input) 30%, transparent); border-color: var(--input); color: var(--foreground); }",
        ":is(.c_euv_tab_item_active){background: var(--background); color: var(--foreground); }",
        ":is(.c_euv_tab_item_active):hover{background: var(--background); color: var(--foreground); }",
        ":is(.c_theme_dark .c_euv_tab_item_active){background: color-mix(in srgb, var(--input) 30%, transparent); border-color: var(--input); color: var(--foreground); }",
        ":is(.c_theme_dark .c_euv_tab_item_active):hover{background: color-mix(in srgb, var(--input) 30%, transparent); color: var(--foreground); }",
    ).replace(CSS_BLOCK_OPEN, " { "));
    assert_eq!(c_euv_tab_item_active().get_pseudo_rules(), &expected);
    assert_eq!(
        class_property(c_form_checkbox_indicator(), "opacity").as_deref(),
        Some("0")
    );
    assert_eq!(
        class_property(c_form_checkbox_indicator_checked(), "opacity").as_deref(),
        Some("1")
    );
    assert_eq!(
        c_form_checkbox_indicator_checked().get_pseudo_rules(),
        &Css::parse_pseudo_rules(":is(.c_form_checkbox_indicator_checked) { opacity: 1; }")
    );
}

#[cfg(feature = "theme-shadcn")]
#[test]
fn formerly_global_canonical_geometry_is_selected_without_scheme_ancestors() {
    for (css, property, expected) in [
        (
            c_euv_avatar_fallback(),
            "border-radius",
            "var(--radius-full)",
        ),
        (c_euv_menubar(), "border-radius", "var(--radius-lg)"),
        (c_euv_menubar(), "gap", "var(--space-2xs)"),
        (
            c_euv_menubar_trigger(),
            "padding-left",
            "var(--space-xs-sm)",
        ),
        (
            c_euv_menubar_trigger_open(),
            "padding-left",
            "var(--space-xs-sm)",
        ),
        (c_euv_pagination_page(), "width", "var(--control-height-sm)"),
        (
            c_euv_pagination_page(),
            "height",
            "var(--control-height-sm)",
        ),
        (c_euv_pagination_page(), "border-radius", "var(--radius-lg)"),
        (
            c_euv_radio_group_item(),
            "border-radius",
            "var(--radius-full)",
        ),
        (
            c_euv_radio_group_item_checked(),
            "border-radius",
            "var(--radius-full)",
        ),
        (c_euv_combobox_chip_remove(), "opacity", "0.5"),
        (c_euv_input_group_button_xs(), "padding-left", "0px"),
        (c_euv_input_group_button_xs(), "padding-right", "0px"),
        (c_euv_navigation_menu_list(), "gap", "0px"),
    ] {
        assert_eq!(
            class_property(css, property).as_deref(),
            Some(expected),
            "{}: {property}",
            css.get_name()
        );
    }
}

#[test]
fn checkbox_indicator_is_mounted_for_both_states_without_an_extra_default_layout_box() {
    for checked in [false, true] {
        let rendered: VirtualNode = HookContext::with(HookContext::default(), || {
            euv_checkbox(VirtualNode::Element {
                tag: Tag::Element(Cow::Borrowed("div")),
                attributes: Vec::new(),
                children: Vec::new(),
                key: None,
                props: Some(Box::new(EuvCheckboxProps {
                    checked: Signal::create(checked),
                    label: "Accept",
                    ..EuvCheckboxProps::default()
                })),
            })
        });
        let VirtualNode::Element { children, .. } = rendered else {
            panic!("expected the checkbox row");
        };
        assert_eq!(children.len(), 3);
        let VirtualNode::Element {
            tag, attributes, ..
        } = &children[1]
        else {
            panic!("expected the mounted indicator");
        };
        assert_eq!(format!("{tag:?}"), "Element(\"span\")");
        let classes: Vec<String> = attributes
            .iter()
            .flat_map(AttributeEntry::class_names)
            .collect();
        assert!(
            classes
                .iter()
                .any(|name: &String| name == "c_form_checkbox_indicator")
        );
        assert_eq!(
            classes
                .iter()
                .any(|name: &String| name == "c_form_checkbox_indicator_checked"),
            checked
        );
    }
}

#[test]
fn root_property_reader_uses_exact_keys_and_last_definition() {
    let css: Css = Css {
        style: String::from(" --size: 3; --size-extra: 8; color: var(--ink); --size: 4 ; invalid;"),
        ..Css::default()
    };
    assert_eq!(class_property(&css, "--size"), Some(String::from("4")));
    assert_eq!(
        class_property(&css, "--size-extra"),
        Some(String::from("8"))
    );
    assert_eq!(
        class_property(&css, "color"),
        Some(String::from("var(--ink)"))
    );
    assert_eq!(class_property(&css, "size"), None);
    assert_eq!(class_property(&Css::default(), "--size"), None);
}

#[test]
fn root_property_reader_does_not_read_pseudo_or_media_rules() {
    let css: Css = Css {
        pseudo_rules: c_euv_button_primary().get_pseudo_rules().to_vec(),
        media_rules: c_euv_pagination_auto_content().get_media_rules().to_vec(),
        ..Css::default()
    };
    assert_eq!(class_property(&css, "background"), None);
    assert_eq!(class_property(&css, "grid-template-columns"), None);
}

#[test]
fn compiled_geometry_metadata_has_concrete_capabilities() {
    let shadcn: bool = cfg!(feature = "theme-shadcn");
    for (css, property, default, selected) in [
        (
            c_euv_dropdown_menu(),
            "--euv-dropdown-align",
            "end",
            "start",
        ),
        (
            c_euv_dropdown_menu(),
            "--euv-dropdown-anchor",
            "root",
            "trigger",
        ),
        (
            c_euv_dropdown_menu(),
            "--euv-dropdown-width-mode",
            "anchor",
            "content",
        ),
        (c_euv_dropdown_menu(), "--euv-dropdown-gap", "8", "4"),
        (c_euv_select_panel(), "--euv-select-align-item", "0", "1"),
        (c_euv_textarea(), "--euv-textarea-rows", "3", "2"),
    ] {
        let expected: Option<String> = Some(String::from(if shadcn { selected } else { default }));
        assert_eq!(class_property(css, property), expected, "{property}");
    }
    let align_item: bool = class_property(c_euv_select_panel(), "--euv-select-align-item")
        .map(|value: String| value == "1")
        .unwrap_or(false);
    assert_eq!(align_item, shadcn);
}

#[test]
fn unconditional_indicators_are_hidden_only_in_the_default_style() {
    let shadcn: bool = cfg!(feature = "theme-shadcn");
    for css in [c_form_checkbox_indicator(), c_euv_select_option_indicator()] {
        let display: Option<String> = class_property(css, "display");
        if shadcn {
            assert_ne!(display.as_deref(), Some("none"));
        } else {
            assert_eq!(display.as_deref(), Some("none"));
        }
    }
}

#[test]
fn primary_and_default_buttons_keep_distinct_names_and_the_same_compiled_paint() {
    assert_ne!(
        c_euv_button_primary().get_name(),
        c_euv_button_default().get_name()
    );
    assert_eq!(
        c_euv_button_primary().get_style(),
        c_euv_button_default().get_style()
    );
    assert_eq!(
        format!("{:?}", c_euv_button_primary().get_pseudo_rules()),
        format!("{:?}", c_euv_button_default().get_pseudo_rules())
    );
}

#[test]
fn disabled_group_contains_its_own_frame_paint_and_disabled_state() {
    let css: &Css = c_euv_input_group_disabled();
    assert_eq!(class_property(css, "display").as_deref(), Some("flex"));
    assert_eq!(
        class_property(css, "background"),
        Some(var!(muted).to_string())
    );
    assert_eq!(class_property(css, "opacity").as_deref(), Some("0.5"));
    assert_eq!(
        class_property(css, "border-radius"),
        Some(if cfg!(feature = "theme-shadcn") {
            var!(radius - lg).to_string()
        } else {
            var!(radius - md).to_string()
        })
    );
}

#[test]
fn pagination_display_policy_preserves_titles_in_default_and_directions_in_shadcn() {
    if cfg!(feature = "theme-shadcn") {
        assert_eq!(
            class_property(c_euv_pagination_text(), "display").as_deref(),
            Some("none")
        );
        assert_eq!(
            class_property(c_euv_pagination_icon(), "display").as_deref(),
            Some("inline-flex")
        );
        assert_eq!(
            class_property(c_euv_pagination_label(), "display").as_deref(),
            Some("inline")
        );
    } else {
        assert!(c_euv_pagination().get_style().is_empty());
        assert_eq!(
            class_property(c_euv_pagination_auto_content(), "display").as_deref(),
            Some("grid")
        );
        assert_ne!(
            class_property(c_euv_pagination_text(), "display").as_deref(),
            Some("none")
        );
        assert_eq!(
            class_property(c_euv_pagination_icon(), "display").as_deref(),
            Some("none")
        );
    }
}

#[test]
fn pagination_manual_and_automatic_content_preserve_ordered_layout_rules() {
    let manual: &Css = c_euv_pagination_content();
    let automatic: &Css = c_euv_pagination_auto_content();
    assert_ne!(manual.get_name(), automatic.get_name());
    for css in [manual, automatic] {
        assert!(css.get_pseudo_rules().is_empty());
    }
    let mut manual_expected: String = format!(
        "display:flex;align-items:center;gap:{};list-style:none;margin:0px;padding:0px;",
        var!(space - 2xs)
    );
    if cfg!(feature = "theme-shadcn") {
        manual_expected.push_str(&format!("gap:{};", var!(space - xs)));
    }
    assert_eq!(
        manual.get_style().split_whitespace().collect::<String>(),
        manual_expected.split_whitespace().collect::<String>()
    );
    assert!(manual.get_media_rules().is_empty());
    let automatic_expected: String = if cfg!(feature = "theme-shadcn") {
        format!(
            "display:grid;grid-template-columns:minmax(0px,1fr)minmax(0px,1fr);align-items:stretch;width:100%;min-width:0px;box-sizing:border-box;list-style:none;margin:0px;padding:0px;gap:{};margin-top:{};",
            var!(gap - component),
            var!(space - 2xl)
        )
    } else {
        format!(
            "display:grid;grid-template-columns:minmax(0px,1fr)minmax(0px,1fr);align-items:stretch;width:100%;box-sizing:border-box;list-style:none;margin:0px;padding:0px;gap:{};margin-top:{};min-width:0px;",
            var!(gap - component),
            var!(space - 2xl)
        )
    };
    assert_eq!(
        automatic.get_style().split_whitespace().collect::<String>(),
        automatic_expected.split_whitespace().collect::<String>()
    );
    assert!(automatic.get_media_rules().is_empty());
}

#[test]
fn pagination_root_never_inherits_automatic_grid_or_spacing() {
    let root: &Css = c_euv_pagination();
    assert!(root.get_pseudo_rules().is_empty());
    assert!(root.get_media_rules().is_empty());
    for property in ["grid-template-columns", "gap", "margin-top"] {
        assert_eq!(class_property(root, property), None, "{property}");
    }
    if !cfg!(feature = "theme-shadcn") {
        assert!(root.get_style().is_empty());
    } else {
        assert_eq!(
            root.get_style().split_whitespace().collect::<String>(),
            "display:flex;justify-content:center;width:100%;min-width:0px;margin:0px;"
        );
    }
}
