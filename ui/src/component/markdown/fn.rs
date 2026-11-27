use super::*;

/// Returns the first non-ASCII-whitespace byte at or after `from`, if any.
///
/// # Arguments
///
/// - `&[u8]` - Source line UTF-8 bytes.
/// - `usize` - Starting byte offset.
///
/// # Returns
///
/// - `Option<u8>` - First non-whitespace byte after `from`.
#[inline(always)]
fn next_non_ws_byte(bytes: &[u8], from: usize) -> Option<u8> {
    let mut idx: usize = from;
    while idx < bytes.len() {
        let b: u8 = bytes[idx];
        if !b.is_ascii_whitespace() {
            return Some(b);
        }
        idx += 1;
    }
    None
}

/// Returns the last non-ASCII-whitespace byte strictly before `before`, if any.
///
/// # Arguments
///
/// - `&[u8]` - Source line UTF-8 bytes.
/// - `usize` - Exclusive upper byte offset.
///
/// # Returns
///
/// - `Option<u8>` - Closest preceding non-whitespace byte.
#[inline(always)]
fn prev_non_ws_byte(bytes: &[u8], before: usize) -> Option<u8> {
    let mut idx: usize = before;
    while idx > 0 {
        idx -= 1;
        let b: u8 = bytes[idx];
        if !b.is_ascii_whitespace() {
            return Some(b);
        }
    }
    None
}

