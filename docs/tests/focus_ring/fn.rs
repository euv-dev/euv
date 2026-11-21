use super::*;

fn rule(source: &str, selector: &str) -> String {
    let marker: String = format!("{selector} {{");
    let tail: &str = source
        .split_once(&marker)
        .unwrap_or_else(|| panic!("{selector} has no rule"))
        .1;
    tail.split_once('}')
        .unwrap_or_else(|| panic!("{selector} has no closed rule"))
        .0
        .to_string()
}

fn declaration(style: &str, property: &str) -> Option<String> {
    style
        .split(';')
        .filter_map(|part: &str| part.split_once(':'))
        .filter(|(name, _): &(&str, &str)| name.trim() == property)
        .map(|(_, value): (&str, &str)| value.trim().to_string())
        .next_back()
}

#[test]
fn the_search_focus_ring_is_a_box_shadow_that_uses_the_theme_ring_tokens() {
    let focus: String = rule(APP_GLOBAL_CSS, "#app .c_docs_header_search:focus-within");
    let shadow: String =
        declaration(&focus, "box-shadow").expect("the focused search wrapper paints a box-shadow");
    assert!(
        shadow.contains("var(--ring-width)") && shadow.contains("var(--ring-offset)"),
        "the ring must spread by the theme's own width and offset, got {shadow}"
    );
    assert!(
        shadow.contains("var(--ring)"),
        "the ring must wear the theme's own ring colour, got {shadow}"
    );
    assert!(
        !shadow.contains("border-color"),
        "a border repaint is a layout change, not a ring"
    );
}

#[test]
fn the_search_wrapper_never_recolours_its_border_to_signal_focus() {
    let base: String = rule(APP_GLOBAL_CSS, "#app .c_docs_header_search");
    assert_eq!(
        declaration(&base, "border").as_deref(),
        Some("1px solid transparent"),
        "the wrapper's border is transparent in both states"
    );
    assert_eq!(declaration(&base, "box-shadow"), None);
    let focus: String = rule(APP_GLOBAL_CSS, "#app .c_docs_header_search:focus-within");
    assert_eq!(declaration(&focus, "border-color"), None);
    assert_eq!(declaration(&focus, "border-width"), None);
}

#[test]
fn the_search_field_carries_no_outline_of_its_own() {
    let field: String = rule(APP_GLOBAL_CSS, "#app .c_docs_header_search_input");
    assert_eq!(declaration(&field, "outline").as_deref(), Some("none"));
    assert_eq!(declaration(&field, "border").as_deref(), Some("none"));
}
