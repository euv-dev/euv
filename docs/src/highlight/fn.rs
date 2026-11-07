use super::*;

/// Tokenizes a code block into styled segments.
///
/// The tokenizer is deliberately simple and deterministic: it recognises
/// comments, strings, numbers and identifiers, and classifies the
/// identifiers by the language's own keyword table. It never fails —
/// an unrecognised language yields the code as one plain segment.
///
/// # Arguments
///
/// - `&str` - The fence language.
/// - `&str` - The raw code.
///
/// # Returns
///
/// - `Vec<CodeToken>` - The segments, in source order.
pub(crate) fn tokenize(lang: &str, code: &str) -> Vec<CodeToken> {
    match normalize_language(lang) {
        CodeLanguage::Plain => plain_tokens(code),
        CodeLanguage::Rust => tokenize_rust(code),
        CodeLanguage::Bash => tokenize_bash(code),
        CodeLanguage::Json => tokenize_json(code),
        CodeLanguage::TypeScript => tokenize_typescript(code),
        CodeLanguage::Html => tokenize_html(code),
        CodeLanguage::Css => tokenize_css(code),
    }
}

/// Renders a code block as HTML.
///
/// The segments become spans that carry one class each; a plain segment
/// stays bare text. The code itself is escaped, so a block that holds
/// markup shows the markup instead of running it.
///
/// # Arguments
///
/// - `&str` - The fence language.
/// - `&str` - The raw code.
///
/// # Returns
///
/// - `String` - The highlighted markup.
pub(crate) fn highlighted_html(lang: &str, code: &str) -> String {
    let mut html: String = String::with_capacity(code.len());
    for token in tokenize(lang, code) {
        match token_class(token.kind) {
            Some(class) => {
                html.push_str("<span class=\"");
                html.push_str(class);
                html.push_str("\">");
                html.push_str(&escape_html(&token.text));
                html.push_str("</span>");
            }
            None => html.push_str(&escape_html(&token.text)),
        }
    }
    html
}

/// Escapes the characters that would otherwise be read as markup.
///
/// # Arguments
///
/// - `&str` - The text to escape.
///
/// # Returns
///
/// - `String` - The escaped text.
fn escape_html(text: &str) -> String {
    let mut escaped: String = String::with_capacity(text.len());
    for character in text.chars() {
        match character {
            '&' => escaped.push_str("&amp;"),
            '<' => escaped.push_str("&lt;"),
            '>' => escaped.push_str("&gt;"),
            _ => escaped.push(character),
        }
    }
    escaped
}

/// Returns the CSS class of a token kind, or `None` for plain text.
///
/// Plain text carries no class: it inherits the code block's own
/// foreground colour, so an unstyled segment is the same colour as the
/// rest of the block and the markup stays free of empty spans.
///
/// # Arguments
///
/// - `CodeTokenKind` - The kind of the token.
///
/// # Returns
///
/// - `Option<&'static str>` - The class, or `None` for plain text.
pub(crate) fn token_class(kind: CodeTokenKind) -> Option<&'static str> {
    match kind {
        CodeTokenKind::Plain => None,
        CodeTokenKind::Keyword => Some(CLASS_TOKEN_KEYWORD),
        CodeTokenKind::String => Some(CLASS_TOKEN_STRING),
        CodeTokenKind::Number => Some(CLASS_TOKEN_NUMBER),
        CodeTokenKind::Comment => Some(CLASS_TOKEN_COMMENT),
        CodeTokenKind::Function => Some(CLASS_TOKEN_FUNCTION),
        CodeTokenKind::Type => Some(CLASS_TOKEN_TYPE),
        CodeTokenKind::Attribute => Some(CLASS_TOKEN_ATTRIBUTE),
        CodeTokenKind::Tag => Some(CLASS_TOKEN_TAG),
        CodeTokenKind::Variable => Some(CLASS_TOKEN_VARIABLE),
    }
}