/// Classifies a non-empty identifier slice into its exact Eva Bold `CodeTokenKind` in O(1) via jump-table matching.
///
/// # Arguments
///
/// - `&str` - Identifier token slice.
/// - `&[u8]` - Full line UTF-8 bytes for lazy neighbor punctuation lookup.
/// - `usize` - Identifier start byte offset in `bytes`.
/// - `usize` - Identifier exclusive end byte offset in `bytes`.
/// - `CodeLang` - Pre-resolved language hint.
///
/// # Returns
///
/// - `CodeTokenKind` - Exact syntax token classification.
#[inline(always)]
fn classify_code_ident(
    word: &str,
    bytes: &[u8],
    start: usize,
    end: usize,
    lang: CodeLang,
) -> CodeTokenKind {
    let first_byte: u8 = bytes[start];
    if first_byte.is_ascii_uppercase() {
        return match word {
            KW_CTRL_SELF_UPPER => CodeTokenKind::Control,
            KW_CONST_NONE | KW_CONST_SOME | KW_CONST_OK => CodeTokenKind::Constant,
            KW_CONST_ERR | KW_CONST_NAN | KW_CONST_INFINITY => CodeTokenKind::Constant,
            KW_CONST_TRUE_PY | KW_CONST_FALSE_PY => CodeTokenKind::Constant,
            KW_RES_RESULT | KW_RES_OPTION => CodeTokenKind::Property,
            _ => CodeTokenKind::Type,
        };
    }
    let word_len: usize = word.len();
    if (2..=9).contains(&word_len) && first_byte != CODE_BYTE_UNDERSCORE {
        let matched: Option<CodeTokenKind> = match word {
            KW_FN | KW_LET | KW_MUT | KW_PUB | KW_MOD => Some(CodeTokenKind::Keyword),
            KW_STRUCT | KW_ENUM | KW_TRAIT | KW_IMPL => Some(CodeTokenKind::Keyword),
            KW_CONST | KW_STATIC | KW_TYPE | KW_REF => Some(CodeTokenKind::Keyword),
            KW_ASYNC | KW_FUNCTION | KW_CLASS => Some(CodeTokenKind::Keyword),
            KW_INTERFACE | KW_VAR | KW_DEF | KW_LAMBDA => Some(CodeTokenKind::Keyword),
            KW_LOCAL | KW_TABLE | KW_CREATE => Some(CodeTokenKind::Keyword),
            KW_CTRL_USE | KW_CTRL_FOR | KW_CTRL_IN => Some(CodeTokenKind::Control),
            KW_CTRL_WHILE | KW_CTRL_LOOP | KW_CTRL_IF => Some(CodeTokenKind::Control),
            KW_CTRL_ELSE | KW_CTRL_MATCH | KW_CTRL_RETURN => Some(CodeTokenKind::Control),
            KW_CTRL_BREAK | KW_CTRL_CONTINUE => Some(CodeTokenKind::Control),
            KW_CTRL_WHERE | KW_CTRL_AS | KW_CTRL_MOVE => Some(CodeTokenKind::Control),
            KW_CTRL_AWAIT | KW_CTRL_UNSAFE => Some(CodeTokenKind::Control),
            KW_CTRL_EXTERN | KW_CTRL_CRATE | KW_CTRL_SUPER => Some(CodeTokenKind::Control),
            KW_CTRL_SELF | KW_CTRL_DYN | KW_CTRL_EXPORT => Some(CodeTokenKind::Control),
            KW_CTRL_IMPORT | KW_CTRL_FROM => Some(CodeTokenKind::Control),
            KW_CTRL_DEFAULT | KW_CTRL_NEW | KW_CTRL_TRY => Some(CodeTokenKind::Control),
            KW_CTRL_CATCH | KW_CTRL_THROW => Some(CodeTokenKind::Control),
            KW_CTRL_FINALLY | KW_CTRL_SWITCH => Some(CodeTokenKind::Control),
            KW_CTRL_CASE | KW_CTRL_WITH | KW_CTRL_YIELD => Some(CodeTokenKind::Control),
            KW_CTRL_PASS | KW_CTRL_ELIF | KW_CTRL_AND => Some(CodeTokenKind::Control),
            KW_CTRL_OR | KW_CTRL_NOT | KW_CTRL_SELECT => Some(CodeTokenKind::Control),
            KW_CTRL_INSERT | KW_CTRL_UPDATE => Some(CodeTokenKind::Control),
            KW_CTRL_DELETE | KW_CTRL_ECHO | KW_CTRL_THEN => Some(CodeTokenKind::Control),
            KW_CTRL_FI | KW_CTRL_DONE | KW_CTRL_DO => Some(CodeTokenKind::Control),
            KW_CONST_TRUE | KW_CONST_FALSE => Some(CodeTokenKind::Constant),
            KW_CONST_NULL | KW_CONST_UNDEFINED => Some(CodeTokenKind::Constant),
            KW_PRIM_BOOL | KW_PRIM_CHAR | KW_PRIM_STR => Some(CodeTokenKind::PrimitiveType),
            KW_PRIM_U8 | KW_PRIM_U16 | KW_PRIM_U32 => Some(CodeTokenKind::PrimitiveType),
            KW_PRIM_U64 | KW_PRIM_U128 | KW_PRIM_USIZE => Some(CodeTokenKind::PrimitiveType),
            KW_PRIM_I8 | KW_PRIM_I16 | KW_PRIM_I32 => Some(CodeTokenKind::PrimitiveType),
            KW_PRIM_I64 | KW_PRIM_I128 | KW_PRIM_ISIZE => Some(CodeTokenKind::PrimitiveType),
            KW_PRIM_F32 | KW_PRIM_F64 | KW_PRIM_NUMBER => Some(CodeTokenKind::PrimitiveType),
            KW_PRIM_STRING | KW_PRIM_BOOLEAN => Some(CodeTokenKind::PrimitiveType),
            KW_PRIM_VOID | KW_PRIM_ANY => Some(CodeTokenKind::PrimitiveType),
            _ => None,
        };
        if let Some(kind) = matched {
            return kind;
        }
    }
    let next_non_ws: Option<u8> = next_non_ws_byte(bytes, end);
    if next_non_ws == Some(CODE_BYTE_LPAREN) {
        return CodeTokenKind::Function;
    }
    if prev_non_ws_byte(bytes, start) == Some(CODE_BYTE_DOT)
        || (next_non_ws == Some(CODE_BYTE_COLON) && matches!(lang, CodeLang::Toml | CodeLang::Yaml))
    {
        return CodeTokenKind::Property;
    }
    CodeTokenKind::Plain
}

