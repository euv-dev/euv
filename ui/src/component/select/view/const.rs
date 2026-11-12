/// The value an ARIA attribute carries when the state it reports is on.
pub(crate) const SELECT_ATTRIBUTE_ON: &str = "true";

/// The value an ARIA attribute carries when the state it reports is off.
pub(crate) const SELECT_ATTRIBUTE_OFF: &str = "false";

/// The value an attribute carries when there is nothing to report.
pub(crate) const SELECT_ATTRIBUTE_EMPTY: &str = "";

/// The glyph drawn by the Select scroll-up indicator.
pub(crate) const SELECT_SCROLL_UP_GLYPH: &str = "⌃";

/// The glyph drawn by the Select scroll-down indicator.
pub(crate) const SELECT_SCROLL_DOWN_GLYPH: &str = "⌄";

/// The check glyph shown by the selected Select option.
pub(crate) const SELECT_CHECKMARK: &str = "✓";

/// Selects the root that owns one Select's trigger and option panel.
pub(crate) const SELECT_ROOT_SELECTOR: &str = "[data-slot='select']";

/// Selects the trigger to restore focus to after a selection closes.
pub(crate) const SELECT_TRIGGER_SELECTOR: &str = "[data-slot='select-trigger']";

/// Selects every enabled option in one Select panel.
pub(crate) const SELECT_OPTION_SELECTOR: &str = "[data-slot='select-item']:not([disabled])";

/// The DOM attribute carrying the value of one Select option.
pub(crate) const SELECT_VALUE_ATTRIBUTE: &str = "data-value";

/// Selects the content panel whose position follows the trigger.
pub(crate) const SELECT_CONTENT_SELECTOR: &str = "[data-slot='select-content']";

/// Selects the scrollable viewport inside the Select content panel.
pub(crate) const SELECT_VIEWPORT_SELECTOR: &str = "[data-slot='select-viewport']";

/// The minimum viewport gap used before falling back to edge alignment.
pub(crate) const SELECT_ALIGNMENT_VIEWPORT_GUTTER_PX: f64 = 20.0;

/// The ArrowDown keyboard key name.
pub(crate) const SELECT_KEY_ARROW_DOWN: &str = "ArrowDown";

/// The ArrowUp keyboard key name.
pub(crate) const SELECT_KEY_ARROW_UP: &str = "ArrowUp";

/// The Home keyboard key name.
pub(crate) const SELECT_KEY_HOME: &str = "Home";

/// The End keyboard key name.
pub(crate) const SELECT_KEY_END: &str = "End";

/// The Enter keyboard key name.
pub(crate) const SELECT_KEY_ENTER: &str = "Enter";

/// The Space keyboard key name.
pub(crate) const SELECT_KEY_SPACE: &str = " ";

/// The Escape keyboard key name.
pub(crate) const SELECT_KEY_ESCAPE: &str = "Escape";

/// The keyboard typeahead prefix is cleared after this idle duration.
pub(crate) const SELECT_TYPEAHEAD_RESET_MS: i32 = 700;
