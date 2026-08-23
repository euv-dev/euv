//! euv
//!
//! A declarative, cross-platform UI framework for Rust with virtual DOM,
//! reactive signals, and HTML macros for WebAssembly.

mod app;
mod event;
mod noderef;
mod reactive;
mod renderer;
mod vdom;

#[cfg(test)]
mod tests;

pub use {app::*, event::*, noderef::*, reactive::*, vdom::*};

pub(crate) use renderer::*;

use std::{
    any::Any,
    cell::{Ref, RefCell, UnsafeCell},
    collections::{HashMap, HashSet},
    fmt::{self, Display, Formatter},
    marker::PhantomData,
    mem::{swap, take},
    num::ParseIntError,
    ops::Deref,
    rc::Rc,
    sync::{
        LazyLock,
        atomic::{AtomicBool, AtomicUsize, Ordering},
    },
};

use {js_sys::*, lombok_macros::*, wasm_bindgen::prelude::*, web_sys::*};