/// Extends or flushes a byte-range pending span to coalesce adjacent same-kind tokens with zero intermediate allocations.
///
/// # Arguments
///
/// - `&mut Vec<CodeSpan>` - Destination span buffer.
/// - `&mut Option<(usize, usize, CodeTokenKind)>` - Pending coalesced byte range.
/// - `&str` - Source line string.
/// - `usize` - Token start byte offset.
/// - `usize` - Token exclusive end byte offset.
/// - `CodeTokenKind` - Token syntax classification.
#[inline(always)]
fn push_merged_code_span(
    out: &mut Vec<CodeSpan>,
    pending: &mut Option<(usize, usize, CodeTokenKind)>,
    line: &str,
    start: usize,
    end: usize,
    kind: CodeTokenKind,
) {
    if let Some((p_start, p_end, p_kind)) = *pending {
        if p_kind == kind && p_end == start {
            *pending = Some((p_start, end, p_kind));
            return;
        }
        out.push(CodeSpan::new(line[p_start..p_end].to_string(), p_kind));
    }
    *pending = Some((start, end, kind));
}

/// Tokenizes a single source-code line into coalesced syntax-highlighted `CodeSpan` runs using a pre-resolved `CodeLang`.
///
/// # Arguments
///
/// - `&str` - Source line to tokenize.
/// - `CodeLang` - Pre-resolved language hint.
///
/// # Returns
///
/// - `Vec<CodeSpan>` - Contiguous coalesced syntax-classified spans covering the full line.
#[must_use]
pub fn tokenize_code_line_with_lang(line: &str, lang: CodeLang) -> Vec<CodeSpan> {
    if line.is_empty() {
        return vec![CodeSpan::new(" ".to_string(), CodeTokenKind::Plain)];
    }
    if lang == CodeLang::Diff {
        if line.starts_with('+') && !line.starts_with("+++") {
            return vec![CodeSpan::new(line.to_string(), CodeTokenKind::DiffAdd)];
        }
        if line.starts_with('-') && !line.starts_with("---") {
            return vec![CodeSpan::new(line.to_string(), CodeTokenKind::DiffDel)];
        }
    }
    let trimmed: &str = line.trim_start();
    if trimmed.starts_with("//")
        || trimmed.starts_with("--")
        || (trimmed.starts_with('#') && !trimmed.starts_with("#[") && !trimmed.starts_with("#!["))
    {
        return vec![CodeSpan::new(line.to_string(), CodeTokenKind::Comment)];
    }
    if trimmed.starts_with("#[") || trimmed.starts_with("#![") {
        return vec![CodeSpan::new(line.to_string(), CodeTokenKind::Macro)];
    }
    let bytes: &[u8] = line.as_bytes();
    let len: usize = bytes.len();
    let mut i: usize = 0;
    let mut out: Vec<CodeSpan> = Vec::with_capacity((len / 8).clamp(2, 16));
    let mut pending: Option<(usize, usize, CodeTokenKind)> = None;
    while i < len {
        let ch: u8 = bytes[i];
        if ch == CODE_BYTE_SLASH && i + 1 < len && bytes[i + 1] == CODE_BYTE_SLASH {
            push_merged_code_span(&mut out, &mut pending, line, i, len, CodeTokenKind::Comment);
            break;
        }
        if ch == CODE_BYTE_SQUOTE
            && lang == CodeLang::Rust
            && i + 1 < len
            && (bytes[i + 1].is_ascii_alphabetic() || bytes[i + 1] == CODE_BYTE_UNDERSCORE)
            && (i + 2 >= len || bytes[i + 2] != CODE_BYTE_SQUOTE)
        {
            let start: usize = i;
            i += 2;
            while i < len && (bytes[i].is_ascii_alphanumeric() || bytes[i] == CODE_BYTE_UNDERSCORE)
            {
                i += 1;
            }
            push_merged_code_span(&mut out, &mut pending, line, start, i, CodeTokenKind::Macro);
            continue;
        }
        if ch == CODE_BYTE_DQUOTE || ch == CODE_BYTE_SQUOTE || ch == CODE_BYTE_BACKTICK {
            let quote: u8 = ch;
            let start: usize = i;
            i += 1;
            while i < len {
                if bytes[i] == CODE_BYTE_BACKSLASH && i + 1 < len {
                    i += 2;
                    continue;
                }
                if bytes[i] == quote {
                    i += 1;
                    break;
                }
                i += 1;
            }
            let kind: CodeTokenKind = if lang == CodeLang::Json {
                let mut look: usize = i;
                while look < len && bytes[look] == CODE_BYTE_SPACE {
                    look += 1;
                }
                if look < len && bytes[look] == CODE_BYTE_COLON {
                    CodeTokenKind::Property
                } else {
                    CodeTokenKind::String
                }
            } else {
                CodeTokenKind::String
            };
            push_merged_code_span(&mut out, &mut pending, line, start, i, kind);
            continue;
        }
        if ch.is_ascii_digit() {
            let start: usize = i;
            i += 1;
            while i < len
                && (bytes[i].is_ascii_alphanumeric()
                    || bytes[i] == CODE_BYTE_DOT
                    || bytes[i] == CODE_BYTE_UNDERSCORE)
            {
                i += 1;
            }
            push_merged_code_span(
                &mut out,
                &mut pending,
                line,
                start,
                i,
                CodeTokenKind::Number,
            );
            continue;
        }
        if ch.is_ascii_alphabetic() || ch == CODE_BYTE_UNDERSCORE {
            let start: usize = i;
            i += 1;
            while i < len && (bytes[i].is_ascii_alphanumeric() || bytes[i] == CODE_BYTE_UNDERSCORE)
            {
                i += 1;
            }
            if i < len && bytes[i] == CODE_BYTE_BANG {
                i += 1;
                push_merged_code_span(&mut out, &mut pending, line, start, i, CodeTokenKind::Macro);
                continue;
            }
            let word: &str = &line[start..i];
            let kind: CodeTokenKind = classify_code_ident(word, bytes, start, i, lang);
            push_merged_code_span(&mut out, &mut pending, line, start, i, kind);
            continue;
        }
        if ch.is_ascii_whitespace() {
            let start: usize = i;
            i += 1;
            while i < len && bytes[i].is_ascii_whitespace() {
                i += 1;
            }
            push_merged_code_span(&mut out, &mut pending, line, start, i, CodeTokenKind::Plain);
            continue;
        }
        let start: usize = i;
        let kind: CodeTokenKind = if ch < 0x80 {
            i += 1;
            match ch {
                b'(' | b')' | b'{' | b'}' | b'[' | b']' => CodeTokenKind::Punctuation,
                b',' | b';' | b'.' | b':' => CodeTokenKind::Punctuation,
                b'=' | b'+' | b'-' | b'*' | b'/' | b'>' => CodeTokenKind::Operator,
                b'<' | b'!' | b'&' | b'|' | b'^' | b'%' => CodeTokenKind::Operator,
                _ => CodeTokenKind::Plain,
            }
        } else {
            let ch_len: usize = line[i..].chars().next().map(char::len_utf8).unwrap_or(1);
            i += ch_len;
            CodeTokenKind::Plain
        };
        push_merged_code_span(&mut out, &mut pending, line, start, i, kind);
    }
    if let Some((p_start, p_end, p_kind)) = pending {
        out.push(CodeSpan::new(line[p_start..p_end].to_string(), p_kind));
    }
    out
}

