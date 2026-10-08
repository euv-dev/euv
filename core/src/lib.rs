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
    iter::Iterator,
    marker::PhantomData,
    mem::{swap, take, zeroed},
    panic::{AssertUnwindSafe, catch_unwind},
    sync::{LazyLock, OnceLock, RwLock, RwLockReadGuard},
    thread::LocalKey,
};

pub use {js_sys::*, lombok_macros::*, wasm_bindgen::prelude::*, web_sys::*};

pub(crate) use bin_encode_decode::{Charset, EncodeError};

/// The wall clock [`now_micros`] reads on a host build. On wasm the clock
/// comes from JS instead, so importing this there is an `unused_imports`
/// warning — the mirror of the dead-const warning the host build would give
/// the other half of the same function.
#[cfg(not(target_arch = "wasm32"))]
pub(crate) use std::time::{SystemTime, UNIX_EPOCH};

pub(crate) use renderer::*;

use std::{
    any::Any,
    cell::{Cell, Ref, RefCell, RefMut, UnsafeCell},
    num::ParseIntError,
    rc::Rc,
    str::from_utf8,
    sync::{
        PoisonError,
        atomic::{AtomicBool, AtomicUsize, Ordering},
    },
    thread::AccessError,
    vec::Vec,
};
