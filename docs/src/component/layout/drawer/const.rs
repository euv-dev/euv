/// The background surfaces excluded from modal interaction.
pub(crate) const DOCS_DRAWER_BACKGROUND_SELECTOR: &str = ".c_mobile_header, .c_mobile_main";
/// The boolean DOM attribute defining a modal interaction boundary.
pub(crate) const DOCS_DRAWER_INERT_ATTRIBUTE: &str = "inert";
/// The ARIA attribute excluding a closed drawer from the accessibility tree.
pub(crate) const DOCS_DRAWER_HIDDEN_ATTRIBUTE: &str = "aria-hidden";
/// The stable target of the mobile menu control.
pub(crate) const DOCS_DRAWER_ID: &str = "docs-mobile-drawer";
/// The keyboard key that moves focus between controls.
pub(crate) const DOCS_DRAWER_KEY_TAB: &str = "Tab";
/// The stable close control that receives initial drawer focus.
pub(crate) const DOCS_DRAWER_CLOSE_ID: &str = "docs-mobile-drawer-close";
/// The native event used by the intercepted drawer home link.
pub(crate) const DOCS_DRAWER_CLICK_EVENT: &str = "click";
/// The serialized true ARIA state.
pub(crate) const DOCS_DRAWER_ARIA_TRUE: &str = "true";
/// The document event used for modal keyboard navigation.
pub(crate) const DOCS_DRAWER_KEY_EVENT: &str = "keydown";
/// The serialized false ARIA state.
pub(crate) const DOCS_DRAWER_ARIA_FALSE: &str = "false";
/// The keyboard key that dismisses the innermost open surface.
pub(crate) const DOCS_DRAWER_KEY_ESCAPE: &str = "Escape";
/// The stable opener used for focus restoration after dismissal.
pub(crate) const DOCS_DRAWER_TRIGGER_ID: &str = "docs-mobile-drawer-trigger";
/// The native button type used by the drawer controls.
pub(crate) const DOCS_DRAWER_BUTTON_TYPE: &str = "button";
/// The modal navigation surface's semantic role.
pub(crate) const DOCS_DRAWER_DIALOG_ROLE: &str = "dialog";
/// The document event used to contain programmatic focus.
pub(crate) const DOCS_DRAWER_FOCUS_EVENT: &str = "focusin";
/// Controls participating in the drawer's sequential focus order.
pub(crate) const DOCS_DRAWER_FOCUSABLE_SELECTOR: &str =
    "a[href], button, input, select, textarea, [tabindex], [contenteditable=true]";
/// The computed property that can hide an otherwise measurable control.
pub(crate) const DOCS_DRAWER_VISIBILITY_PROPERTY: &str = "visibility";
/// Computed visibility values excluded from the modal focus order.
pub(crate) const DOCS_DRAWER_HIDDEN_VISIBILITIES: [&str; 2] = ["hidden", "collapse"];
/// Ancestors that remove a control from the modal focus order.
pub(crate) const DOCS_DRAWER_UNAVAILABLE_SELECTOR: &str =
    "[hidden], [inert], [aria-hidden=true], [disabled]";
