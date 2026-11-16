#[path = "../class/mod.rs"]
mod class;
#[path = "../component/mod.rs"]
mod component;
#[path = "../computed/mod.rs"]
mod computed;
#[path = "../dependency_discovery/mod.rs"]
mod dependency_discovery;
#[path = "../html_static_style/mod.rs"]
mod html_static_style;
#[path = "../unsafe_no_inline/mod.rs"]
mod unsafe_no_inline;
#[path = "../var/mod.rs"]
mod var;
#[path = "../vars/mod.rs"]
mod vars;
#[path = "../watch/mod.rs"]
mod watch;

use euv::*;

use std::{
    cell::Cell,
    env,
    error::Error,
    fs::{create_dir_all, remove_dir_all, write},
    panic::{AssertUnwindSafe, catch_unwind},
    path::{Component, Path, PathBuf},
    process::{self, Command, Output},
    ptr,
    sync::atomic::{AtomicUsize, Ordering},
    thread,
};
