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

pub use {app::*, event::*, noderef::*, reactive::*, vdom::*};

pub use std::{
    borrow::Cow,
    collections::hash_map::DefaultHasher,
    collections::{HashMap, HashSet, VecDeque},
    fmt::{self, Debug, Display, Formatter, Result as FmtResult},
    hash::{Hash, Hasher},
    marker::PhantomData,
    mem::{swap, take, zeroed},
    panic::{AssertUnwindSafe, catch_unwind},
};

pub(crate) use renderer::*;

use std::{
    any::Any,
    cell::{Ref, RefCell, UnsafeCell},
    iter::Iterator,
    num::ParseIntError,
    ops::Deref,
    rc::Rc,
    sync::{
        LazyLock,
        atomic::{AtomicBool, AtomicUsize, Ordering},
    },
    vec::Vec,
};

use {js_sys::*, lombok_macros::*, wasm_bindgen::prelude::*, web_sys::*};
