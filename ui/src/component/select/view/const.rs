/// The value an ARIA attribute carries when the state it reports is on.
pub(crate) const SELECT_ATTRIBUTE_ON: &str = "true";

/// The value an ARIA attribute carries when the state it reports is off.
pub(crate) const SELECT_ATTRIBUTE_OFF: &str = "false";

/// The value an attribute carries when there is nothing to report.
pub(crate) const SELECT_ATTRIBUTE_EMPTY: &str = "";

/// The tag name of Select's hidden native form controls.
pub(crate) const SELECT_FORM_INPUT_TAG_NAME: &str = "input";

/// The non-empty value used by the multiple-select required proxy.
pub(crate) const SELECT_FORM_VALIDATION_VALUE_SELECTED: &str = "selected";

/// The default size label emitted on a Select trigger.
pub(crate) const SELECT_SIZE_DEFAULT: &str = "default";

/// The compact size label emitted on a Select trigger.
pub(crate) const SELECT_SIZE_SM: &str = "sm";

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

/// The app root that carries the active theme tokens and remains outside the page scroll clip.
pub(crate) const SELECT_PORTAL_TARGET: &str = ".c_app_root";

/// The root attribute naming the Select's portaled content element.
pub(crate) const SELECT_PANEL_ID_ATTRIBUTE: &str = "data-select-panel-id";

/// The panel attribute naming the Select root that owns its interactions.
pub(crate) const SELECT_ROOT_ID_ATTRIBUTE: &str = "data-select-root-id";

/// The initial inline style that keeps an unpositioned Select panel hidden.
pub(crate) const SELECT_PANEL_STYLE_HIDDEN: &str = "position:fixed;visibility:hidden;";

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

/// The Tab keyboard key name.
pub(crate) const SELECT_KEY_TAB: &str = "Tab";

/// The document event used to dismiss an open Select from outside clicks.
#[cfg(target_arch = "wasm32")]
pub(crate) const SELECT_OUTSIDE_CLICK_EVENT: &str = "click";

/// Captures outside clicks before descendants can stop propagation.
#[cfg(target_arch = "wasm32")]
pub(crate) const SELECT_OUTSIDE_CLICK_CAPTURE: bool = true;

/// The captured document event that repositions an open Select after scrolling.
#[cfg(target_arch = "wasm32")]
pub(crate) const SELECT_SCROLL_EVENT: &str = "scroll";

/// The window event that repositions an open Select after resizing.
#[cfg(target_arch = "wasm32")]
pub(crate) const SELECT_RESIZE_EVENT: &str = "resize";

/// The keyboard typeahead prefix is cleared after this idle duration.
pub(crate) const SELECT_TYPEAHEAD_RESET_MS: i32 = 700;