/// Tokenizes a single source-code line into syntax-highlighted `CodeSpan` runs for VS Code Eva theming.
///
/// # Arguments
///
/// - `&str` - Source line to tokenize.
/// - `&str` - Language hint (`"rust"`, `"ts"`, `"json"`, `"toml"`, `"bash"`, `"diff"`, etc.).
///
/// # Returns
///
/// - `Vec<CodeSpan>` - Contiguous syntax-classified spans covering the full line.
#[must_use]
pub fn tokenize_code_line(line: &str, language: &str) -> Vec<CodeSpan> {
    tokenize_code_line_with_lang(line, CodeLang::from_name(language))
}

/// Parses rich inline Markdown formatting (`**bold**`, `*italic*`, `~~strike~~`, `` `code` ``, `[links](url)`, `<kbd>Key</kbd>`).
///
/// # Arguments
///
/// - `&str` - Raw single-line or paragraph Markdown string.
///
/// # Returns
///
/// - `Vec<EuvMarkdownInline>` - Ordered inline spans.
#[must_use]
pub fn parse_markdown_inlines(text: &str) -> Vec<EuvMarkdownInline> {
    let mut out: Vec<EuvMarkdownInline> = Vec::new();
    let mut rest: &str = text;
    let mut plain_buf: String = String::new();
    while !rest.is_empty() {
        if let Some(after) = rest.strip_prefix("`")
            && let Some(end) = after.find('`')
        {
            flush_plain(&mut plain_buf, &mut out);
            out.push(EuvMarkdownInline::Code(after[..end].to_string()));
            rest = &after[end + 1..];
            continue;
        }
        if let Some(after) = rest.strip_prefix("**")
            && let Some(end) = after.find("**")
        {
            flush_plain(&mut plain_buf, &mut out);
            out.push(EuvMarkdownInline::Bold(after[..end].to_string()));
            rest = &after[end + 2..];
            continue;
        }
        if let Some(after) = rest.strip_prefix("~~")
            && let Some(end) = after.find("~~")
        {
            flush_plain(&mut plain_buf, &mut out);
            out.push(EuvMarkdownInline::Strike(after[..end].to_string()));
            rest = &after[end + 2..];
            continue;
        }
        if let Some(after) = rest.strip_prefix(MD_KBD_OPEN)
            && let Some(end) = after.find(MD_KBD_CLOSE)
        {
            flush_plain(&mut plain_buf, &mut out);
            out.push(EuvMarkdownInline::Kbd(after[..end].to_string()));
            rest = &after[end + MD_KBD_CLOSE.len()..];
            continue;
        }
        if let Some(after) = rest.strip_prefix('*')
            && !after.starts_with('*')
            && let Some(end) = after.find('*')
        {
            flush_plain(&mut plain_buf, &mut out);
            out.push(EuvMarkdownInline::Italic(after[..end].to_string()));
            rest = &after[end + 1..];
            continue;
        }
        if let Some(after_lb) = rest.strip_prefix('[')
            && let Some(rb) = after_lb.find("](")
            && let Some(rp) = after_lb[rb + 2..].find(')')
        {
            flush_plain(&mut plain_buf, &mut out);
            let label: String = after_lb[..rb].to_string();
            let url: String = after_lb[rb + 2..rb + 2 + rp].to_string();
            out.push(EuvMarkdownInline::Link { label, url });
            rest = &after_lb[rb + 2 + rp + 1..];
            continue;
        }
        if let Some(ch) = rest.chars().next() {
            plain_buf.push(ch);
            rest = &rest[ch.len_utf8()..];
        } else {
            break;
        }
    }
    flush_plain(&mut plain_buf, &mut out);
    out
}

