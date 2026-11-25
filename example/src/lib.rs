//! euv-example
//!
//! Cross-platform native showcase application built with euv and euv-ui,
//! demonstrating fine-grained reactivity, multi-mode rendering, window settings,
//! 17 design schemes, virtualized lists, and overlays.

mod app;
mod pages;
mod state;

#[cfg(test)]
#[path = "../tests/mod.rs"]
mod tests;

pub use {app::*, pages::*, state::*};

#[cfg(target_arch = "wasm32")]
use std::cell::{RefCell, RefMut};
use std::rc::Rc;
#[cfg(not(target_arch = "wasm32"))]
use std::{
    fs,
    io::Error,
    path::{Path, PathBuf},
};
use {
    euv::*,
    euv_ui::*,
    lombok_macros::{Data, Getter, New},
};
#[cfg(target_arch = "wasm32")]
use {
    js_sys::Function,
    wasm_bindgen::{JsCast, JsValue, closure::Closure, prelude::wasm_bindgen},
    web_sys::{
        Document, DomRect, Event, HtmlCanvasElement, KeyboardEvent, MouseEvent, WheelEvent, Window,
        window,
    },
};
