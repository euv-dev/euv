/// The euv macro name for `html!`.
pub const MACRO_NAME_HTML: &str = "html";

/// The euv macro name for `class!`.
pub const MACRO_NAME_CLASS: &str = "class";

/// The euv macro name for `vars!`.
pub const MACRO_NAME_VARS: &str = "vars";

/// The euv macro name for `watch!`.
pub const MACRO_NAME_WATCH: &str = "watch";

/// The euv macro names that should be formatted.
pub const EUV_MACRO_NAMES: &[&str] = &[
    MACRO_NAME_HTML,
    MACRO_NAME_CLASS,
    MACRO_NAME_VARS,
    MACRO_NAME_WATCH,
];

/// The Rust `if` keyword string.
pub const KEYWORD_IF: &str = "if";

/// The Rust `else` keyword string.
pub const KEYWORD_ELSE: &str = "else";

/// The Rust `match` keyword string.
pub const KEYWORD_MATCH: &str = "match";

/// The Rust `for` keyword string.
pub const KEYWORD_FOR: &str = "for";

/// The Rust `in` keyword string.
pub const KEYWORD_IN: &str = "in";

/// The Rust keywords that can follow a colon in euv macro attribute syntax
/// (e.g., `class: if { ... }`), indicating an attribute value expression
/// rather than a CSS selector.
pub const ATTRIBUTE_VALUE_KEYWORDS: &[&str] = &[KEYWORD_IF, KEYWORD_MATCH, KEYWORD_FOR];

/// The Rust fat arrow operator string.
pub const ARROW_FAT: &str = "=>";

/// The directory name to skip when scanning for Rust source files (Rust build output).
pub const TARGET_DIR_NAME: &str = "target";

/// The directory name to skip when scanning for Rust source files (Node.js dependencies).
pub const NODE_MODULES_DIR_NAME: &str = "node_modules";

/// The Rust source file extension.
pub const RS_EXTENSION: &str = "rs";

/// The exclamation mark character used to identify macro invocations.
pub const CHAR_MACRO_BANG: char = '!';

/// The left brace character used to delimit code blocks and macro bodies.
pub const CHAR_BRACE_LEFT: char = '{';

/// The right brace character used to delimit code blocks and macro bodies.
pub const CHAR_BRACE_RIGHT: char = '}';

/// The left parenthesis character, used in CSS functional pseudo-class detection.
pub const CHAR_LEFT_PAREN: char = '(';

/// The right parenthesis character, used in CSS functional pseudo-class detection.
pub const CHAR_RIGHT_PAREN: char = ')';

/// The double quote character used to delimit string literals.
pub const CHAR_DOUBLE_QUOTE: char = '"';

/// The single quote character used to delimit character and string literals.
pub const CHAR_SINGLE_QUOTE: char = '\'';

/// The forward slash character used in comment and path detection.
pub const CHAR_SLASH_FORWARD: char = '/';

/// The forward slash string used in path normalization.
pub const STR_SLASH_FORWARD: &str = "/";

/// The backslash character used as escape prefix in string literals.
pub const CHAR_SLASH_BACK: char = '\\';

/// The underscore character, part of Rust identifiers.
pub const CHAR_UNDERSCORE: char = '_';

/// The hash character, used in raw identifier prefix detection.
pub const CHAR_HASH: char = '#';

/// The hyphen character, used in CSS selector identifier detection.
pub const CHAR_HYPHEN: char = '-';

/// The newline character.
pub const CHAR_NEWLINE: char = '\n';

/// The carriage return character.
pub const CHAR_CARRIAGE_RETURN: char = '\r';

/// The tab character.
pub const CHAR_TAB: char = '\t';

/// The space character.
pub const CHAR_SPACE: char = ' ';

/// The asterisk character, used in block comment end detection.
pub const CHAR_ASTERISK: char = '*';

/// The colon character, used in attribute separator formatting.
pub const CHAR_COLON: char = ':';

/// The equals sign character, used in fat arrow detection.
pub const CHAR_EQUALS: char = '=';

/// The greater-than character, used in fat arrow detection.
pub const CHAR_GREATER_THAN: char = '>';

/// The block comment start delimiter.
pub const BLOCK_COMMENT_START: &str = "/*";

/// The raw identifier prefix string.
pub const RAW_IDENT_PREFIX: &str = "r#";

/// The letter `a`, used in keyword detection.
pub const CHAR_LETTER_A: char = 'a';

/// The letter `c`, used in keyword detection.
pub const CHAR_LETTER_C: char = 'c';

/// The letter `e`, used in keyword detection.
pub const CHAR_LETTER_E: char = 'e';

/// The letter `f`, used in keyword detection.
pub const CHAR_LETTER_F: char = 'f';

/// The letter `h`, used in keyword detection.
pub const CHAR_LETTER_H: char = 'h';

/// The letter `i`, used in keyword detection.
pub const CHAR_LETTER_I: char = 'i';

/// The letter `l`, used in keyword detection.
pub const CHAR_LETTER_L: char = 'l';

/// The letter `m`, used in keyword detection.
pub const CHAR_LETTER_M: char = 'm';

/// The letter `n`, used in keyword detection.
pub const CHAR_LETTER_N: char = 'n';

/// The letter `o`, used in keyword detection.
pub const CHAR_LETTER_O: char = 'o';

/// The letter `r`, used in keyword detection.
pub const CHAR_LETTER_R: char = 'r';

/// The letter `s`, used in keyword detection.
pub const CHAR_LETTER_S: char = 's';

/// The comma character.
pub const CHAR_COMMA: char = ',';

/// The semicolon character.
pub const CHAR_SEMICOLON: char = ';';

/// The letter `t`, used in keyword detection.
pub const CHAR_LETTER_T: char = 't';

/// The error message for a failed directory read during `.rs` file discovery.
pub const ERROR_READ_DIRECTORY: &str = "Failed to read directory";

/// The error message for a failed directory-entry iteration during `.rs` file discovery.
pub const ERROR_READ_ENTRY_IN_DIRECTORY: &str = "Failed to read entry in directory";

/// The error message for a failed source-file read during formatting.
pub const ERROR_READ_FILE: &str = "Failed to read";

/// The error message for a failed source-file write during formatting.
pub const ERROR_WRITE_FILE: &str = "Failed to write";
