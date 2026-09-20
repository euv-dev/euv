//! euv CLI
//!
//! The official CLI tool for the euv UI framework,
//! providing run/build/fmt modes with hot reload and
//! wasm-pack integration.

mod build;
mod error;
mod fmt;
mod hmr;
mod logger;
mod mode;
mod server;

pub use build::inline::*;
use log::SetLoggerError;
pub use std::{
    error::Error,
    ffi::OsStr,
    fmt::{Display, Formatter},
    io::Error as IoError,
    string::FromUtf8Error,
};
pub use {build::*, error::*, fmt::*, hmr::*, logger::*, mode::*, server::*};

use std::{
    collections::HashMap,
    ffi,
    fmt::Arguments,
    io,
    net::{IpAddr, Ipv4Addr},
    path::{Component, Path, PathBuf},
    process::{Output, Stdio},
    slice::Iter,
    sync::{Arc, OnceLock},
    time::Duration,
};

use {
    clap::Parser,
    color_output::*,
    hyperlane::*,
    ignore::gitignore::{Gitignore, GitignoreBuilder},
    lombok_macros::*,
    notify::{Config, Event, RecommendedWatcher, RecursiveMode, Watcher},
    qrcode::{QrCode, render::unicode::Dense1x2},
    serde::Serialize,
    tokio::{
        fs::{
            ReadDir, canonicalize, create_dir_all, metadata, read, read_dir, read_to_string,
            remove_dir_all, remove_file, write,
        },
        process::Command,
        spawn,
        sync::{
            RwLock, RwLockWriteGuard, broadcast,
            mpsc::{Receiver, Sender, channel},
        },
        time::{Interval, interval, sleep},
    },
};
