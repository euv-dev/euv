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

impl EuvAvatarSize {
    /// Returns the square edge length in pixels for this size.
    ///
    /// # Returns
    ///
    /// - `i32` - The edge length in pixels: 24 for `Small`, 32 for `Medium`,
    ///   40 for `Large`.
    pub fn px(&self) -> i32 {
        match self {
            EuvAvatarSize::Small => 24,
            EuvAvatarSize::Medium => 32,
            EuvAvatarSize::Large => 40,
        }
    }
}

/// Props for the `euv_avatar` component.
///
/// Defines the strongly-typed interface for a user or entity avatar that
/// renders either an image or a text fallback.
#[derive(Clone, CustomDebug, Default)]
pub struct EuvAvatarProps {
    /// The image source URL. When empty the `initials` text is rendered instead.
    pub src: &'static str,
    /// The alternative text describing the avatar image.
    pub alt: &'static str,
    /// The initials rendered as the text fallback when `src` is empty.
    pub initials: &'static str,
    /// The square size of the avatar.
    pub size: EuvAvatarSize,
    /// Whether to render the avatar as a sharp square instead of a circle.
    pub square: Signal<bool>,
}
