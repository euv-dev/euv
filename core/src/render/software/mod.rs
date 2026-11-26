mod r#const;
mod r#fn;
mod r#impl;
#[cfg(not(target_arch = "wasm32"))]
mod r#static;
mod r#struct;
mod r#type;

pub use {r#fn::*, r#struct::*};

#[cfg(not(target_arch = "wasm32"))]
pub(crate) use r#static::*;
pub(crate) use {r#const::*, r#type::*};

use super::*;
