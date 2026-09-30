use super::*;

/// A nested route configuration entry.
///
/// Routes form a tree: each route can have its own
/// component (the "layout" or "page") and zero or more
/// child routes. The path of a child route is resolved
/// against the parent path during matching.
#[derive(Clone, CustomDebug, Data)]
pub struct NestedRouteConfig {
    /// The route path (e.g. `/settings/profile`).
    pub path: String,
    /// The component to render for this route.
    #[debug(skip)]
    pub component: Rc<dyn Fn() -> VirtualNode>,
    /// The child routes, if any.
    pub children: Vec<NestedRouteConfig>,
}
