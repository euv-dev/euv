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
