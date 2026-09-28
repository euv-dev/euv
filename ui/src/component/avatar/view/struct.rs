use super::*;

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
