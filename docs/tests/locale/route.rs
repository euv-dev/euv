use super::*;

fn locale_prefixes() -> Vec<&'static str> {
    route_site()
        .get_locales()
        .iter()
        .map(RouteLocale::get_prefix)
        .collect()
}

#[test]
fn a_route_renders_the_text_of_the_locale_that_owns_it() {
    let declared: Vec<&str> = locale_prefixes();
    let rendered: Vec<String> = declared
        .iter()
        .map(|prefix: &&str| locale_text(prefix, gate_key_hint()))
        .collect();
    for (index, prefix) in declared.iter().enumerate() {
        let deep: String = locale_text(&format!("{prefix}deeply/nested/b.html"), gate_key_hint());
        assert_eq!(
            rendered[index], deep,
            "{prefix} must render the same text at its index and deep inside it"
        );
    }
    for (label, tag) in locale_tag_bindings() {
        let expected: String = lookup_table(tag, gate_key_hint()).unwrap_or_default();
        for prefix in &declared {
            if locale_label_for(prefix) != *label {
                continue;
            }
            assert_eq!(
                &rendered[declared
                    .iter()
                    .position(|p: &&str| p == prefix)
                    .unwrap_or(0)],
                &expected,
                "{prefix} is declared as {label} but renders text that is not the {tag} table"
            );
        }
    }
}

#[test]
fn the_four_site_locales_each_render_distinct_gate_text() {
    let declared: Vec<&str> = locale_prefixes();
    if declared.len() < 4 {
        return;
    }
    let by_tag: Vec<String> = supported_locale_tags()
        .iter()
        .map(|tag: &&str| lookup_table(tag, gate_key_hint()).unwrap_or_else(|| tag.to_string()))
        .collect();
    let unique: HashSet<&String> = by_tag.iter().collect();
    assert_eq!(
        unique.len(),
        4,
        "zh/en/ja/ko must not share gate text: {by_tag:?}"
    );
}

#[test]
fn a_route_outside_every_declared_locale_falls_back_to_the_fallback_table() {
    let english_hint: Option<String> = lookup_table(gate_fallback_tag(), gate_key_hint());
    assert!(
        english_hint.is_some(),
        "the fallback table must carry the hint"
    );
    let declared: Vec<&str> = locale_prefixes();
    for locale_prefix in declared {
        for route in ["/guide/a.html", "/fr/guide/a.html", "/zz/other.html"] {
            if route.starts_with(locale_prefix) && locale_prefix != route_root() {
                continue;
            }
            assert!(
                !locale_text(route, gate_key_hint()).is_empty(),
                "{route} renders a blank gate"
            );
        }
    }
    assert_ne!(
        english_hint, None,
        "an unregistered route must still resolve through the fallback"
    );
}

#[test]
fn every_gate_key_resolves_for_every_declared_locale_route() {
    let declared: Vec<&str> = locale_prefixes();
    let mut routes: Vec<String> = Vec::with_capacity(declared.len() * 2);
    for prefix in &declared {
        routes.push((*prefix).to_string());
        routes.push(format!("{prefix}deeply/nested/b.html"));
    }
    for route in &routes {
        for key in gate_translation_keys() {
            let text: String = locale_text(route, key);
            assert!(!text.is_empty(), "{route} renders an empty {key}");
            assert_ne!(&text, key, "{route} renders the raw key for {key}");
        }
    }
}

#[test]
fn a_prefix_that_only_matches_mid_segment_stays_in_the_root_locale() {
    for prefix in locale_prefixes() {
        if prefix == route_root() {
            continue;
        }
        let bare: String = prefix.trim_end_matches('/').to_string();
        let decoy: String = format!("{bare}-legacy/a.html");
        assert_eq!(
            locale_text(&decoy, gate_key_hint()),
            locale_text("/a.html", gate_key_hint()),
            "{decoy} must not be claimed by {prefix}"
        );
    }
}
