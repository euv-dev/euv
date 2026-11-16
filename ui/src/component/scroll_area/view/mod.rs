mod r#const;
mod r#fn;
#[cfg(target_arch = "wasm32")]
mod r#impl;
mod r#struct;
#[cfg(target_arch = "wasm32")]
mod r#type;

pub use {r#fn::*, r#struct::*};

#[cfg(any(target_arch = "wasm32", test))]
pub(crate) use r#const::*;
#[cfg(target_arch = "wasm32")]
pub(crate) use r#type::*;

use super::*;
