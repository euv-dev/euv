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

/// Scans a non-empty source-code line into contiguous `(start, end, CodeTokenKind)` byte ranges without heap allocation.
///
/// # Arguments
///
/// - `&str` - Non-empty source line to scan.
/// - `CodeLang` - Pre-resolved language hint.
/// - `F` - Visitor invoked with `(start, end, kind)` for each contiguous token range.
#[inline(always)]
fn scan_code_line_ranges<F>(line: &str, lang: CodeLang, mut emit: F)
where
    F: FnMut(usize, usize, CodeTokenKind),
{
    let len: usize = line.len();
    if lang == CodeLang::Diff {
        if line.starts_with('+') && !line.starts_with("+++") {
            emit(0, len, CodeTokenKind::DiffAdd);
            return;
        }
        if line.starts_with('-') && !line.starts_with("---") {
            emit(0, len, CodeTokenKind::DiffDel);
            return;
        }
    }
    let trimmed: &str = line.trim_start();
    if trimmed.starts_with("//")
        || trimmed.starts_with("--")
        || (trimmed.starts_with('#') && !trimmed.starts_with("#[") && !trimmed.starts_with("#!["))
    {
        emit(0, len, CodeTokenKind::Comment);
        return;
    }
    if trimmed.starts_with("#[") || trimmed.starts_with("#![") {
        emit(0, len, CodeTokenKind::Macro);
        return;
    }
    let bytes: &[u8] = line.as_bytes();
    let mut i: usize = 0;
    while i < len {
        let ch: u8 = bytes[i];
        if ch == CODE_BYTE_SLASH && i + 1 < len && bytes[i + 1] == CODE_BYTE_SLASH {
            emit(i, len, CodeTokenKind::Comment);
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
            emit(start, i, CodeTokenKind::Macro);
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
            emit(start, i, kind);
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
            emit(start, i, CodeTokenKind::Number);
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
                emit(start, i, CodeTokenKind::Macro);
                continue;
            }
            let word: &str = &line[start..i];
            let kind: CodeTokenKind = classify_code_ident(word, bytes, start, i, lang);
            emit(start, i, kind);
            continue;
        }
        if ch.is_ascii_whitespace() {
            let start: usize = i;
            i += 1;
            while i < len && bytes[i].is_ascii_whitespace() {
                i += 1;
            }
            emit(start, i, CodeTokenKind::Plain);
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
        emit(start, i, kind);
    }
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
    let mut out: Vec<CodeSpan> = Vec::with_capacity((line.len() / 8).clamp(2, 16));
    let mut pending: Option<(usize, usize, CodeTokenKind)> = None;
    scan_code_line_ranges(
        line,
        lang,
        |start: usize, end: usize, kind: CodeTokenKind| {
            push_merged_code_span(&mut out, &mut pending, line, start, end, kind);
        },
    );
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

/// Measures the natural horizontal width, line count, and mounts all syntax-highlighted line rows for `EuvCodeBlock` inside a single `SceneTree` write borrow.
///
/// # Arguments
///
/// - `&str` - Raw source code string.
/// - `CodeLang` - Pre-resolved syntax highlighting language.
/// - `CodeThemePalette` - Resolved Eva Light/Dark Bold palette.
/// - `bool` - Whether the line-number gutter is visible.
/// - `&[usize]` - 1-based highlighted line numbers.
/// - `f32` - Monospace font size in logical pixels.
/// - `bool` - Whether code text is selectable.
///
/// # Returns
///
/// - `(f32, usize, Vec<NodeId>)` - Required natural content width in logical pixels, total line count, and mounted line row handles.
pub(crate) fn build_code_line_nodes(
    code: &str,
    code_lang: CodeLang,
    eva: CodeThemePalette,
    show_nums: bool,
    highlight_lines: &[usize],
    code_font_size: f32,
    is_selectable: bool,
) -> (f32, usize, Vec<NodeId>) {
    let ctx: Context = current_context();
    let mono_cell_w: f32 = code_font_size * MONOSPACE_ADVANCE_RATIO;
    let gutter_w: f32 = if show_nums { CODE_GUTTER_WIDTH } else { 0.0 };
    let has_highlights: bool = !highlight_lines.is_empty();
    let base_span_style: Style = Style::default()
        .flex_shrink(0.0)
        .font_size(code_font_size)
        .font_family(FontFamily::Monospace)
        .selectable(is_selectable)
        .text_overflow(TextOverflow::Clip);
    let base_row_style: Style = Style::default()
        .row()
        .min_width(Length::Percent(100.0))
        .min_height(Length::Px((code_font_size * 1.62).max(20.0)))
        .padding(Edges::symmetric(1.0, SPACE_SM))
        .align(AlignItems::Center)
        .background(COLOR_TRANSPARENT);
    let gutter_row_style: Style = Style::default()
        .row()
        .width(Length::Px(CODE_GUTTER_WIDTH))
        .flex_shrink(0.0)
        .padding(Edges::new(0.0, SPACE_SM, 0.0, 0.0))
        .justify(JustifyContent::End)
        .align(AlignItems::Center)
        .selectable(false);
    let base_gutter_text_style: Style = Style::default()
        .font_size((code_font_size - 1.0).max(10.5))
        .font_family(FontFamily::Monospace)
        .color(eva.get_line_number())
        .selectable(false)
        .text_overflow(TextOverflow::Clip);
    let base_code_row_style: Style = Style::default()
        .row()
        .flex_shrink(0.0)
        .align(AlignItems::Center)
        .selectable(is_selectable);
    let plain_fg: Color = eva.get_foreground();
    let line_hl_bg: Color = eva.get_line_highlight();
    let est_lines: usize = (code.len() / 32).clamp(1, 256);
    let (max_cols, line_nodes): (usize, Vec<NodeId>) = ctx.get_scene().with_write(
        |tree: &mut SceneTree| {
            let mut max_cols: usize = 0;
            let mut line_nodes: Vec<NodeId> = Vec::with_capacity(est_lines);
            let mut span_nodes: Vec<NodeId> = Vec::with_capacity(16);
            let mut line_no: usize = 0;
            for line_text in code.lines().chain(code.is_empty().then_some("")) {
                line_no += 1;
                let is_hl: bool = has_highlights && highlight_lines.contains(&line_no);
                let expanded_opt: Option<String> = if line_text.as_bytes().contains(&b'\t') {
                    Some(line_text.replace('\t', "  "))
                } else {
                    None
                };
                let expanded_line: &str = expanded_opt.as_deref().unwrap_or(line_text);
                let line_cols: usize = if expanded_line.is_ascii() {
                    expanded_line.len()
                } else {
                    expanded_line
                        .chars()
                        .map(|c: char| if c.is_ascii() { 1 } else { 2 })
                        .sum()
                };
                if line_cols > max_cols {
                    max_cols = line_cols;
                }
                let leading_spaces: usize = expanded_line
                    .bytes()
                    .take_while(|b: &u8| *b == CODE_BYTE_SPACE)
                    .count();
                let trimmed_line: &str = &expanded_line[leading_spaces..];
                let indent_px: f32 = (leading_spaces as f32) * mono_cell_w;
                span_nodes.clear();
                if trimmed_line.is_empty() {
                    let mut sp_style: Style = base_span_style;
                    sp_style.get_mut_text().set_color(plain_fg);
                    span_nodes.push(tree.create_node(NodeKind::Text(" ".to_string()), sp_style));
                } else {
                    let mut pending: Option<(usize, usize, Color, FontWeight, bool)> = None;
                    scan_code_line_ranges(
                        trimmed_line,
                        code_lang,
                        |start: usize, end: usize, kind: CodeTokenKind| {
                            let (fg, weight): (Color, FontWeight) = eva.token_style(kind);
                            let is_italic: bool = kind == CodeTokenKind::Comment;
                            if let Some((p_start, p_end, p_fg, p_weight, p_italic)) = pending {
                                if p_fg == fg
                                    && p_weight == weight
                                    && p_italic == is_italic
                                    && p_end == start
                                {
                                    pending = Some((p_start, end, p_fg, p_weight, p_italic));
                                    return;
                                }
                                let mut sp_style: Style = base_span_style;
                                sp_style
                                    .get_mut_text()
                                    .set_weight(p_weight)
                                    .set_italic(p_italic)
                                    .set_color(p_fg);
                                span_nodes.push(tree.create_node(
                                    NodeKind::Text(trimmed_line[p_start..p_end].to_string()),
                                    sp_style,
                                ));
                            }
                            pending = Some((start, end, fg, weight, is_italic));
                        },
                    );
                    if let Some((p_start, p_end, p_fg, p_weight, p_italic)) = pending {
                        let mut sp_style: Style = base_span_style;
                        sp_style
                            .get_mut_text()
                            .set_weight(p_weight)
                            .set_italic(p_italic)
                            .set_color(p_fg);
                        span_nodes.push(tree.create_node(
                            NodeKind::Text(trimmed_line[p_start..p_end].to_string()),
                            sp_style,
                        ));
                    }
                }
                let mut row_style: Style = base_row_style;
                if is_hl {
                    row_style.get_mut_paint().set_background(Some(line_hl_bg));
                }
                let line_row: NodeId = tree.create_node(NodeKind::View, row_style);
                if show_nums {
                    let gutter_row: NodeId = tree.create_node(NodeKind::View, gutter_row_style);
                    let mut num_style: Style = base_gutter_text_style;
                    if is_hl {
                        num_style.get_mut_text().set_color(plain_fg);
                    }
                    let gutter_text: NodeId =
                        tree.create_node(NodeKind::Text(line_no.to_string()), num_style);
                    tree.append_child(gutter_row, gutter_text);
                    tree.append_child(line_row, gutter_row);
                }
                let mut code_row_style: Style = base_code_row_style;
                code_row_style
                    .get_mut_layout()
                    .set_padding(Edges::new(0.0, SPACE_MD, 0.0, indent_px));
                let code_row: NodeId = tree.create_node(NodeKind::View, code_row_style);
                tree.append_children(code_row, &span_nodes);
                tree.append_child(line_row, code_row);
                line_nodes.push(line_row);
            }
            (max_cols, line_nodes)
        },
        (0, Vec::new()),
    );
    let line_count: usize = line_nodes.len();
    let natural_w: f32 = gutter_w + (max_cols as f32) * mono_cell_w + SPACE_MD * 2.0 + 24.0;
    (natural_w, line_count, line_nodes)
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

/// Compares an ASCII-lowercase table key against a candidate slice case-insensitively without heap allocation.
///
/// # Arguments
///
/// - `&str` - ASCII-lowercase lookup table key.
/// - `&str` - Candidate input slice.
///
/// # Returns
///
/// - `Ordering` - Lexicographical ASCII case-insensitive ordering.
#[inline(always)]
fn cmp_lower_with_ascii_ignore_case(lower_key: &str, candidate: &str) -> Ordering {
    let kb: &[u8] = lower_key.as_bytes();
    let cb: &[u8] = candidate.as_bytes();
    let min_len: usize = kb.len().min(cb.len());
    let mut i: usize = 0;
    while i < min_len {
        let c: u8 = cb[i].to_ascii_lowercase();
        if kb[i] != c {
            return kb[i].cmp(&c);
        }
        i += 1;
    }
    kb.len().cmp(&cb.len())
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
    if code.is_empty() || code.len() > MD_EMOJI_MAX_LEN {
        return None;
    }
    MD_EMOJI_TABLE
        .binary_search_by(|&(key, _): &(&str, &str)| cmp_lower_with_ascii_ignore_case(key, code))
        .ok()
        .map(|idx: usize| MD_EMOJI_TABLE[idx].1)
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

/// Reads a brace-delimited LaTeX argument `{...}` (supporting nested braces) or single token from `rest` without heap allocation.
///
/// # Arguments
///
/// - `&str` - Remaining LaTeX source slice.
///
/// # Returns
///
/// - `(&str, &str)` - Borrowed argument body slice and remaining source slice.
#[inline(always)]
fn read_latex_arg(rest: &str) -> (&str, &str) {
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
        let body: &str = &after_open[..end_idx];
        let next_start: usize = (end_idx + 1).min(after_open.len());
        return (body, &after_open[next_start..]);
    }
    if let Some(ch) = trimmed.chars().next() {
        let ch_len: usize = ch.len_utf8();
        return (&trimmed[..ch_len], &trimmed[ch_len..]);
    }
    ("", trimmed)
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
                let (num_raw, r1): (&str, &str) = read_latex_arg(rest);
                let (den_raw, r2): (&str, &str) = read_latex_arg(r1);
                let num: String = format_latex_math(num_raw);
                let den: String = format_latex_math(den_raw);
                if num.chars().count() <= 2 && den.chars().count() <= 2 {
                    out.push_str(&num);
                    out.push('/');
                    out.push_str(&den);
                } else {
                    out.push('(');
                    out.push_str(&num);
                    out.push_str(") / (");
                    out.push_str(&den);
                    out.push(')');
                }
                rest = r2;
                continue;
            }
            if cmd == MD_LATEX_SQRT {
                let (arg_raw, r1): (&str, &str) = read_latex_arg(rest);
                let inner: String = format_latex_math(arg_raw);
                if inner.chars().count() <= 1 {
                    out.push('√');
                    out.push_str(&inner);
                } else {
                    out.push_str("√(");
                    out.push_str(&inner);
                    out.push(')');
                }
                rest = r1;
                continue;
            }
            if cmd == MD_LATEX_TEXT
                || cmd == MD_LATEX_MATHRM
                || cmd == MD_LATEX_MATHBF
                || cmd == MD_LATEX_MATHIT
            {
                let (arg_raw, r1): (&str, &str) = read_latex_arg(rest);
                out.push_str(arg_raw);
                rest = r1;
                continue;
            }
            if cmd == MD_LATEX_LEFT || cmd == MD_LATEX_RIGHT {
                continue;
            }
            if let Ok(sym_idx) =
                MD_LATEX_SYMBOLS.binary_search_by_key(&cmd, |&(sym_k, _): &(&str, &str)| sym_k)
            {
                out.push_str(MD_LATEX_SYMBOLS[sym_idx].1);
            } else {
                out.push_str(cmd);
            }
            continue;
        }
        if let Some(after_caret) = rest.strip_prefix('^') {
            let (arg_raw, r1): (&str, &str) = read_latex_arg(after_caret);
            let inner: String = format_latex_math(arg_raw);
            if inner.chars().all(|c: char| to_unicode_sup(c).is_some()) {
                for c in inner.chars() {
                    if let Some(sup_ch) = to_unicode_sup(c) {
                        out.push(sup_ch);
                    }
                }
            } else {
                out.push_str("^(");
                out.push_str(&inner);
                out.push(')');
            }
            rest = r1;
            continue;
        }
        if let Some(after_sub) = rest.strip_prefix('_') {
            let (arg_raw, r1): (&str, &str) = read_latex_arg(after_sub);
            let inner: String = format_latex_math(arg_raw);
            if inner.chars().all(|c: char| to_unicode_sub(c).is_some()) {
                for c in inner.chars() {
                    if let Some(sub_ch) = to_unicode_sub(c) {
                        out.push(sub_ch);
                    }
                }
            } else {
                out.push_str("_(");
                out.push_str(&inner);
                out.push(')');
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

/// Finds the byte index of the next Markdown inline trigger delimiter at or after `from`.
///
/// # Arguments
///
/// - `&[u8]` - Source UTF-8 bytes.
/// - `usize` - Starting byte offset.
///
/// # Returns
///
/// - `usize` - Byte index of the next trigger delimiter, or `bytes.len()` when none remains.
#[inline(always)]
fn next_inline_trigger(bytes: &[u8], from: usize) -> usize {
    let mut idx: usize = from;
    while idx < bytes.len() {
        if MD_INLINE_TRIGGER_LUT[bytes[idx] as usize] {
            return idx;
        }
        idx += 1;
    }
    bytes.len()
}

/// Checks in O(N) byte-LUT scan whether `text` contains any Markdown inline formatting delimiter.
///
/// # Arguments
///
/// - `&str` - Candidate text slice.
///
/// # Returns
///
/// - `bool` - `true` when at least one Markdown inline trigger byte is present.
#[inline(always)]
#[must_use]
pub(crate) fn has_inline_markdown_trigger(text: &str) -> bool {
    let bytes: &[u8] = text.as_bytes();
    next_inline_trigger(bytes, 0) < bytes.len()
}

/// Returns whether an ASCII byte can be backslash-escaped in inline Markdown text.
///
/// # Arguments
///
/// - `u8` - Candidate byte immediately following `\`.
///
/// # Returns
///
/// - `bool` - `true` when the byte is an escapable Markdown punctuation character.
#[inline(always)]
fn is_md_escapable_byte(b: u8) -> bool {
    MD_ESCAPABLE_BYTE_LUT[b as usize]
}

/// Parses rich inline Markdown formatting using an optional pre-scanned reference link table.
///
/// # Arguments
///
/// - `&str` - Raw single-line or paragraph Markdown string.
/// - `Option<&HashMap<String, String>>` - Optional lowercase link-reference URL map.
///
/// # Returns
///
/// - `Vec<EuvMarkdownInline>` - Ordered inline spans.
#[must_use]
pub(crate) fn parse_markdown_inlines_with_refs(
    text: &str,
    refs: Option<&HashMap<String, String>>,
) -> Vec<EuvMarkdownInline> {
    if text.is_empty() {
        return Vec::new();
    }
    let bytes: &[u8] = text.as_bytes();
    let len: usize = bytes.len();
    let first_trigger: usize = next_inline_trigger(bytes, 0);
    if first_trigger == len {
        return vec![EuvMarkdownInline::Text(text.to_string())];
    }
    let mut out: Vec<EuvMarkdownInline> = Vec::with_capacity(4);
    let mut plain_buf: String = String::new();
    let mut pos: usize = first_trigger;
    if first_trigger > 0 {
        plain_buf.push_str(&text[..first_trigger]);
    }
    while pos < len {
        let rest: &str = &text[pos..];
        let ch_byte: u8 = bytes[pos];
        let mut advanced: usize = 0;
        match ch_byte {
            CODE_BYTE_BACKSLASH => {
                if pos + 1 < len && is_md_escapable_byte(bytes[pos + 1]) {
                    plain_buf.push(bytes[pos + 1] as char);
                    advanced = 2;
                }
            }
            MD_BYTE_NEWLINE => {
                flush_plain(&mut plain_buf, &mut out);
                out.push(EuvMarkdownInline::HardBreak);
                advanced = 1;
            }
            CODE_BYTE_BACKTICK => {
                if let Some(after) = rest.strip_prefix("``")
                    && let Some(end) = after.find("``")
                {
                    flush_plain(&mut plain_buf, &mut out);
                    out.push(EuvMarkdownInline::Code(after[..end].trim().to_string()));
                    advanced = 4 + end;
                } else if let Some(after) = rest.strip_prefix('`')
                    && let Some(end) = after.find('`')
                {
                    flush_plain(&mut plain_buf, &mut out);
                    out.push(EuvMarkdownInline::Code(after[..end].to_string()));
                    advanced = 2 + end;
                }
            }
            MD_BYTE_DOLLAR => {
                let after: &str = &rest[1..];
                if !after.starts_with('$')
                    && !after.starts_with(' ')
                    && let Some(end) = after.find('$')
                    && end > 0
                    && !after[..end].ends_with(' ')
                {
                    flush_plain(&mut plain_buf, &mut out);
                    out.push(EuvMarkdownInline::Math(format_latex_math(&after[..end])));
                    advanced = 2 + end;
                }
            }
            MD_BYTE_STAR => {
                if let Some(after) = rest.strip_prefix("***")
                    && let Some(end) = after.find("***")
                    && end > 0
                {
                    flush_plain(&mut plain_buf, &mut out);
                    out.push(EuvMarkdownInline::BoldItalic(after[..end].to_string()));
                    advanced = 6 + end;
                } else if let Some(after) = rest.strip_prefix("**")
                    && let Some(end) = after.find("**")
                    && end > 0
                {
                    flush_plain(&mut plain_buf, &mut out);
                    out.push(EuvMarkdownInline::Bold(after[..end].to_string()));
                    advanced = 4 + end;
                } else {
                    let after: &str = &rest[1..];
                    if !after.starts_with('*')
                        && !after.starts_with(' ')
                        && let Some(end) = after.find('*')
                        && end > 0
                    {
                        flush_plain(&mut plain_buf, &mut out);
                        out.push(EuvMarkdownInline::Italic(after[..end].to_string()));
                        advanced = 2 + end;
                    }
                }
            }
            CODE_BYTE_UNDERSCORE => {
                if let Some(after) = rest.strip_prefix("__")
                    && let Some(end) = after.find("__")
                    && end > 0
                {
                    flush_plain(&mut plain_buf, &mut out);
                    out.push(EuvMarkdownInline::Bold(after[..end].to_string()));
                    advanced = 4 + end;
                } else {
                    let after: &str = &rest[1..];
                    if !after.starts_with('_')
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
                        advanced = 2 + end;
                    }
                }
            }
            CODE_BYTE_TILDE => {
                if let Some(after) = rest.strip_prefix("~~")
                    && let Some(end) = after.find("~~")
                    && end > 0
                {
                    flush_plain(&mut plain_buf, &mut out);
                    out.push(EuvMarkdownInline::Strike(after[..end].to_string()));
                    advanced = 4 + end;
                } else {
                    let after: &str = &rest[1..];
                    if !after.starts_with('~')
                        && !after.starts_with(' ')
                        && let Some(end) = after.find('~')
                        && end > 0
                        && !after[..end].contains(' ')
                    {
                        flush_plain(&mut plain_buf, &mut out);
                        out.push(EuvMarkdownInline::Subscript(after[..end].to_string()));
                        advanced = 2 + end;
                    }
                }
            }
            MD_BYTE_EQ => {
                if let Some(after) = rest.strip_prefix("==")
                    && let Some(end) = after.find("==")
                    && end > 0
                {
                    flush_plain(&mut plain_buf, &mut out);
                    out.push(EuvMarkdownInline::Highlight(after[..end].to_string()));
                    advanced = 4 + end;
                }
            }
            MD_BYTE_LT => {
                if let Some(after) = rest.strip_prefix(MD_MARK_OPEN)
                    && let Some(end) = after.find(MD_MARK_CLOSE)
                {
                    flush_plain(&mut plain_buf, &mut out);
                    out.push(EuvMarkdownInline::Highlight(after[..end].to_string()));
                    advanced = MD_MARK_OPEN.len() + end + MD_MARK_CLOSE.len();
                } else if let Some(after) = rest.strip_prefix(MD_KBD_OPEN)
                    && let Some(end) = after.find(MD_KBD_CLOSE)
                {
                    flush_plain(&mut plain_buf, &mut out);
                    out.push(EuvMarkdownInline::Kbd(after[..end].to_string()));
                    advanced = MD_KBD_OPEN.len() + end + MD_KBD_CLOSE.len();
                } else if let Some(after) = rest.strip_prefix(MD_SUP_OPEN)
                    && let Some(end) = after.find(MD_SUP_CLOSE)
                {
                    flush_plain(&mut plain_buf, &mut out);
                    out.push(EuvMarkdownInline::Superscript(after[..end].to_string()));
                    advanced = MD_SUP_OPEN.len() + end + MD_SUP_CLOSE.len();
                } else if let Some(after) = rest.strip_prefix(MD_SUB_OPEN)
                    && let Some(end) = after.find(MD_SUB_CLOSE)
                {
                    flush_plain(&mut plain_buf, &mut out);
                    out.push(EuvMarkdownInline::Subscript(after[..end].to_string()));
                    advanced = MD_SUB_OPEN.len() + end + MD_SUB_CLOSE.len();
                } else {
                    let after_lt: &str = &rest[1..];
                    if let Some(gt) = after_lt.find('>') {
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
                            advanced = 2 + gt;
                        }
                    }
                }
            }
            MD_BYTE_CARET => {
                let after: &str = &rest[1..];
                if !after.starts_with(' ')
                    && let Some(end) = after.find('^')
                    && end > 0
                    && !after[..end].contains(' ')
                {
                    flush_plain(&mut plain_buf, &mut out);
                    out.push(EuvMarkdownInline::Superscript(after[..end].to_string()));
                    advanced = 2 + end;
                }
            }
            CODE_BYTE_BANG => {
                if let Some(after_bang) = rest.strip_prefix("![")
                    && let Some(rb) = after_bang.find("](")
                    && let Some(rp) = after_bang[rb + 2..].find(')')
                {
                    flush_plain(&mut plain_buf, &mut out);
                    out.push(EuvMarkdownInline::Image {
                        alt: after_bang[..rb].to_string(),
                        url: after_bang[rb + 2..rb + 2 + rp].to_string(),
                    });
                    advanced = 2 + rb + 2 + rp + 1;
                }
            }
            MD_BYTE_LBRACKET => {
                if let Some(after_fn) = rest.strip_prefix("[^")
                    && let Some(rb) = after_fn.find(']')
                    && rb > 0
                    && !after_fn[..rb].contains(' ')
                {
                    flush_plain(&mut plain_buf, &mut out);
                    out.push(EuvMarkdownInline::FootnoteRef(after_fn[..rb].to_string()));
                    advanced = 2 + rb + 1;
                } else {
                    let after_lb: &str = &rest[1..];
                    if let Some(rb) = after_lb.find("](")
                        && !after_lb[..rb].contains(']')
                        && let Some(rp) = after_lb[rb + 2..].find(')')
                    {
                        flush_plain(&mut plain_buf, &mut out);
                        let label: String = after_lb[..rb].to_string();
                        let url: String = after_lb[rb + 2..rb + 2 + rp].to_string();
                        out.push(EuvMarkdownInline::Link { label, url });
                        advanced = 1 + rb + 2 + rp + 1;
                    } else if let Some(ref_map) = refs
                        && !ref_map.is_empty()
                        && let Some(rb) = after_lb.find("][")
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
                        if let Some(url) = ref_map.get(&lookup) {
                            flush_plain(&mut plain_buf, &mut out);
                            out.push(EuvMarkdownInline::Link {
                                label: label.to_string(),
                                url: url.clone(),
                            });
                            advanced = 1 + rb + 2 + rb2 + 1;
                        }
                    }
                }
            }
            CODE_BYTE_COLON => {
                let after_col: &str = &rest[1..];
                if let Some(end) = after_col.find(':')
                    && end > 0
                    && end <= MD_EMOJI_MAX_LEN
                    && after_col[..end].bytes().all(|b: u8| {
                        b.is_ascii_alphanumeric()
                            || b == CODE_BYTE_UNDERSCORE
                            || b == MD_BYTE_PLUS
                            || b == MD_BYTE_DASH
                    })
                    && let Some(glyph) = resolve_emoji_shortcode(&after_col[..end])
                {
                    plain_buf.push_str(glyph);
                    advanced = 2 + end;
                }
            }
            _ => {}
        }
        if advanced > 0 {
            pos += advanced;
        } else {
            plain_buf.push(ch_byte as char);
            pos += 1;
        }
        let next_pos: usize = next_inline_trigger(bytes, pos);
        if next_pos > pos {
            plain_buf.push_str(&text[pos..next_pos]);
            pos = next_pos;
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
    parse_markdown_inlines_with_refs(text, None)
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

/// Checks whether a trimmed line opens a fenced code block (`` ``` `` or `~~~`) without heap allocation.
///
/// # Arguments
///
/// - `&str` - Trimmed input line.
///
/// # Returns
///
/// - `Option<(u8, usize, &str)>` - `(fence_byte, fence_len, borrowed_language_tag)`.
#[inline(always)]
fn parse_code_fence_open(trimmed: &str) -> Option<(u8, usize, &str)> {
    let first: u8 = *trimmed.as_bytes().first()?;
    if first != CODE_BYTE_BACKTICK && first != CODE_BYTE_TILDE {
        return None;
    }
    let count: usize = trimmed.bytes().take_while(|b: &u8| *b == first).count();
    if count < 3 {
        return None;
    }
    let info: &str = trimmed[count..].trim();
    if first == CODE_BYTE_BACKTICK && info.as_bytes().contains(&CODE_BYTE_BACKTICK) {
        return None;
    }
    let lang: &str = info.split_whitespace().next().unwrap_or("");
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
#[inline(always)]
fn is_code_fence_close(trimmed: &str, fence_byte: u8, min_len: usize) -> bool {
    let count: usize = trimmed
        .bytes()
        .take_while(|b: &u8| *b == fence_byte)
        .count();
    count >= min_len && trimmed[count..].trim().is_empty()
}

/// Checks whether a trimmed line is a thematic horizontal rule (`---`, `***`, `___`) without heap allocation.
///
/// # Arguments
///
/// - `&str` - Trimmed input line.
///
/// # Returns
///
/// - `bool` - `true` when the line is a valid Markdown horizontal rule.
#[inline(always)]
fn is_horizontal_rule(trimmed: &str) -> bool {
    let bytes: &[u8] = trimmed.as_bytes();
    let first: u8 = match bytes.first() {
        Some(&b) if b == MD_BYTE_DASH || b == MD_BYTE_STAR || b == CODE_BYTE_UNDERSCORE => b,
        _ => return false,
    };
    let mut count: usize = 0;
    if trimmed.is_ascii() {
        for &b in bytes {
            if b == first {
                count += 1;
            } else if !b.is_ascii_whitespace() {
                return false;
            }
        }
    } else {
        let first_ch: char = first as char;
        for ch in trimmed.chars() {
            if ch == first_ch {
                count += 1;
            } else if !ch.is_whitespace() {
                return false;
            }
        }
    }
    count >= 3
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

/// Checks in zero allocations whether `body_lines[idx]` starts a new block construct that terminates a paragraph.
///
/// # Arguments
///
/// - `&[&str]` - Filtered document body lines.
/// - `usize` - Candidate continuation line index.
///
/// # Returns
///
/// - `bool` - `true` when line `idx` must terminate the current paragraph.
#[inline(always)]
fn is_paragraph_terminator(body_lines: &[&str], idx: usize) -> bool {
    let cur_raw: &str = body_lines[idx];
    let cur: &str = cur_raw.trim();
    cur.is_empty()
        || parse_code_fence_open(cur).is_some()
        || cur.starts_with("$$")
        || parse_container_callout_open(cur).is_some()
        || parse_heading_line(cur).is_some()
        || cur.starts_with('>')
        || (cur.as_bytes().contains(&MD_BYTE_PIPE)
            && idx + 1 < body_lines.len()
            && is_table_separator(body_lines[idx + 1].trim()))
        || (cur.starts_with("![") && cur.ends_with(')'))
        || cur.starts_with(MD_DETAILS_OPEN_TAG)
        || is_horizontal_rule(cur)
        || is_list_line(cur_raw)
        || (idx + 1 < body_lines.len() && body_lines[idx + 1].trim_start().starts_with(": "))
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
    let mut filtered_lines: Option<Vec<&str>> = None;
    if source.contains("]:") {
        let mut scan_idx: usize = 0;
        let mut in_fence: Option<(u8, usize)> = None;
        while scan_idx < raw_lines.len() {
            let line: &str = raw_lines[scan_idx];
            let trimmed: &str = line.trim();
            if let Some((fb, fl)) = in_fence {
                if is_code_fence_close(trimmed, fb, fl) {
                    in_fence = None;
                }
                if let Some(ref mut buf) = filtered_lines {
                    buf.push(line);
                }
                scan_idx += 1;
                continue;
            }
            if let Some((fb, fl, _)) = parse_code_fence_open(trimmed) {
                in_fence = Some((fb, fl));
                if let Some(ref mut buf) = filtered_lines {
                    buf.push(line);
                }
                scan_idx += 1;
                continue;
            }
            if trimmed.starts_with('[') {
                if let Some((fn_id, mut fn_body)) = parse_footnote_def(trimmed) {
                    if filtered_lines.is_none() {
                        let mut buf: Vec<&str> = Vec::with_capacity(raw_lines.len());
                        buf.extend_from_slice(&raw_lines[..scan_idx]);
                        filtered_lines = Some(buf);
                    }
                    scan_idx += 1;
                    while scan_idx < raw_lines.len()
                        && (raw_lines[scan_idx].starts_with("  ")
                            || raw_lines[scan_idx].starts_with('\t'))
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
                    if filtered_lines.is_none() {
                        let mut buf: Vec<&str> = Vec::with_capacity(raw_lines.len());
                        buf.extend_from_slice(&raw_lines[..scan_idx]);
                        filtered_lines = Some(buf);
                    }
                    let _: Option<String> = refs.insert(key, url);
                    scan_idx += 1;
                    continue;
                }
            }
            if let Some(ref mut buf) = filtered_lines {
                buf.push(line);
            }
            scan_idx += 1;
        }
    }
    let body_lines: &[&str] = filtered_lines.as_deref().unwrap_or(&raw_lines);
    let refs_opt: Option<&HashMap<String, String>> =
        if refs.is_empty() { None } else { Some(&refs) };
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
            let code_start: usize = idx;
            while idx < total && !is_code_fence_close(body_lines[idx].trim(), fb, fl) {
                idx += 1;
            }
            let joined: String = body_lines[code_start..idx].join("\n");
            if idx < total {
                idx += 1;
            }
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
                        lang.to_string()
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
                text: heading_txt.to_string(),
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
        if trimmed.as_bytes().contains(&MD_BYTE_PIPE)
            && idx + 1 < total
            && let Some(alignments) = try_parse_table_alignments(body_lines[idx + 1].trim())
        {
            let headers: Vec<String> = split_table_row(trimmed);
            idx += 2;
            let mut rows: Vec<Vec<String>> = Vec::new();
            while idx < total {
                let row_t: &str = body_lines[idx].trim();
                if row_t.is_empty() || !row_t.as_bytes().contains(&MD_BYTE_PIPE) {
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
        let para_start: usize = idx;
        idx += 1;
        while idx < total && !is_paragraph_terminator(body_lines, idx) {
            idx += 1;
        }
        if idx == para_start + 1 {
            let single_seg: &str = if trimmed.ends_with('\\') {
                trimmed.strip_suffix('\\').unwrap_or(trimmed).trim_end()
            } else {
                trimmed
            };
            blocks.push(EuvMarkdownBlock::Paragraph(
                parse_markdown_inlines_with_refs(single_seg, refs_opt),
            ));
        } else {
            let mut para_cap: usize = idx - para_start;
            for line_ref in &body_lines[para_start..idx] {
                para_cap += line_ref.len();
            }
            let mut para_buf: String = String::with_capacity(para_cap);
            for &cur_raw in &body_lines[para_start..idx] {
                let cur: &str = cur_raw.trim();
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
            }
            let trimmed_para: &str = para_buf.trim_end_matches('\n');
            blocks.push(EuvMarkdownBlock::Paragraph(
                parse_markdown_inlines_with_refs(trimmed_para, refs_opt),
            ));
        }
    }
    if !footnotes.is_empty() {
        blocks.push(EuvMarkdownBlock::Footnotes(footnotes));
    }
    blocks
}

/// Parses an ATX heading line (`# Title` .. `###### Title`) without heap allocation, stripping optional `{#custom-anchor}` suffixes.
///
/// # Arguments
///
/// - `&str` - Trimmed input line.
///
/// # Returns
///
/// - `Option<(u8, &str)>` - Heading level and borrowed trimmed heading text if valid.
#[inline(always)]
fn parse_heading_line(trimmed: &str) -> Option<(u8, &str)> {
    if !trimmed.starts_with('#') {
        return None;
    }
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
    Some((hashes as u8, clean))
}

/// Classifies a blockquote block into a plain quote or GitHub Alert variant (`[!NOTE]`, `[!TIP]`, etc.) without heap allocation.
///
/// # Arguments
///
/// - `Vec<String>` - Raw stripped blockquote lines.
///
/// # Returns
///
/// - `(EuvMarkdownAlertKind, Vec<String>)` - Resolved alert kind and body lines.
pub(crate) fn classify_quote_alert(mut lines: Vec<String>) -> (EuvMarkdownAlertKind, Vec<String>) {
    let first_trimmed: &str = match lines.first() {
        Some(s) => s.trim(),
        None => return (EuvMarkdownAlertKind::Quote, lines),
    };
    let kind: EuvMarkdownAlertKind = if first_trimmed.eq_ignore_ascii_case(MD_ALERT_TAG_NOTE) {
        EuvMarkdownAlertKind::Note
    } else if first_trimmed.eq_ignore_ascii_case(MD_ALERT_TAG_TIP) {
        EuvMarkdownAlertKind::Tip
    } else if first_trimmed.eq_ignore_ascii_case(MD_ALERT_TAG_IMPORTANT) {
        EuvMarkdownAlertKind::Important
    } else if first_trimmed.eq_ignore_ascii_case(MD_ALERT_TAG_WARNING) {
        EuvMarkdownAlertKind::Warning
    } else if first_trimmed.eq_ignore_ascii_case(MD_ALERT_TAG_CAUTION) {
        EuvMarkdownAlertKind::Caution
    } else {
        return (EuvMarkdownAlertKind::Quote, lines);
    };
    let _: String = lines.remove(0);
    (kind, lines)
}

/// Strips optional outer leading and trailing pipe delimiters from a trimmed table row.
///
/// # Arguments
///
/// - `&str` - Trimmed Markdown table row line.
///
/// # Returns
///
/// - `&str` - Inner pipe-delimited cell slice.
#[inline(always)]
fn table_row_inner(trimmed: &str) -> &str {
    let without_lead: &str = trimmed.strip_prefix('|').unwrap_or(trimmed);
    without_lead.strip_suffix('|').unwrap_or(without_lead)
}

/// Checks whether a Markdown line is a GFM table alignment separator row (`| --- | :---: |`) without heap allocation.
///
/// # Arguments
///
/// - `&str` - Trimmed candidate separator line.
///
/// # Returns
///
/// - `bool` - `true` when the row is a valid table separator.
#[inline(always)]
fn is_table_separator(trimmed: &str) -> bool {
    if !trimmed.as_bytes().contains(&MD_BYTE_DASH) {
        return false;
    }
    let inner: &str = table_row_inner(trimmed);
    let mut has_any: bool = false;
    for cell in inner.split('|') {
        has_any = true;
        let t: &str = cell.trim();
        if t.is_empty()
            || !t.as_bytes().contains(&MD_BYTE_DASH)
            || !t
                .bytes()
                .all(|b: u8| b == MD_BYTE_DASH || b == CODE_BYTE_COLON)
        {
            return false;
        }
    }
    has_any
}

/// Validates and parses column alignments from a GFM table separator row in a single pass with zero intermediate string allocations.
///
/// # Arguments
///
/// - `&str` - Trimmed candidate separator row line.
///
/// # Returns
///
/// - `Option<Vec<EuvMarkdownAlign>>` - Parsed column alignments when valid, or `None`.
fn try_parse_table_alignments(trimmed: &str) -> Option<Vec<EuvMarkdownAlign>> {
    if !is_table_separator(trimmed) {
        return None;
    }
    let inner: &str = table_row_inner(trimmed);
    let mut alignments: Vec<EuvMarkdownAlign> = Vec::with_capacity(4);
    for cell in inner.split('|') {
        let t: &str = cell.trim();
        let left: bool = t.starts_with(':');
        let right: bool = t.ends_with(':');
        let align: EuvMarkdownAlign = if left && right {
            EuvMarkdownAlign::Center
        } else if right {
            EuvMarkdownAlign::Right
        } else {
            EuvMarkdownAlign::Left
        };
        alignments.push(align);
    }
    Some(alignments)
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
    table_row_inner(trimmed)
        .split('|')
        .map(|cell: &str| cell.trim().to_string())
        .collect()
}

/// Checks whether a raw line starts an unordered, ordered, or task list item without heap allocation.
///
/// # Arguments
///
/// - `&str` - Raw input line.
///
/// # Returns
///
/// - `bool` - `true` when the line matches a Markdown list item prefix.
#[inline(always)]
fn is_list_line(raw: &str) -> bool {
    let bytes: &[u8] = raw.as_bytes();
    let mut leading: usize = 0;
    while leading < bytes.len() && bytes[leading] == CODE_BYTE_SPACE {
        leading += 1;
    }
    let trimmed: &str = raw[leading..].trim_end();
    let tb: &[u8] = trimmed.as_bytes();
    let first: u8 = match tb.first() {
        Some(&b) => b,
        None => return false,
    };
    if (first == MD_BYTE_DASH || first == MD_BYTE_STAR || first == MD_BYTE_PLUS)
        && tb.len() >= 2
        && tb[1] == CODE_BYTE_SPACE
    {
        return true;
    }
    if !first.is_ascii_digit() {
        return false;
    }
    let mut d: usize = 1;
    while d < tb.len() && tb[d].is_ascii_digit() {
        d += 1;
    }
    d + 1 < tb.len() && tb[d] == CODE_BYTE_DOT && tb[d + 1] == CODE_BYTE_SPACE
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
    let bytes: &[u8] = raw.as_bytes();
    let mut leading_spaces: usize = 0;
    while leading_spaces < bytes.len() && bytes[leading_spaces] == CODE_BYTE_SPACE {
        leading_spaces += 1;
    }
    let trimmed: &str = raw[leading_spaces..].trim_end();
    let first: u8 = *trimmed.as_bytes().first()?;
    if first == MD_BYTE_DASH || first == MD_BYTE_STAR || first == MD_BYTE_PLUS {
        let after: &str = trimmed[1..].strip_prefix(' ')?;
        let depth: usize = leading_spaces / 2;
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
    if !first.is_ascii_digit() {
        return None;
    }
    let digit_len: usize = trimmed
        .bytes()
        .take_while(|b: &u8| b.is_ascii_digit())
        .count();
    if let Some(after_dot) = trimmed[digit_len..].strip_prefix(". ") {
        let depth: usize = leading_spaces / 2;
        let start_num: usize = trimmed[..digit_len].parse::<usize>().unwrap_or(1);
        return Some((
            true,
            start_num,
            EuvMarkdownListItem::new(after_dot.trim().to_string(), depth, None),
        ));
    }
    None
}
