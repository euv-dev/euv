use super::*;

/// The DOM attribute name used to store the dynamic node identifier on a placeholder element.
///
/// This attribute is assigned to every DynamicNode placeholder so the framework
/// can locate and manage the dynamic content during re-renders and cleanup.
pub(crate) const DATA_EUV_DYNAMIC_ID: &str = "data-euv-dynamic-id";

/// The DOM attribute name used to mark an element as a portal placeholder.
///
/// The marker carries the original selector so the renderer can resolve its
/// content wrapper during patching and cleanup.
pub(crate) const DATA_EUV_PORTAL: &str = "data-euv-portal";

/// The inline style value that hides the portal marker element.
///
/// The marker is a bookkeeping node rather than real content, so it is kept
/// out of the layout entirely: a fallback that loses its marker attribute is
/// invisible anyway.
pub(crate) const PORTAL_MARKER_STYLE: &str = "display:none";

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
    pub(crate) static DOCUMENT_CACHE: UnsafeCell<Option<Document>> =
        const { UnsafeCell::new(None) };
}

/// Stable identifier attribute stored on a portal marker.
pub(crate) const DATA_EUV_PORTAL_ID: &str = "data-euv-portal-id";

/// Stable identifier attribute stored on the matching portal content wrapper.
pub(crate) const DATA_EUV_PORTAL_CONTENT_ID: &str = "data-euv-portal-content-id";

/// Marks portal content waiting for its target to enter the mounted document.
pub(crate) const DATA_EUV_PORTAL_PENDING: &str = "data-euv-portal-pending";

/// The attribute value used while a portal waits for its target.
pub(crate) const PORTAL_PENDING_VALUE: &str = "true";

/// Keeps the portal bookkeeping wrapper out of layout while retaining its DOM subtree.
pub(crate) const PORTAL_CONTENT_STYLE: &str = "display:contents";
