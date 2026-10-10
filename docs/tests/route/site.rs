use super::*;

#[test]
fn every_generated_route_survives_every_spelling_mistake() {
    let site: RouteSite = route_site();
    let missing: Vec<String> = site.audit_all_route_variants();
    assert!(
        missing.is_empty(),
        "these spellings of a served route still resolve to no page:\n{}",
        missing.join("\n")
    );
}

#[test]
fn the_prefix_the_probe_uses_is_the_prefix_the_app_resolves() {
    let site: RouteSite = route_site();
    for page in site.get_pages() {
        let route: &str = page.get_route();
        let from_route_layer: &str = site.get_locale(route).get_prefix();
        let from_router: &str = locale_prefix_for(route);
        assert_eq!(
            from_route_layer, from_router,
            "{} resolves to two different locales",
            route
        );
    }
}

#[test]
fn stripping_the_prefix_reaches_a_route_outside_its_locale() {
    let site: RouteSite = route_site();
    for page in site.get_pages() {
        let route: &str = page.get_route();
        let prefix: &str = site.get_locale(route).get_prefix();
        if prefix == route_root() {
            continue;
        }
        let without: String = route_without_locale_prefix(route, prefix);
        assert!(
            without.starts_with(route_root()),
            "{route} does not strip to an absolute route"
        );
        assert!(
            normalize_route(&without) != normalize_route(route) || route == prefix,
            "{route} is owned by {prefix} yet stripping changed nothing"
        );
    }
}

#[test]
fn the_generated_site_has_routes_to_audit() {
    let site: RouteSite = route_site();
    assert!(
        site.pages.len() > 1,
        "the audit only means something when build.rs emitted a real tree"
    );
    for page in site.pages {
        assert_eq!(
            normalize_route(page.get_route()),
            page.get_route(),
            "{} is not in canonical form",
            page.get_route()
        );
    }
}
