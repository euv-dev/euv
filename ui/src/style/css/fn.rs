use super::*;

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
    Css::inject_css(format!(
        "{APP_GLOBAL_CSS_HEAD}{}{APP_GLOBAL_CSS_BACKGROUND_CLOSE}",
        var!(background)
    ));
    Css::inject_css(APP_GLOBAL_CSS_RESET);
    Css::inject_css(APP_GLOBAL_CSS_ROOT);
    Css::inject_css(APP_GLOBAL_CSS_LIST);
    Css::inject_css(APP_GLOBAL_CSS_MEDIA);
    Css::inject_css(APP_GLOBAL_CSS_FORM);
    Css::inject_css(APP_GLOBAL_CSS_BUTTON);
    Css::inject_css(APP_GLOBAL_CSS_LINK);
    Css::inject_css(APP_SCROLLBAR_CSS_THIN);
    Css::inject_css(APP_SCROLLBAR_CSS_WEBKIT);
    Css::inject_css(APP_SCROLLBAR_CSS_TRACK);
    Css::inject_css(APP_SCROLLBAR_CSS_THUMB);
    Css::inject_css(APP_SCROLLBAR_CSS_BUTTON);
    Css::inject_css(APP_SCROLLBAR_CSS_CORNER);
    Css::inject_css(APP_SCROLLBAR_CSS_MOBILE);
    Css::inject_css(APP_KEYFRAMES_SPIN);
    Css::inject_css(APP_KEYFRAMES_FADE_IN);
    Css::inject_css(APP_KEYFRAMES_PULSE);
    Css::inject_css(APP_KEYFRAMES_PROGRESS);
    Css::inject_css(APP_KEYFRAMES_SCALE_IN_MODAL);
    Css::inject_css(APP_A11Y_CSS_FOCUS_VISIBLE);
    Css::inject_css(APP_A11Y_CSS_REDUCED_MOTION);
    Css::inject_css(APP_A11Y_CSS_COARSE_POINTER);
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
