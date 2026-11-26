//! euv-cli
//!
//! Cross-platform command-line toolchain for euv providing native run, build,
//! hot-reload dev server, dynamic WASM OTA hot-update server, Markdown
//! documentation site builder, cross-target check, formatting, environment
//! diagnostics, and project scaffolding across all seven platforms.

mod args;
mod command;
mod hot;
mod platform;

#[cfg(test)]
#[path = "../tests/mod.rs"]
mod tests;

pub use {args::*, command::*, hot::*, platform::*};

use std::{
    collections::BTreeMap,
    env::{self, VarError},
    ffi::OsStr,
    fs::{self, DirEntry, Metadata},
    io::{self, BufRead, BufReader, Error, ErrorKind, Write},
    net::{SocketAddr, TcpListener, TcpStream},
    path::{Path, PathBuf},
    process::{Child, Command, Output, Stdio},
    str::SplitWhitespace,
    sync::{
        Arc, Mutex, MutexGuard,
        atomic::{AtomicBool, AtomicU64, Ordering},
    },
    thread::{self, JoinHandle},
    time::{Duration, SystemTime, UNIX_EPOCH},
};

use {
    clap::{Args, Parser, Subcommand, ValueEnum},
    euv_core::{
        GraphicsBackend, PlatformCapabilities, PlatformKind, RenderConfig, RenderMode, Renderer,
        build_partial_widget_wasm, compute_wasm_checksum, compute_wasm_signature,
    },
    euv_docs::{DocsBuildOptions, GeneratedSiteReport, build_docs_bundle},
    lombok_macros::{Data, Getter, New},
    serde::{Deserialize, Serialize},
    serde_json::to_string_pretty,
};
