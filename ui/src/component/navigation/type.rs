use super::*;

/// Shared immutable list of navigation route paths used by O(1) active-route selectors.
pub(crate) type RoutePathList = Rc<[String]>;
