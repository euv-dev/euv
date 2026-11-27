//! euv-automation
//!
//! Dev-only UI automation testing driver, AI Schema v1/v2 (`euv.ai.schema.v1` /
//! `euv.ai.schema.v2`) same-frame runtime/layout introspection protocol,
//! non-blocking async `DevAutomationBridge`, and versioned `euv.ui-e2e.v1`
//! recorder/replayer/verdict engine. All live dev bridge hooks compile to
//! zero-cost no-ops under release builds (`#[cfg(not(debug_assertions))]`).

mod dev_gate;
mod driver;
mod e2e;
mod schema;
mod selector;

#[cfg(test)]
#[path = "../tests/mod.rs"]
mod tests;

pub use {dev_gate::*, driver::*, e2e::*, schema::*, selector::*};

#[cfg(not(target_arch = "wasm32"))]
use std::time::Instant;
use std::{
    cell::{BorrowMutError, RefCell},
    collections::HashMap,
    fmt::{self, Formatter},
    rc::Rc,
    str::from_utf8,
};
#[cfg(all(debug_assertions, not(target_arch = "wasm32")))]
use std::{
    io::{self, ErrorKind, Read, Write},
    mem::take,
    net::{SocketAddr, TcpListener, TcpStream},
    str::SplitWhitespace,
    sync::{
        Arc,
        atomic::{AtomicBool, AtomicU64, AtomicUsize, Ordering},
        mpsc::{Receiver, SyncSender, TryRecvError, TrySendError, sync_channel},
    },
    thread::{self, Builder, JoinHandle, sleep},
    time::Duration,
};

#[cfg(test)]
use serde_json::{from_value, json};
use {
    euv::*,
    lombok_macros::{CustomDebug, Data, New},
    serde::{Deserialize, Serialize},
    serde_json::{Error, Value, from_str, to_string, to_string_pretty, to_value},
};
