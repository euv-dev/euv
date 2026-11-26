use super::*;

/// Resolved VS Code `Eva Light Bold` / `Eva Dark Bold` color palette for `EuvCodeBlock` and Markdown syntax chrome.
#[derive(Clone, Copy, Data, Debug, Eq, PartialEq)]
pub struct CodeThemePalette {
    /// Theme name (`"Eva Light Bold"` or `"Eva Dark Bold"`).
    pub(crate) theme_name: &'static str,
    /// Editor surface background.
    #[get(type(copy))]
    pub(crate) background: Color,
    /// Header / tab bar background.
    #[get(type(copy))]
    pub(crate) header_bg: Color,
    /// Crisp container border color.
    #[get(type(copy))]
    pub(crate) border: Color,
    /// Default code foreground.
    #[get(type(copy))]
    pub(crate) foreground: Color,
    /// Line number gutter foreground.
    #[get(type(copy))]
    pub(crate) line_number: Color,
    /// Highlighted line background.
    #[get(type(copy))]
    pub(crate) line_highlight: Color,
    /// Comment token color.
    #[get(type(copy))]
    pub(crate) comment: Color,
    /// Keyword / storage / operator token color.
    #[get(type(copy))]
    pub(crate) keyword: Color,
    /// String literal token color.
    #[get(type(copy))]
    pub(crate) string: Color,
    /// Numeric literal token color.
    #[get(type(copy))]
    pub(crate) number: Color,
    /// Constant / variable / tag token color.
    #[get(type(copy))]
    pub(crate) constant: Color,
    /// Function / macro / attribute token color.
    #[get(type(copy))]
    pub(crate) function: Color,
    /// Type / trait / struct token color.
    #[get(type(copy))]
    pub(crate) ty: Color,
    /// Property / heading token color.
    #[get(type(copy))]
    pub(crate) property: Color,
    /// Punctuation token color.
    #[get(type(copy))]
    pub(crate) punctuation: Color,
    /// Inline code chip background.
    #[get(type(copy))]
    pub(crate) inline_code_bg: Color,
    /// Inline code chip foreground.
    #[get(type(copy))]
    pub(crate) inline_code_fg: Color,
    /// Blockquote surface background.
    #[get(type(copy))]
    pub(crate) quote_bg: Color,
    /// Blockquote left accent border.
    #[get(type(copy))]
    pub(crate) quote_border: Color,
    /// Hyperlink foreground color.
    #[get(type(copy))]
    pub(crate) link: Color,
    /// Diff insertion foreground color.
    #[get(type(copy))]
    pub(crate) diff_add: Color,
    /// Diff deletion foreground color.
    #[get(type(copy))]
    pub(crate) diff_del: Color,
}

/// Single syntax-highlighted span within a source code line.
#[derive(Clone, Data, Debug, Eq, New, PartialEq)]
pub struct CodeSpan {
    /// Raw text slice of the token.
    pub(crate) text: String,
    /// Semantic token classification.
    #[get(type(copy))]
    pub(crate) kind: CodeTokenKind,
}

/// Single item inside an `EuvMarkdownList` supporting nesting and interactive GFM checkboxes.
#[derive(Clone, Data, Debug, Eq, New, PartialEq)]
pub struct EuvMarkdownListItem {
    /// Item text (may include inline Markdown syntax).
    pub(crate) text: String,
    /// Nesting indentation level (`0` = top level).
    #[get(type(copy))]
    pub(crate) depth: usize,
    /// Optional GFM task checklist state (`Some(true)` checked, `Some(false)` unchecked, `None` normal item).
    #[get(skip)]
    pub(crate) checked: Option<bool>,
}

/// VS Code `Eva Light Bold` / `Eva Dark Bold` syntax-highlighted native code block with header bar, line numbers, line highlighting, and animated copy action.
#[derive(Clone, CustomDebug, Data, New)]
pub struct EuvCodeBlock {
    /// Source code text to display and highlight.
    pub(crate) code: String,
    /// Active theme tokens controlling Light (`Eva Light Bold`) vs Dark (`Eva Dark Bold`) palette selection.
    #[get(type(copy))]
    pub(crate) tokens: ThemeTokens,
    /// Optional language tag (e.g. `"rust"`, `"typescript"`, `"json"`, `"bash"`). Auto-detected when empty.
    #[new(skip)]
    pub(crate) language: String,
    /// Optional filename displayed in the header bar (e.g. `"src/main.rs"`).
    #[new(skip)]
    #[get(skip)]
    pub(crate) filename: Option<String>,
    /// Whether to display the left line-number gutter (`true` by default for multi-line blocks or via builder).
    #[new(skip)]
    #[get(type(copy))]
    pub(crate) show_line_numbers: bool,
    /// Optional 1-based line numbers rendered with Eva line-highlight tint.
    #[new(skip)]
    pub(crate) highlight_lines: Vec<usize>,
    /// Optional callback invoked with the raw code when the user clicks Copy.
    #[new(skip)]
    #[debug(skip)]
    #[get(skip)]
    pub(crate) on_copy: Option<MarkdownStrCallback>,
}

/// Native Markdown heading component (`H1`–`H6`) with typographic scale and optional accent rule.
#[derive(Clone, Data, Debug, New)]
pub struct EuvMarkdownHeading {
    /// Heading level (`1..=6`).
    #[get(type(copy))]
    pub(crate) level: u8,
    /// Heading text content.
    pub(crate) text: String,
    /// Active theme tokens.
    #[get(type(copy))]
    pub(crate) tokens: ThemeTokens,
}