/// Flushes accumulated plain text into `out` if non-empty.
///
/// # Arguments
///
/// - `&mut String` - Pending plain text buffer.
/// - `&mut Vec<EuvMarkdownInline>` - Destination inline span vector.
fn flush_plain(buf: &mut String, out: &mut Vec<EuvMarkdownInline>) {
    if !buf.is_empty() {
        out.push(EuvMarkdownInline::Text(take(buf)));
    }
}

/// Infers a sensible syntax-highlighting language tag when a code block omits an explicit `.language(...)`.
///
/// # Arguments
///
/// - `&str` - Source code text.
///
/// # Returns
///
/// - `&'static str` - Inferred language hint (`"rust"`, `"json"`, `"http"`, or `"text"`).
#[must_use]
pub(crate) fn infer_code_language(code: &str) -> &'static str {
    let trimmed: &str = code.trim_start();
    if trimmed.starts_with('{') || trimmed.starts_with('[') {
        CODE_LANG_JSON
    } else if trimmed.starts_with(CODE_HINT_GET) || trimmed.starts_with(CODE_HINT_POST) {
        CODE_LANG_HTTP
    } else if code.contains(CODE_HINT_FN)
        || code.contains(CODE_HINT_LET)
        || code.contains(CODE_HINT_IMPL)
        || code.contains(CODE_HINT_PUB)
    {
        CODE_LANG_RUST
    } else {
        CODE_DEFAULT_LANG
    }
}

