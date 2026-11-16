/// The tab selector shared by keyboard focus handling.
pub(crate) const TAB_SELECTOR: &str = "[role='tab']";
/// The nearest owning list used for direction and focus navigation.
pub(crate) const TAB_LIST_SELECTOR: &str = "[role='tablist']";
/// The effective CSS direction property.
pub(crate) const TAB_DIRECTION_PROPERTY: &str = "direction";
/// The CSS value that reverses horizontal arrow navigation.
pub(crate) const TAB_DIRECTION_RTL: &str = "rtl";
/// The horizontal forward key in left-to-right lists.
pub(crate) const TAB_KEY_RIGHT: &str = "ArrowRight";
/// The horizontal backward key in left-to-right lists.
pub(crate) const TAB_KEY_LEFT: &str = "ArrowLeft";
/// The DOM-first navigation key.
pub(crate) const TAB_KEY_HOME: &str = "Home";
/// The DOM-last navigation key.
pub(crate) const TAB_KEY_END: &str = "End";
/// The explicit keyboard activation key.
pub(crate) const TAB_KEY_ENTER: &str = "Enter";
/// The explicit activation key whose default can scroll the page.
pub(crate) const TAB_KEY_SPACE: &str = " ";