/// Maps a fence language onto the language the tokenizer knows.
///
/// # Arguments
///
/// - `&str` - The fence language.
///
/// # Returns
///
/// - `CodeLanguage` - The matching language.
fn normalize_language(lang: &str) -> CodeLanguage {
    match lang.trim().to_ascii_lowercase().as_str() {
        "rust" | "rs" => CodeLanguage::Rust,
        "bash" | "sh" | "shell" | "zsh" | "console" | "terminal" | "shell-session" => {
            CodeLanguage::Bash
        }
        "json" | "jsonc" => CodeLanguage::Json,
        "ts" | "tsx" | "typescript" | "js" | "jsx" | "javascript" => CodeLanguage::TypeScript,
        "html" | "xml" | "vue" | "svg" => CodeLanguage::Html,
        "css" | "scss" | "less" => CodeLanguage::Css,
        _ => CodeLanguage::Plain,
    }
}

/// Returns the code as a single plain segment.
///
/// # Arguments
///
/// - `&str` - The raw code.
///
/// # Returns
///
/// - `Vec<CodeToken>` - One segment, empty when the code is empty.
fn plain_tokens(code: &str) -> Vec<CodeToken> {
    if code.is_empty() {
        return Vec::new();
    }
    vec![CodeToken {
        kind: CodeTokenKind::Plain,
        text: code.to_string(),
    }]
}

/// Appends a segment, merging it into the previous one when both have
/// the same kind.
///
/// Merging keeps the rendered markup small: a line of plain code is
/// one span, not one span per word.
///
/// # Arguments
///
/// - `&mut Vec<CodeToken>` - The segment list.
/// - `CodeTokenKind` - The kind of the segment.
/// - `&str` - The segment text.
fn push_token(tokens: &mut Vec<CodeToken>, kind: CodeTokenKind, text: &str) {
    if text.is_empty() {
        return;
    }
    if let Some(last) = tokens.last_mut()
        && last.kind == kind
    {
        last.text.push_str(text);
        return;
    }
    tokens.push(CodeToken {
        kind,
        text: text.to_string(),
    });
}

/// Moves the pending plain text into the segment list.
///
/// # Arguments
///
/// - `&mut Vec<CodeToken>` - The segment list.
/// - `&mut String` - The pending plain text, emptied by the call.
fn flush_plain(tokens: &mut Vec<CodeToken>, plain: &mut String) {
    if plain.is_empty() {
        return;
    }
    push_token(tokens, CodeTokenKind::Plain, plain);
    plain.clear();
}

/// Collects a range of characters into a string.
///
/// # Arguments
///
/// - `&[char]` - The source characters.
/// - `usize` - The first index.
/// - `usize` - The index past the last one.
///
/// # Returns
///
/// - `String` - The collected text.
fn collect(chars: &[char], start: usize, end: usize) -> String {
    chars[start.min(chars.len())..end.min(chars.len())]
        .iter()
        .collect()
}

/// Whether a character can start an identifier.
///
/// # Arguments
///
/// - `char` - The character.
///
/// # Returns
///
/// - `bool` - `true` when the character starts an identifier.
fn is_ident_start(candidate: char) -> bool {
    candidate.is_alphabetic() || candidate == '_'
}

/// Whether a character can continue an identifier.
///
/// # Arguments
///
/// - `char` - The character.
///
/// # Returns
///
/// - `bool` - `true` when the character continues an identifier.
fn is_ident_part(candidate: char) -> bool {
    candidate.is_alphanumeric() || candidate == '_'
}

/// Whether a word starts with an uppercase letter, which is the usual
/// spelling of a type.
///
/// # Arguments
///
/// - `&str` - The word.
///
/// # Returns
///
/// - `bool` - `true` when the word looks like a type name.
fn starts_uppercase(word: &str) -> bool {
    word.chars().next().is_some_and(|c: char| c.is_uppercase())
}

/// Whether the next meaningful character opens a call parenthesis.
///
/// # Arguments
///
/// - `&[char]` - The source characters.
/// - `usize` - The index to look from.
///
/// # Returns
///
/// - `bool` - `true` when the word is followed by `(`.
fn next_is_call(chars: &[char], from: usize) -> bool {
    let mut index: usize = from;
    while index < chars.len() && chars[index].is_whitespace() {
        index += 1;
    }
    chars.get(index) == Some(&'(')
}

