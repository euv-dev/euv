mod class;
mod component;
mod computed;
mod html_static_style;
mod unsafe_no_inline;
mod var;
mod vars;
mod watch;

use euv::*;

use std::cell::Cell;
use std::panic::{AssertUnwindSafe, catch_unwind};
use std::sync::atomic::{AtomicUsize, Ordering};