/// Native Markdown inline code pill styled with exact VS Code Eva inline-code colors.
#[derive(Clone, Data, Debug, New)]
pub struct EuvMarkdownInlineCode {
    /// Inline code snippet.
    pub(crate) code: String,
    /// Active theme tokens.
    #[get(type(copy))]
    pub(crate) tokens: ThemeTokens,
}

/// Native Markdown rich inline paragraph supporting bold, italic, strikethrough, inline code, links, and `<kbd>` caps.
#[derive(Clone, CustomDebug, Data, New)]
pub struct EuvMarkdownParagraph {
    /// Parsed inline spans to flow cleanly inside the paragraph.
    pub(crate) inlines: Vec<EuvMarkdownInline>,
    /// Active theme tokens.
    #[get(type(copy))]
    pub(crate) tokens: ThemeTokens,
    /// Optional callback invoked when a user taps a Markdown hyperlink.
    #[new(skip)]
    #[debug(skip)]
    #[get(skip)]
    pub(crate) on_link: Option<MarkdownStrCallback>,
}

/// Native Markdown blockquote and GitHub-flavored Alert callout (`NOTE`, `TIP`, `IMPORTANT`, `WARNING`, `CAUTION`).
#[derive(Clone, Data, Debug, New)]
pub struct EuvMarkdownQuote {
    /// Semantic quote or alert variant.
    #[get(type(copy))]
    pub(crate) kind: EuvMarkdownAlertKind,
    /// Text lines rendered inside the quote body.
    pub(crate) lines: Vec<String>,
    /// Active theme tokens.
    #[get(type(copy))]
    pub(crate) tokens: ThemeTokens,
}

/// Native Markdown unordered, ordered, nested, and interactive GFM task-checklist component.
#[derive(Clone, CustomDebug, Data, New)]
pub struct EuvMarkdownList {
    /// Whether list items render with ordinal numbering (`1.`, `2.`, ...).
    #[get(type(copy))]
    pub(crate) ordered: bool,
    /// List items to render.
    pub(crate) items: Vec<EuvMarkdownListItem>,
    /// Active theme tokens.
    #[get(type(copy))]
    pub(crate) tokens: ThemeTokens,
    /// Optional callback invoked with `(item_index, next_checked)` when toggling a task item.
    #[new(skip)]
    #[debug(skip)]
    #[get(skip)]
    pub(crate) on_toggle_task: Option<MarkdownTaskCallback>,
}

/// Native GFM Markdown table with column alignment (`Left`, `Center`, `Right`), tinted header, and zebra striping.
#[derive(Clone, Data, Debug, New)]
pub struct EuvMarkdownTable {
    /// Column header titles.
    pub(crate) headers: Vec<String>,
    /// Column horizontal alignments.
    pub(crate) alignments: Vec<EuvMarkdownAlign>,
    /// Data rows.
    pub(crate) rows: Vec<Vec<String>>,
    /// Active theme tokens.
    #[get(type(copy))]
    pub(crate) tokens: ThemeTokens,
}

/// Native Markdown thematic horizontal divider (`---`).
#[derive(Clone, Copy, Data, Debug, New)]
pub struct EuvMarkdownDivider {
    /// Active theme tokens.
    #[get(type(copy))]
    pub(crate) tokens: ThemeTokens,
}

/// Native Markdown framed image/figure card with alt caption and source pill.
#[derive(Clone, Data, Debug, New)]
pub struct EuvMarkdownImage {
    /// Accessible alt / caption text.
    pub(crate) alt: String,
    /// Image URL or local asset path.
    pub(crate) url: String,
    /// Active theme tokens.
    #[get(type(copy))]
    pub(crate) tokens: ThemeTokens,
}

/// Native Markdown collapsible `<details><summary>` disclosure block with smooth expansion.
#[derive(Clone, Data, Debug, New)]
pub struct EuvMarkdownDetails {
    /// Always-visible disclosure header.
    pub(crate) summary: String,
    /// Collapsible content lines.
    pub(crate) body: Vec<String>,
    /// Reactive expanded state signal.
    #[get(type(copy))]
    pub(crate) open: Signal<bool>,
    /// Active theme tokens.
    #[get(type(copy))]
    pub(crate) tokens: ThemeTokens,
}

/// Complete GitHub-Flavored Markdown native document renderer composing every Markdown component and Eva-themed `EuvCodeBlock`.
#[derive(Clone, CustomDebug, Data, New)]
pub struct EuvMarkdown {
    /// Raw Markdown source text.
    pub(crate) source: String,
    /// Active theme tokens.
    #[get(type(copy))]
    pub(crate) tokens: ThemeTokens,
    /// Optional link-click handler receiving the destination URL.
    #[new(skip)]
    #[debug(skip)]
    #[get(skip)]
    pub(crate) on_link: Option<MarkdownStrCallback>,
    /// Optional code-block copy handler receiving the copied code.
    #[new(skip)]
    #[debug(skip)]
    #[get(skip)]
    pub(crate) on_copy: Option<MarkdownStrCallback>,
    /// Optional interactive task-checkbox toggle handler receiving `(index, checked)`.
    #[new(skip)]
    #[debug(skip)]
    #[get(skip)]
    pub(crate) on_toggle_task: Option<MarkdownTaskCallback>,
}
