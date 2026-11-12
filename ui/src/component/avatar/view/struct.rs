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
    /// Whether to render the avatar as a rounded square instead of a circle.
    pub square: Signal<bool>,
}

/// Props for the `euv_avatar_fallback` component.
///
/// The fallback stands in for the picture when there is none, so it
/// carries the initials it shows.
#[derive(Clone, CustomDebug, Data, Default, New)]
pub struct EuvAvatarFallbackProps {
    /// The initials shown inside the fallback.
    pub text: &'static str,
}

/// Props for the `euv_avatar_badge` component.
///
/// The badge marks the avatar with a short label.
#[derive(Clone, CustomDebug, Data, Default, New)]
pub struct EuvAvatarBadgeProps {
    /// The badge text.
    pub text: &'static str,
}

/// Props for the `euv_avatar_group` component.
///
/// The group holds the avatars that belong together, so it takes no
/// settings of its own.
#[derive(Clone, CustomDebug, Default)]
pub struct EuvAvatarGroupProps {}

/// Props for the `euv_avatar_group_count` component.
///
/// The count names how many avatars the group holds beyond the ones it
/// shows.
#[derive(Clone, CustomDebug, Data, Default, New)]
pub struct EuvAvatarGroupCountProps {
    /// The count text.
    pub text: &'static str,
}
