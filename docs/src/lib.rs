//! euv-docs
//!
//! The documentation-site crate for the euv UI framework, built by
//! `docs/build.rs` from the example app's locale markdown so the
//! generated pages stay in sync with the source tree.

mod app;
mod component;
mod data;
mod locale;
mod route;
mod router;

mod generated {
    include!(concat!(env!("OUT_DIR"), "/docs_gen.rs"));
}

pub use {
    locale::*,
    route::*,
    router::*,
    std::{rc::Rc, sync::Arc},
};

pub(crate) use {
    app::*,
    component::*,
    data::*,
    js_sys::{Promise, decode_uri_component, eval},
    {Event, HtmlInputElement, KeyboardEvent, Location},
};

use {
    euv::{wasm_bindgen::prelude::*, *},
    euv_ui::*,
    wasm_bindgen_futures::{JsFuture, spawn_local},
};

/// Wasm entry point: injects the global stylesheet, mounts the docs
/// shell into the host page, and installs the two DOM side effects the
/// router cannot express declaratively (image load marking and the
/// table-of-contents scroll spy).
#[wasm_bindgen]
pub fn main() {
    console_error_panic_hook::set_once();
    inject_app_global_css();
    Css::inject_css(euv_md_css());
    Css::inject_css(APP_GLOBAL_CSS);
    App::mount(APP_MOUNT_SELECTOR, app);
    let _: Result<JsValue, JsValue> = js_sys::eval(IMAGE_LOAD_WATCHER_JS);
    let _: Result<JsValue, JsValue> = js_sys::eval(TOC_SCROLL_SPY_JS);
}
