//! euv-cli
//!
//! Cross-platform command-line toolchain for euv providing native run, build,
//! cross-target check, formatting, environment diagnostics, and project
//! scaffolding across Web, Android, iOS, iPadOS, macOS, Windows, and Linux.

mod args;
mod command;
mod target;

#[cfg(test)]
#[path = "../tests/mod.rs"]
mod tests;

pub use {args::*, command::*, target::*};

use std::{
    env::{self, VarError},
    fs,
    io::{self, Error},
    path::{Path, PathBuf},
    process::{Command, Output},
};

use {
    clap::{Parser, Subcommand, ValueEnum},
    euv_core::{
        GraphicsBackend, PlatformCapabilities, PlatformKind, RenderConfig, RenderMode, Renderer,
    },
    lombok_macros::{Getter, New},
    serde::{Deserialize, Serialize},
    serde_json::to_string_pretty,
};
