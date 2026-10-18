use super::*;

static LOCALES: [RouteLocale; 3] = [
    RouteLocale { prefix: "/" },
    RouteLocale { prefix: "/zh/" },
    RouteLocale { prefix: "/en/" },
];

static PAGES: [RoutePage; 6] = [
    RoutePage { route: "/" },
    RoutePage { route: "/zh/" },
    RoutePage {
        route: "/zh/guide/",
    },
    RoutePage {
        route: "/zh/guide/intro.html",
    },
    RoutePage {
        route: "/zh/guide/deep/nested.html",
    },
    RoutePage {
        route: "/en/guide/intro.html",
    },
];

fn site() -> RouteSite {
    RouteSite::new(&LOCALES, &PAGES)
}

#[test]
fn normalize_collapses_repeated_separators() {
    assert_eq!(
        normalize_route("//zh//guide//intro.html"),
        "/zh/guide/intro.html"
    );
}

#[test]
fn normalize_collapses_trailing_separators_to_one() {
    assert_eq!(normalize_route("/zh/guide//"), "/zh/guide/");
}

#[test]
fn normalize_does_not_add_a_separator_a_leaf_route_lacks() {
    assert_eq!(
        normalize_route("/zh/guide"),
        "/zh/guide",
        "adding a separator would turn a leaf request into a directory route"
    );
}

#[test]
fn normalize_keeps_the_root_as_the_root() {
    assert_eq!(normalize_route("///"), "/");
}

#[test]
fn normalize_maps_the_empty_route_onto_the_root() {
    assert_eq!(normalize_route(""), "/");
}

#[test]
fn normalize_gives_a_relative_route_a_leading_separator() {
    assert_eq!(
        normalize_route("zh/guide/intro.html"),
        "/zh/guide/intro.html"
    );
}

#[test]
fn normalize_preserves_the_html_suffix_and_case() {
    assert_eq!(normalize_route("/zh/A/B.html"), "/zh/A/B.html");
}

#[test]
fn stem_strips_the_html_suffix_only() {
    assert_eq!(route_stem("/zh/a/b.html"), "/zh/a/b");
}

#[test]
fn stem_keeps_a_route_that_has_no_html_suffix() {
    assert_eq!(route_stem("/zh/a/"), "/zh/a/");
}

#[test]
fn stem_of_a_bare_html_suffix_is_unchanged() {
    assert_eq!(route_stem(".html"), ".html");
}

#[test]
fn candidates_offer_the_stem_and_its_directory_form() {
    assert_eq!(
        route_candidates("/zh/guide/intro.html"),
        vec![
            "/zh/guide/intro".to_string(),
            "/zh/guide/intro/".to_string(),
            "/zh/guide/intro.html".to_string(),
        ]
    );
}

#[test]
fn candidates_of_a_directory_route_keep_it_and_add_the_file_form() {
    assert_eq!(
        route_candidates("/zh/guide/"),
        vec![
            "/zh/guide".to_string(),
            "/zh/guide/".to_string(),
            "/zh/guide.html".to_string(),
        ],
        "the directory route and the file route are the two shapes a link can name"
    );
}

#[test]
fn candidates_offer_the_directory_and_html_forms_for_a_stem() {
    assert_eq!(
        route_candidates("/zh/guide/intro"),
        vec![
            "/zh/guide/intro".to_string(),
            "/zh/guide/intro/".to_string(),
            "/zh/guide/intro.html".to_string(),
        ]
    );
}

#[test]
fn candidates_never_repeat_a_spelling() {
    for path in ["/", "/zh/", "/zh/a.html", "/zh/a/"] {
        let listed: Vec<String> = route_candidates(path);
        let unique: HashSet<&String> = listed.iter().collect();
        assert_eq!(
            listed.len(),
            unique.len(),
            "{path} produced a duplicate candidate spelling"
        );
    }
}

#[test]
fn an_exact_route_resolves_to_itself() {
    assert_eq!(
        site()
            .find_page("/zh/guide/intro.html")
            .map(RoutePage::get_route),
        Some("/zh/guide/intro.html")
    );
}

#[test]
fn a_route_missing_the_html_suffix_resolves() {
    assert_eq!(
        site()
            .find_page("/zh/guide/intro")
            .map(RoutePage::get_route),
        Some("/zh/guide/intro.html")
    );
}

