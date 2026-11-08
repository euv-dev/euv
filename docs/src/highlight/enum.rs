use super::*;

/// The kind of one highlighted code segment.
///
/// Every kind maps to exactly one CSS class, and every one of those
/// classes reads its colour from a CSS custom property. The palette
/// therefore lives in a single stylesheet constant, and the dark theme
/// only has to override those properties — no colour is ever spelled
/// out in the Rust code.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum CodeTokenKind {
    /// Ordinary code: identifiers, operators, punctuation.
    Plain,
    /// A language keyword.
    Keyword,
    /// A string literal.
    String,
    /// A numeric literal.
    Number,
    /// A comment.
    Comment,
    /// A function or command name.
    Function,
    /// A type name.
    Type,
    /// An attribute, a property name or an object key.
    Attribute,
    /// An HTML tag name.
    Tag,
    /// A shell variable.
    Variable,
}

/// The languages the tokenizer knows.
///
/// A fence whose language is not listed falls back to
/// [`CodeLanguage::Plain`], which emits the code as one unstyled
/// segment — the block still gets its header and its copy button.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum CodeLanguage {
    /// No highlighting.
    Plain,
    /// Rust.
    Rust,
    /// Bash / shell.
    Bash,
    /// JSON.
    Json,
    /// TypeScript / JavaScript.
    TypeScript,
    /// HTML / XML.
    Html,
    /// CSS.
    Css,
}
