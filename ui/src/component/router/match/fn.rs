use super::*;

/// Normalizes a path by stripping a trailing `/` (except
/// for the root path `/`, which stays as `/`).
///
/// # Arguments
///
/// - `&str` - Shared reference to a `str`.
///
/// # Returns
///
/// - `String` - A `String` value.
pub fn normalize_path(path: &str) -> String {
    if path == "/" {
        return "/".to_string();
    }
    path.trim_end_matches('/').to_string()
}

/// Returns `true` if `path` matches the route's pattern.
///
/// Matching rules:
/// - Exact match after normalization wins.
/// - For a parent route with children, a path that
///   starts with `route.path + "/"` matches the parent
///   (so `/settings` matches `/settings/profile`).
/// - The root route `/` matches everything except the
///   empty string.
///
/// # Arguments
///
/// - `&str` - Shared reference to a `str`.
/// - `&str` - Shared reference to a `str`.
///
/// # Returns
///
/// - `bool` - A boolean.
pub fn route_matches(route_path: &str, request_path: &str) -> bool {
    let normalized_route: String = normalize_path(route_path);
    let normalized_request: String = normalize_path(request_path);
    if normalized_route == normalized_request {
        return true;
    }
    // Parent-route match: request starts with
    // route_path + "/".
    if normalized_request.starts_with(&normalized_route)
        && normalized_request.chars().nth(normalized_route.len()) == Some('/')
    {
        return true;
    }
    // Root catch-all.
    if normalized_route == "/" && !normalized_request.is_empty() {
        return true;
    }
    false
}

/// Recursively finds the deepest matching route in the
/// configuration tree.
///
/// Exact matches on children win over parent-prefix
/// matches. If multiple children match, the first one
/// in declaration order wins (matching `euv_routes`'s
/// existing "first match wins" semantics).
///
/// Returns `None` if no route matches.
///
/// # Arguments
///
/// - `&str` - Shared reference to a `str`.
/// - `&'a [NestedRouteConfig]` - Shared reference to a `'a [NestedRouteConfig]`.
///
/// # Returns
///
/// - `Option<'a NestedRouteConfig>` - `Some(...)` on success, `None` otherwise.
pub fn find_active_route<'a>(
    path: &str,
    routes: &'a [NestedRouteConfig],
) -> Option<&'a NestedRouteConfig> {
    // First pass: exact match wins.
    for route in routes.iter() {
        let normalized: String = normalize_path(&route.path);
        let normalized_request: String = normalize_path(path);
        if normalized == normalized_request {
            // Try to find a deeper match in the
            // children of this route.
            if !route.children.is_empty()
                && let Some(child_match) = find_active_route(path, &route.children)
            {
                return Some(child_match);
            }
            return Some(route);
        }
    }
    // Second pass: parent-prefix match. Walk all
    // routes; for each, if it could be a parent of the
    // request, recurse into its children. If a child
    // matches, return the child. If no child matches,
    // return the parent.
    for route in routes.iter() {
        if route_matches(&route.path, path) && route.path != normalize_path(path) {
            if !route.children.is_empty()
                && let Some(child_match) = find_active_route(path, &route.children)
            {
                return Some(child_match);
            }
            return Some(route);
        }
    }
    None
}

/// Returns the chain of routes from the root to the
/// matched route, in render order (root first).
///
/// This is the "breadcrumb" chain that the layout
/// component walks to render `<Outlet>` for each parent.
/// Returns an empty vec if no route matches.
///
/// # Arguments
///
/// - `&str` - Shared reference to a `str`.
/// - `&'a [NestedRouteConfig]` - Shared reference to a `'a [NestedRouteConfig]`.
///
/// # Returns
///
/// - `Vec<'a NestedRouteConfig>` - A `Vec<'a NestedRouteConfig>` value.
pub fn route_chain<'a>(path: &str, routes: &'a [NestedRouteConfig]) -> Vec<&'a NestedRouteConfig> {
    let mut chain: Vec<&'a NestedRouteConfig> = Vec::new();
    build_chain(path, routes, &mut chain);
    chain
}

/// Recursively walks the route tree looking for the deepest
/// match for `path`. Each matching route is appended to
/// `chain` in nesting order, so the layout component can
/// render `<Outlet>` for every parent. Returns `true` as
/// soon as a match is appended so callers can short-circuit
/// the second pass.
///
/// # Arguments
///
/// - `&str` - The current request path.
/// - `&'a [NestedRouteConfig]` - The slice of routes to search.
/// - `&mut Vec<&'a NestedRouteConfig>` - Out-parameter that
///   accumulates the matched parent chain.
///
/// # Returns
///
/// - `bool` - `true` when a route was appended to `chain`,
///   `false` otherwise.
pub(crate) fn build_chain<'a>(
    path: &str,
    routes: &'a [NestedRouteConfig],
    chain: &mut Vec<&'a NestedRouteConfig>,
) -> bool {
    // First pass: exact match.
    for route in routes.iter() {
        let normalized: String = normalize_path(&route.path);
        let normalized_request: String = normalize_path(path);
        if normalized == normalized_request {
            chain.push(route);
            // Recurse into children for deeper matches.
            if !route.children.is_empty() {
                let _ = build_chain(path, &route.children, chain);
            }
            return true;
        }
    }
    // Second pass: parent-prefix match.
    for route in routes.iter() {
        if route_matches(&route.path, path) && route.path != normalize_path(path) {
            chain.push(route);
            if !route.children.is_empty() {
                let _ = build_chain(path, &route.children, chain);
            }
            return true;
        }
    }
    false
}