#[test]
fn a_leaf_route_missing_its_trailing_separator_resolves() {
    assert_eq!(
        site()
            .find_page("/zh/guide/intro.html/")
            .map(RoutePage::get_route),
        Some("/zh/guide/intro.html")
    );
}

#[test]
fn a_directory_route_missing_its_trailing_separator_resolves() {
    assert_eq!(
        site().find_page("/zh/guide").map(RoutePage::get_route),
        Some("/zh/guide/")
    );
}

#[test]
fn a_route_with_a_duplicated_separator_resolves() {
    assert_eq!(
        site()
            .find_page("//zh//guide/intro.html")
            .map(RoutePage::get_route),
        Some("/zh/guide/intro.html")
    );
}

#[test]
fn a_route_with_a_doubled_html_suffix_resolves() {
    assert_eq!(
        site()
            .find_page("/zh/guide/intro.html.html")
            .map(RoutePage::get_route),
        Some("/zh/guide/intro.html"),
        "a suffix appended twice is the same shape error as one appended once"
    );
}

#[test]
fn a_leaf_route_missing_its_separator_still_resolves_after_normalisation() {
    assert_eq!(
        site()
            .find_page("/zh/guide/intro.html//")
            .map(RoutePage::get_route),
        Some("/zh/guide/intro.html")
    );
}

#[test]
fn a_route_under_an_undeclared_prefix_does_not_resolve() {
    assert_eq!(
        site().find_page("/fr/guide/intro.html"),
        None,
        "a prefix the site never served is not a locale the gate can guess"
    );
}

#[test]
fn a_route_without_its_locale_prefix_resolves_only_when_the_root_locale_serves_it() {
    assert_eq!(
        site().find_page("/guide/intro.html"),
        None,
        "the root locale of this fixture serves no intro page"
    );
}

#[test]
fn locale_picks_the_longest_matching_prefix() {
    let owner: Option<&RouteLocale> = site().locale_of("/zh/guide/intro.html");
    assert_eq!(
        owner.map(|locale: &RouteLocale| locale.get_prefix()),
        Some("/zh/")
    );
}

#[test]
fn locale_falls_back_to_the_root_for_a_root_route() {
    let owner: Option<&RouteLocale> = site().locale_of("/guide/intro.html");
    assert_eq!(
        owner.map(|locale: &RouteLocale| locale.get_prefix()),
        Some("/")
    );
}

#[test]
fn locale_matches_a_prefix_only_at_a_segment_boundary() {
    let owner: Option<&RouteLocale> = site().locale_of("/zh-legacy/guide/");
    assert_eq!(
        owner.map(|locale: &RouteLocale| locale.get_prefix()),
        Some("/"),
        "a locale whose name shares a prefix with another route must not claim it"
    );
}

#[test]
fn variants_cover_the_shapes_a_link_author_misses() {
    let variants: Vec<String> = route_variants("/zh/guide/intro.html");
    assert_eq!(variants.len(), route_variant_capacity());
    assert!(variants.contains(&"/zh/guide/intro.html".to_string()));
    assert!(variants.contains(&"/zh/guide/intro".to_string()));
    assert!(variants.contains(&"/zh/guide/intro/".to_string()));
    assert!(variants.contains(&"/zh/guide/intro.html/".to_string()));
    assert!(variants.contains(&"//zh//guide//intro.html".to_string()));
    assert!(variants.contains(&"//zh/guide/intro.html".to_string()));
    assert!(variants.contains(&"/zh/guide/intro.html.html".to_string()));
}

#[test]
fn variants_never_repeat_a_spelling() {
    for route in ["/", "/zh/", "/zh/a.html", "/zh/a/"] {
        let listed: Vec<String> = route_variants(route);
        let unique: HashSet<&String> = listed.iter().collect();
        assert_eq!(
            listed.len(),
            unique.len(),
            "{route} produced a duplicate variant"
        );
    }
}

