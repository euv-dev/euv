use super::*;

fn declaration(style: &str, property: &str) -> Option<String> {
    style
        .split(';')
        .filter_map(|part: &str| part.split_once(':'))
        .filter(|(name, _): &(&str, &str)| name.trim() == property)
        .map(|(_, value): (&str, &str)| value.trim().to_string())
        .next_back()
}

fn scheme_literal(source: &str, name: &str) -> String {
    let marker: String = format!("{name}:");
    let tail: &str = source
        .split_once(&marker)
        .unwrap_or_else(|| panic!("{name} is defined"))
        .1;
    let value: &str = tail
        .split_once('"')
        .unwrap_or_else(|| panic!("{name} is a literal"))
        .1;
    value
        .split_once('"')
        .unwrap_or_else(|| panic!("{name} is quoted"))
        .0
        .to_string()
}

#[test]
fn docs_toc_active_text_colour_differs_from_fill() {
    for selector in [
        "#app .c_euv_toc_link_active, #app .c_euv_toc_link_nested_active",
        "#app .c_euv_toc_link_active:hover, #app .c_euv_toc_link_nested_active:hover",
    ] {
        let marker: String = format!("{selector} {{");
        if let Some((_, tail)) = APP_GLOBAL_CSS.split_once(&marker) {
            let body: &str = tail
                .split_once('}')
                .map(|(body, _): (&str, &str)| body)
                .unwrap_or(tail);
            assert!(
                declaration(body, "color").is_none(),
                "{selector} must not declare color: the framework's var(--primary-foreground) must apply"
            );
        }
    }
    for css in [c_euv_toc_link_active(), c_euv_toc_link_nested_active()] {
        assert_eq!(
            declaration(css.get_style(), "color").as_deref(),
            Some("var(--primary-foreground)"),
            "{} paints its label in the primary foreground",
            css.get_name()
        );
        assert_eq!(
            declaration(css.get_style(), "background").as_deref(),
            Some("var(--primary)"),
            "{} fills itself with the primary colour",
            css.get_name()
        );
    }
    let scheme: &str = include_str!("../../../ui/src/style/var/scheme/monochrome/fn.rs");
    let light_fill: String = scheme_literal(scheme, "role-primary-light");
    let light_ink: String = scheme_literal(scheme, "role-primary-foreground-light");
    let dark_fill: String = scheme_literal(scheme, "role-primary-dark");
    let dark_ink: String = scheme_literal(scheme, "role-primary-foreground-dark");
    assert_ne!(
        light_fill, light_ink,
        "the monochrome light scheme must contrast its primary fill and ink"
    );
    assert_ne!(
        dark_fill, dark_ink,
        "the monochrome dark scheme must contrast its primary fill and ink"
    );
}
