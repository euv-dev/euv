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

/// Measures the natural horizontal width and mounts syntax-highlighted line rows for `EuvCodeBlock`.
///
/// # Arguments
///
/// - `&[String]` - Raw code lines.
/// - `CodeLang` - Pre-resolved syntax highlighting language.
/// - `CodeThemePalette` - Resolved Eva Light/Dark Bold palette.
/// - `bool` - Whether the line-number gutter is visible.
/// - `&[usize]` - 1-based highlighted line numbers.
/// - `f32` - Monospace font size in logical pixels.
/// - `bool` - Whether code text is selectable.
///
/// # Returns
///
/// - `(f32, Vec<NodeId>)` - Required natural content width in logical pixels and mounted line row handles.
pub(crate) fn build_code_line_nodes(
    lines_vec: &[String],
    code_lang: CodeLang,
    eva: CodeThemePalette,
    show_nums: bool,
    highlight_lines: &[usize],
    code_font_size: f32,
    is_selectable: bool,
) -> (f32, Vec<NodeId>) {
    let mono_cell_w: f32 = code_font_size * MONOSPACE_ADVANCE_RATIO;
    let gutter_w: f32 = if show_nums { CODE_GUTTER_WIDTH } else { 0.0 };
    let mut max_cols: usize = 0;
    let line_nodes: Vec<NodeId> = lines_vec
        .iter()
        .enumerate()
        .map(|(idx, line_text): (usize, &String)| {
            let line_no: usize = idx + 1;
            let is_hl: bool = highlight_lines.contains(&line_no);
            let expanded_line: String = line_text.replace('\t', "  ");
            let line_cols: usize = expanded_line
                .chars()
                .map(|c: char| if c.is_ascii() { 1 } else { 2 })
                .sum();
            if line_cols > max_cols {
                max_cols = line_cols;
            }
            let leading_spaces: usize = expanded_line
                .bytes()
                .take_while(|b: &u8| *b == CODE_BYTE_SPACE)
                .count();
            let trimmed_line: &str = &expanded_line[leading_spaces..];
            let indent_px: f32 = (leading_spaces as f32) * mono_cell_w;
            let spans: Vec<CodeSpan> = tokenize_code_line_with_lang(trimmed_line, code_lang);
            let mut merged: Vec<(String, Color, FontWeight, bool)> =
                Vec::with_capacity(spans.len());
            for sp in spans.into_iter() {
                let kind: CodeTokenKind = sp.get_kind();
                let (fg, weight): (Color, FontWeight) = eva.token_style(kind);
                let is_italic: bool = kind == CodeTokenKind::Comment;
                if let Some(last) = merged.last_mut()
                    && last.1 == fg
                    && last.2 == weight
                    && last.3 == is_italic
                {
                    last.0.push_str(sp.get_text());
                } else {
                    merged.push((sp.get_text().clone(), fg, weight, is_italic));
                }
            }
            let span_nodes: Vec<NodeId> = merged
                .into_iter()
                .map(
                    |(txt, fg, weight, is_italic): (String, Color, FontWeight, bool)| {
                        view! {
                            text {
                                flex_shrink: 0.0
                                font_size: code_font_size
                                font_family: FontFamily::Monospace
                                font_weight: weight
                                italic: is_italic
                                color: fg
                                selectable: is_selectable
                                text_overflow: TextOverflow::Clip
                                { txt }
                            }
                        }
                    },
                )
                .collect();
            let row_bg: Color = if is_hl {
                eva.get_line_highlight()
            } else {
                COLOR_TRANSPARENT
            };
            let line_num_col: Color = if is_hl {
                eva.get_foreground()
            } else {
                eva.get_line_number()
            };
            view! {
                row {
                    min_width: Length::Percent(100.0)
                    min_height: Length::Px((code_font_size * 1.62).max(20.0))
                    padding: Edges::symmetric(1.0, SPACE_SM)
                    align: AlignItems::Center
                    background: row_bg
                    if show_nums {
                        row {
                            width: Length::Px(CODE_GUTTER_WIDTH)
                            flex_shrink: 0.0
                            padding: Edges::new(0.0, SPACE_SM, 0.0, 0.0)
                            justify: JustifyContent::End
                            align: AlignItems::Center
                            selectable: false
                            text {
                                font_size: (code_font_size - 1.0).max(10.5)
                                font_family: FontFamily::Monospace
                                color: line_num_col
                                selectable: false
                                text_overflow: TextOverflow::Clip
                                { format!("{line_no}") }
                            }
                        }
                    }
                    row {
                        flex_shrink: 0.0
                        padding: Edges::new(0.0, SPACE_MD, 0.0, indent_px)
                        align: AlignItems::Center
                        selectable: is_selectable
                        ..span_nodes
                    }
                }
            }
        })
        .collect();
    let natural_w: f32 = gutter_w + (max_cols as f32) * mono_cell_w + SPACE_MD * 2.0 + 24.0;
    (natural_w, line_nodes)
}