#[test]
fn dropping_the_locale_prefix_is_a_probe_rather_than_a_normalisation() {
    assert_eq!(
        route_without_locale_prefix("/zh/guide/intro.html", "/zh/"),
        "/guide/intro.html"
    );
    assert_eq!(
        route_without_locale_prefix("/zh-legacy/guide/", "/zh/"),
        "/zh-legacy/guide/",
        "a prefix that only matches mid-segment is not a locale prefix"
    );
    assert_eq!(
        route_without_locale_prefix("/guide/intro.html", "/"),
        "/guide/intro.html",
        "the root locale owns every route, so nothing is stripped"
    );
}

#[test]
fn every_variant_that_should_resolve_reaches_the_original_page() {
    let audited: Vec<(String, bool)> = site().audit_route_variants("/zh/guide/intro.html");
    for (variant, resolved) in audited {
        assert!(
            resolved,
            "{variant} is a spelling of a served route and must not 404"
        );
    }
}

#[test]
fn auditing_the_whole_site_reports_no_route_as_missing() {
    assert_eq!(
        site().audit_all_route_variants(),
        Vec::<String>::new(),
        "every spelling of a served route must resolve"
    );
}

#[test]
fn a_raw_and_a_percent_encoded_heading_name_the_same_anchor() {
    assert!(
        same_route_anchor(
            "#/euv/macros/html/html.html#%E8%AF%AD%E6%B3%95%E9%AA%A8%E6%9E%B6",
            "#/euv/macros/html/html.html#\u{8BED}\u{6CD5}\u{9AA8}\u{67B6}"
        ),
        "the spy writes a raw slug and the browser reads it back percent-encoded, \
so this equality is what stops it from re-reading its own write as an \
external change every tick"
    );
}

#[test]
fn a_percent_encoded_heading_decodes_to_the_slug_the_heading_carries() {
    assert_eq!(
        parse_route("/a.html#%E8%AF%AD%E6%B3%95").1,
        Some("\u{8BED}\u{6CD5}".to_string()),
        "the decoded form is what `getElementById` is queried with"
    );
}

#[test]
fn an_escape_that_spells_itself_is_decoded_rather_than_kept() {
    assert_eq!(
        parse_route("/a.html#%23%2F%20b").1,
        Some("#/ b".to_string()),
        "each escape is decoded once, and '+' is not folded to a space"
    );
}

#[test]
fn an_ascii_heading_matches_itself_in_both_spellings() {
    assert!(same_route_anchor(
        "#/euv/macros/html/html.html#portal",
        "#/euv/macros/html/html.html#portal"
    ));
}

#[test]
fn two_routes_sharing_a_heading_slug_are_different_anchors() {
    assert!(
        !same_route_anchor("#/a.html#\u{6982}\u{89C8}", "#/b.html#\u{6982}\u{89C8}"),
        "a route change re-arms the fragment intent even when the heading repeats"
    );
}

#[test]
fn two_bare_route_fragments_match_only_their_own_route() {
    assert!(same_route_anchor("#/a.html", "#/a.html"));
    assert!(
        !same_route_anchor("#/a.html", "#/b.html"),
        "a bare fragment carries no anchor, so the page path is its whole identity"
    );
}

#[test]
fn a_bare_route_fragment_never_matches_a_heading_on_the_same_page() {
    assert!(!same_route_anchor("#/a.html", "#/a.html#\u{6982}\u{89C8}"));
}

#[test]
fn a_hash_inside_the_anchor_never_splits_the_route_again() {
    assert_eq!(
        parse_route("/a.html#one#two").0,
        "/a.html".to_string(),
        "only the first '#' separates the page from its anchor"
    );
    assert_eq!(
        parse_route("/a.html#one#two").1,
        Some("one#two".to_string()),
        "a later '#' belongs to the heading text"
    );
}

#[test]
fn an_undecodable_heading_is_kept_verbatim_rather_than_dropped() {
    assert_eq!(
        parse_route("/a.html#%E4%").1,
        Some("%E4%".to_string()),
        "a truncated escape still compares as one anchor instead of vanishing"
    );
}

#[test]
fn a_malformed_heading_matches_itself() {
    assert!(same_route_anchor("#/a.html#%E4%", "#/a.html#%E4%"));
}

#[test]
fn nested_slugs_of_one_page_stay_distinct_anchors() {
    assert!(!same_route_anchor(
        "#/a.html#section",
        "#/a.html#section-nested"
    ));
}
