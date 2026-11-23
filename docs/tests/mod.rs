mod anchor;
#[path = "../src/document_language/mod.rs"]
mod build_document_language;
mod catalog;
mod code_block;
mod code_palette;
mod description;
#[path = "../src/document_html/mod.rs"]
mod document_html;
mod document_language;
mod focus_ring;
mod footer;
mod header_links;
mod header_search;
mod highlight;
mod hover;
mod locale;
mod mermaid;
mod route;
mod scroll;
mod sidebar;
mod toc_active_contrast;
mod toc_navigation;

use {super::*, build_document_language::*, document_html::*, euv_ui::EuvSidebarItem};

use std::{
    borrow::Cow,
    collections::HashSet,
    env,
    ffi::OsStr,
    fs, io,
    path::{Path, PathBuf},
    process,
};
