//! euv-docs
//!
//! Cross-platform documentation viewer and static site generator powered by
//! euv and euv-ui, parsing Markdown and YAML frontmatter into native UI scenes
//! or static HTML/JSON bundles.

mod generator;
mod model;
mod parser;
mod viewer;

#[cfg(test)]
#[path = "../tests/mod.rs"]
mod tests;

pub use {generator::*, model::*, parser::*, viewer::*};

#[cfg(test)]
use std::env;
use std::{
    ffi::OsStr,
    fs::{self, DirEntry},
    io::{self, Error, ErrorKind},
    path::{Component, Path, PathBuf},
};

use {
    chrono::Utc,
    euv::*,
    euv_ui::*,
    lombok_macros::{Getter, New},
    pulldown_cmark::{CodeBlockKind, Event, HeadingLevel, Options, Parser, Tag, TagEnd},
    serde::{Deserialize, Serialize},
    serde_json::to_string_pretty,
    serde_yaml::from_str,
};
