use super::*;

fn reset_chunks() -> [&'static str; 5] {
    [
        crate::APP_GLOBAL_CSS_RESET,
        crate::APP_GLOBAL_CSS_ROOT,
        crate::APP_GLOBAL_CSS_LIST,
        crate::APP_GLOBAL_CSS_MEDIA,
        crate::APP_GLOBAL_CSS_FORM,
    ]
}

#[test]
fn universal_reset_covers_generated_boxes_without_flattening_semantic_layout() {
    let reset: &str = crate::APP_GLOBAL_CSS_RESET;
    for selector in [
        ":where(*)",
        ":where(*)::before",
        ":where(*)::after",
        ":where(*)::backdrop",
    ] {
        assert!(reset.contains(selector), "missing {selector}");
    }
    for declaration in [
        "box-sizing: border-box;",
        "margin: 0;",
        "padding: 0;",
        "border: 0 solid;",
        "font: inherit;",
        "color: inherit;",
        "background: transparent;",
        "text-decoration: none;",
        "text-transform: none;",
    ] {
        assert!(reset.contains(declaration), "missing {declaration}");
    }
    for css in reset_chunks() {
        for forbidden in [
            "all:",
            "display: block",
            "display: inline;",
            "fill:",
            "stroke:",
            "outline: none",
        ] {
            assert!(!css.contains(forbidden), "destructive reset: {forbidden}");
        }
    }
    let media: &str = crate::APP_GLOBAL_CSS_MEDIA;
    for forbidden in ["width:", "height:", "display:"] {
        assert!(
            !media.contains(forbidden),
            "intrinsic media sizing: {forbidden}"
        );
    }
}

#[test]
fn hidden_resources_and_closed_dialogs_stay_hidden_without_disabling_until_found() {
    let reset: &str = crate::APP_GLOBAL_CSS_RESET;
    assert!(reset.contains("[hidden]:not([hidden=\"until-found\" i])"));
    assert!(reset.contains("input[type=\"hidden\" i],script,style,template,dialog:not([open])"));
    assert!(reset.contains("display: none !important;"));
    assert_eq!(reset.matches("!important").count(), 1);
    for css in &reset_chunks()[1..] {
        assert!(!css.contains("!important"));
    }
}

#[test]
fn normalization_covers_lists_tables_and_engine_specific_form_parts() {
    let list: &str = crate::APP_GLOBAL_CSS_LIST;
    for fragment in [
        "ol,ul,menu,summary",
        "::-webkit-details-marker",
        "border-collapse: collapse",
        "border-spacing: 0",
        "text-indent: 0",
        ":where(fieldset)",
        ":where(legend)",
    ] {
        assert!(list.contains(fragment), "missing {fragment}");
    }
    let form: &str = crate::APP_GLOBAL_CSS_FORM;
    for fragment in [
        "input,button,textarea,select,optgroup,option,output,meter,progress",
        "appearance: none;",
        "-webkit-appearance: none;",
        "-moz-appearance: none;",
        "::placeholder",
        "opacity: 1",
        "::-moz-focus-inner",
        "::-moz-number-spin-box",
        "-moz-appearance: textfield",
        "::-webkit-search-decoration",
        "::-webkit-search-cancel-button",
        "::-webkit-search-results-button",
        "::-webkit-search-results-decoration",
        "::-webkit-inner-spin-button",
        "::-webkit-outer-spin-button",
        "::-webkit-file-upload-button",
        "::file-selector-button",
        "::-webkit-date-and-time-value",
        "::-webkit-datetime-edit-fields-wrapper",
        "::-webkit-datetime-edit-year-field",
        "::-webkit-datetime-edit-month-field",
        "::-webkit-datetime-edit-day-field",
        "::-webkit-datetime-edit-hour-field",
        "::-webkit-datetime-edit-minute-field",
        "::-webkit-datetime-edit-second-field",
        "::-webkit-datetime-edit-millisecond-field",
        "::-webkit-datetime-edit-ampm-field",
        "::-webkit-calendar-picker-indicator",
    ] {
        assert!(form.contains(fragment), "missing {fragment}");
    }
}

#[test]
fn reset_selectors_are_weaker_than_component_classes_and_vendor_rules_are_isolated() {
    for css in reset_chunks() {
        for rule in css.split('}').filter(|rule: &&str| !rule.trim().is_empty()) {
            let (selectors, _): (&str, &str) = rule.split_once('{').expect("a CSS rule");
            assert!(
                selectors.trim().starts_with(":where("),
                "strong reset: {selectors}"
            );
            assert!(!selectors.contains('#') && !selectors.contains('.'));
            for argument in selectors.split(":where(").skip(1) {
                let argument: &str = argument.split(')').next().expect("a where argument");
                assert!(
                    !argument.contains("::"),
                    "pseudo-element inside :where: {selectors}"
                );
            }
            if selectors.contains("::-webkit-")
                || selectors.contains("::-moz-")
                || selectors.contains("::backdrop")
                || selectors.contains("::file-selector-button")
            {
                assert_eq!(selectors.matches(":where(").count(), 1);
                let (_, pseudo): (&str, &str) =
                    selectors.split_once(')').expect("a pseudo selector");
                assert!(
                    !pseudo.contains(','),
                    "non-forgiving selector list: {selectors}"
                );
                assert_eq!(
                    selectors.matches("::").count(),
                    1,
                    "potentially unsupported pseudo-element must be isolated"
                );
            }
        }
    }
}