/// Tokenizes Rust source.
///
/// # Arguments
///
/// - `&str` - The raw code.
///
/// # Returns
///
/// - `Vec<CodeToken>` - The segments, in source order.
fn tokenize_rust(code: &str) -> Vec<CodeToken> {
    let chars: Vec<char> = code.chars().collect();
    let mut tokens: Vec<CodeToken> = Vec::new();
    let mut plain: String = String::new();
    let mut index: usize = 0;
    let mut pending: Option<CodeTokenKind> = None;
    while index < chars.len() {
        let current: char = chars[index];
        let next: char = chars.get(index + 1).copied().unwrap_or('\0');
        if current == '/' && next == '/' {
            flush_plain(&mut tokens, &mut plain);
            let start: usize = index;
            while index < chars.len() && chars[index] != '\n' {
                index += 1;
            }
            push_token(
                &mut tokens,
                CodeTokenKind::Comment,
                &collect(&chars, start, index),
            );
            continue;
        }
        if current == '/' && next == '*' {
            flush_plain(&mut tokens, &mut plain);
            let start: usize = index;
            index += 2;
            while index < chars.len()
                && !(chars[index] == '*' && chars.get(index + 1) == Some(&'/'))
            {
                index += 1;
            }
            index = (index + 2).min(chars.len());
            push_token(
                &mut tokens,
                CodeTokenKind::Comment,
                &collect(&chars, start, index),
            );
            continue;
        }
        if current == '"' {
            flush_plain(&mut tokens, &mut plain);
            let start: usize = index;
            index += 1;
            while index < chars.len() {
                if chars[index] == '\\' {
                    index += 2;
                    continue;
                }
                if chars[index] == '"' {
                    index += 1;
                    break;
                }
                index += 1;
            }
            push_token(
                &mut tokens,
                CodeTokenKind::String,
                &collect(&chars, start, index),
            );
            continue;
        }
        if current == '\'' {
            let is_char_literal: bool =
                chars.get(index + 1) == Some(&'\\') || chars.get(index + 2) == Some(&'\'');
            if is_char_literal {
                flush_plain(&mut tokens, &mut plain);
                let start: usize = index;
                index += 1;
                while index < chars.len() {
                    if chars[index] == '\\' {
                        index += 2;
                        continue;
                    }
                    if chars[index] == '\'' {
                        index += 1;
                        break;
                    }
                    index += 1;
                }
                push_token(
                    &mut tokens,
                    CodeTokenKind::String,
                    &collect(&chars, start, index),
                );
                continue;
            }
            plain.push(current);
            index += 1;
            continue;
        }
        if current == '#' && next == '[' {
            flush_plain(&mut tokens, &mut plain);
            let start: usize = index;
            let mut depth: usize = 0;
            while index < chars.len() {
                if chars[index] == '[' {
                    depth += 1;
                }
                if chars[index] == ']' {
                    depth -= 1;
                    if depth == 0 {
                        index += 1;
                        break;
                    }
                }
                index += 1;
            }
            push_token(
                &mut tokens,
                CodeTokenKind::Attribute,
                &collect(&chars, start, index),
            );
            continue;
        }
        if current.is_ascii_digit() {
            flush_plain(&mut tokens, &mut plain);
            let start: usize = index;
            while index < chars.len()
                && (chars[index].is_ascii_alphanumeric()
                    || chars[index] == '.'
                    || chars[index] == '_')
            {
                index += 1;
            }
            push_token(
                &mut tokens,
                CodeTokenKind::Number,
                &collect(&chars, start, index),
            );
            continue;
        }
        if is_ident_start(current) {
            let start: usize = index;
            while index < chars.len() && is_ident_part(chars[index]) {
                index += 1;
            }
            let word: String = collect(&chars, start, index);
            flush_plain(&mut tokens, &mut plain);
            let (kind, announced): (CodeTokenKind, Option<CodeTokenKind>) = match pending.take() {
                Some(kind) => (kind, None),
                None => classify_rust_word(&word, &chars, index),
            };
            pending = announced;
            push_token(&mut tokens, kind, &word);
            continue;
        }
        plain.push(current);
        index += 1;
    }
    flush_plain(&mut tokens, &mut plain);
    tokens
}

