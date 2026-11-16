use super::*;

#[test]
fn dropdown_submenu_caret_uses_logical_alignment_and_rtl_rotation() {
    let caret: &Css = c_euv_dropdown_menu_sub_caret();
    for property in ["width", "height"] {
        assert_eq!(
            menu_style_value(caret, property).as_deref(),
            Some("var(--space-lg)")
        );
    }
    assert_eq!(
        menu_style_value(caret, "margin-inline-start").as_deref(),
        Some("auto")
    );
    assert_eq!(menu_style_value(caret, "flex-shrink").as_deref(), Some("0"));
    assert!(caret.get_pseudo_rules().contains(&PseudoRule::new(
        ":dir(rtl)".to_string(),
        format!("{} ", Css::style_string(&[("transform", "rotate(180deg)")])),
    )));
}

#[test]
fn sheet_shadcn_factories_compose_official_spacing_and_type_without_default_changes() {
    if cfg!(feature = "theme-shadcn") {
        for (class, property, expected) in [
            (c_euv_sheet_content(), "background", "var(--popover)"),
            (c_euv_sheet_content(), "color", "var(--popover-foreground)"),
            (c_euv_sheet_content_left(), "max-width", "none"),
            (c_euv_sheet_content_right(), "max-width", "none"),
            (c_euv_sheet_content(), "padding", "0px"),
            (c_euv_sheet_content(), "gap", "var(--space-lg)"),
            (c_euv_sheet_content(), "font-size", "var(--font-sm)"),
            (c_euv_sheet_content(), "line-height", "var(--space-xl)"),
            (c_euv_sheet_header(), "padding", "var(--space-lg)"),
            (c_euv_sheet_header(), "gap", "var(--space-2xs)"),
            (c_euv_sheet_header(), "text-align", "left"),
            (c_euv_sheet_footer(), "padding", "var(--space-lg)"),
            (c_euv_sheet_footer(), "gap", "var(--space-sm)"),
            (c_euv_sheet_footer(), "flex-direction", "column"),
            (c_euv_sheet_footer(), "margin-top", "auto"),
            (c_euv_sheet_title(), "font-size", "var(--font-base)"),
            (
                c_euv_sheet_title(),
                "font-weight",
                "var(--font-weight-medium)",
            ),
            (c_euv_sheet_title(), "line-height", "var(--space-2xl)"),
            (c_euv_sheet_description(), "font-size", "var(--font-sm)"),
            (c_euv_sheet_description(), "line-height", "var(--space-xl)"),
        ] {
            assert_eq!(
                menu_style_value(class, property).as_deref(),
                Some(expected),
                "{property}"
            );
        }
        for class in [c_euv_sheet_content_left(), c_euv_sheet_content_right()] {
            let media: String = format!("{:?}", class.get_media_rules());
            assert!(media.contains("min-width: 640px"), "{media}");
            assert!(media.contains("384px"), "{media}");
            assert_eq!(menu_style_value(class, "width").as_deref(), Some("75%"));
        }
    } else {
        for (class, property, expected) in [
            (c_euv_sheet_content_left(), "max-width", "384px"),
            (c_euv_sheet_content_right(), "max-width", "384px"),
            (c_euv_sheet_content(), "padding", "var(--space-2xl)"),
            (c_euv_sheet_content(), "gap", "var(--space-lg)"),
            (c_euv_sheet_header(), "gap", "var(--space-sm)"),
            (c_euv_sheet_header(), "text-align", "center"),
            (c_euv_sheet_footer(), "flex-direction", "column-reverse"),
            (c_euv_sheet_title(), "font-size", "var(--font-lg)"),
            (c_euv_sheet_title(), "font-weight", "600"),
            (c_euv_sheet_title(), "line-height", "1"),
        ] {
            assert_eq!(
                menu_style_value(class, property).as_deref(),
                Some(expected),
                "{property}"
            );
        }
        assert_eq!(menu_style_value(c_euv_sheet_header(), "padding"), None);
        assert_eq!(menu_style_value(c_euv_sheet_footer(), "margin-top"), None);
    }
    assert_eq!(c_euv_sheet_content().get_name(), "c_euv_sheet_content");
    assert_eq!(c_euv_sheet_title().get_name(), "c_euv_sheet_title");
    assert_eq!(
        c_euv_sheet_description().get_name(),
        "c_euv_sheet_description"
    );
}

#[test]
fn sheet_close_hitbox_override_preserves_the_default_geometry() {
    if !cfg!(feature = "theme-shadcn") {
        for property in ["width", "height", "top", "right"] {
            assert_eq!(
                menu_style_value(c_euv_sheet_close(), property).as_deref(),
                Some("var(--space-lg)")
            );
        }
    } else {
        for property in ["width", "height"] {
            assert_eq!(
                menu_style_value(c_euv_sheet_close(), property).as_deref(),
                Some("calc(var(--control-height-sm) - var(--space-xs))")
            );
        }
        for property in ["top", "right"] {
            assert_eq!(
                menu_style_value(c_euv_sheet_close(), property).as_deref(),
                Some("var(--space-md)")
            );
        }
        assert_eq!(
            menu_style_value(c_euv_sheet_close(), "border-radius").as_deref(),
            Some("min(var(--radius-md), var(--space-md))")
        );
        assert!(
            c_euv_sheet_close()
                .get_pseudo_rules()
                .contains(&PseudoRule::new(
                    ":hover".to_string(),
                    format!(
                        "{} ",
                        Css::style_string(&[
                            ("background", "var(--muted)"),
                            ("color", "var(--foreground)"),
                        ])
                    ),
                ))
        );
    }
}

