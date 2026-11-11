/// Scheme prefix marking a navbar link that opens outside the app.
pub(crate) const NAVBAR_LINK_SCHEME_HTTP_PREFIX: &str = "http";

pub(crate) const MENUBAR_TRIGGER_SELECTOR: &str = "[data-slot=menubar-trigger]";
pub(crate) const MENUBAR_CONTENT_ID_SUFFIX: &str = "-content";
pub(crate) const MENUBAR_CONTENT_SELECTOR: &str = "[data-slot=menubar-content]";
pub(crate) const MENUBAR_ITEM_SELECTOR: &str = "[role^=menuitem]";
pub(crate) const MENUBAR_ROOT_SELECTOR: &str = "[role=menubar]";
#[cfg(target_arch = "wasm32")]
pub(crate) const MENUBAR_SUB_CONTENT_SELECTOR: &str = "[data-slot=menubar-sub-content]";
pub(crate) const MENUBAR_TRIGGER_ID_PREFIX: &str = "euv-menubar-";
pub(crate) const MENUBAR_STATE_OPEN: &str = "open";
pub(crate) const MENUBAR_STATE_CLOSED: &str = "closed";
/// The expanded-state attribute name for menubar triggers.
pub(crate) const MENUBAR_ARIA_EXPANDED_ATTRIBUTE: &str = "aria-expanded";
/// The disabled-state attribute name for menubar items.
pub(crate) const MENUBAR_ARIA_DISABLED_ATTRIBUTE: &str = "aria-disabled";
/// The native disabled attribute name for interactive elements.
pub(crate) const MENUBAR_DISABLED_ATTRIBUTE: &str = "disabled";
/// The optional disabled-state marker name for menu items.
pub(crate) const MENUBAR_DATA_DISABLED_ATTRIBUTE: &str = "data-disabled";
/// The string value for an enabled ARIA state.
pub(crate) const MENUBAR_TRUE_VALUE: &str = "true";
/// The string value for a disabled ARIA state.
pub(crate) const MENUBAR_FALSE_VALUE: &str = "false";
/// The right-arrow keyboard key name.
pub(crate) const MENUBAR_KEY_ARROW_RIGHT: &str = "ArrowRight";
/// The left-arrow keyboard key name.
pub(crate) const MENUBAR_KEY_ARROW_LEFT: &str = "ArrowLeft";
/// The down-arrow keyboard key name.
pub(crate) const MENUBAR_KEY_ARROW_DOWN: &str = "ArrowDown";
/// The up-arrow keyboard key name.
pub(crate) const MENUBAR_KEY_ARROW_UP: &str = "ArrowUp";
/// The Home keyboard key name.
pub(crate) const MENUBAR_KEY_HOME: &str = "Home";
/// The End keyboard key name.
pub(crate) const MENUBAR_KEY_END: &str = "End";
/// The Escape keyboard key name.
pub(crate) const MENUBAR_KEY_ESCAPE: &str = "Escape";
/// The role value for a normal menu item.
pub(crate) const MENUBAR_ROLE_MENU_ITEM: &str = "menuitem";
pub(crate) const MENUBAR_SUBMENU_INDICATOR: &str = "›";