/// Parses a complete GitHub-Flavored Markdown document into structured `EuvMarkdownBlock` nodes.
///
/// # Arguments
///
/// - `&str` - Raw Markdown source text.
///
/// # Returns
///
/// - `Vec<EuvMarkdownBlock>` - Ordered block-level Markdown AST.
#[must_use]
pub fn parse_markdown_blocks(source: &str) -> Vec<EuvMarkdownBlock> {
    let lines: Vec<&str> = source.lines().collect();
    let total: usize = lines.len();
    let mut idx: usize = 0;
    let mut blocks: Vec<EuvMarkdownBlock> = Vec::new();
    while idx < total {
        let raw: &str = lines[idx];
        let trimmed: &str = raw.trim();
        if trimmed.is_empty() {
            idx += 1;
            continue;
        }
        if let Some(fence_rest) = trimmed.strip_prefix("```") {
            let lang: String = fence_rest.trim().to_string();
            idx += 1;
            let mut code_lines: Vec<&str> = Vec::new();
            while idx < total && !lines[idx].trim().starts_with("```") {
                code_lines.push(lines[idx]);
                idx += 1;
            }
            if idx < total {
                idx += 1;
            }
            blocks.push(EuvMarkdownBlock::CodeBlock {
                language: if lang.is_empty() {
                    CODE_DEFAULT_LANG.to_string()
                } else {
                    lang
                },
                code: code_lines.join("\n"),
            });
            continue;
        }
        if trimmed.starts_with(MD_DETAILS_OPEN_TAG) {
            idx += 1;
            let mut summary: String = MD_DETAILS_DEFAULT_SUMMARY.to_string();
            let mut body: Vec<String> = Vec::new();
            while idx < total && !lines[idx].trim().starts_with(MD_DETAILS_CLOSE_TAG) {
                let lt: &str = lines[idx].trim();
                if let Some(after) = lt.strip_prefix(MD_SUMMARY_OPEN_TAG) {
                    summary = after
                        .strip_suffix(MD_SUMMARY_CLOSE_TAG)
                        .unwrap_or(after)
                        .trim()
                        .to_string();
                } else if !lt.is_empty() {
                    body.push(lt.to_string());
                }
                idx += 1;
            }
            if idx < total {
                idx += 1;
            }
            blocks.push(EuvMarkdownBlock::Details { summary, body });
            continue;
        }
        if trimmed == "---" || trimmed == "***" || trimmed == "___" {
            blocks.push(EuvMarkdownBlock::Divider);
            idx += 1;
            continue;
        }
        if let Some((level, heading_txt)) = parse_heading_line(trimmed) {
            blocks.push(EuvMarkdownBlock::Heading {
                level,
                text: heading_txt,
            });
            idx += 1;
            continue;
        }
        if let Some(after_bang) = trimmed.strip_prefix("![")
            && let Some(rb) = after_bang.find("](")
            && let Some(rp) = after_bang[rb + 2..].find(')')
        {
            blocks.push(EuvMarkdownBlock::Image {
                alt: after_bang[..rb].to_string(),
                url: after_bang[rb + 2..rb + 2 + rp].to_string(),
            });
            idx += 1;
            continue;
        }
        if trimmed.starts_with('>') {
            let mut q_lines: Vec<String> = Vec::new();
            while idx < total && lines[idx].trim().starts_with('>') {
                let inner: &str = lines[idx]
                    .trim()
                    .strip_prefix('>')
                    .unwrap_or("")
                    .trim_start();
                q_lines.push(inner.to_string());
                idx += 1;
            }
            let (kind, clean_lines): (EuvMarkdownAlertKind, Vec<String>) =
                classify_quote_alert(q_lines);
            blocks.push(EuvMarkdownBlock::Quote {
                kind,
                lines: clean_lines,
            });
            continue;
        }
        if trimmed.starts_with('|') && idx + 1 < total && is_table_separator(lines[idx + 1].trim())
        {
            let headers: Vec<String> = split_table_row(trimmed);
            let alignments: Vec<EuvMarkdownAlign> = parse_table_alignments(lines[idx + 1].trim());
            idx += 2;
            let mut rows: Vec<Vec<String>> = Vec::new();
            while idx < total && lines[idx].trim().starts_with('|') {
                rows.push(split_table_row(lines[idx].trim()));
                idx += 1;
            }
            blocks.push(EuvMarkdownBlock::Table {
                headers,
                alignments,
                rows,
            });
            continue;
        }
        if let Some((ordered, first_item)) = parse_list_line(raw) {
            let mut items: Vec<EuvMarkdownListItem> = vec![first_item];
            idx += 1;
            while idx < total {
                if let Some((item_ord, item)) = parse_list_line(lines[idx])
                    && item_ord == ordered
                {
                    items.push(item);
                    idx += 1;
                } else {
                    break;
                }
            }
            blocks.push(EuvMarkdownBlock::List { ordered, items });
            continue;
        }
        let mut para_parts: Vec<&str> = Vec::new();
        while idx < total {
            let cur: &str = lines[idx].trim();
            if cur.is_empty()
                || cur.starts_with("```")
                || cur.starts_with('#')
                || cur.starts_with('>')
                || cur.starts_with('|')
                || cur.starts_with("![")
                || cur.starts_with(MD_DETAILS_OPEN_TAG)
                || cur == "---"
                || parse_list_line(lines[idx]).is_some()
            {
                break;
            }
            para_parts.push(cur);
            idx += 1;
        }
        let joined: String = para_parts.join(" ");
        blocks.push(EuvMarkdownBlock::Paragraph(parse_markdown_inlines(&joined)));
    }
    blocks
}

