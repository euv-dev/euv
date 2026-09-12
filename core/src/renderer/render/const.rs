use super::*;

/// The DOM attribute name used to store the dynamic node identifier on a placeholder element.
///
/// This attribute is assigned to every DynamicNode placeholder so the framework
/// can locate and manage the dynamic content during re-renders and cleanup.
pub(crate) const DATA_EUV_DYNAMIC_ID: &str = "data-euv-dynamic-id";

/// CSS selector that matches every element participating in framework
/// The HTML tag name used for fragment placeholder elements.
///
/// Fragments are rendered as `<slot>` elements with `display:contents` style
/// so they don't create an extra DOM wrapper.
pub(crate) const FRAGMENT_TAG: &str = "slot";

/// The HTML tag name used for DynamicNode placeholder elements.
///
/// Dynamic nodes use `<div>` as a placeholder container with `display: contents`
/// style so the placeholder is invisible in the rendered output.
pub(crate) const DYNAMIC_PLACEHOLDER_TAG: &str = "div";

/// The inline style value used to make dynamic node placeholder elements invisible.
///
/// Applied to dynamic node placeholder elements so they
/// don't affect the visual layout of the page.
pub(crate) const DISPLAY_CONTENTS_STYLE: &str = "display: contents;";

/// The inline style value for fragment placeholder elements.
///
/// Used to make `<slot>` fragment wrappers invisible in the rendered output.
pub(crate) const FRAGMENT_STYLE: &str = "display:contents";

/// The HTML `style` attribute name.
pub(crate) const ATTR_STYLE: &str = "style";

/// The CSS selector prefix for selecting by element ID.
pub(crate) const ID_SELECTOR_PREFIX: &str = "#";

/// The CSS selector prefix for selecting by class name.
pub(crate) const CLASS_SELECTOR_PREFIX: &str = ".";

/// The HTML `body` tag name, used as a default mount target.
pub(crate) const BODY_TAG: &str = "body";

thread_local! {
    /// Single-threaded cache for the page's `Document`. SAFETY: euv
    /// runs on the main thread in WASM contexts, and `Document::clone`
    /// is a cheap reference count bump on the underlying `JsValue`.
    pub static DOCUMENT_CACHE: UnsafeCell<Option<Document>> =
        const { UnsafeCell::new(None) };
}
