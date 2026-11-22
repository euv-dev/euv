/// The Rust keywords.
pub(crate) const RUST_KEYWORDS: &[&str] = &[
    "as", "async", "await", "break", "const", "continue", "crate", "dyn", "else", "enum", "extern",
    "false", "fn", "for", "if", "impl", "in", "let", "loop", "match", "mod", "move", "mut", "pub",
    "ref", "return", "self", "Self", "static", "struct", "super", "trait", "true", "type",
    "unsafe", "use", "where", "while",
];

/// The Rust keywords whose following identifier names a type.
pub(crate) const RUST_TYPE_KEYWORDS: &[&str] = &["enum", "struct", "trait", "type", "union"];

/// The Rust keyword whose following identifier names a function.
pub(crate) const RUST_FUNCTION_KEYWORD: &str = "fn";

/// The Bash keywords and builtins.
pub(crate) const BASH_KEYWORDS: &[&str] = &[
    "alias", "break", "case", "continue", "declare", "do", "done", "elif", "else", "esac", "eval",
    "exec", "exit", "export", "fi", "for", "function", "if", "in", "local", "readonly", "return",
    "set", "shift", "source", "then", "trap", "unset", "until", "while",
];

/// The TypeScript and JavaScript keywords.
pub(crate) const TS_KEYWORDS: &[&str] = &[
    "abstract",
    "any",
    "as",
    "async",
    "await",
    "boolean",
    "break",
    "case",
    "catch",
    "class",
    "const",
    "continue",
    "declare",
    "default",
    "delete",
    "do",
    "else",
    "enum",
    "export",
    "extends",
    "false",
    "finally",
    "for",
    "from",
    "function",
    "get",
    "if",
    "implements",
    "import",
    "in",
    "instanceof",
    "interface",
    "keyof",
    "let",
    "new",
    "null",
    "number",
    "of",
    "package",
    "private",
    "protected",
    "public",
    "readonly",
    "return",
    "satisfies",
    "set",
    "static",
    "string",
    "super",
    "switch",
    "symbol",
    "this",
    "throw",
    "true",
    "try",
    "type",
    "typeof",
    "undefined",
    "unknown",
    "var",
    "void",
    "while",
    "yield",
];

/// The JSON literals.
pub(crate) const JSON_KEYWORDS: &[&str] = &["false", "null", "true"];

/// The `class` of a keyword token.
pub(crate) const CLASS_TOKEN_KEYWORD: &str = "c_docs_code_kw";

/// The `class` of a string token.
pub(crate) const CLASS_TOKEN_STRING: &str = "c_docs_code_str";

/// The `class` of a number token.
pub(crate) const CLASS_TOKEN_NUMBER: &str = "c_docs_code_num";

/// The `class` of a comment token.
pub(crate) const CLASS_TOKEN_COMMENT: &str = "c_docs_code_com";

/// The `class` of a function token.
pub(crate) const CLASS_TOKEN_FUNCTION: &str = "c_docs_code_fn";

/// The `class` of a type token.
pub(crate) const CLASS_TOKEN_TYPE: &str = "c_docs_code_ty";

/// The `class` of an attribute token.
pub(crate) const CLASS_TOKEN_ATTRIBUTE: &str = "c_docs_code_attr";

/// The `class` of a tag token.
pub(crate) const CLASS_TOKEN_TAG: &str = "c_docs_code_tag";

/// The `class` of a variable token.
pub(crate) const CLASS_TOKEN_VARIABLE: &str = "c_docs_code_var";

/// The syntax palette and the token classes that read it.
///
/// The palette is the one exception to the "no hardcoded colour" rule:
/// the euv token layer describes the *chrome* of the interface (surfaces,
/// text, borders) and has no vocabulary for the colours of a syntax
/// highlighter, so the highlighter's palette is declared here as CSS
/// custom properties. Everything else in the docs site reads its
/// colours from the euv role tokens.
///
/// The two palettes are the reference editor theme's own light and dark
/// faces: the light one in `:root`, the dark one under `.c_theme_dark`.
/// They are never collapsed into one palette — a colour that reads as a
/// keyword on the light surface is unreadable on the dark one, and the
/// editor's answer is a second palette, not a compromise.
///
/// Both are consumed through `var(--docs-code-…)`, so switching the theme
/// switches the palette without a second stylesheet.
pub(crate) const CODE_CSS: &str = ":root { \
         --docs-code-keyword: #7C4DFF; \
         --docs-code-string: #53A053; \
         --docs-code-number: #FF6D12; \
         --docs-code-comment: #A9A9AA; \
         --docs-code-function: #437AED; \
         --docs-code-type: #00BEC4; \
         --docs-code-attribute: #00BEC4; \
         --docs-code-tag: #CD6069; \
         --docs-code-variable: #5D5D5F; \
         --docs-code-surface: #EBEEF5; \
         --docs-code-foreground: #5D5D5F; \
         --docs-code-font-family: Menlo, Monaco, 'Courier New', monospace; \
     } \
     .c_theme_dark { \
         --docs-code-keyword: #A78CFA; \
         --docs-code-string: #98C379; \
         --docs-code-number: #FF9070; \
         --docs-code-comment: #676E95; \
         --docs-code-function: #6495EE; \
         --docs-code-type: #56B7C3; \
         --docs-code-attribute: #56B7C3; \
         --docs-code-tag: #E06C75; \
         --docs-code-variable: #B0B7C3; \
         --docs-code-surface: #282c34; \
         --docs-code-foreground: #9DA5B3; \
     } \
     .c_docs_code_kw { color: var(--docs-code-keyword); font-weight: bold; } \
     .c_docs_code_str { color: var(--docs-code-string); } \
     .c_docs_code_num { color: var(--docs-code-number); } \
     .c_docs_code_com { color: var(--docs-code-comment); font-style: italic; } \
     .c_docs_code_fn { color: var(--docs-code-function); font-weight: bold; } \
     .c_docs_code_ty { color: var(--docs-code-type); } \
     .c_docs_code_attr { color: var(--docs-code-attribute); } \
     .c_docs_code_tag { color: var(--docs-code-tag); } \
     .c_docs_code_var { color: var(--docs-code-variable); }";
