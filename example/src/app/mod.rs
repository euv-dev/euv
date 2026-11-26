mod r#const;
mod r#fn;
#[cfg(target_arch = "wasm32")]
mod r#static;
#[cfg(target_arch = "wasm32")]
mod r#type;

pub use {r#const::*, r#fn::*};

#[cfg(target_arch = "wasm32")]
pub(crate) use {r#static::*, r#type::*};

use super::*;
