//! euv-automation
//!
//! Real-browser UI operations, assertions and a reproducible euv component fixture.

mod contract;

#[cfg(not(target_arch = "wasm32"))]
mod browser;
#[cfg(not(target_arch = "wasm32"))]
mod config;
#[cfg(target_arch = "wasm32")]
mod fixture;
#[cfg(not(target_arch = "wasm32"))]
mod host;
#[cfg(not(target_arch = "wasm32"))]
mod report;
#[cfg(target_arch = "wasm32")]
mod style;
#[cfg(test)]
#[path = "../tests/mod.rs"]
mod tests;
#[cfg(not(target_arch = "wasm32"))]
mod workflow;

#[cfg(target_arch = "wasm32")]
pub use fixture::mount_fixture;
#[cfg(not(target_arch = "wasm32"))]
pub use {browser::*, config::*, workflow::run};

pub(crate) use contract::*;
#[cfg(target_arch = "wasm32")]
pub(crate) use style::*;
#[cfg(not(target_arch = "wasm32"))]
pub(crate) use {host::*, report::*};

#[cfg(all(test, not(target_arch = "wasm32")))]
use workflow::Scenario;

#[cfg(target_arch = "wasm32")]
use euv::*;
#[cfg(not(target_arch = "wasm32"))]
use euv::{Data, Getter, New};

#[cfg(target_arch = "wasm32")]
use euv_ui::*;
#[cfg(target_arch = "wasm32")]
use std::rc::Rc;

#[cfg(not(target_arch = "wasm32"))]
use std::{
    env,
    fs::{self, File},
    io::{self, Read, Write},
    net::TcpListener,
    num::ParseIntError,
    path::{Path, PathBuf},
    process::{Child, ChildStdin, Command, Output, Stdio},
    sync::OnceLock,
    thread::sleep,
    time::{Duration, Instant},
};
#[cfg(not(target_arch = "wasm32"))]
use {
    hyperlane::*,
    serde::Serialize,
    serde_json::{Error, Value, from_slice, json, to_string_pretty, to_vec},
};

#[cfg(not(target_arch = "wasm32"))]
use tokio::task::{JoinError, spawn_blocking};
