mod r#const;
mod r#fn;
mod r#impl;
#[cfg(not(target_arch = "wasm32"))]
mod r#static;
mod r#struct;

pub use {r#fn::*, r#struct::*};

pub(crate) use r#const::*;
#[cfg(not(target_arch = "wasm32"))]
pub(crate) use r#static::*;

use super::*;