/// Classifies a Rust word, and reports what the *next* word will be
/// when the current one announces it (`fn` announces a function name,
/// `struct` announces a type name).
///
/// # Arguments
///
/// - `&str` - The word.
/// - `&[char]` - The source characters.
/// - `usize` - The index just past the word.
///
/// # Returns
///
/// - `(CodeTokenKind, Option<CodeTokenKind>)` - The kind of the word,
///   and the kind the next word gets when this one announces it.
fn classify_rust_word(
    word: &str,
    chars: &[char],
    after: usize,
) -> (CodeTokenKind, Option<CodeTokenKind>) {
    if RUST_KEYWORDS.contains(&word) {
        if word == RUST_FUNCTION_KEYWORD {
            return (CodeTokenKind::Keyword, Some(CodeTokenKind::Function));
        }
        if RUST_TYPE_KEYWORDS.contains(&word) {
            return (CodeTokenKind::Keyword, Some(CodeTokenKind::Type));
        }
        return (CodeTokenKind::Keyword, None);
    }
    if starts_uppercase(word) {
        return (CodeTokenKind::Type, None);
    }
    if next_is_call(chars, after) {
        return (CodeTokenKind::Function, None);
    }
    (CodeTokenKind::Plain, None)
}

/// Tokenizes a shell script.
///
/// # Arguments
///
/// - `&str` - The raw code.
///
/// # Returns
///
/// - `Vec<CodeToken>` - The segments, in source order.
fn tokenize_bash(code: &str) -> Vec<CodeToken> {
    let chars: Vec<char> = code.chars().collect();
    let mut tokens: Vec<CodeToken> = Vec::new();
    let mut plain: String = String::new();
    let mut index: usize = 0;
    let mut at_command_start: bool = true;
    while index < chars.len() {
        let current: char = chars[index];
        if current == '#' {
            flush_plain(&mut tokens, &mut plain);
            let start: usize = index;
            while index < chars.len() && chars[index] != '\n' {
                index += 1;
            }
            push_token(
                &mut tokens,
                CodeTokenKind::Comment,
                &collect(&chars, start, index),
            );
            continue;
        }
        if current == '"' || current == '\'' {
            flush_plain(&mut tokens, &mut plain);
            let start: usize = index;
            index += 1;
            while index < chars.len() {
                if chars[index] == '\\' {
                    index += 2;
                    continue;
                }
                if chars[index] == current {
                    index += 1;
                    break;
                }
                index += 1;
            }
            push_token(
                &mut tokens,
                CodeTokenKind::String,
                &collect(&chars, start, index),
            );
            continue;
        }
        if current == '$' {
            flush_plain(&mut tokens, &mut plain);
            let start: usize = index;
            index += 1;
            if chars.get(index) == Some(&'{') {
                while index < chars.len() && chars[index] != '}' {
                    index += 1;
                }
                index = (index + 1).min(chars.len());
            } else {
                while index < chars.len() && is_ident_part(chars[index]) {
                    index += 1;
                }
            }
            push_token(
                &mut tokens,
                CodeTokenKind::Variable,
                &collect(&chars, start, index),
            );
            continue;
        }
        if current.is_ascii_digit() {
            flush_plain(&mut tokens, &mut plain);
            let start: usize = index;
            while index < chars.len() && (chars[index].is_ascii_digit() || chars[index] == '.') {
                index += 1;
            }
            push_token(
                &mut tokens,
                CodeTokenKind::Number,
                &collect(&chars, start, index),
            );
            continue;
        }
        if is_ident_start(current) {
            let start: usize = index;
            while index < chars.len() && is_ident_part(chars[index]) {
                index += 1;
            }
            let word: String = collect(&chars, start, index);
            flush_plain(&mut tokens, &mut plain);
            let kind: CodeTokenKind = if BASH_KEYWORDS.contains(&word.as_str()) {
                CodeTokenKind::Keyword
            } else if at_command_start {
                CodeTokenKind::Function
            } else {
                CodeTokenKind::Plain
            };
            if kind != CodeTokenKind::Keyword {
                at_command_start = false;
            }
            push_token(&mut tokens, kind, &word);
            continue;
        }
        plain.push(current);
        index += 1;
        if current == '\n' || current == ';' || current == '|' || current == '&' {
            at_command_start = true;
        }
    }
    flush_plain(&mut tokens, &mut plain);
    tokens
}

