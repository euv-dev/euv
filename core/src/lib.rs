//! euv-core
//!
//! Core cross-platform runtime for euv featuring arena-based fine-grained reactivity,
//! deterministic Flexbox/Grid layout, flat retained scene graph, multi-mode rendering
//! (Auto, Software, Hardware, Hybrid), unified window abstractions, and direct OS API
//! backends for Web, Android, iOS, iPadOS, macOS, Windows, and Linux.

mod app;
mod font;
mod layout;
mod platform;
mod reactive;
mod render;
mod scene;
mod style;
mod window;

#[cfg(test)]
#[path = "../tests/mod.rs"]
mod tests;

pub use {
    app::*, font::*, layout::*, platform::*, reactive::*, render::*, scene::*, style::*, window::*,
};

#[cfg(target_os = "macos")]
use std::ffi::CStr;
#[cfg(any(target_os = "macos", target_os = "linux"))]
use std::ffi::CString;
#[cfg(any(target_os = "macos", target_os = "ios", target_os = "linux"))]
use std::ffi::c_char;
#[cfg(any(unix, windows))]
use std::ffi::c_void;
#[cfg(target_os = "windows")]
use std::mem::size_of;
#[cfg(any(target_os = "macos", target_os = "ios", target_os = "linux"))]
use std::mem::transmute;
#[cfg(target_os = "android")]
use std::ptr::copy_nonoverlapping;
#[cfg(any(target_os = "macos", target_os = "ios", target_os = "linux", windows))]
use std::ptr::null;
#[cfg(any(unix, windows))]
use std::ptr::null_mut;
use std::{
    any::Any,
    cell::{Ref, RefCell, RefMut},
    collections::HashMap,
    error::Error,
    fmt::{self, Formatter},
    hash::{Hash, Hasher},
    marker::PhantomData,
    mem::take,
    rc::Rc,
};

use lombok_macros::{CustomDebug, Data, Getter, New};

#[cfg(target_arch = "wasm32")]
use {
    js_sys::Object,
    wasm_bindgen::{Clamped, JsCast, JsValue},
    web_sys::{
        CanvasRenderingContext2d, Document, Element, HtmlCanvasElement, ImageData, MediaQueryList,
        Window, window,
    },
};
