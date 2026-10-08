/// The `window` property name under which the page's single
/// `IntersectionObserver` instance is cached, so re-binding reuses the
/// live observer instead of stacking a second one on the same element.
pub(crate) const OBSERVER_WINDOW_PROPERTY_INSTANCE: &str = "__euv_observer_instance";

/// The `window` property name of the microtask debounce guard that
/// collapses several schedule requests into one `queueMicrotask` call.
pub(crate) const OBSERVER_WINDOW_PROPERTY_PENDING: &str = "__euv_observer_pending";

/// The `window` property name of the one-shot guard that keeps the
/// initial observer binding from being registered more than once.
pub(crate) const OBSERVER_WINDOW_PROPERTY_LISTENER: &str = "__euv_observer_listener";

/// The `data-` attribute carrying the zero-based index of an observed
/// list item, read off the intersection target.
pub(crate) const OBSERVER_ITEM_INDEX_ATTRIBUTE: &str = "data_index";

/// The CSS selector matching every observed list item, used to collect
/// the children whose count is reported alongside the intersection ratio.
pub(crate) const OBSERVER_ITEM_INDEX_SELECTOR: &str = "[data_index]";
