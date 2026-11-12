/// Selects every enabled accordion trigger inside one accordion root.
pub(crate) const ACCORDION_TRIGGER_SELECTOR: &str =
    "[data-slot='accordion-trigger']:not([disabled])";

/// Selects an accordion root from one of its keyboard-event targets.
pub(crate) const ACCORDION_ROOT_SELECTOR: &str = "[data-slot='accordion']";

/// The DOM attribute containing the stable value of a section trigger.
pub(crate) const ACCORDION_VALUE_ATTRIBUTE: &str = "data-value";
