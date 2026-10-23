mod r#const;
mod r#fn;
mod r#impl;
mod r#struct;

pub use {r#fn::*, r#struct::*};

#[cfg(target_arch = "wasm32")]
pub(crate) use r#const::*;

use super::*;
