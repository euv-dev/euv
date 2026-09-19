use super::*;

/// Injects application-level global CSS into the DOM.
///
/// Registers the global reset styles and built-in animation keyframes.
/// Responsive media queries are now handled directly within `class!` macro
/// definitions via the `media()` block syntax.
/// Must be called once during application initialisation before any
/// rendering occurs.
///
/// # Panics
///
/// Panics if `window()` or `document()` is unavailable on the current platform.
pub fn inject_app_global_css() {
    let global: &str = concat!(
        "html, body, #app { height: 100%; margin: 0; padding: 0; background: ",
        var!(background),
        "; } ",
        "* { -webkit-tap-highlight-color: transparent; box-sizing: border-box; margin: 0; padding: 0; border: 0; font: inherit; vertical-align: baseline; } ",
        "html { line-height: 1.5; -webkit-text-size-adjust: 100%; } ",
        "ol, ul { list-style: none; } ",
        "img, picture, video, canvas, svg { display: block; max-width: 100%; } ",
        "input, button, textarea, select { font: inherit; color: inherit; background: transparent; } ",
        "button { cursor: pointer; } ",
        "a { text-decoration: none; color: inherit; }"
    );
    let scrollbar: &str = concat!(
        "* { scrollbar-width: thin; } ",
        "::-webkit-scrollbar { width: 6px; height: 6px; } ",
        "::-webkit-scrollbar-track { background: transparent; } ",
        "::-webkit-scrollbar-thumb { border-radius: 0; } ",
        "::-webkit-scrollbar-button { display: none !important; width: 0 !important; height: 0 !important; } ",
        "::-webkit-scrollbar-corner { background: transparent; }",
        "@media (max-width: 767px) { * { scrollbar-width: none; } ::-webkit-scrollbar { width: 0px; height: 0px; } }",
    );
    let keyframes: &str = concat!(
        "@keyframes euv-spin { from { transform: rotate(0deg); } to { transform: rotate(360deg); } } ",
        "@keyframes euv-fade-in { from { opacity: 0; transform: translateY(4px); } to { opacity: 1; transform: translateY(0); } } ",
        "@keyframes euv-pulse { 0%, 100% { transform: scale(1); } 50% { transform: scale(1.15); } } ",
        "@keyframes euv-progress { from { width: 0%; } to { width: 100%; } } ",
    );
    let extra_keyframes: &str = "@keyframes euv-scale-in-modal { from { opacity: 0; transform: translateY(24px) scale(0.95); } to { opacity: 1; transform: translateY(0) scale(1); } } ";
    let a11y_css: &str = concat!(
        ":focus-visible { outline: none }",
        "@media (prefers-reduced-motion: reduce) { *, *::before, *::after { animation-iteration-count: 1 !important; scroll-behavior: auto !important; } } ",
        "@media (hover: none) and (pointer: coarse) { * { -webkit-tap-highlight-color: transparent; } .c_card:hover, .c_home_stat_card:hover { transform: none !important; } .c_home_btn_primary:hover, .c_home_btn_secondary:hover { transform: none !important; } } ",
    );
    Css::inject_css(global);
    Css::inject_css(scrollbar);
    Css::inject_css(keyframes);
    Css::inject_css(extra_keyframes);
    Css::inject_css(a11y_css);
}
