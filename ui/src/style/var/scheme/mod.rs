#[cfg(feature = "theme-antdesign")]
mod antdesign;
#[cfg(feature = "theme-bootstrap")]
mod bootstrap;
#[cfg(feature = "theme-cupertino")]
mod cupertino;
#[cfg(feature = "theme-fluent")]
mod fluent;
#[cfg(feature = "theme-material")]
mod material;
#[rustfmt::skip]
#[cfg(not(any(feature = "theme-antdesign", feature = "theme-material", feature = "theme-bootstrap", feature = "theme-cupertino", feature = "theme-fluent")))]
mod builtin;

#[cfg(feature = "theme-antdesign")]
pub use antdesign::c_scheme_antdesign;
#[cfg(feature = "theme-bootstrap")]
pub use bootstrap::c_scheme_bootstrap;
#[cfg(feature = "theme-cupertino")]
pub use cupertino::c_scheme_cupertino;
#[cfg(feature = "theme-fluent")]
pub use fluent::c_scheme_fluent;
#[cfg(feature = "theme-material")]
pub use material::c_scheme_material;
#[rustfmt::skip]
#[cfg(not(any(feature = "theme-antdesign", feature = "theme-material", feature = "theme-bootstrap", feature = "theme-cupertino", feature = "theme-fluent")))]
pub use builtin::c_scheme_builtin;

use super::*;
