use super::*;

fn route(path: &str) -> NestedRouteConfig {
    NestedRouteConfig::new(path.to_string(), || VirtualNode::Empty, Vec::new())
}

fn parent(path: &str, children: Vec<NestedRouteConfig>) -> NestedRouteConfig {
    NestedRouteConfig::new(path.to_string(), || VirtualNode::Empty, children)
}

#[test]
fn the_root_path_normalizes_to_itself() {
    let observed: String = normalize_path("/");
    assert_eq!(observed, "/", "the root must keep its single slash");
}

#[test]
fn a_trailing_slash_is_stripped() {
    let observed: String = normalize_path("/settings/");
    assert_eq!(observed, "/settings", "one trailing slash is removed");
}

#[test]
fn several_trailing_slashes_are_all_stripped() {
    let observed: String = normalize_path("/settings///");
    assert_eq!(
        observed, "/settings",
        "every trailing slash is removed, not just one"
    );
}

#[test]
fn a_path_without_a_trailing_slash_is_unchanged() {
    let observed: String = normalize_path("/settings");
    assert_eq!(observed, "/settings", "nothing to strip means no change");
}

#[test]
fn an_empty_path_normalizes_to_empty() {
    let observed: String = normalize_path("");
    assert_eq!(observed, "", "the empty path stays empty");
}

#[test]
fn a_nested_path_keeps_its_interior_slashes() {
    let observed: String = normalize_path("/a/b/c/");
    assert_eq!(
        observed, "/a/b/c",
        "only the trailing slash is removed, interior ones stay"
    );
}

#[test]
fn an_exact_route_matches() {
    let observed: bool = route_matches("/about", "/about");
    assert!(observed, "an identical path is an exact match");
}

#[test]
fn a_trailing_slash_does_not_break_an_exact_match() {
    let observed: bool = route_matches("/about/", "/about");
    assert!(observed, "both sides are normalized before comparing");
}

#[test]
fn a_parent_route_matches_a_descendant_path() {
    let observed: bool = route_matches("/settings", "/settings/profile");
    assert!(observed, "a parent matches anything below it");
}

#[test]
fn a_parent_route_does_not_match_a_sibling_prefix() {
    let observed: bool = route_matches("/set", "/settings");
    assert!(
        !observed,
        "a partial segment prefix is not a parent relationship"
    );
}

#[test]
fn unrelated_routes_do_not_match() {
    let observed: bool = route_matches("/about", "/contact");
    assert!(!observed, "different paths are different routes");
}

#[test]
fn the_root_route_matches_any_non_empty_path() {
    let observed: bool = route_matches("/", "/anything/at/all");
    assert!(observed, "the root is a catch-all");
}

#[test]
fn the_root_route_does_not_match_the_empty_path() {
    let observed: bool = route_matches("/", "");
    assert!(!observed, "the catch-all excludes the empty path");
}

#[test]
fn a_deep_path_matches_every_level_of_its_ancestry() {
    let root: bool = route_matches("/", "/a/b/c");
    let middle: bool = route_matches("/a", "/a/b/c");
    let leaf: bool = route_matches("/a/b", "/a/b/c");
    assert!(
        root && middle && leaf,
        "each ancestor is a parent of the request"
    );
}

#[test]
fn find_active_route_returns_an_exact_match() {
    let routes: Vec<NestedRouteConfig> = vec![route("/home"), route("/about")];
    let observed: Option<&NestedRouteConfig> = find_active_route("/about", &routes);
    match observed {
        Some(found) => assert_eq!(found.path, "/about", "the matching route is found"),
        None => panic!("an exact match must be found"),
    }
}

#[test]
fn find_active_route_prefers_a_child_over_its_parent() {
    let routes: Vec<NestedRouteConfig> = vec![parent(
        "/settings",
        vec![route("/settings/profile"), route("/settings/billing")],
    )];
    let observed: Option<&NestedRouteConfig> = find_active_route("/settings/profile", &routes);
    match observed {
        Some(found) => assert_eq!(
            found.path, "/settings/profile",
            "the deeper exact match wins over the parent prefix match"
        ),
        None => panic!("the child match must be found"),
    }
}

#[test]
fn find_active_route_falls_back_to_the_parent_when_no_child_matches() {
    let routes: Vec<NestedRouteConfig> =
        vec![parent("/settings", vec![route("/settings/profile")])];
    let observed: Option<&NestedRouteConfig> = find_active_route("/settings/other", &routes);
    match observed {
        Some(found) => assert_eq!(
            found.path, "/settings",
            "with no child match the parent is the active route"
        ),
        None => panic!("the parent prefix must still match"),
    }
}

#[test]
fn find_active_route_is_none_when_nothing_matches() {
    let routes: Vec<NestedRouteConfig> = vec![route("/home"), route("/about")];
    let observed: Option<&NestedRouteConfig> = find_active_route("/missing", &routes);
    assert!(observed.is_none(), "an unknown path matches no route");
}

#[test]
fn find_active_route_is_none_for_an_empty_configuration() {
    let routes: Vec<NestedRouteConfig> = Vec::new();
    let observed: Option<&NestedRouteConfig> = find_active_route("/home", &routes);
    assert!(observed.is_none(), "no routes means no match");
}

