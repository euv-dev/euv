/// The debounce interval in milliseconds for the resize event handler.
pub(crate) const RESIZE_DEBOUNCE_MILLIS: i32 = 16;

/// The `window.open` target name that forces the URL to open in the
/// system browser rather than a web-view tab.
pub(crate) const SYSTEM_BROWSER_TARGET: &str = "_system";

/// CSS selector for the mobile navigation drawer element.
pub(crate) const DRAWER_NAV_SELECTOR: &str = "nav.c_mobile_nav_drawer";

/// CSS selector for the currently active navigation item link inside the mobile drawer.
pub(crate) const ACTIVE_NAV_ITEM_SELECTOR: &str = ".c_nav_item_active";

/// CSS selector for the scrollable navigation items container inside the nav drawer.
pub(crate) const NAV_ITEMS_SCROLL_SELECTOR: &str = ".c_nav_items_scroll";

/// The DOM event type bound to an external link that opens in a new tab.
pub const ROUTER_EXTERNAL_LINK_EVENT_TYPE: &str = "click";

/// The JS property name of `window.open`.
pub const ROUTER_WINDOW_OPEN_KEY: &str = "open";

/// The JS property name read from `window.location.pathname` fallbacks.
pub const ROUTER_MAIN_ELEMENT_SELECTOR: &str = "main";

/// The window event fired when the URL hash changes.
pub const ROUTER_WINDOW_EVENT_HASH_CHANGE: &str = "hashchange";

/// The window event fired when the browser navigates through history.
pub const ROUTER_WINDOW_EVENT_POP_STATE: &str = "popstate";
