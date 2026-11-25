//! euv-ui
//!
//! Cross-platform UI component library and 17-scheme design system for euv,
//! rendering pixel-consistently across Web, Android, iOS, iPadOS, macOS,
//! Windows, and Linux with zero CSS runtime overhead.

mod component;
mod hook;
mod theme;

#[cfg(test)]
#[path = "../tests/mod.rs"]
mod tests;

pub use {component::*, hook::*, theme::*};

use std::{
    cell::{Cell, RefCell},
    collections::HashMap,
    rc::Rc,
};

use {
    euv::*,
    lombok_macros::{CustomDebug, Data, Getter, New},
};
