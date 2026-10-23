//! euv-ui
//!
//! Reusable UI component library for the euv framework,
//! providing buttons, cards, modals, inputs, theme management, and more.

mod component;
mod dom;
mod hook;
mod style;

pub use {component::*, hook::*, style::*};

use dom::*;

use euv::*;

use std::{
    any::Any,
    cell::{Cell, Ref, RefCell, RefMut, UnsafeCell},
    collections::{HashMap, HashSet},
    fmt::{self, Debug, Display, Formatter},
    hash::Hash,
    ops::Deref,
    panic::{AssertUnwindSafe, UnwindSafe, catch_unwind},
    ptr::{addr_of_mut, eq},
    rc::Rc,
    sync::{
        LazyLock, OnceLock, PoisonError, RwLock, RwLockReadGuard, RwLockWriteGuard,
        atomic::{AtomicBool, AtomicUsize, Ordering},
    },
};

#[cfg(not(target_arch = "wasm32"))]
use std::time::{Duration, SystemTime, UNIX_EPOCH};

// The weak handle the splitter's drag teardown uses to reach its own
// slot without keeping it alive. Only the browser build has a drag.
#[cfg(target_arch = "wasm32")]
use std::rc::Weak;

use wasm_bindgen_futures::*;
