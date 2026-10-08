/// Viewport width threshold below which the application switches to the mobile layout.
pub(crate) const MOBILE_BREAKPOINT: i32 = 768;

/// The hash prefix used for hash-based routing.
pub(crate) const ROUTE_HASH_PREFIX: &str = "#";

/// The default route path when no hash is present.
pub(crate) const DEFAULT_ROUTE_PATH: &str = "/";

/// The DOM event type bound to an internal route link.
pub(crate) const ROUTER_LINK_EVENT_TYPE: &str = "click";
