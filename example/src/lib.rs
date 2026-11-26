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

#[cfg(debug_assertions)]
use euv_automation::AI_SCHEMA_PROTOCOL_VERSION;
#[cfg(test)]
use euv_automation::{
    AiAction, AiAnomalyKind, AiLayoutAnomaly, AiRuntimeSnapshot, AiSchemaRequest, AiSchemaResponse,
};
#[cfg(any(debug_assertions, test))]
use euv_automation::{AiAppHooks, AiJsonValue, AiSchemaProtocol};
#[cfg(target_arch = "wasm32")]
use std::cell::{RefCell, RefMut};
use std::{cell::Cell, f32::consts::PI, rc::Rc};
#[cfg(not(target_arch = "wasm32"))]
use std::{
    fs,
    io::{Error, Read, Write},
    net::{SocketAddr, TcpListener},
    path::{Path, PathBuf},
    thread::spawn,
};
use {
    euv::*,
    euv_automation::DevAutomationBridge,
    euv_ui::*,
    lombok_macros::{Data, Getter},
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
