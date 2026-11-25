use super::*;

/// Structured content block extracted from a Markdown documentation file.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum DocBlock {
    /// Section heading with level (1..=6), anchor slug, and text.
    Heading {
        /// Heading depth from 1 to 6.
        level: u8,
        /// URL anchor slug.
        slug: String,
        /// Visible heading text.
        text: String,
    },
    /// Plain paragraph text.
    Paragraph(String),
    /// Fenced or indented code block with optional language tag.
    Code {
        /// Language identifier (for example `rust` or `shell`).
        language: String,
        /// Source code text.
        code: String,
    },
    /// Unordered or ordered list item text.
    ListItem(String),
}
