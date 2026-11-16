/// Single-touch hold duration in milliseconds.
#[cfg(any(target_arch = "wasm32", test))]
pub(crate) const CONTEXT_MENU_LONG_PRESS_MS: i32 = 500;
/// Maximum movement along either viewport axis before cancellation.
pub(crate) const CONTEXT_MENU_TOUCH_SLOP: i32 = 10;

/// The keyboard key dedicated to opening a context menu.
pub(crate) const CONTEXT_MENU_KEY: &str = "ContextMenu";
/// Serialized true state used by menu navigation.
pub(crate) const CONTEXT_MENU_ATTR_TRUE: &str = "true";
/// The function key used with Shift to open a context menu.
pub(crate) const CONTEXT_MENU_F10_KEY: &str = "F10";
/// The viewport x coordinate used to position the context menu.
pub(crate) const CONTEXT_MENU_POSITION_X_PROPERTY: &str = "--euv-context-menu-x";
/// The viewport y coordinate used to position the context menu.
pub(crate) const CONTEXT_MENU_POSITION_Y_PROPERTY: &str = "--euv-context-menu-y";
/// The selector for the context menu root that owns a trigger.
pub(crate) const CONTEXT_MENU_ROOT_SELECTOR: &str = "[data-slot=\"context-menu\"]";
/// The selector for a context menu trigger.
pub(crate) const CONTEXT_MENU_TRIGGER_SELECTOR: &str = "[data-slot=\"context-menu-trigger\"]";
/// The selector for the context menu popup.
pub(crate) const CONTEXT_MENU_CONTENT_SELECTOR: &str = "[data-slot=\"context-menu-content\"]";
/// The keyboard key that closes an open context menu.
pub(crate) const CONTEXT_MENU_ESCAPE_KEY: &str = "Escape";

/// The menu and submenu popup selector.
pub(crate) const CONTEXT_MENU_POPUP_SELECTOR: &str = "[role=menu]";
/// The navigable row selector.
pub(crate) const CONTEXT_MENU_ITEM_SELECTOR: &str = "[role^=menuitem]";
/// The nested menu owner selector.
pub(crate) const CONTEXT_MENU_SUB_SELECTOR: &str = "[data-slot=context-menu-sub]";
/// The submenu trigger selector.
pub(crate) const CONTEXT_MENU_SUB_TRIGGER_SELECTOR: &str = "[data-slot=context-menu-sub-trigger]";
/// The submenu popup selector.
pub(crate) const CONTEXT_MENU_SUB_CONTENT_SELECTOR: &str = "[data-slot=context-menu-sub-content]";
/// The typeahead buffer attribute.
pub(crate) const CONTEXT_MENU_SEARCH_ATTRIBUTE: &str = "data-euv-menu-search";
/// The typeahead timestamp attribute.
pub(crate) const CONTEXT_MENU_SEARCH_TIME_ATTRIBUTE: &str = "data-euv-menu-search-time";
/// The disabled-state attribute.
pub(crate) const CONTEXT_MENU_DISABLED_ATTRIBUTE: &str = "aria-disabled";
/// The down navigation key.
pub(crate) const CONTEXT_MENU_DOWN_KEY: &str = "ArrowDown";
/// The up navigation key.
pub(crate) const CONTEXT_MENU_UP_KEY: &str = "ArrowUp";
/// The first-row navigation key.
pub(crate) const CONTEXT_MENU_HOME_KEY: &str = "Home";
/// The last-row navigation key.
pub(crate) const CONTEXT_MENU_END_KEY: &str = "End";
/// The submenu entry key.
pub(crate) const CONTEXT_MENU_RIGHT_KEY: &str = "ArrowRight";
/// The submenu exit key.
pub(crate) const CONTEXT_MENU_LEFT_KEY: &str = "ArrowLeft";
/// The activation key.
pub(crate) const CONTEXT_MENU_ENTER_KEY: &str = "Enter";
/// The typeahead reset interval in milliseconds.
pub(crate) const CONTEXT_MENU_SEARCH_RESET_MS: f64 = 500.0;
/// The viewport resize event.
#[cfg(target_arch = "wasm32")]
pub(crate) const CONTEXT_MENU_RESIZE_EVENT: &str = "resize";
/// The inherited writing direction property.
pub(crate) const CONTEXT_MENU_DIRECTION_PROPERTY: &str = "direction";
/// The computed right-to-left direction value.
pub(crate) const CONTEXT_MENU_RTL: &str = "rtl";
/// The collision-adjusted physical horizontal property.
#[cfg(target_arch = "wasm32")]
pub(crate) const CONTEXT_MENU_LEFT_PROPERTY: &str = "left";
/// The collision-adjusted physical vertical property.
#[cfg(target_arch = "wasm32")]
pub(crate) const CONTEXT_MENU_TOP_PROPERTY: &str = "top";
/// The transform that would otherwise create a fixed-position containing block.
#[cfg(target_arch = "wasm32")]
pub(crate) const CONTEXT_MENU_TRANSFORM_PROPERTY: &str = "transform";
/// The transform-free position used while viewport-positioned submenus are open.
#[cfg(target_arch = "wasm32")]
pub(crate) const CONTEXT_MENU_NO_TRANSFORM: &str = "none";
/// Scoped structural constraints for viewport-measured submenu panels.
#[cfg(target_arch = "wasm32")]
pub(crate) const CONTEXT_MENU_SUB_POSITION_STYLES: [(&str, &str); 9] = [
    ("position", "fixed"),
    ("inset-inline-start", "auto"),
    ("right", "auto"),
    ("bottom", "auto"),
    ("box-sizing", "border-box"),
    ("min-width", "min(160px, 100vw)"),
    ("max-width", "100vw"),
    ("max-height", "100vh"),
    ("overflow", "auto"),
];

