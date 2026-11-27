mod r#const;
mod r#fn;
mod r#impl;
#[cfg(any(target_os = "macos", target_os = "ios"))]
mod r#static;
mod r#struct;

pub use {r#const::*, r#fn::*, r#struct::*};

#[cfg(any(target_os = "macos", target_os = "ios"))]
pub(crate) use r#static::*;

use super::*;
