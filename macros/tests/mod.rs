mod class;
mod component;
mod computed;
mod html_static_style;
mod unsafe_no_inline;
mod var;
mod vars;
mod watch;

use euv::*;

use std::{
    cell::Cell,
    panic::{AssertUnwindSafe, catch_unwind},
    ptr,
    sync::atomic::{AtomicUsize, Ordering},
    thread,
};
