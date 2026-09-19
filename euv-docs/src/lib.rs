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
    js_sys::{decode_uri_component, eval, Promise},
    router::*,
    web_sys::{window, Event, HtmlInputElement, KeyboardEvent, Location},
};

use {
    crate::generated::*,
    euv::{wasm_bindgen::prelude::*, web_sys::*, *},
    euv_ui::*,
    wasm_bindgen_futures::{spawn_local, JsFuture},
};

#[wasm_bindgen]
pub fn main() {
    console_error_panic_hook::set_once();
    inject_app_global_css();
    Css::inject_css(EUV_MD_CSS);
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