/// Tokenizes JSON.
///
/// # Arguments
///
/// - `&str` - The raw code.
///
/// # Returns
///
/// - `Vec<CodeToken>` - The segments, in source order.
fn tokenize_json(code: &str) -> Vec<CodeToken> {
    let chars: Vec<char> = code.chars().collect();
    let mut tokens: Vec<CodeToken> = Vec::new();
    let mut plain: String = String::new();
    let mut index: usize = 0;
    while index < chars.len() {
        let current: char = chars[index];
        if current == '"' {
            flush_plain(&mut tokens, &mut plain);
            let start: usize = index;
            index += 1;
            while index < chars.len() {
                if chars[index] == '\\' {
                    index += 2;
                    continue;
                }
                if chars[index] == '"' {
                    index += 1;
                    break;
                }
                index += 1;
            }
            let kind: CodeTokenKind = if json_key_follows(&chars, index) {
                CodeTokenKind::Attribute
            } else {
                CodeTokenKind::String
            };
            push_token(&mut tokens, kind, &collect(&chars, start, index));
            continue;
        }
        if current.is_ascii_digit()
            || (current == '-'
                && chars
                    .get(index + 1)
                    .is_some_and(|c: &char| c.is_ascii_digit()))
        {
            flush_plain(&mut tokens, &mut plain);
            let start: usize = index;
            index += 1;
            while index < chars.len()
                && (chars[index].is_ascii_digit() || chars[index] == '.' || chars[index] == 'e')
            {
                index += 1;
            }
            push_token(
                &mut tokens,
                CodeTokenKind::Number,
                &collect(&chars, start, index),
            );
            continue;
        }
        if is_ident_start(current) {
            let start: usize = index;
            while index < chars.len() && is_ident_part(chars[index]) {
                index += 1;
            }
            let word: String = collect(&chars, start, index);
            flush_plain(&mut tokens, &mut plain);
            let kind: CodeTokenKind = if JSON_KEYWORDS.contains(&word.as_str()) {
                CodeTokenKind::Keyword
            } else {
                CodeTokenKind::Plain
            };
            push_token(&mut tokens, kind, &word);
            continue;
        }
        plain.push(current);
        index += 1;
    }
    flush_plain(&mut tokens, &mut plain);
    tokens
}

/// Whether the string that ends at `after` is an object key.
///
/// # Arguments
///
/// - `&[char]` - The source characters.
/// - `usize` - The index just past the closing quote.
///
/// # Returns
///
/// - `bool` - `true` when a colon follows the string.
fn json_key_follows(chars: &[char], after: usize) -> bool {
    let mut index: usize = after;
    while index < chars.len() && chars[index].is_whitespace() {
        index += 1;
    }
    chars.get(index) == Some(&':')
}

