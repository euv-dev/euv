use super::*;

/// Syntax token classification mapped directly to VS Code `Eva Light Bold` / `Eva Dark Bold` TextMate scopes.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub enum CodeTokenKind {
    /// Default editor foreground text.
    #[default]
    Plain,
    /// Language keyword, storage modifier, or control flow (`#7C4DFF` / `#A78CFA` bold).
    Keyword,
    /// Type, struct, enum, or trait identifier (`#7C4DFF` / `#A78CFA` bold).
    Type,
    /// Function or method name (`#437AED` / `#6495EE` bold).
    Function,
    /// Macro invocation or attribute annotation (`#437AED` / `#6495EE` bold).
    Macro,
    /// Quoted string or character literal (`#437AED` / `#6495EE`).
    String,
    /// Numeric literal (`#FF6D12` / `#FF9070`).
    Number,
    /// Language constant (`true`, `false`, `None`, `Some`) or tag (`#00BEC4` / `#56B7C3`).
    Constant,
    /// Line or block comment (`#A0A1A7` / `#79859D`).
    Comment,
    /// Object/JSON/TOML property key (`#CD6069` / `#E06C75`).
    Property,
    /// Punctuation or delimiter (`#A0A1A7` / `#79859D`).
    Punctuation,
    /// Diff insertion line (`+...`, `#44C145` / `#4EC150`).
    DiffAdd,
    /// Diff deletion line (`-...`, `#F14C4C`).
    DiffDel,
}

/// Semantic variant for `EuvMarkdownQuote` supporting plain blockquotes and GitHub-flavored Alerts.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub enum EuvMarkdownAlertKind {
    /// Standard Markdown blockquote (`> ...`).
    #[default]
    Quote,
    /// Informational note alert (`> [!NOTE]`).
    Note,
    /// Helpful tip alert (`> [!TIP]`).
    Tip,
    /// Essential requirement alert (`> [!IMPORTANT]`).
    Important,
    /// Warning alert (`> [!WARNING]`).
    Warning,
    /// Critical caution alert (`> [!CAUTION]`).
    Caution,
}

/// Column alignment parsed from GFM Markdown tables.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub enum EuvMarkdownAlign {
    /// Left-aligned column (`:---`).
    #[default]
    Left,
    /// Center-aligned column (`:---:`).
    Center,
    /// Right-aligned column (`---:`).
    Right,
}

/// Parsed inline Markdown segment rendered inside `EuvMarkdownParagraph` and list items.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum EuvMarkdownInline {
    /// Plain body text span.
    Text(String),
    /// Bold emphasis (`**text**`).
    Bold(String),
    /// Italic emphasis (`*text*`).
    Italic(String),
    /// Strikethrough (`~~text~~`).
    Strike(String),
    /// Inline monospace code chip (`` `code` ``).
    Code(String),
    /// Clickable hyperlink (`[label](url)`).
    Link {
        /// Visible anchor text.
        label: String,
        /// Target URL or route.
        url: String,
    },
    /// Keyboard shortcut cap (`<kbd>Cmd+K</kbd>`).
    Kbd(String),
}

/// Parsed block-level Markdown AST node rendered natively by `EuvMarkdown`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum EuvMarkdownBlock {
    /// ATX heading (`#` through `######`).
    Heading {
        /// Heading level (`1..=6`).
        level: u8,
        /// Heading text.
        text: String,
    },
    /// Rich inline paragraph.
    Paragraph(Vec<EuvMarkdownInline>),
    /// Fenced syntax-highlighted code block.
    CodeBlock {
        /// Language identifier (e.g. `"rust"`, `"ts"`, `"json"`).
        language: String,
        /// Raw code body.
        code: String,
    },
    /// Blockquote or GitHub-flavored Alert callout.
    Quote {
        /// Plain quote or alert kind.
        kind: EuvMarkdownAlertKind,
        /// Content lines inside the callout.
        lines: Vec<String>,
    },
    /// Unordered, ordered, nested, or task-checklist list.
    List {
        /// Whether items render with ordinal numbers.
        ordered: bool,
        /// Ordered list items.
        items: Vec<EuvMarkdownListItem>,
    },
    /// GFM aligned data table.
    Table {
        /// Column header labels.
        headers: Vec<String>,
        /// Column alignments.
        alignments: Vec<EuvMarkdownAlign>,
        /// Body rows.
        rows: Vec<Vec<String>>,
    },
    /// Thematic horizontal divider (`---`).
    Divider,
    /// Image figure (`![alt](url)`).
    Image {
        /// Accessible caption / alt text.
        alt: String,
        /// Image source URI or asset path.
        url: String,
    },
    /// Collapsible `<details><summary>` disclosure block.
    Details {
        /// Toggle summary header.
        summary: String,
        /// Content lines revealed when expanded.
        body: Vec<String>,
    },
}
