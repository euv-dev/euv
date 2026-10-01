/// The fewest placeholder lines a skeleton block will render.
pub const SKELETON_LINES_MIN: usize = 1;

/// The most placeholder lines a skeleton block will render.
///
/// A loading placeholder is a stand-in for a short block of content; past
/// a handful of lines a skeleton stops reading as a placeholder and the
/// height is better owned by an explicit `euv_loading` indicator.
pub const SKELETON_LINES_MAX: usize = 8;

/// The width applied to skeleton lines when the `width` prop is unusable.
pub const SKELETON_WIDTH_DEFAULT: &str = "100%";

/// The border radius applied to skeleton lines when `rounded` is true.
///
/// The design system is square by default; rounding is opt-in per block so
/// a caller can match a circular avatar or a pill-shaped chip without the
/// rest of the component library losing its `border-radius: 0px` rule.
pub const SKELETON_RADIUS: &str = "2px";

/// The inline-style property name carrying a skeleton line's width.
pub const SKELETON_STYLE_WIDTH: &str = "width: ";

/// The inline-style property name carrying a skeleton line's height.
pub const SKELETON_STYLE_HEIGHT: &str = " height: ";

/// The inline-style property name carrying a skeleton line's corner radius.
pub const SKELETON_STYLE_RADIUS: &str = " border-radius: ";
