/// The value an ARIA attribute carries when the state it reports is on.
pub(crate) const TOGGLE_GROUP_ATTRIBUTE_ON: &str = "true";

/// The value an ARIA attribute carries when the state it reports is off.
pub(crate) const TOGGLE_GROUP_ATTRIBUTE_OFF: &str = "false";

/// The value an attribute carries when there is nothing to report.
pub(crate) const TOGGLE_GROUP_ATTRIBUTE_EMPTY: &str = "";

/// Selects the toggle group root for keyboard focus navigation.
pub(crate) const TOGGLE_GROUP_ROOT_SELECTOR: &str = "[data-slot='toggle-group']";

/// Selects enabled entries in one toggle group.
pub(crate) const TOGGLE_GROUP_TRIGGER_SELECTOR: &str =
    "[data-slot='toggle-group-item']:not([disabled])";

/// The DOM attribute holding the stable value of an entry.
pub(crate) const TOGGLE_GROUP_VALUE_ATTRIBUTE: &str = "data-value";
