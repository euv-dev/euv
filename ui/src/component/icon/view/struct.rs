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
    /// The markup of a glyph drawn as a vector.
    ///
    /// The markup is the body of an `svg` element. When it is set the icon
    /// is drawn as a vector and the `name` is not rendered: a drawing and a
    /// character cannot share one glyph.
    pub svg: Option<EuvSvgMarkup>,
}
