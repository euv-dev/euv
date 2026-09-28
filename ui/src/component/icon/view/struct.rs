use super::*;

/// The size scale of the `euv_icon` component.
///
/// The steps match the shared monochrome type scale: an icon is a glyph, so it
/// never carries colour — only the foreground token, inherited by the class.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum EuvIconSize {
    /// A 12px glyph for inline hints and metadata rows.
    Xs,
    /// A 16px glyph for dense rows and button-adjacent marks.
    Sm,
    /// A 20px glyph, the default size for inline body content.
    #[default]
    Md,
    /// A 24px glyph for section headers and empty-state marks.
    Lg,
    /// A 32px glyph for hero and page-header glyphs.
    Xl,
}

impl EuvIconSize {
    /// Returns the glyph edge length in pixels for this size.
    ///
    /// # Returns
    ///
    /// - `i32` - The edge length in pixels: 12 for `Xs`, 16 for `Sm`, 20 for
    ///   `Md`, 24 for `Lg`, 32 for `Xl`.
    pub fn px(&self) -> i32 {
        match self {
            EuvIconSize::Xs => 12,
            EuvIconSize::Sm => 16,
            EuvIconSize::Md => 20,
            EuvIconSize::Lg => 24,
            EuvIconSize::Xl => 32,
        }
    }
}

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
