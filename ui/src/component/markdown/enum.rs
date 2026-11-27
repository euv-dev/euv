use super::*;

/// Pre-resolved syntax language used by the Eva code tokenizer without per-line string allocations.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub enum CodeLang {
    /// Generic programming language syntax.
    #[default]
    Generic,
    /// Rust source syntax (`"rust"`).
    Rust,
    /// Unified diff patch syntax (`"diff"`).
    Diff,
    /// JSON object/array syntax (`"json"`).
    Json,
    /// TOML key-value configuration syntax (`"toml"`).
    Toml,
    /// YAML key-value configuration syntax (`"yaml"`).
    Yaml,
}

/// Syntax token classification mapped directly to VS Code `Eva Light Bold` / `Eva Dark Bold` TextMate scopes.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub enum CodeTokenKind {
    /// Default editor foreground text.
    #[default]
    Plain,
    /// Storage keyword or modifier (`fn`, `let`, `mut`, `pub`, `struct`; `#7C4DFF` / `#A78CFA` bold).
    Keyword,
    /// Control-flow or module keyword (`if`, `match`, `return`, `use`, `where`; `#C838C6` / `#CF68E1` bold).
    Control,
    /// Symbolic operator (`=`, `->`, `=>`, `&`, `+`, `-`; `#00BEC4` / `#56B7C3`).
    Operator,
    /// User type, struct, enum, or trait identifier (`#00BEC4` / `#56B7C3`).
    Type,
    /// Built-in primitive or numeric type (`u32`, `f32`, `bool`, `str`; `#EF8ED8` / `#FF6AB3`).
    PrimitiveType,
    /// Function or method name (`#437AED` / `#6495EE` bold).
    Function,
    /// Macro invocation, attribute, or lifetime (`#437AED` / `#6495EE` bold).
    Macro,
    /// Quoted string or character literal (`#53A053` / `#98C379`).
    String,
    /// Numeric literal (`#FF6D12` / `#FF9070`).
    Number,
    /// Language constant (`true`, `false`, `None`, `Some`; `#C838C6` / `#CF68E1` bold).
    Constant,
    /// Line or block comment (`#A9A9AA` / `#676E95` italic).
    Comment,
    /// Property key, field, or `Result`/`Option` type (`#CD6069` / `#E06C75`).
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
    /// Bold emphasis (`**text**` or `__text__`).
    Bold(String),
    /// Italic emphasis (`*text*` or `_text_`).
    Italic(String),
    /// Combined bold and italic emphasis (`***text***`).
    BoldItalic(String),
    /// Strikethrough (`~~text~~`).
    Strike(String),
    /// Highlighted text (`==text==` or `<mark>text</mark>`).
    Highlight(String),
    /// Superscript text (`^text^` or `<sup>text</sup>`).
    Superscript(String),
    /// Subscript text (`~text~` or `<sub>text</sub>`).
    Subscript(String),
    /// Inline monospace code chip (`` `code` ``).
    Code(String),
    /// Inline math expression (`$expr$`) formatted with Unicode math symbols.
    Math(String),
    /// Inline footnote reference badge (`[^id]`).
    FootnoteRef(String),
    /// Inline image chip (`![alt](url)`).
    Image {
        /// Accessible alt caption.
        alt: String,
        /// Image source URI.
        url: String,
    },
    /// Clickable hyperlink (`[label](url)`, `[label][ref]`, or `<https://...>`).
    Link {
        /// Visible anchor text.
        label: String,
        /// Target URL or route.
        url: String,
    },
    /// Keyboard shortcut cap (`<kbd>Cmd+K</kbd>`).
    Kbd(String),
    /// Explicit inline hard line break (`  \n` or `\\\n`).
    HardBreak,
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
    /// Native vector Mermaid diagram block (` ```mermaid `).
    Mermaid {
        /// Raw Mermaid diagram source text.
        source: String,
    },
    /// Display math block (`$$...$$`).
    MathBlock(String),
    /// Blockquote or GitHub-flavored Alert callout.
    Quote {
        /// Plain quote or alert kind.
        kind: EuvMarkdownAlertKind,
        /// Nesting depth (`1` = top level).
        depth: usize,
        /// Content lines inside the callout.
        lines: Vec<String>,
    },
    /// Unordered, ordered, nested, or task-checklist list.
    List {
        /// Whether items render with ordinal numbers.
        ordered: bool,
        /// Starting ordinal number for ordered lists (`1` by default).
        start: usize,
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
    /// Extended Markdown definition list (`Term` + `: Definition`).
    DefinitionList(Vec<EuvMarkdownDefItem>),
    /// Bottom footnote definitions section (`[^id]: text`).
    Footnotes(Vec<EuvMarkdownFootnote>),
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
