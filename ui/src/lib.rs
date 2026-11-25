//! euv-ui
//!
//! Cross-platform UI component library and 17-scheme design system for euv,
//! rendering pixel-consistently across Web, Android, iOS, iPadOS, macOS,
//! Windows, and Linux with zero CSS runtime overhead.

mod component;
mod hook;
mod overlay_stack;
mod router;
mod theme;

#[cfg(test)]
#[path = "../tests/mod.rs"]
mod tests;

pub use {component::*, hook::*, overlay_stack::*, router::*, theme::*};

use std::{
    cell::{Cell, Ref, RefCell},
    collections::HashMap,
    rc::Rc,
};

use {
    euv::*,
    lombok_macros::{CustomDebug, Data, Getter, New},
};
