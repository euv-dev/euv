use super::*;

#[test]
fn route_slug_strips_the_html_suffix() {
    assert_eq!(route_slug("/guide/getting-started.html"), "getting-started");
}

#[test]
fn route_slug_strips_the_trailing_slash() {
    assert_eq!(route_slug("/guide/"), "guide");
}

#[test]
fn route_slug_of_the_site_root_is_empty() {
    assert_eq!(route_slug("/"), "");
}

#[test]
fn route_slug_of_an_empty_route_is_empty() {
    assert_eq!(route_slug(""), "");
}

#[test]
fn description_is_found_for_a_known_slug() {
    assert_eq!(
        description_for_slug("code-block"),
        Some(
            "Displays code, terminal output and diffs with an optional header, line numbers and a copy button."
        )
    );
}

#[test]
fn description_is_absent_for_an_unknown_slug() {
    assert_eq!(description_for_slug("getting-started"), None);
}

#[test]
fn description_is_absent_for_the_empty_slug() {
    assert_eq!(description_for_slug(""), None);
}

#[test]
fn description_of_a_route_uses_the_route_slug() {
    assert_eq!(
        description_for_route("/components/code-block.html"),
        description_for_slug("code-block")
    );
}

#[test]
fn description_of_a_route_without_an_entry_is_absent() {
    assert_eq!(description_for_route("/guide/getting-started.html"), None);
}

#[test]
fn the_table_has_no_duplicate_slugs() {
    let mut slugs: Vec<&str> = REF_DESCRIPTIONS
        .iter()
        .map(|(key, _): &(&str, &str)| *key)
        .collect();
    slugs.sort_unstable();
    let count: usize = slugs.len();
    slugs.dedup();
    assert_eq!(slugs.len(), count);
}

#[test]
fn the_table_has_no_empty_description() {
    for (key, description) in REF_DESCRIPTIONS {
        assert!(!key.is_empty());
        assert!(!description.is_empty());
    }
}
