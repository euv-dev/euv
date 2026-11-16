pub(crate) const DROPDOWN_ROOT_SELECTOR: &str = "[data-slot=\"dropdown-menu\"]";

/// The decorative submenu glyph parsed in the SVG namespace.
pub(crate) const DROPDOWN_SUB_CARET_MARKUP: &str = "<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"16\" height=\"16\" viewBox=\"0 0 24 24\" fill=\"none\" stroke=\"currentColor\" stroke-width=\"2\" stroke-linecap=\"round\" stroke-linejoin=\"round\" aria-hidden=\"true\" focusable=\"false\"><path d=\"m9 18 6-6-6-6\"/></svg>";
pub(crate) const DROPDOWN_TRIGGER_SELECTOR: &str = "[data-slot=\"dropdown-menu-trigger\"]";
pub(crate) const DROPDOWN_CONTENT_SELECTOR: &str =
    "[data-slot=\"dropdown-menu-content\"], [data-slot=\"dropdown-menu-sub-content\"]";
pub(crate) const DROPDOWN_SUB_CONTENT_SLOT: &str = "dropdown-menu-sub-content";
pub(crate) const DROPDOWN_SLOT_ATTRIBUTE: &str = "data-slot";
pub(crate) const DROPDOWN_KEY_LEFT: &str = "ArrowLeft";
pub(crate) const DROPDOWN_KEY_RIGHT: &str = "ArrowRight";
pub(crate) const DROPDOWN_DIRECTION_PROPERTY: &str = "direction";
pub(crate) const DROPDOWN_RTL: &str = "rtl";
#[cfg(target_arch = "wasm32")]
pub(crate) const DROPDOWN_LTR: &str = "ltr";
#[cfg(target_arch = "wasm32")]
pub(crate) const DROPDOWN_DIR_ATTRIBUTE: &str = "dir";
#[cfg(target_arch = "wasm32")]
pub(crate) const DROPDOWN_MAX_CONTENT: &str = "max-content";
pub(crate) const DROPDOWN_ROW_SELECTOR: &str =
    "[role=\"menuitem\"], [role=\"menuitemcheckbox\"], [role=\"menuitemradio\"]";
pub(crate) const DROPDOWN_ITEM_SELECTOR: &str = ":is([role=\"menuitem\"], [role=\"menuitemcheckbox\"], [role=\"menuitemradio\"]):not([disabled]):not([aria-disabled=\"true\"])";
pub(crate) const DROPDOWN_LABEL_ATTRIBUTE: &str = "data-label";
pub(crate) const DROPDOWN_KEY_ARROW_DOWN: &str = "ArrowDown";
pub(crate) const DROPDOWN_KEY_ARROW_UP: &str = "ArrowUp";
pub(crate) const DROPDOWN_KEY_HOME: &str = "Home";
pub(crate) const DROPDOWN_KEY_END: &str = "End";
pub(crate) const DROPDOWN_KEY_ESCAPE: &str = "Escape";
pub(crate) const DROPDOWN_ATTR_TRUE: &str = "true";
pub(crate) const DROPDOWN_ATTR_FALSE: &str = "false";
pub(crate) const DROPDOWN_STATE_OPEN: &str = "open";
pub(crate) const DROPDOWN_STATE_CLOSED: &str = "closed";
pub(crate) const DROPDOWN_MENU_ARIA_LABEL: &str = "Dropdown menu";

pub(crate) const DROPDOWN_KEY_TAB: &str = "Tab";
pub(crate) const DROPDOWN_KEY_SPACE: &str = " ";
pub(crate) const DROPDOWN_SEARCH_RESET_MS: f64 = 500.0;
pub(crate) const DROPDOWN_COMPOSITION_KEY_CODE: u32 = 229;
pub(crate) const DROPDOWN_CHECKED_INDICATOR: &str = "✓";

