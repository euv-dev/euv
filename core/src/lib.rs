//! euv-core
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
    fmt::{self, Debug, Display, Formatter},
    hash::{Hash, Hasher},
    marker::PhantomData,
    mem::{swap, take, zeroed},
    panic::{AssertUnwindSafe, catch_unwind},
    sync::{LazyLock, OnceLock, RwLock, RwLockReadGuard},
    thread::LocalKey,
};

pub use {js_sys::*, lombok_macros::*, wasm_bindgen::prelude::*, web_sys::*};

pub use bin_encode_decode::{Charset, EncodeError};

pub(crate) use std::iter::Iterator;

pub(crate) use renderer::*;

use std::{
    any::Any,
    cell::{Cell, Ref, RefCell, UnsafeCell},
    num::ParseIntError,
    rc::Rc,
    sync::atomic::{AtomicBool, AtomicUsize, Ordering},
    time::{SystemTime, UNIX_EPOCH},
    vec::Vec,
};
