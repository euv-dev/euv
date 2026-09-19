//! euv-docs — a VuePress-style documentation site powered by euv + euv-ui.
//!
//! Markdown sources live in `docs/`; `build.rs` compiles them into the
//! `DocsSite` static consumed here at runtime.

mod component;
mod data;
mod router;

mod generated {
    include!(concat!(env!("OUT_DIR"), "/docs_gen.rs"));
}

pub use std::{cell::RefCell, fmt::Debug, rc::Rc};

use {
    data::*,
    euv::{wasm_bindgen::prelude::*, web_sys::*, *},
    euv_ui::*,
    router::*,
};

use crate::component::*;

/// WASM entry point: injects global styles and mounts the app.
#[wasm_bindgen]
pub fn main() {
    console_error_panic_hook::set_once();
    inject_app_global_css();
    Css::inject_css(EUV_MD_CSS);
    // Site-level CSS override: hide the heading anchor `#` glyph
    // entirely on every viewport, and stop it from reserving any
    // horizontal space next to the heading.
    //
    // euv-ui's `euv_markdown` wraps every heading in
    // `<a class="header-anchor"><span>#</span></a>`. The upstream
    // desktop rule is `float: left; margin-left: -0.9em; opacity: 0`
    // and reveals the glyph on `h*:hover` via `opacity: 1`. The
    // mobile rule is `display: inline-flex; width: 1.6em;
    // margin-left: -1.6em` with a matching `padding-left: 1.6em` on
    // the heading. Both rules reserve horizontal space for the
    // anchor even when it is invisible (opacity: 0), so heading
    // text is pushed right by ~12px (desktop) or ~1.6em (mobile)
    // regardless of hover state.
    //
    // On this docs site the design intent is to drop the `#` glyph
    // entirely — it is not part of the heading's reading content,
    // and it added extra spacing and visual noise for no benefit.
    // The URL fragment that the anchor encodes is still present in
    // the DOM (so deep links continue to resolve), but the glyph is
    // never painted and the anchor takes no inline or visual space.
    //
    // Loaded after `EUV_MD_CSS` so cascade order places these rules
    // after the upstream defaults; `!important` keeps the rule safe
    // against future selector-specificity bumps from upstream.
    Css::inject_css(
        ".md-body h1, .md-body h2, .md-body h3, .md-body h4, .md-body h5, .md-body h6 { padding-left: 0 !important; } \
         .md-body .header-anchor, .md-body h1:hover .header-anchor, .md-body h2:hover .header-anchor, .md-body h3:hover .header-anchor, .md-body h4:hover .header-anchor, .md-body h5:hover .header-anchor, .md-body h6:hover .header-anchor { display: none !important; } \
         .c_euv_sidebar_group_title { padding: 0.4rem 1.25rem !important; box-sizing: border-box !important; } \
         .c_euv_sidebar_children { padding: 0 !important; margin: 0 !important; } \
         .c_euv_sidebar_link { display: block !important; padding: 0.4rem 1.25rem !important; } \
         .c_euv_sidebar_link:hover { font-weight: 700 !important; background: var(--accent-muted, rgba(0,0,0,0.05)) !important; } \
         .c_app_main { padding-top: 1.5rem !important; } \
         .c_feature_card { border: 1px dashed var(--foreground, #000) !important; border-radius: 0 !important; padding: 1rem !important; background: transparent !important; } \
         .c_home_btn_secondary { background: #fff !important; color: #000 !important; border-color: #fff !important; } \
         .c_home_btn_secondary:hover { border-color: #000 !important; }",
    );
    App::mount("#app", app);
}