pub(crate) const DROPDOWN_PORTAL_TARGET: &str = ".c_app_root";
pub(crate) const DROPDOWN_MODAL_CONTENT_SELECTOR: &str = "[data-slot=\"sheet-content\"]";
#[cfg(any(target_arch = "wasm32", test))]
pub(crate) const DROPDOWN_VIEWPORT_PADDING: f64 = 8.0;
#[cfg(any(target_arch = "wasm32", test))]
pub(crate) const DROPDOWN_DEFAULT_GAP: f64 = 8.0;
pub(crate) const DROPDOWN_FOCUS_ATTRIBUTE: &str = "data-dropdown-focus";
pub(crate) const DROPDOWN_POSITION_EVENT: &str = "euv-dropdown-position";
#[cfg(target_arch = "wasm32")]
pub(crate) const DROPDOWN_POSITION_EVENTS: [&str; 3] =
    ["scroll", "resize", DROPDOWN_POSITION_EVENT];
#[cfg(target_arch = "wasm32")]
pub(crate) const DROPDOWN_PRESS_EVENTS: [&str; 2] = ["mousedown", "touchstart"];

pub(crate) const DROPDOWN_LABELLED_BY_ATTRIBUTE: &str = "aria-labelledby";
pub(crate) const DROPDOWN_FOCUS_FIRST: &str = "first";
pub(crate) const DROPDOWN_FOCUS_LAST: &str = "last";
pub(crate) const DROPDOWN_TABBABLE_SELECTOR: &str =
    "a[href],button,input,select,textarea,[tabindex]";
pub(crate) const DROPDOWN_DISABLED_ATTRIBUTE: &str = "disabled";
#[cfg(target_arch = "wasm32")]
pub(crate) const DROPDOWN_SCROLL_EVENT: &str = "scroll";
#[cfg(target_arch = "wasm32")]
pub(crate) const DROPDOWN_RESIZE_EVENT: &str = "resize";
#[cfg(target_arch = "wasm32")]
pub(crate) const DROPDOWN_LEFT_PROPERTY: &str = "left";
#[cfg(target_arch = "wasm32")]
pub(crate) const DROPDOWN_TOP_PROPERTY: &str = "top";
#[cfg(target_arch = "wasm32")]
pub(crate) const DROPDOWN_WIDTH_PROPERTY: &str = "width";
#[cfg(target_arch = "wasm32")]
pub(crate) const DROPDOWN_HEIGHT_PROPERTY: &str = "height";
#[cfg(target_arch = "wasm32")]
pub(crate) const DROPDOWN_PIXEL_SUFFIX: &str = "px";
#[cfg(target_arch = "wasm32")]
pub(crate) const DROPDOWN_MIN_WIDTH_PROPERTY: &str = "min-width";
#[cfg(target_arch = "wasm32")]
pub(crate) const DROPDOWN_MAX_WIDTH_PROPERTY: &str = "max-width";
#[cfg(target_arch = "wasm32")]
pub(crate) const DROPDOWN_MAX_HEIGHT_PROPERTY: &str = "max-height";
#[cfg(target_arch = "wasm32")]
pub(crate) const DROPDOWN_MEASURE_STYLES: [(&str, &str); 4] = [
    ("left", "0px"),
    ("top", "0px"),
    ("right", "auto"),
    ("max-height", "none"),
];

#[cfg(any(target_arch = "wasm32", test))]
pub(crate) const DROPDOWN_POINTER_MOUSE: &str = "mouse";
#[cfg(any(target_arch = "wasm32", test))]
pub(crate) const DROPDOWN_POINTER_PEN: &str = "pen";
#[cfg(target_arch = "wasm32")]
pub(crate) const DROPDOWN_POINTER_TYPE: &str = "pointerType";
#[cfg(target_arch = "wasm32")]
pub(crate) const DROPDOWN_HOVER_MEDIA: &str = "(any-hover: hover)";
#[cfg(target_arch = "wasm32")]
pub(crate) const DROPDOWN_POINTER_MOVE: &str = "pointermove";
#[cfg(target_arch = "wasm32")]
pub(crate) const DROPDOWN_POINTER_OUT: &str = "pointerout";
#[cfg(target_arch = "wasm32")]
pub(crate) const DROPDOWN_HOVER_EVENTS: [&str; 4] = [
    DROPDOWN_POINTER_MOVE,
    DROPDOWN_POINTER_OUT,
    "pointerdown",
    "pointercancel",
];
#[cfg(target_arch = "wasm32")]
pub(crate) const DROPDOWN_STATE_ATTRIBUTE: &str = "data-state";
