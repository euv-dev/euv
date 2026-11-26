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

pub(crate) use parser::{
    deduplicate_slug, heading_level_num, load_doc_site_excluding, section_sort_rank,
    should_skip_dir, slugify,
};

#[cfg(test)]
use std::env;
use std::{
    collections::{BTreeMap, BTreeSet},
    ffi::OsStr,
    fs::{self, DirEntry},
    io::{self, Error, ErrorKind},
    mem::take,
    path::{Component, Path, PathBuf},
    str::Chars,
};

use {
    chrono::Utc,
    euv::*,
    euv_ui::*,
    lombok_macros::{Data, Getter, New},
    pulldown_cmark::{
        Alignment, CodeBlockKind, CowStr, Event, HeadingLevel, Options, Parser, Tag, TagEnd,
    },
    serde::{Deserialize, Serialize},
    serde_json::{to_string, to_string_pretty},
    serde_yaml::from_str,
};
