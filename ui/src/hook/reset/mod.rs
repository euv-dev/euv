mod r#const;
mod r#fn;

#[cfg(target_arch = "wasm32")]
pub(crate) use r#const::*;
pub(crate) use r#fn::use_form_reset;

use super::*;