#[test]
fn dropdown_popups_keep_their_surface_fill_and_rows_do_not_shrink() {
    for class in [c_euv_dropdown_menu(), c_euv_dropdown_menu()] {
        assert_eq!(
            menu_style_value(class, "background").as_deref(),
            Some("var(--popover)")
        );
    }
    for class in [
        c_euv_dropdown_item(),
        c_euv_dropdown_menu_item(),
        c_euv_dropdown_menu_checkbox_item(),
        c_euv_dropdown_menu_group(),
    ] {
        assert_eq!(menu_style_value(class, "flex-shrink").as_deref(), Some("0"));
    }
}

#[cfg(feature = "theme-shadcn")]
#[test]
fn context_menu_submenu_fallback_uses_logical_inline_end() {
    let panel: &Css = c_euv_context_menu_sub_content();
    assert_eq!(
        menu_style_value(panel, "inset-inline-start").as_deref(),
        Some("100%")
    );
    assert_eq!(menu_style_value(panel, "left"), None);
    assert_eq!(menu_style_value(panel, "right"), None);
    assert_eq!(
        menu_style_value(c_euv_context_menu_sub_trigger(), "text-align").as_deref(),
        Some("start")
    );
}

#[test]
fn sidebar_rail_stays_on_its_containing_sidebar_edge() {
    let class: &Css = c_euv_sidebar_rail();
    for (property, value) in [
        ("position", "absolute"),
        ("right", "0px"),
        ("top", "0px"),
        ("bottom", "0px"),
    ] {
        assert_eq!(menu_style_value(class, property).as_deref(), Some(value));
    }
}

fn full_width_menu_rows() -> [&'static Css; 10] {
    [
        c_euv_dropdown_menu_item(),
        c_euv_dropdown_menu_checkbox_item(),
        c_euv_dropdown_menu_radio_item(),
        c_euv_dropdown_menu_sub_trigger(),
        c_euv_menubar_item(),
        c_euv_context_menu_item(),
        c_euv_context_menu_item_disabled(),
        c_euv_context_menu_checkbox_item(),
        c_euv_context_menu_radio_item(),
        c_euv_context_menu_sub_trigger(),
    ]
}

fn menu_style_value(class: &Css, property: &str) -> Option<String> {
    class
        .get_style()
        .split(';')
        .filter_map(|part: &str| part.split_once(':'))
        .filter(|(name, _): &(&str, &str)| name.trim() == property)
        .map(|(_, value): (&str, &str)| value.trim().to_string())
        .next_back()
}

#[test]
fn menu_rows_include_padding_inside_the_same_full_width_hover_box() {
    for class in full_width_menu_rows() {
        assert_eq!(
            menu_style_value(class, "width").as_deref(),
            Some("100%"),
            "{}",
            class.get_name()
        );
        assert_eq!(
            menu_style_value(class, "box-sizing").as_deref(),
            Some("border-box"),
            "{}",
            class.get_name()
        );
        assert_eq!(
            menu_style_value(class, "padding").as_deref(),
            Some(
                if cfg!(feature = "theme-shadcn") && class.get_name() == "c_euv_menubar_item" {
                    "calc(var(--space-xs) + var(--space-2xs)) var(--space-sm)"
                } else {
                    "var(--space-xs) var(--space-sm)"
                }
            ),
            "{}",
            class.get_name()
        );
        assert_eq!(
            menu_style_value(class, "line-height").as_deref(),
            Some("1.5"),
            "{}",
            class.get_name()
        );
    }
}

#[test]
fn sibling_menu_rows_share_tokenized_corners_and_hover_ink() {
    let mut rows: Vec<&Css> = full_width_menu_rows().to_vec();
    rows.extend([
        c_euv_menubar_checkbox_item(),
        c_euv_menubar_radio_item(),
        c_euv_menubar_sub_trigger(),
        c_euv_menubar_sub_trigger_inset(),
    ]);
    let hover: PseudoRule = PseudoRule::new(
        ":hover".to_string(),
        format!(
            "{} ",
            Css::style_string(&[
                ("background", "var(--accent)"),
                ("color", "var(--accent-foreground)"),
            ])
        ),
    );
    for class in rows {
        assert_eq!(
            menu_style_value(class, "border-radius").as_deref(),
            Some("var(--radius-sm)"),
            "{} must preserve the scheme's radius token",
            class.get_name()
        );
        assert!(
            class.get_pseudo_rules().contains(&hover),
            "{} must paint the same hover accent",
            class.get_name()
        );
    }
}

#[test]
fn menu_rows_offer_keyboard_focus_and_suppress_disabled_pointer_events() {
    let expected: Vec<PseudoRule> = vec![
        PseudoRule::new(
            ":focus-visible".to_string(),
            format!(
                "{} ",
                Css::style_string(&[
                    ("background", "var(--accent)"),
                    ("color", "var(--accent-foreground)"),
                ])
            ),
        ),
        PseudoRule::new(
            ":disabled".to_string(),
            format!(
                "{} ",
                Css::style_string(&[("pointer-events", "none"), ("opacity", "0.5")])
            ),
        ),
    ];
    for class in full_width_menu_rows() {
        for rule in &expected {
            assert!(
                class.get_pseudo_rules().contains(rule),
                "{} is missing {rule:?}",
                class.get_name()
            );
        }
    }
    for class in [
        c_euv_dropdown_menu_item_disabled(),
        c_euv_context_menu_item_disabled(),
    ] {
        assert_eq!(
            menu_style_value(class, "pointer-events").as_deref(),
            Some("none"),
            "{}",
            class.get_name()
        );
    }
}
