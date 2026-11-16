use super::*;

#[test]
fn structural_wrappers_do_not_break_the_viewport_height_chain() {
    assert_eq!(
        declaration("#app .c_euv_display_contents", "display"),
        Some("contents")
    );
    for selector in ["#app .c_app_root", "#app .c_mobile_app_root"] {
        assert_eq!(declaration(selector, "height"), Some("100%"));
        assert_eq!(declaration(selector, "min-height"), Some("0"));
        assert_eq!(declaration(selector, "overflow"), Some("hidden"));
    }
    for selector in ["#app .c_app_main", "#app .c_mobile_main"] {
        assert_eq!(declaration(selector, "min-height"), Some("0"));
        assert_eq!(declaration(selector, "overflow-y"), Some("auto"));
    }
    assert_eq!(declaration("#app .c_euv_doc_tail", "position"), None);
}

#[test]
fn custom_theme_glyphs_have_intrinsic_size_without_a_color_override() {
    for selector in ["#app .c_theme_icon_sun", "#app .c_theme_icon_moon"] {
        assert_eq!(declaration(selector, "display"), Some("inline-flex"));
        assert_eq!(
            declaration(selector, "width"),
            Some("var(--space-xl, 20px)")
        );
        assert_eq!(
            declaration(selector, "height"),
            Some("var(--space-xl, 20px)")
        );
        assert_eq!(declaration(selector, "color"), None);
    }
    assert_eq!(
        declaration("#app .c_theme_icon_sun::before", "content"),
        Some(r#""\2600\FE0E" / """#)
    );
    assert_eq!(
        declaration("#app .c_theme_icon_moon::before", "content"),
        Some(r#""\263E\FE0E" / """#)
    );
}

#[test]
fn custom_document_footer_keeps_equal_tracks_and_full_clickable_cells() {
    assert_eq!(
        declaration("#app .c_euv_pagination_auto_content", "display"),
        Some("grid")
    );
    assert_eq!(
        declaration(
            "#app .c_euv_pagination_auto_content",
            "grid-template-columns"
        ),
        Some("minmax(0, 1fr) minmax(0, 1fr)")
    );
    let link: &str = "#app .c_euv_pagination_auto_content .c_euv_pagination_link";
    for (property, expected) in [
        ("width", "100%"),
        ("height", "auto"),
        ("flex", "1"),
        ("box-sizing", "border-box"),
        ("min-width", "0"),
    ] {
        assert_eq!(declaration(link, property), Some(expected));
    }
    assert_eq!(
        declaration(
            "#app .c_euv_pagination_auto_content .c_euv_pagination_text",
            "grid-row"
        ),
        Some("2")
    );
    assert_eq!(
        declaration(
            "#app .c_euv_pagination_auto_content .c_euv_pagination_next .c_euv_pagination_icon",
            "grid-column"
        ),
        Some("3")
    );
    assert!(!APP_GLOBAL_CSS.contains("#app .c_euv_pagination_link {"));
}

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
fn docs_footer_flows_in_content_and_toc_has_a_bounded_desktop_column() {
    assert_eq!(declaration("#app .c_docs_body", "min-height"), Some("0"));
    assert_eq!(
        declaration("#app .c_euv_doc_layout", "min-height"),
        Some("100%")
    );
    assert_eq!(
        declaration("#app .c_euv_doc_content", "min-width"),
        Some("0")
    );
    assert_eq!(
        declaration("#app .c_euv_doc_content", "justify-content"),
        Some("space-between")
    );
    assert_eq!(
        declaration("#app .c_euv_doc_tail", "flex"),
        Some("0 0 auto")
    );
    assert_eq!(declaration("#app .c_euv_footer", "position"), None);
    assert_eq!(declaration("#app .c_euv_doc_tail", "position"), None);
    assert_eq!(
        declaration("#app .c_euv_doc_toc", "overflow-y"),
        Some("auto")
    );
    assert!(APP_CUSTOM_LAYOUT_CSS.contains("#app .c_euv_doc_toc { display: none; }"));
    let entry: &str = include_str!("../../src/lib.rs");
    assert!(
        entry
            .find("Css::inject_css(CODE_CSS)")
            .expect("existing CSS")
            < entry
                .find("Css::inject_css(APP_CUSTOM_LAYOUT_CSS)")
                .expect("custom CSS")
    );
}
