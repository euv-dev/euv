//! euv-example
//!
//! A demonstration application showcasing the euv component system,
//! reactive signals, routing, and HTML macros.

mod app;
mod component;
mod page;
mod style;

pub(crate) use {app::*, component::*, page::*, style::*};

pub(crate) use std::{
    cell::RefMut,
    cmp::Ordering,
    collections::HashSet,
    f64::consts::{FRAC_PI_2, PI, TAU},
    fmt::{self, Debug, Display, Formatter},
    ops::Range,
};

use std::{
    cell::{Cell, RefCell},
    collections::HashMap,
    iter::Iterator,
    rc::Rc,
};

use {
    compare_version::*,
    euv::{
        js_sys::*,
        wasm_bindgen::{Clamped, prelude::*},
        wasm_bindgen_futures::*,
        *,
    },
    euv_engine::*,
    euv_ui::*,
};

use {
    qrcode::{QrCode, render::svg, types::QrError},
    serde::{Deserialize, Serialize},
};

/// Mounts the demo application into the page.
///
/// This is the wasm entry point: the engine calls it on load, so it is
/// never invoked by a test and is not a function anything can name.
#[wasm_bindgen]
pub fn main() {
    console_error_panic_hook::set_once();
    inject_app_global_css();
    App::mount(APP_MOUNT_SELECTOR, app);
}
