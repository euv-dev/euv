/// The `target` that opens a footer link in a new browsing context.
pub(crate) const LINK_TARGET_BLANK: &str = "_blank";

/// The `rel` applied to every `target="_blank"` footer link, so the opened
/// page cannot reach back through `window.opener`.
pub(crate) const LINK_REL_NOOPENER: &str = "noopener noreferrer";