#[test]
fn find_active_route_takes_the_first_of_several_exact_matches() {
    let routes: Vec<NestedRouteConfig> = vec![route("/home"), route("/home")];
    let observed: Option<&NestedRouteConfig> = find_active_route("/home", &routes);
    match observed {
        Some(found) => assert_eq!(
            found.path, "/home",
            "declaration order decides between identical routes"
        ),
        None => panic!("an exact match must be found"),
    }
}

#[test]
fn find_active_route_ignores_a_trailing_slash_on_the_request() {
    let routes: Vec<NestedRouteConfig> = vec![route("/about")];
    let observed: Option<&NestedRouteConfig> = find_active_route("/about/", &routes);
    assert!(
        observed.is_some(),
        "the request is normalized before matching"
    );
}

#[test]
fn route_chain_of_a_flat_route_has_one_entry() {
    let routes: Vec<NestedRouteConfig> = vec![route("/home")];
    let observed: Vec<&NestedRouteConfig> = route_chain("/home", &routes);
    assert_eq!(observed.len(), 1, "one level means one entry");
}

#[test]
fn route_chain_lists_the_ancestors_root_first() {
    let routes: Vec<NestedRouteConfig> = vec![parent(
        "/settings",
        vec![parent(
            "/settings/profile",
            vec![route("/settings/profile/name")],
        )],
    )];
    let observed: Vec<&NestedRouteConfig> = route_chain("/settings/profile/name", &routes);
    let paths: Vec<&str> = observed
        .iter()
        .map(|r: &&NestedRouteConfig| r.path.as_str())
        .collect();
    assert_eq!(
        paths,
        vec!["/settings", "/settings/profile", "/settings/profile/name"],
        "the chain runs outermost to innermost"
    );
}

#[test]
fn route_chain_stops_at_the_parent_when_no_child_matches() {
    let routes: Vec<NestedRouteConfig> =
        vec![parent("/settings", vec![route("/settings/profile")])];
    let observed: Vec<&NestedRouteConfig> = route_chain("/settings/other", &routes);
    assert_eq!(
        observed.len(),
        1,
        "only the matching ancestor is part of the chain"
    );
}

#[test]
fn route_chain_is_empty_when_nothing_matches() {
    let routes: Vec<NestedRouteConfig> = vec![route("/home")];
    let observed: Vec<&NestedRouteConfig> = route_chain("/missing", &routes);
    assert!(observed.is_empty(), "no match means no breadcrumb");
}

#[test]
fn a_new_route_keeps_its_path_and_starts_childless() {
    let observed: NestedRouteConfig = route("/home");
    assert_eq!(observed.path, "/home", "the path is stored verbatim");
    assert!(observed.children.is_empty(), "no children were supplied");
}

#[test]
fn a_new_route_keeps_its_children() {
    let observed: NestedRouteConfig = parent("/settings", vec![route("/settings/a")]);
    assert_eq!(observed.children.len(), 1, "the child is stored");
}

#[test]
fn the_children_accessor_returns_the_stored_children() {
    let built: NestedRouteConfig = parent(
        "/settings",
        vec![route("/settings/a"), route("/settings/b")],
    );
    let observed: &[NestedRouteConfig] = built.children();
    assert_eq!(observed.len(), 2, "both children come back");
}

#[test]
fn the_children_accessor_is_empty_for_a_leaf_route() {
    let built: NestedRouteConfig = route("/home");
    let observed: &[NestedRouteConfig] = built.children();
    assert!(observed.is_empty(), "a leaf has no children");
}

#[test]
fn the_component_accessor_returns_a_callable_builder() {
    let built: NestedRouteConfig = route("/home");
    let observed: Rc<dyn Fn() -> VirtualNode> = built.component();
    let node: VirtualNode = observed();
    assert!(
        matches!(node, VirtualNode::Empty),
        "the stored builder is the one that was handed in"
    );
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
#[cfg_attr(
    not(target_arch = "wasm32"),
    ignore = "navigate defers to a microtask, and spawn_local panics on a native target"
)]
fn navigating_on_a_host_with_no_window_is_a_no_op_rather_than_a_panic() {
    Router::navigate("/home");
    Router::navigate("/home");
    Router::navigate("");

    assert_eq!(
        Router::current_route(),
        String::new(),
        "with no window there is no hash to read and none to write, so the route stays where it \
         was. What matters is that reaching for the router at all - a link handler, a redirect \
         during mount - does not take the frame down on a host with no window"
    );
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
#[cfg_attr(
    not(target_arch = "wasm32"),
    ignore = "navigate defers to a microtask, and spawn_local panics on a native target"
)]
fn a_route_target_carrying_an_anchor_still_reads_as_its_bare_path() {
    Router::navigate("/settings#profile");

    assert_eq!(
        Router::current_route(),
        String::new(),
        "the anchor is carried through to the URL but stripped for the comparison, so a link \
         that scrolls to a section is not mistaken for a different route and skipped"
    );
}
