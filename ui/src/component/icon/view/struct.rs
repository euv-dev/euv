use super::*;

/// Props for the `euv_icon` component.
///
/// Defines the strongly-typed interface for a monochrome glyph wrapper. The
/// `name` prop carries the glyph itself (an emoji or a text symbol) — the
/// design system has no icon font and no coloured icon set.
#[derive(Clone, CustomDebug, Default)]
pub struct EuvIconProps {
    /// The glyph text rendered as the icon content.
    pub name: &'static str,
    /// The accessible label. When empty the icon is treated as decorative.
    pub label: &'static str,
    /// The size of the glyph.
    pub size: EuvIconSize,
}
