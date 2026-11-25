/// Font weight classification consistent across native and web renderers.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum FontWeight {
    /// Light weight (`300`).
    Light,
    /// Regular weight (`400`).
    #[default]
    Normal,
    /// Medium weight (`500`).
    Medium,
    /// Semi-bold weight (`600`).
    SemiBold,
    /// Bold weight (`700`).
    Bold,
}

/// Horizontal text alignment within its layout box.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum TextAlign {
    /// Left-aligned text.
    #[default]
    Left,
    /// Centered text.
    Center,
    /// Right-aligned text.
    Right,
}

/// Text overflow handling when a line exceeds available width.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum TextOverflow {
    /// Wrap text across multiple lines.
    #[default]
    Wrap,
    /// Clip overflowing glyphs cleanly.
    Clip,
    /// Truncate single line with trailing ellipsis.
    Ellipsis,
}

/// Font family semantic category.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum FontFamily {
    /// Clean geometric sans-serif UI font.
    #[default]
    Sans,
    /// Fixed-width monospace font for code and metrics.
    Monospace,
    /// Serif editorial font.
    Serif,
}
