mod hook;
mod lifecycle;
mod view;

pub use lifecycle::mount_fixture;

pub(crate) use {hook::*, view::*};

use super::*;
