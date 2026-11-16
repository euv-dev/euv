use super::*;

/// Reads the last root declaration of a compiled class, excluding pseudo/media rules.
///
/// # Arguments
///
/// - `&Css` - The class whose root declarations carry the property.
/// - `&str` - The exact CSS property name.
///
/// # Returns
///
/// - `Option<String>` - The trimmed value of the last matching declaration.
pub(crate) fn class_property(css: &Css, property: &str) -> Option<String> {
    css.get_style()
        .split(';')
        .filter_map(|declaration: &str| declaration.split_once(':'))
        .filter(|(key, _): &(&str, &str)| key.trim() == property)
        .map(|(_, value): (&str, &str)| value.trim().to_owned())
        .next_back()
}

/// Injects application-level global CSS into the DOM.
///
/// Registers the global reset styles and built-in animation keyframes.
/// Responsive media queries are now handled directly within `class!` macro
/// definitions via the `media()` block syntax.
/// Must be called once during application initialisation before any
/// rendering occurs.
///
/// Each stylesheet chunk is a separate `const.rs` constant and is injected
/// on its own: `concat!` only accepts string *literals*, so a named
/// constant cannot be spliced into a `concat!` call at compile time.
///
/// Degrades to a no-op when no window or document is reachable:
/// `Css::inject_css` returns early off wasm, and again when `window()` or
/// `document()` yields nothing, so calling this from a non-browser host is
/// safe rather than fatal.
pub fn inject_app_global_css() {
    Css::inject_css(APP_GLOBAL_CSS_RESET);
    Css::inject_css(APP_GLOBAL_CSS_ROOT);
    Css::inject_css(APP_GLOBAL_CSS_LIST);
    Css::inject_css(APP_GLOBAL_CSS_MEDIA);
    Css::inject_css(APP_GLOBAL_CSS_FORM);
    Css::inject_css(APP_KEYFRAMES_SPIN);
    Css::inject_css(APP_KEYFRAMES_FADE_IN);
    Css::inject_css(APP_KEYFRAMES_PULSE);
    Css::inject_css(APP_KEYFRAMES_SKELETON_PULSE);
    Css::inject_css(APP_KEYFRAMES_PROGRESS);
    Css::inject_css(APP_KEYFRAMES_SCALE_IN_MODAL);
    Css::inject_css(APP_A11Y_CSS_REDUCED_MOTION);
}

/// Injects the markdown stylesheet content. Callers pass the result to
/// [`Css::inject_css`] once at startup.
///
/// # Returns
///
/// The markdown stylesheet as a static string slice.
pub fn euv_md_css() -> &'static str {
    EUV_MD_CSS
}
