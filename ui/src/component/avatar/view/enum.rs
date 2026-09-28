use super::*;

/// The square edge length of the `euv_avatar` component.
///
/// The sizes mirror the `c_euv_logo_nav` (32px) square convention used by the
/// brand logo: `Small` for dense list rows, `Medium` as the default inline
/// size, and `Large` for profile headers.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum EuvAvatarSize {
    /// A 24×24 avatar for dense rows such as comment lists.
    Small,
    /// A 32×32 avatar, the default size matching the navigation logo square.
    #[default]
    Medium,
    /// A 40×40 avatar for profile headers and user cards.
    Large,
}
