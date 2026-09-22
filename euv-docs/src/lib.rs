mod component;
mod data;
mod router;

mod generated {
    include!(concat!(env!("OUT_DIR"), "/docs_gen.rs"));
}

pub use std::{cell::RefCell, fmt::Debug, rc::Rc};

pub(crate) use {
    component::*,
    data::*,
    js_sys::{Promise, decode_uri_component, eval},
    router::*,
    web_sys::{Event, HtmlInputElement, KeyboardEvent, Location},
};

use {
    euv::{wasm_bindgen::prelude::*, web_sys::*, *},
    euv_ui::*,
    wasm_bindgen_futures::{JsFuture, spawn_local},
};

#[wasm_bindgen]
pub fn main() {
    console_error_panic_hook::set_once();
    inject_app_global_css();
    Css::inject_css(EUV_MD_CSS);
    Css::inject_css(
        ".md-body h1, .md-body h2, .md-body h3, .md-body h4, .md-body h5, .md-body h6 { padding-left: 0 !important; } \
             .md-body .header-anchor, .md-body h1:hover .header-anchor, .md-body h2:hover .header-anchor, .md-body h3:hover .header-anchor, .md-body h4:hover .header-anchor, .md-body h5:hover .header-anchor, .md-body h6:hover .header-anchor { display: none !important; } \
             .md-body img { display: inline-block; width: auto !important; max-width: 100% !important; height: auto; vertical-align: baseline; } \
             .md-body a > img { display: inline-block; } \
             .md-body img[src$='.svg'], .md-body img[src*='shields.io'], .md-body img[src*='github.com'] { max-height: 20px; max-width: 100%; } \
             .md-body table img { max-height: 1.4em; } \
             \
             .c_app_main { padding-top: 4.75rem !important; } \
             \
             .c_euv_sidebar_group_title { padding: 0.4rem 0.75rem !important; box-sizing: border-box !important; } \
             .c_euv_sidebar_children { padding-left: 0.5rem !important; margin-left: 0.75rem !important; } \
             .c_nav_footer_divider { left: 0.75rem !important; right: 0.75rem !important; } \
             .c_nav_section_label { padding-left: 0.75rem !important; } \
             .c_nav_footer { padding-left: 0.75rem !important; } \
             .c_euv_sidebar_link { display: block !important; padding: 0.4rem 0.75rem !important; } \
             .c_euv_sidebar_link_active { padding: 0.4rem 0.75rem !important; } \
             .c_euv_sidebar_link:hover { font-weight: 700 !important; background: var(--accent-muted, rgba(0,0,0,0.05)) !important; box-shadow: inset 4px 0px 0px var(--foreground, #000) !important; } \
             .c_euv_sidebar_children .c_euv_sidebar_link, .c_euv_sidebar_children .c_euv_sidebar_link_active, .c_euv_sidebar_children .c_euv_sidebar_group_title { margin-left: -9px !important; padding-left: calc(0.75rem + 9px) !important; } \
             \
             .c_euv_doc_layout { max-width: 1160px !important; } \
             .c_euv_doc_toc { width: 280px !important; flex-shrink: 0 !important; } \
             .c_euv_toc_link_nested { padding-left: 0.75rem !important; font-size: var(--font-sm, 0.875rem) !important; color: var(--muted-foreground, #555) !important; line-height: 1.5 !important; } \
             .c_euv_toc_link_nested:hover { color: var(--accent, #000) !important; } \
             \
             .c_docs_page_title { font-size: 2.25rem; font-weight: 800; letter-spacing: -0.02em; margin: 0 0 1rem 0; padding-top: 0; color: var(--foreground, #000); } \
             \
             .c_feature_card { border: 1px dashed var(--foreground, #000) !important; border-radius: 0 !important; padding: 1rem !important; background: transparent !important; } \
             .c_home_btn_secondary { background: transparent !important; color: #000 !important; border: 1.5px solid #000 !important; } \
             .c_home_btn_secondary:hover { background: rgba(0,0,0,0.06) !important; } \
             .c_theme_dark .c_home_btn_secondary { background: transparent !important; color: #fff !important; border-color: #fff !important; } \
             .c_theme_dark .c_home_btn_secondary:hover { background: rgba(255,255,255,0.10) !important; } \
             \
             .c_docs_feature_grid { display: grid; grid-template-columns: repeat(2, minmax(0, 1fr)); gap: 1rem; margin: 1rem 0; } \
             @media (max-width: 767px) { .c_docs_feature_grid { grid-template-columns: minmax(0, 1fr); } } \
             .c_docs_feature_card { display: flex; flex-direction: column; gap: 0.4rem; padding: 1rem; border: 1px dashed var(--foreground, #000); border-radius: 0; background: transparent; text-decoration: none; color: inherit; transition: background 0.15s ease-out, border-color 0.15s ease-out; min-width: 0; } \
             .c_docs_feature_card:hover { background: var(--accent-muted, rgba(0,0,0,0.06)); border-color: var(--foreground, #000); } \
             .c_theme_dark .c_docs_feature_card:hover { background: var(--accent-muted, rgba(255,255,255,0.08)); } \
             .c_docs_feature_card_inner { display: flex; flex-direction: column; gap: 0.4rem; min-width: 0; } \
             .c_docs_feature_card_icon { font-size: 1.5rem; line-height: 1; flex-shrink: 0; } \
             .c_docs_feature_card_title { font-size: 1.125rem; font-weight: 600; overflow-wrap: anywhere; } \
             .c_docs_feature_card_details { font-size: 0.875rem; color: var(--muted-foreground, #555); overflow-wrap: anywhere; } \
             \
             .docs-container-tip, .docs-container-note, .docs-container-important, .docs-container-info { border: 1px dashed var(--foreground, #000); border-left-width: 4px; padding: 0.75rem 1rem; margin: 1rem 0; background: var(--accent-muted, rgba(0,0,0,0.04)); } \
             .docs-container-warning, .docs-container-caution { border: 1px solid var(--foreground, #000); border-left-width: 4px; padding: 0.75rem 1rem; margin: 1rem 0; background: var(--accent-muted, rgba(0,0,0,0.04)); } \
             .docs-container-danger { border: 1px solid var(--foreground, #000); border-left-width: 4px; padding: 0.75rem 1rem; margin: 1rem 0; background: rgba(0,0,0,0.06); } \
             .docs-container-title { font-weight: 600; margin: 0 0 0.25rem 0; font-size: 0.875rem; text-transform: uppercase; letter-spacing: 0.05em; } \
             .docs-container-title:empty { display: none; } \
             \
             .c_euv_pagination { margin-top: var(--space-4xl) !important; margin-bottom: var(--space-4xl) !important; } \
             .c_euv_footer { margin-top: var(--space-4xl) !important; padding-top: var(--space-2xl) !important; padding-bottom: var(--space-xs) !important; } \
             \
             .md-body img:not([data-loaded]) { height: 0px !important; margin: 0px !important; visibility: hidden; } \
             .md-body img[data-loaded] { transition: opacity 0.2s ease-out; }",
    );
    App::mount("#app", app);
    let _ = js_sys::eval(
        "(function(){var p=function(i){if(i.dataset.loaded)return;var m=function(){i.dataset.loaded='1';};if(i.complete&&i.naturalWidth>0){m();}else{i.addEventListener('load',m);i.addEventListener('error',m);}};var o=new MutationObserver(function(ms){ms.forEach(function(d){d.addedNodes.forEach(function(n){if(n.tagName==='IMG'){p(n);}if(n.querySelectorAll){n.querySelectorAll('img').forEach(p);}});});});o.observe(document.body,{childList:true,subtree:true});document.querySelectorAll('img').forEach(p);}());",
    );
}