/// Tokenizes TypeScript or JavaScript source.
///
/// # Arguments
///
/// - `&str` - The raw code.
///
/// # Returns
///
/// - `Vec<CodeToken>` - The segments, in source order.
fn tokenize_typescript(code: &str) -> Vec<CodeToken> {
    let chars: Vec<char> = code.chars().collect();
    let mut tokens: Vec<CodeToken> = Vec::new();
    let mut plain: String = String::new();
    let mut index: usize = 0;
    while index < chars.len() {
        let current: char = chars[index];
        let next: char = chars.get(index + 1).copied().unwrap_or('\0');
        if current == '/' && next == '/' {
            flush_plain(&mut tokens, &mut plain);
            let start: usize = index;
            while index < chars.len() && chars[index] != '\n' {
                index += 1;
            }
            push_token(
                &mut tokens,
                CodeTokenKind::Comment,
                &collect(&chars, start, index),
            );
            continue;
        }
        if current == '/' && next == '*' {
            flush_plain(&mut tokens, &mut plain);
            let start: usize = index;
            index += 2;
            while index < chars.len()
                && !(chars[index] == '*' && chars.get(index + 1) == Some(&'/'))
            {
                index += 1;
            }
            index = (index + 2).min(chars.len());
            push_token(
                &mut tokens,
                CodeTokenKind::Comment,
                &collect(&chars, start, index),
            );
            continue;
        }
        if current == '"' || current == '\'' || current == '`' {
            flush_plain(&mut tokens, &mut plain);
            let start: usize = index;
            index += 1;
            while index < chars.len() {
                if chars[index] == '\\' {
                    index += 2;
                    continue;
                }
                if chars[index] == current {
                    index += 1;
                    break;
                }
                index += 1;
            }
            push_token(
                &mut tokens,
                CodeTokenKind::String,
                &collect(&chars, start, index),
            );
            continue;
        }
        if current == '@'
            && chars
                .get(index + 1)
                .is_some_and(|c: &char| is_ident_start(*c))
        {
            flush_plain(&mut tokens, &mut plain);
            let start: usize = index;
            index += 1;
            while index < chars.len() && is_ident_part(chars[index]) {
                index += 1;
            }
            push_token(
                &mut tokens,
                CodeTokenKind::Attribute,
                &collect(&chars, start, index),
            );
            continue;
        }
        if current.is_ascii_digit() {
            flush_plain(&mut tokens, &mut plain);
            let start: usize = index;
            while index < chars.len()
                && (chars[index].is_ascii_alphanumeric()
                    || chars[index] == '.'
                    || chars[index] == '_')
            {
                index += 1;
            }
            push_token(
                &mut tokens,
                CodeTokenKind::Number,
                &collect(&chars, start, index),
            );
            continue;
        }
        if is_ident_start(current) {
            let start: usize = index;
            while index < chars.len() && is_ident_part(chars[index]) {
                index += 1;
            }
            let word: String = collect(&chars, start, index);
            flush_plain(&mut tokens, &mut plain);
            let kind: CodeTokenKind = if TS_KEYWORDS.contains(&word.as_str()) {
                CodeTokenKind::Keyword
            } else if starts_uppercase(&word) {
                CodeTokenKind::Type
            } else if next_is_call(&chars, index) {
                CodeTokenKind::Function
            } else {
                CodeTokenKind::Plain
            };
            push_token(&mut tokens, kind, &word);
            continue;
        }
        plain.push(current);
        index += 1;
    }
    flush_plain(&mut tokens, &mut plain);
    tokens
}

/// Tokenizes HTML.
///
/// # Arguments
///
/// - `&str` - The raw code.
///
/// # Returns
///
/// - `Vec<CodeToken>` - The segments, in source order.
fn tokenize_html(code: &str) -> Vec<CodeToken> {
    let chars: Vec<char> = code.chars().collect();
    let mut tokens: Vec<CodeToken> = Vec::new();
    let mut plain: String = String::new();
    let mut index: usize = 0;
    while index < chars.len() {
        let current: char = chars[index];
        if current == '<'
            && chars.get(index + 1) == Some(&'!')
            && chars.get(index + 2) == Some(&'-')
        {
            flush_plain(&mut tokens, &mut plain);
            let start: usize = index;
            while index < chars.len()
                && !(chars[index] == '-' && chars.get(index + 1) == Some(&'-'))
            {
                index += 1;
            }
            index = (index + 2).min(chars.len());
            push_token(
                &mut tokens,
                CodeTokenKind::Comment,
                &collect(&chars, start, index),
            );
            continue;
        }
        if current == '<' {
            flush_plain(&mut tokens, &mut plain);
            index += 1;
            if chars.get(index) == Some(&'/') {
                index += 1;
            }
            let name_start: usize = index;
            while index < chars.len() && is_ident_part(chars[index]) {
                index += 1;
            }
            push_token(
                &mut tokens,
                CodeTokenKind::Tag,
                &collect(&chars, name_start, index),
            );
            while index < chars.len() && chars[index] != '>' {
                let attribute: char = chars[index];
                if attribute == '"' || attribute == '\'' {
                    let quote: char = attribute;
                    let value_start: usize = index;
                    index += 1;
                    while index < chars.len() && chars[index] != quote {
                        index += 1;
                    }
                    index = (index + 1).min(chars.len());
                    push_token(
                        &mut tokens,
                        CodeTokenKind::String,
                        &collect(&chars, value_start, index),
                    );
                    continue;
                }
                if is_ident_start(attribute) {
                    let attribute_start: usize = index;
                    while index < chars.len() && is_ident_part(chars[index]) {
                        index += 1;
                    }
                    push_token(
                        &mut tokens,
                        CodeTokenKind::Attribute,
                        &collect(&chars, attribute_start, index),
                    );
                    continue;
                }
                index += 1;
            }
            index = (index + 1).min(chars.len());
            continue;
        }
        plain.push(current);
        index += 1;
    }
    flush_plain(&mut tokens, &mut plain);
    tokens
}