/// Draws the crisp four-corner vector expand icon used on `EuvCodeBlock` Fullscreen buttons.
///
/// # Arguments
///
/// - `Rect` - Logical icon canvas bounds.
/// - `&mut DisplayList` - Active draw command list.
/// - `Color` - Stroke color.
pub(crate) fn draw_code_fullscreen_icon(bounds: Rect, dl: &mut DisplayList, col: Color) {
    let x0: f32 = bounds.get_x() + 1.5;
    let y0: f32 = bounds.get_y() + 1.5;
    let x1: f32 = bounds.right() - 1.5;
    let y1: f32 = bounds.bottom() - 1.5;
    let arm: f32 = 4.0;
    for (a, b) in [
        (Point::new(x0, y0 + arm), Point::new(x0, y0)),
        (Point::new(x0, y0), Point::new(x0 + arm, y0)),
        (Point::new(x1 - arm, y0), Point::new(x1, y0)),
        (Point::new(x1, y0), Point::new(x1, y0 + arm)),
        (Point::new(x0, y1 - arm), Point::new(x0, y1)),
        (Point::new(x0, y1), Point::new(x0 + arm, y1)),
        (Point::new(x1 - arm, y1), Point::new(x1, y1)),
        (Point::new(x1, y1), Point::new(x1, y1 - arm)),
    ]
    .into_iter()
    {
        dl.push(DrawCommand::Line {
            from: a,
            to: b,
            color: col,
            width: 1.4,
        });
    }
}

/// Resolves an emoji shortcode (without surrounding `:`) into its Unicode emoji glyph.
///
/// # Arguments
///
/// - `&str` - Shortcode identifier (e.g. `"rocket"`, `"sparkles"`).
///
/// # Returns
///
/// - `Option<&'static str>` - Matching emoji glyph when recognized.
#[must_use]
pub fn resolve_emoji_shortcode(code: &str) -> Option<&'static str> {
    for &(key, glyph) in MD_EMOJI_TABLE {
        if key.eq_ignore_ascii_case(code) {
            return Some(glyph);
        }
    }
    None
}

/// Maps a single ASCII character to its Unicode superscript equivalent when available.
///
/// # Arguments
///
/// - `char` - Base character.
///
/// # Returns
///
/// - `Option<char>` - Unicode superscript glyph.
fn to_unicode_sup(ch: char) -> Option<char> {
    match ch {
        '0' => Some('⁰'),
        '1' => Some('¹'),
        '2' => Some('²'),
        '3' => Some('³'),
        '4' => Some('⁴'),
        '5' => Some('⁵'),
        '6' => Some('⁶'),
        '7' => Some('⁷'),
        '8' => Some('⁸'),
        '9' => Some('⁹'),
        '+' => Some('⁺'),
        '-' => Some('⁻'),
        '=' => Some('⁼'),
        '(' => Some('⁽'),
        ')' => Some('⁾'),
        'n' => Some('ⁿ'),
        'i' => Some('ⁱ'),
        _ => None,
    }
}

/// Maps a single ASCII character to its Unicode subscript equivalent when available.
///
/// # Arguments
///
/// - `char` - Base character.
///
/// # Returns
///
/// - `Option<char>` - Unicode subscript glyph.
fn to_unicode_sub(ch: char) -> Option<char> {
    match ch {
        '0' => Some('₀'),
        '1' => Some('₁'),
        '2' => Some('₂'),
        '3' => Some('₃'),
        '4' => Some('₄'),
        '5' => Some('₅'),
        '6' => Some('₆'),
        '7' => Some('₇'),
        '8' => Some('₈'),
        '9' => Some('₉'),
        '+' => Some('₊'),
        '-' => Some('₋'),
        '=' => Some('₌'),
        '(' => Some('₍'),
        ')' => Some('₎'),
        'a' => Some('ₐ'),
        'e' => Some('ₑ'),
        'o' => Some('ₒ'),
        'x' => Some('ₓ'),
        'i' => Some('ᵢ'),
        'n' => Some('ₙ'),
        'k' => Some('ₖ'),
        _ => None,
    }
}

/// Reads a brace-delimited LaTeX argument `{...}` (supporting nested braces) or single token from `rest`.
///
/// # Arguments
///
/// - `&str` - Remaining LaTeX source slice.
///
/// # Returns
///
/// - `(String, &str)` - Extracted argument body and remaining source slice.
fn read_latex_arg(rest: &str) -> (String, &str) {
    let trimmed: &str = rest.trim_start();
    if let Some(after_open) = trimmed.strip_prefix('{') {
        let mut depth: usize = 1;
        let mut end_idx: usize = after_open.len();
        for (i, ch) in after_open.char_indices() {
            if ch == '{' {
                depth += 1;
            } else if ch == '}' {
                depth -= 1;
                if depth == 0 {
                    end_idx = i;
                    break;
                }
            }
        }
        let body: String = after_open[..end_idx].to_string();
        let next_start: usize = (end_idx + 1).min(after_open.len());
        return (body, &after_open[next_start..]);
    }
    if let Some(ch) = trimmed.chars().next() {
        return (ch.to_string(), &trimmed[ch.len_utf8()..]);
    }
    (String::new(), trimmed)
}

