use super::*;

/// The typographic role a piece of text plays.
///
/// The roles map one-to-one onto the registry's text classes, so a
/// caller can pick the role without knowing the class names.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum EuvTextVariant {
    /// Running text at the body size.
    #[default]
    Body,
    /// A section headline.
    Title,
    /// A page headline, the largest of the three.
    TitleLg,
    /// A sub-headline, the smallest of the three.
    TitleSm,
    /// A paragraph of running text.
    Paragraph,
    /// Supporting text in the muted style.
    Secondary,
    /// Text in the monospace style, for code and identifiers.
    Mono,
    /// Text marked as removed.
    Deletion,
    /// Text marked as inserted.
    Underline,
}
