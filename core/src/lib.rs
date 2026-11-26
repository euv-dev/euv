//! euv-core
//!
//! Core cross-platform runtime for euv featuring arena-based fine-grained reactivity,
//! deterministic Flexbox/Grid layout, flat retained scene graph, multi-mode rendering
//! (Auto, Software, Hardware, Hybrid), unified window abstractions, and direct OS API
//! backends for Web, Android, iOS, iPadOS, macOS, Windows, and Linux.

mod app;
mod dynamic;
mod font;
mod game;
mod layout;
mod media;
mod platform;
mod reactive;
mod render;
mod scene;
mod style;
mod system;
mod window;

#[cfg(test)]
#[path = "../tests/mod.rs"]
mod tests;

pub use {
    app::*, dynamic::*, font::*, game::*, layout::*, media::*, platform::*, reactive::*, render::*,
    scene::*, style::*, system::*, window::*,
};

#[cfg(target_os = "linux")]
use std::env;
#[cfg(any(target_os = "macos", target_os = "ios", target_os = "linux"))]
use std::ffi::CStr;
#[cfg(unix)]
use std::ffi::CString;
#[cfg(unix)]
use std::ffi::c_char;
#[cfg(any(unix, windows))]
use std::ffi::c_void;
#[cfg(target_os = "windows")]
use std::mem::size_of;
#[cfg(any(target_os = "macos", target_os = "ios"))]
use std::mem::size_of_val;
#[cfg(any(target_os = "linux", target_os = "android"))]
use std::path::Path;
#[cfg(target_os = "android")]
use std::ptr::copy_nonoverlapping;
#[cfg(any(target_os = "macos", target_os = "ios", target_os = "linux", windows))]
use std::ptr::null;
use std::{
    any::{Any, TypeId},
    cell::{Cell, Ref, RefCell, RefMut},
    collections::{HashMap, HashSet},
    error::Error,
    f32::consts::{FRAC_PI_2, PI, SQRT_2, TAU},
    fmt::{self, Formatter},
    hash::{Hash, Hasher},
    marker::PhantomData,
    mem::{swap, take, transmute},
    ops::{Add, Mul, Sub},
    ptr::null_mut,
    rc::Rc,
    slice::from_raw_parts_mut,
    str::{Chars, from_utf8},
    sync::atomic::{AtomicPtr, AtomicUsize, Ordering},
};
#[cfg(not(target_arch = "wasm32"))]
use std::{
    fs,
    io::{self, Read, Write},
    net::{SocketAddr, TcpStream, ToSocketAddrs},
    slice::from_raw_parts,
    sync::{Arc, Condvar, Mutex, MutexGuard, OnceLock, PoisonError},
    thread::{Builder, JoinHandle, available_parallelism, sleep},
    time::{Duration, Instant},
};
#[cfg(all(test, not(target_arch = "wasm32")))]
use std::{net::TcpListener, thread::spawn};

use {
    lombok_macros::{CustomDebug, Data, Getter, New},
    serde::{Deserialize, Serialize},
    serde_json::{Value, from_str, to_string, to_string_pretty},
};

#[cfg(target_arch = "wasm32")]
use {
    js_sys::Object,
    wasm_bindgen::{Clamped, JsCast, JsValue},
    web_sys::{
        CanvasRenderingContext2d, Document, DomRect, Element, HtmlCanvasElement, ImageData,
        MediaQueryList, TextMetrics, WebGl2RenderingContext, WebGlProgram, WebGlShader, Window,
        window,
    },
};