/// Formats a LaTeX math expression into rich Unicode mathematical typography.
///
/// # Arguments
///
/// - `&str` - Raw LaTeX expression (without surrounding `$` or `$$`).
///
/// # Returns
///
/// - `String` - Formatted Unicode mathematical string.
#[must_use]
pub fn format_latex_math(raw: &str) -> String {
    let mut out: String = String::with_capacity(raw.len());
    let mut rest: &str = raw.trim();
    while !rest.is_empty() {
        if let Some(after_bs) = rest.strip_prefix('\\') {
            let cmd_len: usize = after_bs
                .bytes()
                .take_while(|b: &u8| b.is_ascii_alphabetic())
                .count();
            if cmd_len == 0 {
                if let Some(ch) = after_bs.chars().next() {
                    if ch == ',' || ch == ';' || ch == ' ' {
                        out.push(' ');
                    } else if ch != '!' {
                        out.push(ch);
                    }
                    rest = &after_bs[ch.len_utf8()..];
                } else {
                    break;
                }
                continue;
            }
            let cmd: &str = &after_bs[..cmd_len];
            rest = &after_bs[cmd_len..];
            if cmd == MD_LATEX_FRAC {
                let (num_raw, r1): (String, &str) = read_latex_arg(rest);
                let (den_raw, r2): (String, &str) = read_latex_arg(r1);
                let num: String = format_latex_math(&num_raw);
                let den: String = format_latex_math(&den_raw);
                if num.chars().count() <= 2 && den.chars().count() <= 2 {
                    out.push_str(&format!("{num}/{den}"));
                } else {
                    out.push_str(&format!("({num}) / ({den})"));
                }
                rest = r2;
                continue;
            }
            if cmd == MD_LATEX_SQRT {
                let (arg_raw, r1): (String, &str) = read_latex_arg(rest);
                let inner: String = format_latex_math(&arg_raw);
                if inner.chars().count() <= 1 {
                    out.push('√');
                    out.push_str(&inner);
                } else {
                    out.push_str(&format!("√({inner})"));
                }
                rest = r1;
                continue;
            }
            if cmd == MD_LATEX_TEXT
                || cmd == MD_LATEX_MATHRM
                || cmd == MD_LATEX_MATHBF
                || cmd == MD_LATEX_MATHIT
            {
                let (arg_raw, r1): (String, &str) = read_latex_arg(rest);
                out.push_str(&arg_raw);
                rest = r1;
                continue;
            }
            if cmd == MD_LATEX_LEFT || cmd == MD_LATEX_RIGHT {
                continue;
            }
            let mut found: Option<&'static str> = None;
            for &(sym_k, sym_v) in MD_LATEX_SYMBOLS {
                if sym_k == cmd {
                    found = Some(sym_v);
                    break;
                }
            }
            if let Some(glyph) = found {
                out.push_str(glyph);
            } else {
                out.push_str(cmd);
            }
            continue;
        }
        if let Some(after_caret) = rest.strip_prefix('^') {
            let (arg_raw, r1): (String, &str) = read_latex_arg(after_caret);
            let inner: String = format_latex_math(&arg_raw);
            let mapped: Option<String> = inner.chars().map(to_unicode_sup).collect();
            if let Some(sup_s) = mapped {
                out.push_str(&sup_s);
            } else {
                out.push_str(&format!("^({inner})"));
            }
            rest = r1;
            continue;
        }
        if let Some(after_sub) = rest.strip_prefix('_') {
            let (arg_raw, r1): (String, &str) = read_latex_arg(after_sub);
            let inner: String = format_latex_math(&arg_raw);
            let mapped: Option<String> = inner.chars().map(to_unicode_sub).collect();
            if let Some(sub_s) = mapped {
                out.push_str(&sub_s);
            } else {
                out.push_str(&format!("_({inner})"));
            }
            rest = r1;
            continue;
        }
        if let Some(ch) = rest.chars().next() {
            if ch != '{' && ch != '}' {
                out.push(ch);
            }
            rest = &rest[ch.len_utf8()..];
        } else {
            break;
        }
    }
    out
}

