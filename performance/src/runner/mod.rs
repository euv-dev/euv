mod r#const;
mod r#fn;
mod r#struct;

pub use r#fn::run;

#[cfg(test)]
pub(crate) use r#fn::{measure, validate_work};
pub(crate) use {r#const::*, r#struct::*};

use super::*;