/// Default hover entry deadline, independent of touch long-press.
pub(crate) const CONTEXT_MENU_HOVER_DELAY_MS: u32 = 100;
/// Default immediate branch dismissal.
pub(crate) const CONTEXT_MENU_HOVER_CLOSE_DELAY_MS: u32 = 0;
/// Browser observer names avoid adding framework-wide web-sys features.
#[cfg(target_arch = "wasm32")]
pub(crate) const CONTEXT_MENU_MUTATION_OBSERVER: &str = "MutationObserver";
#[cfg(target_arch = "wasm32")]
pub(crate) const CONTEXT_MENU_OBSERVE: &str = "observe";
#[cfg(target_arch = "wasm32")]
pub(crate) const CONTEXT_MENU_DISCONNECT: &str = "disconnect";
#[cfg(target_arch = "wasm32")]
pub(crate) const CONTEXT_MENU_OBSERVER_OPTIONS: [&str; 2] = ["childList", "subtree"];
#[cfg(any(target_arch = "wasm32", test))]
pub(crate) const CONTEXT_MENU_POINTER_MOUSE: &str = "mouse";
#[cfg(any(target_arch = "wasm32", test))]
pub(crate) const CONTEXT_MENU_POINTER_PEN: &str = "pen";
#[cfg(target_arch = "wasm32")]
pub(crate) const CONTEXT_MENU_POINTER_TYPE: &str = "pointerType";
#[cfg(target_arch = "wasm32")]
pub(crate) const CONTEXT_MENU_HOVER_MEDIA: &str = "(any-hover: hover)";
#[cfg(target_arch = "wasm32")]
pub(crate) const CONTEXT_MENU_POINTER_MOVE: &str = "pointermove";
#[cfg(target_arch = "wasm32")]
pub(crate) const CONTEXT_MENU_POINTER_OUT: &str = "pointerout";
#[cfg(target_arch = "wasm32")]
pub(crate) const CONTEXT_MENU_HOVER_EVENTS: [&str; 4] = [
    CONTEXT_MENU_POINTER_MOVE,
    CONTEXT_MENU_POINTER_OUT,
    "pointerdown",
    "pointercancel",
];
