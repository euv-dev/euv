/// Typographic scale variants for `EuvTypography`.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub enum EuvTypographyVariant {
    /// Large display title.
    H1,
    /// Section heading.
    H2,
    /// Subsection heading.
    H3,
    /// Standard body copy.
    #[default]
    Body,
    /// Secondary muted caption.
    Caption,
}

/// Built-in crisp native vector glyph identifiers rendered directly via `DisplayList` primitives by `EuvIcon` and `draw_euv_icon`.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub enum EuvIconName {
    /// Clipboard copy glyph (overlapping sheets).
    #[default]
    Copy,
    /// Confirmation checkmark glyph.
    Check,
    /// Four-corner expand / enter fullscreen glyph.
    Maximize,
    /// Four-corner collapse / exit fullscreen glyph.
    Minimize,
    /// Diagonal cross dismiss / close glyph.
    Close,
    /// Magnifying glass search / command palette glyph.
    Search,
    /// Light theme sun glyph.
    Sun,
    /// Dark theme crescent moon glyph.
    Moon,
    /// Design scheme color palette glyph.
    Palette,
    /// Three-bar navigation drawer menu glyph.
    Menu,
    /// Media playback start triangle glyph.
    Play,
    /// Media playback pause bars glyph.
    Pause,
    /// Reload / refresh circular arrow glyph.
    Refresh,
    /// Delete / clear trash bin glyph.
    Trash,
    /// Downward disclosure chevron glyph.
    ChevronDown,
    /// Leftward navigation arrow glyph.
    ArrowLeft,
    /// Rightward navigation arrow glyph.
    ArrowRight,
}