/// Parses an ATX heading line (`# Title` .. `###### Title`).
///
/// # Arguments
///
/// - `&str` - Trimmed input line.
///
/// # Returns
///
/// - `Option<(u8, String)>` - Heading level and trimmed text if valid.
fn parse_heading_line(trimmed: &str) -> Option<(u8, String)> {
    let hashes: usize = trimmed
        .bytes()
        .take_while(|b: &u8| *b == CODE_BYTE_HASH)
        .count();
    if hashes == 0 || hashes > 6 {
        return None;
    }
    let rest: &str = trimmed[hashes..].strip_prefix(' ')?;
    Some((hashes as u8, rest.trim().to_string()))
}

/// Classifies a blockquote block into a plain quote or GitHub Alert variant (`[!NOTE]`, `[!TIP]`, etc.).
///
/// # Arguments
///
/// - `Vec<String>` - Raw stripped blockquote lines.
///
/// # Returns
///
/// - `(EuvMarkdownAlertKind, Vec<String>)` - Resolved alert kind and body lines.
fn classify_quote_alert(mut lines: Vec<String>) -> (EuvMarkdownAlertKind, Vec<String>) {
    let Some(first) = lines.first().map(|s: &String| s.trim().to_string()) else {
        return (EuvMarkdownAlertKind::Quote, lines);
    };
    let kind: EuvMarkdownAlertKind = match first.as_str() {
        MD_ALERT_TAG_NOTE => EuvMarkdownAlertKind::Note,
        MD_ALERT_TAG_TIP => EuvMarkdownAlertKind::Tip,
        MD_ALERT_TAG_IMPORTANT => EuvMarkdownAlertKind::Important,
        MD_ALERT_TAG_WARNING => EuvMarkdownAlertKind::Warning,
        MD_ALERT_TAG_CAUTION => EuvMarkdownAlertKind::Caution,
        _ => return (EuvMarkdownAlertKind::Quote, lines),
    };
    let _: String = lines.remove(0);
    (kind, lines)
}

