mod r#impl;
mod r#static;
mod r#struct;
mod r#trait;
mod r#type;

pub use r#struct::*;

pub(crate) use {r#static::*, r#trait::*, r#type::*};

use super::*;
