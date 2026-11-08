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
/// The light values are the reference site's own highlighter colours;
/// the dark ones are its dark counterpart. Both are consumed through
/// `var(--docs-code-…)`, so switching the theme switches the palette
/// without a second stylesheet.
pub(crate) const CODE_CSS: &str = ":root { \
         --docs-code-keyword: #d73a49; \
         --docs-code-string: #032f62; \
         --docs-code-number: #005cc5; \
         --docs-code-comment: #6a737d; \
         --docs-code-function: #6f42c1; \
         --docs-code-type: #22863a; \
         --docs-code-attribute: #6f42c1; \
         --docs-code-tag: #22863a; \
         --docs-code-variable: #e36209; \
     } \
     .c_theme_dark { \
         --docs-code-keyword: #f97583; \
         --docs-code-string: #9ecbff; \
         --docs-code-number: #79c0ff; \
         --docs-code-comment: #8b949e; \
         --docs-code-function: #b392f0; \
         --docs-code-type: #7ee787; \
         --docs-code-attribute: #b392f0; \
         --docs-code-tag: #7ee787; \
         --docs-code-variable: #ffa657; \
     } \
     .c_docs_code_kw { color: var(--docs-code-keyword); } \
     .c_docs_code_str { color: var(--docs-code-string); } \
     .c_docs_code_num { color: var(--docs-code-number); } \
     .c_docs_code_com { color: var(--docs-code-comment); font-style: italic; } \
     .c_docs_code_fn { color: var(--docs-code-function); } \
     .c_docs_code_ty { color: var(--docs-code-type); } \
     .c_docs_code_attr { color: var(--docs-code-attribute); } \
     .c_docs_code_tag { color: var(--docs-code-tag); } \
     .c_docs_code_var { color: var(--docs-code-variable); }";