/// Checks whether a Markdown line is a GFM table alignment separator row (`| --- | :---: |`).
///
/// # Arguments
///
/// - `&str` - Trimmed candidate separator line.
///
/// # Returns
///
/// - `bool` - `true` when the row is a valid table separator.
fn is_table_separator(trimmed: &str) -> bool {
    if !trimmed.starts_with('|') {
        return false;
    }
    let cells: Vec<String> = split_table_row(trimmed);
    !cells.is_empty()
        && cells.iter().all(|c: &String| {
            let t: &str = c.trim();
            !t.is_empty() && t.chars().all(|ch: char| ch == '-' || ch == ':')
        })
}

/// Splits a pipe-delimited Markdown table row into trimmed cell strings.
///
/// # Arguments
///
/// - `&str` - Trimmed table row line.
///
/// # Returns
///
/// - `Vec<String>` - Cell strings.
fn split_table_row(trimmed: &str) -> Vec<String> {
    let inner: &str = trimmed
        .strip_prefix('|')
        .unwrap_or(trimmed)
        .strip_suffix('|')
        .unwrap_or(trimmed.strip_prefix('|').unwrap_or(trimmed));
    inner
        .split('|')
        .map(|cell: &str| cell.trim().to_string())
        .collect()
}

/// Parses column alignments from a GFM table separator row.
///
/// # Arguments
///
/// - `&str` - Trimmed separator row line.
///
/// # Returns
///
/// - `Vec<EuvMarkdownAlign>` - Parsed column alignments.
fn parse_table_alignments(trimmed: &str) -> Vec<EuvMarkdownAlign> {
    split_table_row(trimmed)
        .into_iter()
        .map(|cell: String| {
            let left: bool = cell.starts_with(':');
            let right: bool = cell.ends_with(':');
            if left && right {
                EuvMarkdownAlign::Center
            } else if right {
                EuvMarkdownAlign::Right
            } else {
                EuvMarkdownAlign::Left
            }
        })
        .collect()
}

/// Parses an unordered (`- `, `* `), ordered (`1. `), or GFM task (`- [x] `) list line.
///
/// # Arguments
///
/// - `&str` - Raw input line (leading spaces determine nesting depth).
///
/// # Returns
///
/// - `Option<(bool, EuvMarkdownListItem)>` - `(ordered, parsed_item)` when the line is a list item.
fn parse_list_line(raw: &str) -> Option<(bool, EuvMarkdownListItem)> {
    let leading_spaces: usize = raw
        .bytes()
        .take_while(|b: &u8| *b == CODE_BYTE_SPACE)
        .count();
    let depth: usize = leading_spaces / 2;
    let trimmed: &str = raw[leading_spaces..].trim_end();
    if let Some(after) = trimmed
        .strip_prefix("- ")
        .or_else(|| trimmed.strip_prefix("* "))
        .or_else(|| trimmed.strip_prefix("+ "))
    {
        let (checked, body): (Option<bool>, &str) = if let Some(rest) = after
            .strip_prefix("[x] ")
            .or_else(|| after.strip_prefix("[X] "))
        {
            (Some(true), rest)
        } else if let Some(rest) = after.strip_prefix("[ ] ") {
            (Some(false), rest)
        } else {
            (None, after)
        };
        return Some((
            false,
            EuvMarkdownListItem::new(body.trim().to_string(), depth, checked),
        ));
    }
    let digit_len: usize = trimmed
        .bytes()
        .take_while(|b: &u8| b.is_ascii_digit())
        .count();
    if digit_len > 0
        && let Some(after_dot) = trimmed[digit_len..].strip_prefix(". ")
    {
        return Some((
            true,
            EuvMarkdownListItem::new(after_dot.trim().to_string(), depth, None),
        ));
    }
    None
}
