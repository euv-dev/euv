use super::*;

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
    if line.is_empty() {
        return vec![CodeSpan::new(" ".to_string(), CodeTokenKind::Plain)];
    }
    let lang: String = language.trim().to_ascii_lowercase();
    if lang == CODE_LANG_DIFF {
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
    let mut out: Vec<CodeSpan> = Vec::new();
    while i < len {
        let ch: u8 = bytes[i];
        if ch == b'/' && i + 1 < len && bytes[i + 1] == b'/' {
            out.push(CodeSpan::new(line[i..].to_string(), CodeTokenKind::Comment));
            break;
        }
        if ch == b'\''
            && lang == CODE_LANG_RUST
            && i + 1 < len
            && (bytes[i + 1].is_ascii_alphabetic() || bytes[i + 1] == b'_')
            && (i + 2 >= len || bytes[i + 2] != b'\'')
        {
            let start: usize = i;
            i += 2;
            while i < len && (bytes[i].is_ascii_alphanumeric() || bytes[i] == b'_') {
                i += 1;
            }
            out.push(CodeSpan::new(
                line[start..i].to_string(),
                CodeTokenKind::Macro,
            ));
            continue;
        }
        if ch == b'"' || ch == b'\'' || ch == b'`' {
            let quote: u8 = ch;
            let start: usize = i;
            i += 1;
            while i < len {
                if bytes[i] == b'\\' && i + 1 < len {
                    i += 2;
                    continue;
                }
                if bytes[i] == quote {
                    i += 1;
                    break;
                }
                i += 1;
            }
            let slice: &str = &line[start..i];
            let mut look: usize = i;
            while look < len && bytes[look] == b' ' {
                look += 1;
            }
            let is_json_key: bool = lang == CODE_LANG_JSON && look < len && bytes[look] == b':';
            let kind: CodeTokenKind = if is_json_key {
                CodeTokenKind::Property
            } else {
                CodeTokenKind::String
            };
            out.push(CodeSpan::new(slice.to_string(), kind));
            continue;
        }
        if ch.is_ascii_digit() {
            let start: usize = i;
            i += 1;
            while i < len
                && (bytes[i].is_ascii_alphanumeric() || bytes[i] == b'.' || bytes[i] == b'_')
            {
                i += 1;
            }
            out.push(CodeSpan::new(
                line[start..i].to_string(),
                CodeTokenKind::Number,
            ));
            continue;
        }
        if ch.is_ascii_alphabetic() || ch == b'_' {
            let start: usize = i;
            i += 1;
            while i < len && (bytes[i].is_ascii_alphanumeric() || bytes[i] == b'_') {
                i += 1;
            }
            if i < len && bytes[i] == b'!' {
                i += 1;
                out.push(CodeSpan::new(
                    line[start..i].to_string(),
                    CodeTokenKind::Macro,
                ));
                continue;
            }
            let word: &str = &line[start..i];
            let next_non_ws: Option<u8> = line[i..].bytes().find(|b: &u8| !b.is_ascii_whitespace());
            let prev_non_ws: Option<u8> = line[..start]
                .bytes()
                .rev()
                .find(|b: &u8| !b.is_ascii_whitespace());
            let kind: CodeTokenKind = if CODE_KEYWORDS.contains(&word) {
                CodeTokenKind::Keyword
            } else if CODE_CONTROL_KEYWORDS.contains(&word) {
                CodeTokenKind::Control
            } else if CODE_CONSTANTS.contains(&word) {
                CodeTokenKind::Constant
            } else if CODE_PRIMITIVE_TYPES.contains(&word) {
                CodeTokenKind::PrimitiveType
            } else if CODE_RESULT_TYPES.contains(&word) {
                CodeTokenKind::Property
            } else if CODE_TYPES.contains(&word)
                || word
                    .chars()
                    .next()
                    .map(|c: char| c.is_ascii_uppercase())
                    .unwrap_or(false)
            {
                CodeTokenKind::Type
            } else if next_non_ws == Some(b'(') {
                CodeTokenKind::Function
            } else if prev_non_ws == Some(b'.')
                || (next_non_ws == Some(b':') && (lang == CODE_LANG_TOML || lang == CODE_LANG_YAML))
            {
                CodeTokenKind::Property
            } else {
                CodeTokenKind::Plain
            };
            out.push(CodeSpan::new(word.to_string(), kind));
            continue;
        }
        if ch.is_ascii_whitespace() {
            let start: usize = i;
            while i < len && bytes[i].is_ascii_whitespace() {
                i += 1;
            }
            out.push(CodeSpan::new(
                line[start..i].to_string(),
                CodeTokenKind::Plain,
            ));
            continue;
        }
        let start: usize = i;
        let ch_len: usize = line[i..].chars().next().map(char::len_utf8).unwrap_or(1);
        i += ch_len;
        let sym: &str = &line[start..i];
        let kind: CodeTokenKind = if matches!(
            sym,
            "(" | ")" | "{" | "}" | "[" | "]" | "," | ";" | "." | ":"
        ) {
            CodeTokenKind::Punctuation
        } else if matches!(
            sym,
            "=" | "+" | "-" | "*" | "/" | ">" | "<" | "!" | "&" | "|" | "^" | "%"
        ) {
            CodeTokenKind::Operator
        } else {
            CodeTokenKind::Plain
        };
        out.push(CodeSpan::new(sym.to_string(), kind));
    }
    out
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
    let hashes: usize = trimmed.bytes().take_while(|b: &u8| *b == b'#').count();
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
    let leading_spaces: usize = raw.bytes().take_while(|b: &u8| *b == b' ').count();
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
