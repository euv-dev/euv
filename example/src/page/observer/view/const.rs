/// The CSS selector matching the observed container element, passed to
/// `use_intersection_observer`.
pub(crate) const OBSERVER_CONTAINER_SELECTOR: &str = "[data-observer-container]";

/// The value of the marker attribute identifying the observed container.
pub(crate) const OBSERVER_CONTAINER_ATTR_VALUE: &str = "true";

/// The header title shown at the top of the observer page.
pub(crate) const OBSERVER_HEADER_TITLE: &str = "Observer";

/// The header subtitle explaining the viewport intersection detection.
pub(crate) const OBSERVER_HEADER_SUBTITLE: &str = "Detect when elements enter or leave the viewport using the IntersectionObserver API. Open the browser console to see intersection events.";

/// The card title of the intersection observer demo.
pub(crate) const OBSERVER_INTERSECTION_CARD_TITLE: &str = "Intersection Observer";

/// The description of the observed container list.
pub(crate) const OBSERVER_CONTAINER_DESC: &str =
    "The container below is observed for viewport intersection changes.";

/// The hint telling the reader where the intersection events are logged.
pub(crate) const OBSERVER_CONSOLE_HINT: &str =
    "Open the browser console to see intersection events logged as you scroll.";
