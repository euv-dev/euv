/// The media query reporting the user's reduced-motion preference.
pub(crate) const REDUCED_MOTION_QUERY: &str = "(prefers-reduced-motion: reduce)";

/// The media-query-list event fired when a query's match result changes.
#[cfg(target_arch = "wasm32")]
pub(crate) const EVENT_REDUCED_MOTION_CHANGE: &str = "change";
