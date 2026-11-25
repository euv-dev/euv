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
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::wasm_bindgen;
use {
    euv::*,
    euv_ui::*,
    lombok_macros::{Data, Getter, New},
};
