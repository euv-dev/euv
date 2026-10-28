#[cfg(feature = "theme-antdesign")]
mod antdesign;
#[cfg(feature = "theme-bootstrap")]
mod bootstrap;
#[cfg(feature = "theme-cupertino")]
mod cupertino;
#[cfg(feature = "theme-daisyui")]
mod daisyui;
#[cfg(feature = "theme-fluent")]
mod fluent;
#[cfg(feature = "theme-geist")]
mod geist;
#[cfg(feature = "theme-heroui")]
mod heroui;
#[cfg(feature = "theme-mantine")]
mod mantine;
#[cfg(feature = "theme-material")]
mod material;
#[cfg(feature = "theme-monochrome")]
mod monochrome;
#[cfg(feature = "theme-park")]
mod park;
#[cfg(feature = "theme-radix")]
mod radix;
mod shadcn;

#[cfg(feature = "theme-antdesign")]
pub use antdesign::c_scheme_antdesign;
#[cfg(feature = "theme-bootstrap")]
pub use bootstrap::c_scheme_bootstrap;
#[cfg(feature = "theme-cupertino")]
pub use cupertino::c_scheme_cupertino;
#[cfg(feature = "theme-daisyui")]
pub use daisyui::c_scheme_daisyui;
#[cfg(feature = "theme-fluent")]
pub use fluent::c_scheme_fluent;
#[cfg(feature = "theme-geist")]
pub use geist::c_scheme_geist;
#[cfg(feature = "theme-heroui")]
pub use heroui::c_scheme_heroui;
#[cfg(feature = "theme-mantine")]
pub use mantine::c_scheme_mantine;
#[cfg(feature = "theme-material")]
pub use material::c_scheme_material;
#[cfg(feature = "theme-monochrome")]
pub use monochrome::c_scheme_monochrome;
#[cfg(feature = "theme-park")]
pub use park::c_scheme_park;
#[cfg(feature = "theme-radix")]
pub use radix::c_scheme_radix;
pub use shadcn::c_scheme_shadcn;

use super::*;
