#[cfg(all(feature = "theme-shadcn", not(feature = "theme-custom")))]
mod r#const;
mod r#fn;

#[cfg(all(feature = "theme-shadcn", not(feature = "theme-custom")))]
use r#const::*;

use super::*;
