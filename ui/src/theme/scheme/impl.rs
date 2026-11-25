use super::*;

/// Enumeration and string helpers for `DesignScheme`.
impl DesignScheme {
    /// Returns all 17 built-in design schemes in canonical order.
    ///
    /// # Returns
    ///
    /// - `[Self; 17]` - Complete array of supported design schemes.
    #[must_use]
    pub const fn all() -> [Self; 17] {
        [
            Self::Shadcn,
            Self::Cupertino,
            Self::Material,
            Self::Fluent,
            Self::AntDesign,
            Self::Bootstrap,
            Self::DaisyUi,
            Self::Geist,
            Self::HeroUi,
            Self::Linear,
            Self::Mantine,
            Self::Monochrome,
            Self::Notion,
            Self::Park,
            Self::Radix,
            Self::Stripe,
            Self::Supabase,
        ]
    }

    /// Returns the canonical lowercase identifier for this design scheme.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Design scheme variant.
    ///
    /// # Returns
    ///
    /// - `&'static str` - Scheme identifier string.
    #[must_use]
    pub const fn as_str(&self) -> &'static str {
        match *self {
            Self::Shadcn => SCHEME_NAME_SHADCN,
            Self::Cupertino => SCHEME_NAME_CUPERTINO,
            Self::Material => SCHEME_NAME_MATERIAL,
            Self::Fluent => SCHEME_NAME_FLUENT,
            Self::AntDesign => SCHEME_NAME_ANTDESIGN,
            Self::Bootstrap => SCHEME_NAME_BOOTSTRAP,
            Self::DaisyUi => SCHEME_NAME_DAISYUI,
            Self::Geist => SCHEME_NAME_GEIST,
            Self::HeroUi => SCHEME_NAME_HEROUI,
            Self::Linear => SCHEME_NAME_LINEAR,
            Self::Mantine => SCHEME_NAME_MANTINE,
            Self::Monochrome => SCHEME_NAME_MONOCHROME,
            Self::Notion => SCHEME_NAME_NOTION,
            Self::Park => SCHEME_NAME_PARK,
            Self::Radix => SCHEME_NAME_RADIX,
            Self::Stripe => SCHEME_NAME_STRIPE,
            Self::Supabase => SCHEME_NAME_SUPABASE,
        }
    }
}
