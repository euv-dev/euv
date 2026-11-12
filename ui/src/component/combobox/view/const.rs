/// Selects a Combobox root from a trigger or option event.
pub(crate) const COMBOBOX_ROOT_SELECTOR: &str = "[data-slot='combobox']";

/// Selects enabled options in one Combobox.
pub(crate) const COMBOBOX_ITEM_SELECTOR: &str =
    "[data-slot='combobox-item']:not([aria-disabled='true'])";

/// Selects the Combobox text field for restoring focus.
pub(crate) const COMBOBOX_INPUT_SELECTOR: &str = "[data-slot='combobox-input']";

/// The attribute holding one option's stable value.
pub(crate) const COMBOBOX_VALUE_ATTRIBUTE: &str = "data-value";

/// The ArrowDown keyboard key name.
pub(crate) const COMBOBOX_KEY_ARROW_DOWN: &str = "ArrowDown";

/// The ArrowUp keyboard key name.
pub(crate) const COMBOBOX_KEY_ARROW_UP: &str = "ArrowUp";

/// The Home keyboard key name.
pub(crate) const COMBOBOX_KEY_HOME: &str = "Home";

/// The End keyboard key name.
pub(crate) const COMBOBOX_KEY_END: &str = "End";

/// The Enter keyboard key name.
pub(crate) const COMBOBOX_KEY_ENTER: &str = "Enter";

/// The Space keyboard key name.
pub(crate) const COMBOBOX_KEY_SPACE: &str = " ";

/// The Escape keyboard key name.
pub(crate) const COMBOBOX_KEY_ESCAPE: &str = "Escape";