/// Parses rich inline Markdown formatting using a pre-scanned reference link table.
///
/// # Arguments
///
/// - `&str` - Raw single-line or paragraph Markdown string.
/// - `&HashMap<String, String>` - Lowercase link-reference URL map.
///
/// # Returns
///
/// - `Vec<EuvMarkdownInline>` - Ordered inline spans.
#[must_use]
pub(crate) fn parse_markdown_inlines_with_refs(
    text: &str,
    refs: &HashMap<String, String>,
) -> Vec<EuvMarkdownInline> {
    let mut out: Vec<EuvMarkdownInline> = Vec::new();
    let mut rest: &str = text;
    let mut plain_buf: String = String::new();
    while !rest.is_empty() {
        if let Some(after_bs) = rest.strip_prefix('\\')
            && let Some(esc_ch) = after_bs.chars().next()
            && matches!(
                esc_ch,
                '\\' | '`'
                    | '*'
                    | '_'
                    | '{'
                    | '}'
                    | '['
                    | ']'
                    | '('
                    | ')'
                    | '#'
                    | '+'
                    | '-'
                    | '.'
                    | '!'
                    | '~'
                    | '^'
                    | '='
                    | '$'
                    | ':'
                    | '<'
                    | '>'
            )
        {
            plain_buf.push(esc_ch);
            rest = &after_bs[esc_ch.len_utf8()..];
            continue;
        }
        if let Some(after_nl) = rest.strip_prefix('\n') {
            flush_plain(&mut plain_buf, &mut out);
            out.push(EuvMarkdownInline::HardBreak);
            rest = after_nl;
            continue;
        }
        if let Some(after) = rest.strip_prefix("``")
            && let Some(end) = after.find("``")
        {
            flush_plain(&mut plain_buf, &mut out);
            out.push(EuvMarkdownInline::Code(after[..end].trim().to_string()));
            rest = &after[end + 2..];
            continue;
        }
        if let Some(after) = rest.strip_prefix('`')
            && let Some(end) = after.find('`')
        {
            flush_plain(&mut plain_buf, &mut out);
            out.push(EuvMarkdownInline::Code(after[..end].to_string()));
            rest = &after[end + 1..];
            continue;
        }
        if let Some(after) = rest.strip_prefix('$')
            && !after.starts_with('$')
            && !after.starts_with(' ')
            && let Some(end) = after.find('$')
            && end > 0
            && !after[..end].ends_with(' ')
        {
            flush_plain(&mut plain_buf, &mut out);
            out.push(EuvMarkdownInline::Math(format_latex_math(&after[..end])));
            rest = &after[end + 1..];
            continue;
        }
        if let Some(after) = rest.strip_prefix("***")
            && let Some(end) = after.find("***")
            && end > 0
        {
            flush_plain(&mut plain_buf, &mut out);
            out.push(EuvMarkdownInline::BoldItalic(after[..end].to_string()));
            rest = &after[end + 3..];
            continue;
        }
        if let Some(after) = rest.strip_prefix("**")
            && let Some(end) = after.find("**")
            && end > 0
        {
            flush_plain(&mut plain_buf, &mut out);
            out.push(EuvMarkdownInline::Bold(after[..end].to_string()));
            rest = &after[end + 2..];
            continue;
        }
        if let Some(after) = rest.strip_prefix("__")
            && let Some(end) = after.find("__")
            && end > 0
        {
            flush_plain(&mut plain_buf, &mut out);
            out.push(EuvMarkdownInline::Bold(after[..end].to_string()));
            rest = &after[end + 2..];
            continue;
        }
        if let Some(after) = rest.strip_prefix("~~")
            && let Some(end) = after.find("~~")
            && end > 0
        {
            flush_plain(&mut plain_buf, &mut out);
            out.push(EuvMarkdownInline::Strike(after[..end].to_string()));
            rest = &after[end + 2..];
            continue;
        }
        if let Some(after) = rest.strip_prefix("==")
            && let Some(end) = after.find("==")
            && end > 0
        {
            flush_plain(&mut plain_buf, &mut out);
            out.push(EuvMarkdownInline::Highlight(after[..end].to_string()));
            rest = &after[end + 2..];
            continue;
        }
        if let Some(after) = rest.strip_prefix(MD_MARK_OPEN)
            && let Some(end) = after.find(MD_MARK_CLOSE)
        {
            flush_plain(&mut plain_buf, &mut out);
            out.push(EuvMarkdownInline::Highlight(after[..end].to_string()));
            rest = &after[end + MD_MARK_CLOSE.len()..];
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
        if let Some(after) = rest.strip_prefix(MD_SUP_OPEN)
            && let Some(end) = after.find(MD_SUP_CLOSE)
        {
            flush_plain(&mut plain_buf, &mut out);
            out.push(EuvMarkdownInline::Superscript(after[..end].to_string()));
            rest = &after[end + MD_SUP_CLOSE.len()..];
            continue;
        }
        if let Some(after) = rest.strip_prefix(MD_SUB_OPEN)
            && let Some(end) = after.find(MD_SUB_CLOSE)
        {
            flush_plain(&mut plain_buf, &mut out);
            out.push(EuvMarkdownInline::Subscript(after[..end].to_string()));
            rest = &after[end + MD_SUB_CLOSE.len()..];
            continue;
        }
        if let Some(after) = rest.strip_prefix('^')
            && !after.starts_with(' ')
            && let Some(end) = after.find('^')
            && end > 0
            && !after[..end].contains(' ')
        {
            flush_plain(&mut plain_buf, &mut out);
            out.push(EuvMarkdownInline::Superscript(after[..end].to_string()));
            rest = &after[end + 1..];
            continue;
        }
        if let Some(after) = rest.strip_prefix('~')
            && !after.starts_with('~')
            && !after.starts_with(' ')
            && let Some(end) = after.find('~')
            && end > 0
            && !after[..end].contains(' ')
        {
            flush_plain(&mut plain_buf, &mut out);
            out.push(EuvMarkdownInline::Subscript(after[..end].to_string()));
            rest = &after[end + 1..];
            continue;
        }
        if let Some(after) = rest.strip_prefix('*')
            && !after.starts_with('*')
            && !after.starts_with(' ')
            && let Some(end) = after.find('*')
            && end > 0
        {
            flush_plain(&mut plain_buf, &mut out);
            out.push(EuvMarkdownInline::Italic(after[..end].to_string()));
            rest = &after[end + 1..];
            continue;
        }
        if let Some(after) = rest.strip_prefix('_')
            && !after.starts_with('_')
            && !after.starts_with(' ')
            && plain_buf
                .chars()
                .last()
                .map(|c: char| !c.is_alphanumeric())
                .unwrap_or(true)
            && let Some(end) = after.find('_')
            && end > 0
            && after[end + 1..]
                .chars()
                .next()
                .map(|c: char| !c.is_alphanumeric())
                .unwrap_or(true)
        {
            flush_plain(&mut plain_buf, &mut out);
            out.push(EuvMarkdownInline::Italic(after[..end].to_string()));
            rest = &after[end + 1..];
            continue;
        }
        if let Some(after_bang) = rest.strip_prefix("![")
            && let Some(rb) = after_bang.find("](")
            && let Some(rp) = after_bang[rb + 2..].find(')')
        {
            flush_plain(&mut plain_buf, &mut out);
            out.push(EuvMarkdownInline::Image {
                alt: after_bang[..rb].to_string(),
                url: after_bang[rb + 2..rb + 2 + rp].to_string(),
            });
            rest = &after_bang[rb + 2 + rp + 1..];
            continue;
        }
        if let Some(after_fn) = rest.strip_prefix("[^")
            && let Some(rb) = after_fn.find(']')
            && rb > 0
            && !after_fn[..rb].contains(' ')
        {
            flush_plain(&mut plain_buf, &mut out);
            out.push(EuvMarkdownInline::FootnoteRef(after_fn[..rb].to_string()));
            rest = &after_fn[rb + 1..];
            continue;
        }
        if let Some(after_lb) = rest.strip_prefix('[') {
            if let Some(rb) = after_lb.find("](")
                && !after_lb[..rb].contains(']')
                && let Some(rp) = after_lb[rb + 2..].find(')')
            {
                flush_plain(&mut plain_buf, &mut out);
                let label: String = after_lb[..rb].to_string();
                let url: String = after_lb[rb + 2..rb + 2 + rp].to_string();
                out.push(EuvMarkdownInline::Link { label, url });
                rest = &after_lb[rb + 2 + rp + 1..];
                continue;
            }
            if let Some(rb) = after_lb.find("][")
                && !after_lb[..rb].contains(']')
                && let Some(rb2) = after_lb[rb + 2..].find(']')
            {
                let label: &str = &after_lb[..rb];
                let raw_key: &str = &after_lb[rb + 2..rb + 2 + rb2];
                let lookup: String = if raw_key.is_empty() {
                    label.to_ascii_lowercase()
                } else {
                    raw_key.to_ascii_lowercase()
                };
                if let Some(url) = refs.get(&lookup) {
                    flush_plain(&mut plain_buf, &mut out);
                    out.push(EuvMarkdownInline::Link {
                        label: label.to_string(),
                        url: url.clone(),
                    });
                    rest = &after_lb[rb + 2 + rb2 + 1..];
                    continue;
                }
            }
        }
        if let Some(after_lt) = rest.strip_prefix('<')
            && let Some(gt) = after_lt.find('>')
        {
            let inner: &str = &after_lt[..gt];
            if inner.starts_with(MD_SCHEME_HTTPS)
                || inner.starts_with(MD_SCHEME_HTTP)
                || inner.starts_with(MD_SCHEME_MAILTO)
            {
                flush_plain(&mut plain_buf, &mut out);
                let label: String = inner
                    .strip_prefix(MD_SCHEME_MAILTO)
                    .unwrap_or(inner)
                    .to_string();
                out.push(EuvMarkdownInline::Link {
                    label,
                    url: inner.to_string(),
                });
                rest = &after_lt[gt + 1..];
                continue;
            }
        }
        if let Some(after_col) = rest.strip_prefix(':')
            && let Some(end) = after_col.find(':')
            && end > 0
            && end <= 24
            && after_col[..end]
                .bytes()
                .all(|b: u8| b.is_ascii_alphanumeric() || b == b'_' || b == b'+' || b == b'-')
            && let Some(glyph) = resolve_emoji_shortcode(&after_col[..end])
        {
            plain_buf.push_str(glyph);
            rest = &after_col[end + 1..];
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

/// Parses rich inline Markdown formatting (`**bold**`, `*italic*`, `***bold italic***`, `~~strike~~`, `==highlight==`, `^sup^`, `~sub~`, `` `code` ``, `$math$`, `[^fn]`, `[links](url)`, `<url>`, `![img](url)`, `<kbd>Key</kbd>`, and `:emoji:` shortcodes).
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
    let empty_refs: HashMap<String, String> = HashMap::new();
    parse_markdown_inlines_with_refs(text, &empty_refs)
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

/// Parses a link reference definition line (`[key]: https://...`).
///
/// # Arguments
///
/// - `&str` - Trimmed input line.
///
/// # Returns
///
/// - `Option<(String, String)>` - Lowercase reference key and resolved URL.
fn parse_link_ref_def(trimmed: &str) -> Option<(String, String)> {
    let after: &str = trimmed.strip_prefix('[')?;
    if after.starts_with('^') {
        return None;
    }
    let split: usize = after.find("]:")?;
    let key: &str = after[..split].trim();
    if key.is_empty() || key.contains(']') {
        return None;
    }
    let val_raw: &str = after[split + 2..].trim();
    if val_raw.is_empty() {
        return None;
    }
    let url_tok: &str = val_raw.split_whitespace().next()?;
    let clean_url: &str = url_tok
        .strip_prefix('<')
        .and_then(|s: &str| s.strip_suffix('>'))
        .unwrap_or(url_tok);
    Some((key.to_ascii_lowercase(), clean_url.to_string()))
}

/// Parses a footnote definition header line (`[^id]: text`).
///
/// # Arguments
///
/// - `&str` - Trimmed input line.
///
/// # Returns
///
/// - `Option<(String, String)>` - Footnote identifier and initial content line.
fn parse_footnote_def(trimmed: &str) -> Option<(String, String)> {
    let after: &str = trimmed.strip_prefix("[^")?;
    let split: usize = after.find("]:")?;
    let id: &str = after[..split].trim();
    if id.is_empty() || id.contains(' ') {
        return None;
    }
    let body: &str = after[split + 2..].trim();
    Some((id.to_string(), body.to_string()))
}

/// Checks whether a trimmed line opens a fenced code block (`` ``` `` or `~~~`).
///
/// # Arguments
///
/// - `&str` - Trimmed input line.
///
/// # Returns
///
/// - `Option<(u8, usize, String)>` - `(fence_byte, fence_len, language_tag)`.
fn parse_code_fence_open(trimmed: &str) -> Option<(u8, usize, String)> {
    let first: u8 = *trimmed.as_bytes().first()?;
    if first != CODE_BYTE_BACKTICK && first != CODE_BYTE_TILDE {
        return None;
    }
    let count: usize = trimmed.bytes().take_while(|b: &u8| *b == first).count();
    if count < 3 {
        return None;
    }
    let info: &str = trimmed[count..].trim();
    if first == CODE_BYTE_BACKTICK && info.contains('`') {
        return None;
    }
    let lang: String = info.split_whitespace().next().unwrap_or("").to_string();
    Some((first, count, lang))
}

/// Checks whether a trimmed line closes a fenced code block opened with `(fence_byte, min_len)`.
///
/// # Arguments
///
/// - `&str` - Trimmed input line.
/// - `u8` - Fence marker byte.
/// - `usize` - Minimum fence length.
///
/// # Returns
///
/// - `bool` - `true` when the line closes the fence.
fn is_code_fence_close(trimmed: &str, fence_byte: u8, min_len: usize) -> bool {
    let count: usize = trimmed
        .bytes()
        .take_while(|b: &u8| *b == fence_byte)
        .count();
    count >= min_len && trimmed[count..].trim().is_empty()
}

/// Checks whether a trimmed line is a thematic horizontal rule (`---`, `***`, `___`).
///
/// # Arguments
///
/// - `&str` - Trimmed input line.
///
/// # Returns
///
/// - `bool` - `true` when the line is a valid Markdown horizontal rule.
fn is_horizontal_rule(trimmed: &str) -> bool {
    let compact: String = trimmed
        .chars()
        .filter(|c: &char| !c.is_whitespace())
        .collect();
    if compact.len() < 3 {
        return false;
    }
    let first: char = compact.chars().next().unwrap_or(' ');
    matches!(first, '-' | '*' | '_') && compact.chars().all(|c: char| c == first)
}

/// Parses a VuePress/container callout opening line (`::: tip`, `::: warning`, etc.).
///
/// # Arguments
///
/// - `&str` - Trimmed input line.
///
/// # Returns
///
/// - `Option<EuvMarkdownAlertKind>` - Matching alert kind if this opens a container callout.
fn parse_container_callout_open(trimmed: &str) -> Option<EuvMarkdownAlertKind> {
    let after: &str = trimmed.strip_prefix(MD_CALLOUT_FENCE)?.trim();
    if after.is_empty() {
        return None;
    }
    let kw: &str = after.split_whitespace().next()?;
    if kw.eq_ignore_ascii_case(MD_CALLOUT_NOTE) {
        Some(EuvMarkdownAlertKind::Note)
    } else if kw.eq_ignore_ascii_case(MD_CALLOUT_TIP) {
        Some(EuvMarkdownAlertKind::Tip)
    } else if kw.eq_ignore_ascii_case(MD_CALLOUT_IMPORTANT) {
        Some(EuvMarkdownAlertKind::Important)
    } else if kw.eq_ignore_ascii_case(MD_CALLOUT_WARNING) {
        Some(EuvMarkdownAlertKind::Warning)
    } else if kw.eq_ignore_ascii_case(MD_CALLOUT_DANGER)
        || kw.eq_ignore_ascii_case(MD_CALLOUT_CAUTION)
    {
        Some(EuvMarkdownAlertKind::Caution)
    } else {
        Some(EuvMarkdownAlertKind::Note)
    }
}

/// Parses a complete GitHub-Flavored & Extended Markdown document into structured `EuvMarkdownBlock` nodes.
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
    let raw_lines: Vec<&str> = source.lines().collect();
    let mut refs: HashMap<String, String> = HashMap::new();
    let mut footnotes: Vec<EuvMarkdownFootnote> = Vec::new();
    let mut body_lines: Vec<&str> = Vec::with_capacity(raw_lines.len());
    let mut scan_idx: usize = 0;
    let mut in_fence: Option<(u8, usize)> = None;
    while scan_idx < raw_lines.len() {
        let line: &str = raw_lines[scan_idx];
        let trimmed: &str = line.trim();
        if let Some((fb, fl)) = in_fence {
            if is_code_fence_close(trimmed, fb, fl) {
                in_fence = None;
            }
            body_lines.push(line);
            scan_idx += 1;
            continue;
        }
        if let Some((fb, fl, _)) = parse_code_fence_open(trimmed) {
            in_fence = Some((fb, fl));
            body_lines.push(line);
            scan_idx += 1;
            continue;
        }
        if let Some((fn_id, mut fn_body)) = parse_footnote_def(trimmed) {
            scan_idx += 1;
            while scan_idx < raw_lines.len()
                && (raw_lines[scan_idx].starts_with("  ") || raw_lines[scan_idx].starts_with('\t'))
            {
                let cont: &str = raw_lines[scan_idx].trim();
                if !cont.is_empty() {
                    if !fn_body.is_empty() {
                        fn_body.push(' ');
                    }
                    fn_body.push_str(cont);
                }
                scan_idx += 1;
            }
            footnotes.push(EuvMarkdownFootnote::new(fn_id, fn_body));
            continue;
        }
        if let Some((key, url)) = parse_link_ref_def(trimmed) {
            let _: Option<String> = refs.insert(key, url);
            scan_idx += 1;
            continue;
        }
        body_lines.push(line);
        scan_idx += 1;
    }
    let total: usize = body_lines.len();
    let mut idx: usize = 0;
    let mut blocks: Vec<EuvMarkdownBlock> = Vec::new();
    while idx < total {
        let raw: &str = body_lines[idx];
        let trimmed: &str = raw.trim();
        if trimmed.is_empty() {
            idx += 1;
            continue;
        }
        if let Some((fb, fl, lang)) = parse_code_fence_open(trimmed) {
            idx += 1;
            let mut code_lines: Vec<&str> = Vec::new();
            while idx < total && !is_code_fence_close(body_lines[idx].trim(), fb, fl) {
                code_lines.push(body_lines[idx]);
                idx += 1;
            }
            if idx < total {
                idx += 1;
            }
            let joined: String = code_lines.join("\n");
            if lang.eq_ignore_ascii_case(CODE_LANG_MERMAID) {
                blocks.push(EuvMarkdownBlock::Mermaid { source: joined });
            } else if lang.eq_ignore_ascii_case(CODE_LANG_MATH)
                || lang.eq_ignore_ascii_case(CODE_LANG_LATEX)
            {
                blocks.push(EuvMarkdownBlock::MathBlock(joined));
            } else {
                blocks.push(EuvMarkdownBlock::CodeBlock {
                    language: if lang.is_empty() {
                        CODE_DEFAULT_LANG.to_string()
                    } else {
                        lang
                    },
                    code: joined,
                });
            }
            continue;
        }
        if let Some(after_math) = trimmed.strip_prefix("$$") {
            if let Some(inline_body) = after_math.strip_suffix("$$")
                && !inline_body.trim().is_empty()
            {
                blocks.push(EuvMarkdownBlock::MathBlock(inline_body.trim().to_string()));
                idx += 1;
                continue;
            }
            idx += 1;
            let mut math_lines: Vec<&str> = Vec::new();
            let first_rem: &str = after_math.trim();
            if !first_rem.is_empty() {
                math_lines.push(first_rem);
            }
            while idx < total && !body_lines[idx].trim().ends_with("$$") {
                math_lines.push(body_lines[idx].trim());
                idx += 1;
            }
            if idx < total {
                let tail: &str = body_lines[idx]
                    .trim()
                    .strip_suffix("$$")
                    .unwrap_or("")
                    .trim();
                if !tail.is_empty() {
                    math_lines.push(tail);
                }
                idx += 1;
            }
            blocks.push(EuvMarkdownBlock::MathBlock(math_lines.join("\n")));
            continue;
        }
        if let Some(callout_kind) = parse_container_callout_open(trimmed) {
            idx += 1;
            let mut c_lines: Vec<String> = Vec::new();
            while idx < total && body_lines[idx].trim() != MD_CALLOUT_FENCE {
                let lt: &str = body_lines[idx].trim();
                if !lt.is_empty() {
                    c_lines.push(lt.to_string());
                }
                idx += 1;
            }
            if idx < total {
                idx += 1;
            }
            blocks.push(EuvMarkdownBlock::Quote {
                kind: callout_kind,
                depth: 1,
                lines: c_lines,
            });
            continue;
        }
        if trimmed.starts_with(MD_DETAILS_OPEN_TAG) {
            idx += 1;
            let mut summary: String = MD_DETAILS_DEFAULT_SUMMARY.to_string();
            let mut body: Vec<String> = Vec::new();
            while idx < total && !body_lines[idx].trim().starts_with(MD_DETAILS_CLOSE_TAG) {
                let lt: &str = body_lines[idx].trim();
                if let Some(after) = lt.strip_prefix(MD_SUMMARY_OPEN_TAG) {
                    summary = after
                        .strip_suffix(MD_SUMMARY_CLOSE_TAG)
                        .unwrap_or(after)
                        .trim()
                        .to_string();
                } else {
                    body.push(body_lines[idx].to_string());
                }
                idx += 1;
            }
            if idx < total {
                idx += 1;
            }
            blocks.push(EuvMarkdownBlock::Details { summary, body });
            continue;
        }
        if is_horizontal_rule(trimmed) {
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
            && after_bang[rb + 2 + rp + 1..].trim().is_empty()
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
            while idx < total && body_lines[idx].trim().starts_with('>') {
                let inner: &str = body_lines[idx]
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
                depth: 1,
                lines: clean_lines,
            });
            continue;
        }
        if trimmed.contains('|')
            && idx + 1 < total
            && is_table_separator(body_lines[idx + 1].trim())
        {
            let headers: Vec<String> = split_table_row(trimmed);
            let alignments: Vec<EuvMarkdownAlign> =
                parse_table_alignments(body_lines[idx + 1].trim());
            idx += 2;
            let mut rows: Vec<Vec<String>> = Vec::new();
            while idx < total {
                let row_t: &str = body_lines[idx].trim();
                if row_t.is_empty() || !row_t.contains('|') {
                    break;
                }
                rows.push(split_table_row(row_t));
                idx += 1;
            }
            blocks.push(EuvMarkdownBlock::Table {
                headers,
                alignments,
                rows,
            });
            continue;
        }
        if let Some((ordered, start_num, first_item)) = parse_list_line(raw) {
            let mut items: Vec<EuvMarkdownListItem> = vec![first_item];
            idx += 1;
            while idx < total {
                if let Some((item_ord, _, item)) = parse_list_line(body_lines[idx])
                    && (item_ord == ordered || item.get_depth() > 0)
                {
                    items.push(item);
                    idx += 1;
                } else {
                    break;
                }
            }
            blocks.push(EuvMarkdownBlock::List {
                ordered,
                start: start_num,
                items,
            });
            continue;
        }
        if idx + 1 < total && body_lines[idx + 1].trim_start().starts_with(": ") {
            let mut def_items: Vec<EuvMarkdownDefItem> = Vec::new();
            while idx + 1 < total && body_lines[idx + 1].trim_start().starts_with(": ") {
                let term: String = body_lines[idx].trim().to_string();
                idx += 1;
                let mut defs: Vec<String> = Vec::new();
                while idx < total {
                    let dt: &str = body_lines[idx].trim_start();
                    if let Some(after_def) = dt.strip_prefix(": ") {
                        defs.push(after_def.trim().to_string());
                        idx += 1;
                    } else {
                        break;
                    }
                }
                def_items.push(EuvMarkdownDefItem::new(term, defs));
                while idx < total && body_lines[idx].trim().is_empty() {
                    if idx + 2 < total && body_lines[idx + 2].trim_start().starts_with(": ") {
                        idx += 1;
                    } else {
                        break;
                    }
                }
            }
            blocks.push(EuvMarkdownBlock::DefinitionList(def_items));
            continue;
        }
        let mut para_buf: String = String::new();
        while idx < total {
            let cur_raw: &str = body_lines[idx];
            let cur: &str = cur_raw.trim();
            if cur.is_empty()
                || parse_code_fence_open(cur).is_some()
                || cur.starts_with("$$")
                || parse_container_callout_open(cur).is_some()
                || parse_heading_line(cur).is_some()
                || cur.starts_with('>')
                || (cur.contains('|')
                    && idx + 1 < total
                    && is_table_separator(body_lines[idx + 1].trim()))
                || (cur.starts_with("![") && cur.ends_with(')'))
                || cur.starts_with(MD_DETAILS_OPEN_TAG)
                || is_horizontal_rule(cur)
                || parse_list_line(cur_raw).is_some()
                || (idx + 1 < total && body_lines[idx + 1].trim_start().starts_with(": "))
            {
                break;
            }
            let hard_break: bool = cur_raw.ends_with("  ") || cur.ends_with('\\');
            let clean_seg: &str = if cur.ends_with('\\') {
                cur.strip_suffix('\\').unwrap_or(cur).trim_end()
            } else {
                cur
            };
            if !para_buf.is_empty() && !para_buf.ends_with('\n') {
                para_buf.push(' ');
            }
            para_buf.push_str(clean_seg);
            if hard_break {
                para_buf.push('\n');
            }
            idx += 1;
        }
        let trimmed_para: &str = para_buf.trim_end_matches('\n');
        blocks.push(EuvMarkdownBlock::Paragraph(
            parse_markdown_inlines_with_refs(trimmed_para, &refs),
        ));
    }
    if !footnotes.is_empty() {
        blocks.push(EuvMarkdownBlock::Footnotes(footnotes));
    }
    blocks
}

/// Parses an ATX heading line (`# Title` .. `###### Title`), stripping optional `{#custom-anchor}` suffixes.
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
    let no_closing: &str = rest.trim().trim_end_matches('#').trim();
    let clean: &str = if let Some(anchor_pos) = no_closing.rfind("{#")
        && no_closing.ends_with('}')
    {
        no_closing[..anchor_pos].trim_end()
    } else {
        no_closing
    };
    Some((hashes as u8, clean.to_string()))
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
pub(crate) fn classify_quote_alert(mut lines: Vec<String>) -> (EuvMarkdownAlertKind, Vec<String>) {
    let Some(first) = lines
        .first()
        .map(|s: &String| s.trim().to_ascii_uppercase())
    else {
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
    if !trimmed.contains('|') && !trimmed.contains('-') {
        return false;
    }
    let cells: Vec<String> = split_table_row(trimmed);
    !cells.is_empty()
        && cells.iter().all(|c: &String| {
            let t: &str = c.trim();
            !t.is_empty() && t.contains('-') && t.chars().all(|ch: char| ch == '-' || ch == ':')
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
    let without_lead: &str = trimmed.strip_prefix('|').unwrap_or(trimmed);
    let inner: &str = without_lead.strip_suffix('|').unwrap_or(without_lead);
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

/// Parses an unordered (`- `, `* `, `+ `), ordered (`1. `), or GFM task (`- [x] `) list line.
///
/// # Arguments
///
/// - `&str` - Raw input line (leading spaces determine nesting depth).
///
/// # Returns
///
/// - `Option<(bool, usize, EuvMarkdownListItem)>` - `(ordered, ordinal_start, parsed_item)` when the line is a list item.
fn parse_list_line(raw: &str) -> Option<(bool, usize, EuvMarkdownListItem)> {
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
            1,
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
        let start_num: usize = trimmed[..digit_len].parse::<usize>().unwrap_or(1);
        return Some((
            true,
            start_num,
            EuvMarkdownListItem::new(after_dot.trim().to_string(), depth, None),
        ));
    }
    None
}
