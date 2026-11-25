use super::*;

/// Resolves the compile-time default `DesignScheme` selected by Cargo feature flags.
///
/// # Returns
///
/// - `DesignScheme` - Active default design scheme.
#[must_use]
pub const fn active_feature_scheme() -> DesignScheme {
    if cfg!(feature = "theme-shadcn") {
        DesignScheme::Shadcn
    } else if cfg!(feature = "theme-cupertino") {
        DesignScheme::Cupertino
    } else if cfg!(feature = "theme-material") {
        DesignScheme::Material
    } else if cfg!(feature = "theme-fluent") {
        DesignScheme::Fluent
    } else if cfg!(feature = "theme-antdesign") {
        DesignScheme::AntDesign
    } else if cfg!(feature = "theme-bootstrap") {
        DesignScheme::Bootstrap
    } else if cfg!(feature = "theme-daisyui") {
        DesignScheme::DaisyUi
    } else if cfg!(feature = "theme-geist") {
        DesignScheme::Geist
    } else if cfg!(feature = "theme-heroui") {
        DesignScheme::HeroUi
    } else if cfg!(feature = "theme-linear") {
        DesignScheme::Linear
    } else if cfg!(feature = "theme-mantine") {
        DesignScheme::Mantine
    } else if cfg!(feature = "theme-monochrome") {
        DesignScheme::Monochrome
    } else if cfg!(feature = "theme-notion") {
        DesignScheme::Notion
    } else if cfg!(feature = "theme-park") {
        DesignScheme::Park
    } else if cfg!(feature = "theme-radix") {
        DesignScheme::Radix
    } else if cfg!(feature = "theme-stripe") {
        DesignScheme::Stripe
    } else if cfg!(feature = "theme-supabase") {
        DesignScheme::Supabase
    } else {
        DesignScheme::Shadcn
    }
}
