//! euv-performance
//!
//! Reproducible CPU, reactive propagation, retained layout cache, and
//! multi-mode rendering benchmark suite for the cross-platform euv runtime.

mod suite;

#[cfg(test)]
#[path = "../tests/mod.rs"]
mod tests;

pub use suite::*;

use std::{
    rc::Rc,
    time::{Duration, Instant},
};

use {
    euv::*,
    euv_ui::*,
    lombok_macros::{Getter, New},
    serde::{Deserialize, Serialize},
    serde_json::to_string_pretty,
};
