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
