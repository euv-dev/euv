//! euv-example
//!
//! A demonstration application showcasing the euv component system,
//! reactive signals, routing, and HTML macros.

mod app;
mod component;
mod page;
mod style;

pub(crate) use {app::*, component::*, page::*, style::*};

pub use std::{
    cell::RefMut,
    cmp::Ordering,
    collections::HashSet,
    fmt::{self, Debug, Display, Formatter},
    iter::FilterMap,
    ops::Range,
    str::Split,
};

use std::{
    cell::{Cell, RefCell},
    collections::HashMap,
    rc::Rc,
};

use {
    compare_version::*,
    euv::{js_sys::*, wasm_bindgen::prelude::*, wasm_bindgen_futures::*, web_sys::*, *},
    euv_engine::*,
    euv_ui::*,
};

use {
    qrcode::{QrCode, render::svg, types::QrError},
    serde::{Deserialize, Serialize},
};

/// Entry point for the euv example application.
#[wasm_bindgen]
pub fn main() {
    console_error_panic_hook::set_once();
    inject_app_global_css();
    App::mount(APP_MOUNT_SELECTOR, app);
}
