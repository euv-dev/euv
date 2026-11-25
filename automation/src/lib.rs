//! euv-automation
//!
//! Dev-only UI automation testing driver and AI Schema v1 (`euv.ai.schema.v1`)
//! runtime/layout introspection protocol for euv. All live dev bridge hooks compile
//! to zero-cost no-ops under release builds (`#[cfg(not(debug_assertions))]`).

mod dev_gate;
mod driver;
mod schema;
mod selector;

#[cfg(test)]
#[path = "../tests/mod.rs"]
mod tests;

pub use {dev_gate::*, driver::*, schema::*, selector::*};

use std::fmt::{self, Formatter};
#[cfg(all(debug_assertions, not(target_arch = "wasm32")))]
use std::io::{Error, ErrorKind, Read, Write};
#[cfg(all(debug_assertions, not(target_arch = "wasm32")))]
use std::net::{SocketAddr, TcpListener, TcpStream};
#[cfg(all(debug_assertions, not(target_arch = "wasm32")))]
use std::time::Duration;
#[cfg(not(target_arch = "wasm32"))]
use std::time::Instant;

#[cfg(test)]
use serde_json::from_value;
use {
    euv::*,
    lombok_macros::{CustomDebug, Data, New},
    serde::{Deserialize, Serialize},
    serde_json::{Value, from_str, to_string, to_string_pretty, to_value},
};
