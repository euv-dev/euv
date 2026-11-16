mod r#const;
mod r#fn;
mod r#impl;
#[cfg(target_arch = "wasm32")]
mod r#static;
mod r#struct;

pub use {r#fn::*, r#struct::*};

pub(crate) use r#const::*;
#[cfg(target_arch = "wasm32")]
pub(crate) use r#static::*;

use super::*;
