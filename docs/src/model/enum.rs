use super::*;

/// Structured content block extracted from a Markdown documentation file.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
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
    /// Blockquote or callout note paragraph text.
    BlockQuote(String),
    /// GitHub-Flavored Markdown table with header cells and body rows.
    Table {
        /// Column header labels.
        headers: Vec<String>,
        /// Table body rows aligned with `headers`.
        rows: Vec<Vec<String>>,
    },
    /// Embedded image with alt text and source URL.
    Image {
        /// Image alternative text description.
        alt: String,
        /// Image source URL or relative asset path.
        src: String,
    },
    /// Thematic horizontal section divider (`---`).
    HorizontalRule,
}
