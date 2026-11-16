use super::*;

fn rows() -> [&'static Css; 7] {
    [
        c_euv_sidebar_group_title(),
        c_euv_sidebar_group_title_root(),
        c_euv_sidebar_group_title_active(),
        c_euv_sidebar_group_title_root_active(),
        c_euv_sidebar_link(),
        c_euv_sidebar_link_active(),
        c_euv_sidebar_link_active_flush(),
    ]
}

fn arrows() -> [&'static Css; 4] {
    [
        c_euv_sidebar_group_arrow(),
        c_euv_sidebar_group_arrow_open(),
        c_euv_sidebar_group_arrow_active(),
        c_euv_sidebar_group_arrow_active_open(),
    ]
}

#[cfg(not(feature = "theme-custom"))]
fn value(css: &Css, property: &str) -> Option<String> {
    css.get_style()
        .split(';')
        .filter_map(|part: &str| part.split_once(':'))
        .filter(|(name, _): &(&str, &str)| name.trim() == property)
        .map(|(_, value): (&str, &str)| value.trim().to_string())
        .next_back()
}

#[cfg(all(feature = "theme-shadcn", not(feature = "theme-custom")))]
#[test]
fn sidebar_shadcn_rows_share_canonical_geometry_and_four_rounded_corners() {
    let canonical: &Css = c_euv_sidebar_menu_button();
    for row in rows() {
        for property in [
            "height",
            "padding",
            "width",
            "font-size",
            "line-height",
            "border-radius",
        ] {
            assert_eq!(
                value(row, property),
                value(canonical, property),
                "{}: {property}",
                row.get_name()
            );
        }
        assert_eq!(
            value(row, "border-radius").as_deref(),
            Some("var(--radius-md)")
        );
        for corner in [
            "border-top-left-radius",
            "border-top-right-radius",
            "border-bottom-left-radius",
            "border-bottom-right-radius",
        ] {
            if let Some(radius) = value(row, corner) {
                assert_eq!(radius, "var(--radius-md)");
            }
        }
        for property in [
            "margin-left",
            "margin-right",
            "margin-top",
            "margin-bottom",
            "padding-left",
        ] {
            assert_eq!(value(row, property), None, "{}: {property}", row.get_name());
        }
        assert_eq!(row.get_pseudo_rules(), canonical.get_pseudo_rules());
        let rules: String = format!("{:?}", row.get_pseudo_rules());
        assert!(!rules.contains("box-shadow") && !rules.contains("::before"));
        assert!(!row.get_style().contains("calc(100% +"));
    }
}

#[cfg(all(feature = "theme-shadcn", not(feature = "theme-custom")))]
#[test]
fn sidebar_shadcn_active_and_inactive_use_canonical_colors() {
    for (row, canonical) in [
        (c_euv_sidebar_group_title(), c_euv_sidebar_menu_button()),
        (
            c_euv_sidebar_group_title_root(),
            c_euv_sidebar_menu_button(),
        ),
        (c_euv_sidebar_link(), c_euv_sidebar_menu_button()),
        (
            c_euv_sidebar_group_title_active(),
            c_euv_sidebar_menu_button_active(),
        ),
        (
            c_euv_sidebar_group_title_root_active(),
            c_euv_sidebar_menu_button_active(),
        ),
        (
            c_euv_sidebar_link_active(),
            c_euv_sidebar_menu_button_active(),
        ),
        (
            c_euv_sidebar_link_active_flush(),
            c_euv_sidebar_menu_button_active(),
        ),
    ] {
        for property in ["background", "color", "font-weight"] {
            assert_eq!(
                value(row, property),
                value(canonical, property),
                "{}: {property}",
                row.get_name()
            );
        }
        assert_eq!(row.get_pseudo_rules(), canonical.get_pseudo_rules());
    }
}

#[cfg(all(feature = "theme-shadcn", not(feature = "theme-custom")))]
#[test]
fn sidebar_shadcn_arrows_compose_action_without_expanding_rows() {
    let canonical: &Css = c_euv_sidebar_menu_action();
    for (index, arrow) in arrows().into_iter().enumerate() {
        for property in ["width", "height", "padding", "border-radius", "flex-shrink"] {
            assert_eq!(value(arrow, property), value(canonical, property));
        }
        for property in ["margin-left", "margin-right", "margin-top", "margin-bottom"] {
            assert_eq!(value(arrow, property), None);
        }
        assert_eq!(arrow.get_pseudo_rules(), canonical.get_pseudo_rules());
        assert_eq!(
            value(arrow, "transform").as_deref(),
            if index % 2 == 1 {
                Some("rotate(90deg)")
            } else {
                None
            }
        );
        assert_eq!(
            value(arrow, "color").as_deref(),
            Some(if index >= 2 {
                "var(--accent-foreground)"
            } else {
                "var(--muted-foreground)"
            })
        );
    }
}

#[cfg(all(not(feature = "theme-shadcn"), not(feature = "theme-custom")))]
#[test]
fn sidebar_shadcn_does_not_change_other_theme_rows() {
    for row in [
        c_euv_sidebar_group_title_active(),
        c_euv_sidebar_link_active_flush(),
    ] {
        assert_eq!(
            value(row, "margin-left").as_deref(),
            Some("calc(-1 * var(--side-indent))")
        );
        assert_eq!(
            value(row, "width").as_deref(),
            Some("calc(100% + var(--side-indent))")
        );
        assert_eq!(value(row, "background").as_deref(), Some("var(--primary)"));
    }
    for row in [
        c_euv_sidebar_group_title(),
        c_euv_sidebar_group_title_root(),
        c_euv_sidebar_link(),
    ] {
        assert_eq!(
            value(row, "padding").as_deref(),
            Some("var(--control-row-pad) var(--space-md)")
        );
        assert!(format!("{:?}", row.get_pseudo_rules()).contains("inset"));
    }
    for arrow in arrows() {
        assert_eq!(value(arrow, "margin-top").as_deref(), Some("-9px"));
    }
    assert_eq!(rows().len(), 7);
}

#[cfg(feature = "theme-custom")]
#[test]
fn sidebar_shadcn_custom_has_priority_and_keeps_styles_empty() {
    for css in rows().into_iter().chain(arrows()) {
        assert!(css.get_style().is_empty());
        assert!(css.get_pseudo_rules().is_empty());
        assert!(css.get_media_rules().is_empty());
    }
}
