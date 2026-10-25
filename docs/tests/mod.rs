mod anchor;
mod locale;
mod route;
mod scroll;
mod sidebar;

use super::*;
use euv_ui::EuvSidebarItem;

use std::{
    collections::HashSet,
    ffi::OsStr,
    fs, io,
    path::{Path, PathBuf},
};
