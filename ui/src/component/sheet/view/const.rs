/// Inline properties exclusively owned by the document scroll lock.
#[cfg(any(target_arch = "wasm32", test))]
pub(crate) const SHEET_OVERFLOW_PROPERTIES: [&str; 5] = [
    "overflow",
    "overflow-x",
    "overflow-y",
    "overflow-block",
    "overflow-inline",
];

/// Optional stable gutter reservation on a document with a classic scrollbar.
#[cfg(any(target_arch = "wasm32", test))]
pub(crate) const SHEET_GUTTER_PROPERTY: &str = "scrollbar-gutter";

/// Native scroll lock value; no positioning or scroll-coordinate mutation is used.
#[cfg(target_arch = "wasm32")]
pub(crate) const SHEET_SCROLL_HIDDEN: &str = "hidden";

/// The lock must override consumer stylesheet and inline priorities while owned.
#[cfg(target_arch = "wasm32")]
pub(crate) const SHEET_SCROLL_PRIORITY: &str = "important";

/// Stable gutter value for a document with a visible classic scrollbar.
#[cfg(target_arch = "wasm32")]
pub(crate) const SHEET_GUTTER_STABLE: &str = "stable";

/// Preserve a consumer's symmetric gutter when reserving its existing space.
#[cfg(target_arch = "wasm32")]
pub(crate) const SHEET_GUTTER_BOTH: &str = "stable both-edges";

/// Stable trigger ID suffix.
pub(crate) const SHEET_TRIGGER_PART: &str = "trigger";

/// The decorative close glyph parsed in the SVG namespace.
pub(crate) const SHEET_CLOSE_MARKUP: &str = "<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"16\" height=\"16\" viewBox=\"0 0 24 24\" fill=\"none\" stroke=\"currentColor\" stroke-width=\"2\" stroke-linecap=\"round\" stroke-linejoin=\"round\" aria-hidden=\"true\" focusable=\"false\"><path d=\"M18 6 6 18M6 6l12 12\"/></svg>";
/// Stable content ID suffix.
pub(crate) const SHEET_CONTENT_PART: &str = "content";
/// Nearest composition boundary.
#[cfg(target_arch = "wasm32")]
pub(crate) const SHEET_ROOT_SELECTOR: &str = "[data-slot=sheet]";
/// Own trigger selector.
#[cfg(target_arch = "wasm32")]
pub(crate) const SHEET_TRIGGER_SELECTOR: &str = "[data-slot=sheet-trigger]";
/// Own content selector.
#[cfg(target_arch = "wasm32")]
pub(crate) const SHEET_CONTENT_SELECTOR: &str = "[data-slot=sheet-content]";
/// Potentially keyboard-focusable descendants, filtered for visibility and enabled state.
#[cfg(target_arch = "wasm32")]
pub(crate) const SHEET_FOCUSABLE_SELECTOR: &str = "a[href],area[href],button,input:not([type=hidden]),select,textarea,iframe,[tabindex],[contenteditable=true]";

/// Nearest open modal composition boundary.
#[cfg(target_arch = "wasm32")]
pub(crate) const SHEET_OPEN_ROOT_SELECTOR: &str = "[data-slot=sheet][data-state=open]";

/// Modal DOM contract value.
#[cfg(target_arch = "wasm32")]
pub(crate) const SHEET_LABEL_ATTRIBUTE: &str = "aria-labelledby";

/// Modal DOM contract value.
#[cfg(target_arch = "wasm32")]
pub(crate) const SHEET_DESCRIPTION_ATTRIBUTE: &str = "aria-describedby";

/// Modal DOM contract value.
#[cfg(target_arch = "wasm32")]
pub(crate) const SHEET_INERT_ATTRIBUTE: &str = "inert";

/// Modal DOM contract value.
#[cfg(target_arch = "wasm32")]
pub(crate) const SHEET_CLICK_EVENT: &str = "click";

/// Modal DOM contract value.
#[cfg(target_arch = "wasm32")]
pub(crate) const SHEET_KEYDOWN_EVENT: &str = "keydown";

/// Modal DOM contract value.
#[cfg(target_arch = "wasm32")]
pub(crate) const SHEET_ESCAPE_KEY: &str = "Escape";

/// Modal DOM contract value.
#[cfg(target_arch = "wasm32")]
pub(crate) const SHEET_TAB_KEY: &str = "Tab";

/// Modal DOM contract value.
#[cfg(target_arch = "wasm32")]
pub(crate) const SHEET_VISIBILITY_PROPERTY: &str = "visibility";

/// Modal DOM contract value.
#[cfg(target_arch = "wasm32")]
pub(crate) const SHEET_VISIBLE_VALUE: &str = "visible";

/// Modal DOM contract value.
#[cfg(target_arch = "wasm32")]
pub(crate) const SHEET_TITLE_SELECTOR: &str = "[data-slot=sheet-title]";

/// Modal DOM contract value.
#[cfg(target_arch = "wasm32")]
pub(crate) const SHEET_DESCRIPTION_SELECTOR: &str = "[data-slot=sheet-description]";

/// Modal DOM contract value.
#[cfg(target_arch = "wasm32")]
pub(crate) const SHEET_DISMISS_SELECTOR: &str = "[data-slot=sheet-close],[data-slot=sheet-overlay]";

/// Modal DOM contract value.
#[cfg(target_arch = "wasm32")]
pub(crate) const SHEET_DISABLED_SELECTOR: &str = ":disabled,[disabled]";

/// Modal DOM contract value.
#[cfg(target_arch = "wasm32")]
pub(crate) const SHEET_HIDDEN_SELECTOR: &str =
    "[hidden],[inert],[data-slot=sheet-content][data-state=closed]";
