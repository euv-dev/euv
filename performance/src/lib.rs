//! euv-performance
//!
//! Reproducible CPU and allocation benchmarks for the complete euv runtime.

mod config;
mod load;
mod memory;
mod runner;
#[cfg(test)]
#[path = "../tests/mod.rs"]
mod tests;

pub use runner::*;

pub(crate) use {config::*, load::*, memory::*};

#[cfg(feature = "allocation-tracking")]
use std::{
    alloc::{GlobalAlloc, Layout, System},
    thread::AccessError,
};
use std::{
    cell::Cell,
    env::{
        self,
        consts::{ARCH, OS},
    },
    error::Error,
    fs,
    hint::black_box,
    mem::{replace, take},
    num::NonZeroUsize,
    path::PathBuf,
    process::{Command, Output},
    rc::Rc,
    thread::available_parallelism,
    time::{Instant, SystemTime, UNIX_EPOCH},
};

use {euv::*, euv_chart::*, euv_engine::*, serde::Serialize, serde_json::to_string_pretty};
