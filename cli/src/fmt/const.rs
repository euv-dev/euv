/// Supported euv procedural macro names formatted inside `{ ... }` blocks by `euv fmt`.
pub(crate) const EUV_FMT_MACRO_NAMES: &[&str] = &[
    "view",
    "style",
    "tokens",
    "watch",
    "computed",
    "on_event",
    "emit",
    "global_state",
    "local_state",
];
/// Directory names skipped during recursive workspace `.rs` macro formatting.
pub(crate) const EUV_FMT_SKIP_DIRS: &[&str] = &[
    "target",
    ".git",
    "node_modules",
    "dist",
    "build",
    ".cargo",
    "vendor",
];
/// File extension matched by the workspace macro formatter.
pub(crate) const EUV_FMT_RS_EXT: &str = "rs";
/// Indentation width in spaces per nesting level inside formatted euv macro blocks.
pub(crate) const EUV_FMT_INDENT_SPACES: usize = 4;
/// Minimum identifier length among supported `euv` procedural macro names.
pub(crate) const EUV_FMT_MACRO_MIN_LEN: usize = 4;
/// Maximum identifier length among supported `euv` procedural macro names.
pub(crate) const EUV_FMT_MACRO_MAX_LEN: usize = 12;
/// Precomputed 128-space indentation buffer for zero-loop bulk indentation slicing.
pub(crate) const INDENT_SPACES_128: &str = "                                                                                                                                ";
/// Error message returned when `euv fmt --check` detects unformatted macro blocks.
pub(crate) const ERR_FMT_CHECK_FAILED: &str = "unformatted euv macro blocks found in";
/// ASCII byte for `!`.
pub(crate) const BYTE_BANG: u8 = b'!';
/// ASCII byte for ` `.
pub(crate) const BYTE_SPACE: u8 = b' ';
/// ASCII byte for `\t`.
pub(crate) const BYTE_TAB: u8 = b'\t';
/// ASCII byte for `\n`.
pub(crate) const BYTE_NEWLINE: u8 = b'\n';
/// ASCII byte for `{`.
pub(crate) const BYTE_LBRACE: u8 = b'{';
/// ASCII byte for `}`.
pub(crate) const BYTE_RBRACE: u8 = b'}';
/// ASCII byte for `(`.
pub(crate) const BYTE_LPAREN: u8 = b'(';
/// ASCII byte for `)`.
pub(crate) const BYTE_RPAREN: u8 = b')';
/// ASCII byte for `[`.
pub(crate) const BYTE_LBRACKET: u8 = b'[';
/// ASCII byte for `]`.
pub(crate) const BYTE_RBRACKET: u8 = b']';
/// ASCII byte for `*`.
pub(crate) const BYTE_STAR: u8 = b'*';
/// ASCII byte for `/`.
pub(crate) const BYTE_SLASH: u8 = b'/';
/// ASCII byte for `r`.
pub(crate) const BYTE_LOWER_R: u8 = b'r';
/// ASCII byte for `"`.
pub(crate) const BYTE_DQUOTE: u8 = b'"';
/// ASCII byte for `'`.
pub(crate) const BYTE_SQUOTE: u8 = b'\'';
/// ASCII byte for `\`.
pub(crate) const BYTE_BACKSLASH: u8 = b'\\';
/// ASCII byte for `#`.
pub(crate) const BYTE_HASH: u8 = b'#';
/// ASCII byte for `_`.
pub(crate) const BYTE_UNDERSCORE: u8 = b'_';
