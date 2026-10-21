mod locale;
mod route;
mod scroll;
mod sidebar;

use euv_docs::*;
use euv_ui::EuvSidebarItem;

use std::{
    collections::HashSet,
    ffi::OsStr,
    fs, io,
    path::{Path, PathBuf},
};
