use super::*;

fn empty_node() -> VirtualNode {
    VirtualNode::Empty
}

fn route(path: &str, children: Vec<NestedRouteConfig>) -> NestedRouteConfig {
    NestedRouteConfig {
        path: path.to_string(),
        component: Rc::new(empty_node),
        children,
    }
}

#[test]
fn normalizing_a_path_drops_a_trailing_slash() {
    assert_eq!(normalize_path("/settings/"), "/settings");
    assert_eq!(normalize_path("/settings///"), "/settings");
    assert_eq!(normalize_path("/settings"), "/settings");
}

#[test]
fn normalizing_the_root_keeps_its_slash() {
    assert_eq!(
        normalize_path("/"),
        "/",
        "trimming the root's only slash would leave an empty path that matches nothing"
    );
}

#[test]
fn normalizing_an_empty_path_stays_empty() {
    assert_eq!(normalize_path(""), "");
}

#[test]
fn a_route_matches_itself_exactly() {
    assert!(route_matches("/settings", "/settings"));
    assert!(
        route_matches("/settings", "/settings/"),
        "a trailing slash is not a different route"
    );
}

#[test]
fn a_parent_route_matches_its_descendants() {
    assert!(route_matches("/settings", "/settings/profile"));
    assert!(route_matches("/settings", "/settings/a/b/c"));
}

#[test]
fn a_parent_route_does_not_match_a_sibling_with_a_shared_prefix() {
    assert!(
        !route_matches("/set", "/settings"),
        "the boundary check must require a slash, not just a string prefix"
    );
    assert!(!route_matches("/settings", "/settings-archive"));
    assert!(!route_matches("/settings", "/settings2/profile"));
}

#[test]
fn the_root_route_matches_every_non_empty_path() {
    assert!(route_matches("/", "/anything/at/all"));
    assert!(
        !route_matches("/", ""),
        "an empty request is not a navigation target"
    );
}

#[test]
fn find_active_route_prefers_the_exact_child_over_its_parent() {
    let routes: Vec<NestedRouteConfig> = vec![route(
        "/settings",
        vec![route("/settings/profile", vec![])],
    )];
    let found: Option<&NestedRouteConfig> = find_active_route("/settings/profile", &routes);
    assert_eq!(
        found.map(|r: &NestedRouteConfig| r.path.as_str()),
        Some("/settings/profile")
    );
}

#[test]
fn find_active_route_falls_back_to_the_parent_when_no_child_matches() {
    let routes: Vec<NestedRouteConfig> = vec![route(
        "/settings",
        vec![route("/settings/profile", vec![])],
    )];
    let found: Option<&NestedRouteConfig> = find_active_route("/settings", &routes);
    assert_eq!(
        found.map(|r: &NestedRouteConfig| r.path.as_str()),
        Some("/settings")
    );
}

#[test]
fn find_active_route_returns_the_first_of_several_matching_siblings() {
    let routes: Vec<NestedRouteConfig> = vec![
        route("/a", vec![route("/a/x", vec![])]),
        route("/a", vec![route("/a/x", vec![])]),
    ];
    let found: Option<&NestedRouteConfig> = find_active_route("/a/x", &routes);
    assert!(
        found.is_some(),
        "first match wins, so this must resolve rather than give up"
    );
}

#[test]
fn find_active_route_returns_none_when_nothing_matches() {
    let routes: Vec<NestedRouteConfig> = vec![route("/settings", vec![])];
    assert!(find_active_route("/elsewhere", &routes).is_none());
    assert!(find_active_route("/anything", &[]).is_none());
}

#[test]
fn the_route_chain_lists_every_ancestor_in_nesting_order() {
    let routes: Vec<NestedRouteConfig> = vec![route(
        "/app",
        vec![route(
            "/app/settings",
            vec![route("/app/settings/profile", vec![])],
        )],
    )];
    let chain: Vec<&NestedRouteConfig> = route_chain("/app/settings/profile", &routes);
    let mut paths: Vec<&str> = Vec::new();
    for entry in &chain {
        paths.push(entry.path.as_str());
    }
    assert_eq!(
        paths,
        vec!["/app", "/app/settings", "/app/settings/profile"],
        "a layout renders an Outlet for each parent, so order is parent-first"
    );
}

#[test]
fn the_route_chain_of_a_leaf_is_just_that_route() {
    let routes: Vec<NestedRouteConfig> = vec![route("/solo", vec![])];
    let chain: Vec<&NestedRouteConfig> = route_chain("/solo", &routes);
    assert_eq!(chain.len(), 1);
    assert_eq!(chain[0].path, "/solo");
}

#[test]
fn the_route_chain_is_empty_when_nothing_matches() {
    let routes: Vec<NestedRouteConfig> = vec![route("/settings", vec![])];
    assert!(route_chain("/elsewhere", &routes).is_empty());
}

#[test]
fn a_progress_percent_is_clamped_into_the_zero_to_hundred_range() {
    assert_eq!(progress_percent_clamp(0.0), 0.0);
    assert_eq!(progress_percent_clamp(50.0), 50.0);
    assert_eq!(progress_percent_clamp(100.0), 100.0);
    assert_eq!(progress_percent_clamp(-20.0), 0.0);
    assert_eq!(progress_percent_clamp(140.0), PERCENT_MAX);
}

#[test]
fn a_nan_progress_percent_becomes_zero_rather_than_propagating() {
    let clamped: f64 = progress_percent_clamp(f64::NAN);
    assert_eq!(clamped, 0.0);
    assert!(!clamped.is_nan(), "clamp leaves NaN alone, so it must be special-cased");
}

#[test]
fn each_space_size_maps_to_its_own_class_name() {
    assert_eq!(EuvSpaceSize::Xs.token(), "space-xs");
    assert_eq!(EuvSpaceSize::Sm.token(), "space-sm");
    assert_eq!(EuvSpaceSize::Md.token(), "space-md");
    assert_eq!(EuvSpaceSize::Lg.token(), "space-lg");
    assert_eq!(EuvSpaceSize::Xl.token(), "space-xl");
}

#[test]
fn each_icon_size_maps_to_its_edge_length_in_pixels() {
    assert_eq!(EuvIconSize::Xs.px(), 12);
    assert_eq!(EuvIconSize::Sm.px(), 16);
    assert_eq!(EuvIconSize::Md.px(), 20);
    assert_eq!(EuvIconSize::Lg.px(), 24);
    assert_eq!(EuvIconSize::Xl.px(), 32);
}

#[test]
fn each_avatar_size_maps_to_its_edge_length_in_pixels() {
    assert_eq!(EuvAvatarSize::Small.px(), 24);
    assert_eq!(EuvAvatarSize::Medium.px(), 32);
    assert_eq!(EuvAvatarSize::Large.px(), 40);
}

#[test]
fn each_rating_size_maps_to_its_star_edge_length_in_pixels() {
    assert_eq!(EuvRatingSize::Sm.px(), 14);
    assert_eq!(EuvRatingSize::Md.px(), 18);
    assert_eq!(EuvRatingSize::Lg.px(), 24);
}