#[test]
fn reset_contains_no_theme_appearance_or_nonzero_design_dimensions() {
    for css in reset_chunks() {
        for forbidden in [
            "var(",
            "theme-",
            ".c_",
            "cursor:",
            "font-family:",
            "scrollbar",
            "color-mix(",
            "#",
        ] {
            assert!(!css.contains(forbidden), "global appearance: {forbidden}");
        }
        for rule in css.split('}').filter(|rule: &&str| !rule.trim().is_empty()) {
            let (_, body): (&str, &str) = rule.split_once('{').expect("a CSS rule");
            for declaration in body
                .split(';')
                .filter(|item: &&str| !item.trim().is_empty())
            {
                let (name, value): (&str, &str) =
                    declaration.split_once(':').expect("a declaration");
                let name: &str = name.trim();
                let value: &str = value.trim();
                if name == "color"
                    || name == "font"
                    || name == "line-height"
                    || name == "letter-spacing"
                {
                    assert_eq!(value, "inherit", "{name} must not choose appearance");
                }
                if name == "background" {
                    assert_eq!(value, "transparent");
                }
                if name == "box-shadow" || name == "text-shadow" {
                    assert_eq!(value, "none");
                }
                if name == "margin"
                    || name == "padding"
                    || name == "border-radius"
                    || name == "border-spacing"
                {
                    assert_eq!(value, "0", "{name} must only remove UA geometry");
                }
            }
        }
    }
}

#[test]
fn startup_injects_the_same_reset_for_every_theme_and_no_global_appearance() {
    let source: &str = include_str!("../../src/style/css/fn.rs");
    let startup: &str = source
        .split("pub fn inject_app_global_css() {")
        .nth(1)
        .expect("the global CSS entry point")
        .split("\n}")
        .next()
        .expect("the function body");
    let injected: Vec<&str> = startup
        .lines()
        .map(str::trim)
        .filter(|line: &&str| !line.is_empty())
        .collect();
    assert_eq!(
        injected,
        vec![
            "Css::inject_css(APP_GLOBAL_CSS_RESET);",
            "Css::inject_css(APP_GLOBAL_CSS_ROOT);",
            "Css::inject_css(APP_GLOBAL_CSS_LIST);",
            "Css::inject_css(APP_GLOBAL_CSS_MEDIA);",
            "Css::inject_css(APP_GLOBAL_CSS_FORM);",
            "Css::inject_css(APP_KEYFRAMES_SPIN);",
            "Css::inject_css(APP_KEYFRAMES_FADE_IN);",
            "Css::inject_css(APP_KEYFRAMES_PULSE);",
            "Css::inject_css(APP_KEYFRAMES_SKELETON_PULSE);",
            "Css::inject_css(APP_KEYFRAMES_PROGRESS);",
            "Css::inject_css(APP_KEYFRAMES_SCALE_IN_MODAL);",
            "Css::inject_css(APP_A11Y_CSS_REDUCED_MOTION);",
        ]
    );
    let constants: &str = include_str!("../../src/style/css/const.rs");
    let reset_constants: &str = constants
        .split("pub(crate) const APP_GLOBAL_CSS_RESET")
        .nth(1)
        .expect("the universal reset")
        .split("/// Keyframes:")
        .next()
        .expect("the reset constants");
    assert!(!reset_constants.contains("#[cfg"));
    assert!(!reset_constants.contains("feature ="));
}

#[test]
fn inherited_tap_highlighting_is_transparent_without_suppressing_control_states() {
    let reset: &str = crate::APP_GLOBAL_CSS_ROOT;
    assert!(reset.contains("-webkit-tap-highlight-color: transparent;"));
    for forbidden in ["outline:", ":focus", ":active", "user-select:"] {
        assert!(!reset.contains(forbidden), "{forbidden}");
    }
    let nav: String = format!("{:?}", c_nav_item_inactive().get_pseudo_rules());
    assert!(nav.contains(":focus-visible"));
    assert!(nav.contains("var(--ring)"));
    let action: String = format!("{:?}", c_home_btn_primary().get_pseudo_rules());
    assert!(action.contains(":active"));
    assert!(action.contains("var(--press-scale)"));
    assert!(action.contains(":focus-visible"));
}
