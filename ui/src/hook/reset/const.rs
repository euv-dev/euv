/// The DOM event fired when a form resets its controls.
#[cfg(target_arch = "wasm32")]
pub(crate) const FORM_RESET_EVENT_NAME: &str = "reset";

/// The selector that resolves the form owning an ancestor-associated control.
#[cfg(target_arch = "wasm32")]
pub(crate) const FORM_RESET_FORM_SELECTOR: &str = "form";
