mod anchor;
mod code_block;
#[cfg(feature = "theme-custom")]
mod custom_layout;
mod description;
mod footer;
mod header_links;
mod header_search;
mod highlight;
mod hover;
mod locale;
mod route;
mod scroll;
mod sidebar;
mod toc_navigation;

use super::*;
use euv_ui::EuvSidebarItem;

use std::{
    borrow::Cow,
    collections::HashSet,
    ffi::OsStr,
    fs,
    path::{Path, PathBuf},
};