/// Tokenizes CSS.
///
/// # Arguments
///
/// - `&str` - The raw code.
///
/// # Returns
///
/// - `Vec<CodeToken>` - The segments, in source order.
fn tokenize_css(code: &str) -> Vec<CodeToken> {
    let chars: Vec<char> = code.chars().collect();
    let mut tokens: Vec<CodeToken> = Vec::new();
    let mut plain: String = String::new();
    let mut index: usize = 0;
    let mut in_block: bool = false;
    while index < chars.len() {
        let current: char = chars[index];
        let next: char = chars.get(index + 1).copied().unwrap_or('\0');
        if current == '/' && next == '*' {
            flush_plain(&mut tokens, &mut plain);
            let start: usize = index;
            index += 2;
            while index < chars.len()
                && !(chars[index] == '*' && chars.get(index + 1) == Some(&'/'))
            {
                index += 1;
            }
            index = (index + 2).min(chars.len());
            push_token(
                &mut tokens,
                CodeTokenKind::Comment,
                &collect(&chars, start, index),
            );
            continue;
        }
        if current == '"' || current == '\'' {
            flush_plain(&mut tokens, &mut plain);
            let start: usize = index;
            index += 1;
            while index < chars.len() && chars[index] != current {
                index += 1;
            }
            index = (index + 1).min(chars.len());
            push_token(
                &mut tokens,
                CodeTokenKind::String,
                &collect(&chars, start, index),
            );
            continue;
        }
        if current == '{' {
            in_block = true;
            plain.push(current);
            index += 1;
            continue;
        }
        if current == '}' {
            in_block = false;
            plain.push(current);
            index += 1;
            continue;
        }
        if current.is_ascii_digit() {
            flush_plain(&mut tokens, &mut plain);
            let start: usize = index;
            while index < chars.len()
                && (chars[index].is_ascii_alphanumeric()
                    || chars[index] == '.'
                    || chars[index] == '%')
            {
                index += 1;
            }
            push_token(
                &mut tokens,
                CodeTokenKind::Number,
                &collect(&chars, start, index),
            );
            continue;
        }
        if is_ident_start(current) || current == '-' {
            let start: usize = index;
            index += 1;
            while index < chars.len() && (is_ident_part(chars[index]) || chars[index] == '-') {
                index += 1;
            }
            let word: String = collect(&chars, start, index);
            flush_plain(&mut tokens, &mut plain);
            let kind: CodeTokenKind = if in_block && css_is_property(&chars, index) {
                CodeTokenKind::Attribute
            } else if in_block {
                CodeTokenKind::Plain
            } else {
                CodeTokenKind::Tag
            };
            push_token(&mut tokens, kind, &word);
            continue;
        }
        plain.push(current);
        index += 1;
    }
    flush_plain(&mut tokens, &mut plain);
    tokens
}

/// Whether the word that ends at `after` is a property name, i.e. is
/// followed by a colon.
///
/// # Arguments
///
/// - `&[char]` - The source characters.
/// - `usize` - The index just past the word.
///
/// # Returns
///
/// - `bool` - `true` when a colon follows the word.
fn css_is_property(chars: &[char], after: usize) -> bool {
    let mut index: usize = after;
    while index < chars.len() && chars[index].is_whitespace() {
        index += 1;
    }
    chars.get(index) == Some(&':')
}
