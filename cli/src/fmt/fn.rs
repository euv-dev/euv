use super::*;

/// Formats all `euv` procedural macro bodies (`view!`, `style!`, `tokens!`, etc.) in a single Rust source string deterministically and idempotently.
///
/// # Arguments
///
/// - `&str` - Raw Rust source code.
///
/// # Returns
///
/// - `(String, usize)` - Tuple of `(formatted_source, formatted_macro_count)`.
#[must_use]
pub fn format_euv_macros_in_source(source: &str) -> (String, usize) {
    format_euv_macros_if_needed(source).unwrap_or_else(|| (source.to_string(), 0))
}

/// Formats `euv` macro blocks in `source`, returning `None` with zero whole-file copy when unchanged.
///
/// # Arguments
///
/// - `&str` - Raw Rust source code.
///
/// # Returns
///
/// - `Option<(String, usize)>` - `Some((formatted_source, formatted_macro_count))` when modified.
pub(crate) fn format_euv_macros_if_needed(source: &str) -> Option<(String, usize)> {
    if !may_contain_euv_macro(source) {
        return None;
    }
    let bytes: &[u8] = source.as_bytes();
    let len: usize = bytes.len();
    let mut out: Option<String> = None;
    let mut copy_start: usize = 0;
    let mut block_buf: String = String::new();
    let mut indent_stack: Vec<usize> = Vec::with_capacity(16);
    let mut idx: usize = 0;
    let mut count: usize = 0;
    while idx < len {
        let b: u8 = bytes[idx];
        if (b == BYTE_SLASH || b == BYTE_DQUOTE || b == BYTE_SQUOTE || b == BYTE_LOWER_R)
            && let Some(next_idx) = skip_string_or_comment(bytes, idx)
        {
            idx = next_idx;
            continue;
        }
        if is_ident_start(b) && (idx == 0 || !is_ident_continue(bytes[idx - 1])) {
            let ident_start: usize = idx;
            let mut ident_end: usize = idx + 1;
            while ident_end < len && is_ident_continue(bytes[ident_end]) {
                ident_end += 1;
            }
            let ident_len: usize = ident_end - ident_start;
            if ident_end < len
                && bytes[ident_end] == BYTE_BANG
                && (EUV_FMT_MACRO_MIN_LEN..=EUV_FMT_MACRO_MAX_LEN).contains(&ident_len)
            {
                let ident: &str = &source[ident_start..ident_end];
                if EUV_FMT_MACRO_NAMES.contains(&ident) {
                    let mut probe: usize = ident_end + 1;
                    while probe < len && (bytes[probe] == BYTE_SPACE || bytes[probe] == BYTE_TAB) {
                        probe += 1;
                    }
                    if probe < len
                        && bytes[probe] == BYTE_LBRACE
                        && let Some(close_brace) = find_matching_brace(bytes, probe)
                    {
                        let inner: &str = &source[(probe + 1)..close_brace];
                        let base_indent: usize = line_leading_spaces(source, ident_start);
                        format_macro_brace_body_into(
                            inner,
                            base_indent,
                            &mut block_buf,
                            &mut indent_stack,
                        );
                        if block_buf.as_str() != &source[probe..=close_brace]
                            || &source[(ident_end + 1)..probe] != " "
                        {
                            count += 1;
                            let buf: &mut String =
                                out.get_or_insert_with(|| String::with_capacity(len + 64));
                            buf.push_str(&source[copy_start..ident_start]);
                            buf.push_str(ident);
                            buf.push_str("! ");
                            buf.push_str(&block_buf);
                            copy_start = close_brace + 1;
                        }
                        idx = close_brace + 1;
                        continue;
                    }
                }
            }
            idx = ident_end;
            continue;
        }
        idx += 1;
    }
    if let Some(mut buf) = out {
        buf.push_str(&source[copy_start..]);
        Some((buf, count))
    } else {
        None
    }
}

/// Fast zero-allocation pre-scan checking whether `source` contains any candidate `<euv_macro>!` token.
///
/// # Arguments
///
/// - `&str` - Raw Rust source text.
///
/// # Returns
///
/// - `bool` - `true` if a supported `euv` macro name immediately precedes `!` anywhere in `source`.
fn may_contain_euv_macro(source: &str) -> bool {
    let bytes: &[u8] = source.as_bytes();
    let len: usize = bytes.len();
    let mut pos: usize = 0;
    while pos < len {
        if bytes[pos] == BYTE_BANG {
            let mut start: usize = pos;
            while start > 0 && is_ident_continue(bytes[start - 1]) {
                start -= 1;
            }
            let ident_len: usize = pos - start;
            if (EUV_FMT_MACRO_MIN_LEN..=EUV_FMT_MACRO_MAX_LEN).contains(&ident_len)
                && is_ident_start(bytes[start])
            {
                let ident: &str = &source[start..pos];
                if EUV_FMT_MACRO_NAMES.contains(&ident) {
                    return true;
                }
            }
        }
        pos += 1;
    }
    false
}

/// Runs `crate-cli`'s unified Prettier-aligned text, derive, and Rust formatting pass before `euv` macro normalization.
///
/// # Arguments
///
/// - `&Path` - Root directory, `Cargo.toml` manifest path, or single file path.
/// - `bool` - When `true`, checks formatting without modifying files.
///
/// # Returns
///
/// - `io::Result<UnifiedFmtReport>` - Summary of scanned and formatted non-Rust text files.
pub fn run_unified_base_fmt(target_root: &Path, check: bool) -> io::Result<UnifiedFmtReport> {
    let is_non_manifest_file: bool = target_root.is_file()
        && target_root.file_name().and_then(OsStr::to_str) != Some(CARGO_TOML_NAME);
    if is_non_manifest_file {
        return format_workspace_text_files(target_root, check);
    }
    let runtime: Runtime = Builder::new_current_thread().enable_all().build()?;
    runtime.block_on(execute_unified_fmt(target_root, check))
}

/// Formats all `.rs` files under `root` (or a single `.rs` file) in place or in `--check` mode.
///
/// # Arguments
///
/// - `&Path` - Root directory or single Rust source file.
/// - `bool` - When `true`, verifies formatting without modifying files and returns an error if changes are needed.
///
/// # Returns
///
/// - `io::Result<FmtReport>` - Structured formatting report or error on I/O / check failure.
pub fn format_workspace_macros(root: &Path, check: bool) -> io::Result<FmtReport> {
    let mut files: Vec<PathBuf> = Vec::new();
    collect_rs_files(root, &mut files)?;
    files.sort();
    let mut report: FmtReport = FmtReport::new(files.len(), 0, 0, 0, 0, check, Vec::new());
    for file_path in files.into_iter() {
        let original: String = fs::read_to_string(&file_path)?;
        if let Some((formatted, macro_hits)) = format_euv_macros_if_needed(&original) {
            let next_files: usize = report.get_files_formatted() + 1;
            let next_macros: usize = report.get_macros_formatted() + macro_hits.max(1);
            report.set_files_formatted(next_files);
            report.set_macros_formatted(next_macros);
            report
                .get_mut_changed_files()
                .push(file_path.display().to_string());
            if !check {
                fs::write(&file_path, formatted)?;
            }
        }
    }
    if check && report.get_files_formatted() > 0 {
        return Err(Error::other(format!(
            "{ERR_FMT_CHECK_FAILED}: {}",
            report.get_changed_files().join(", ")
        )));
    }
    Ok(report)
}

/// Recursively collects `.rs` source files under `path`, skipping build/VCS directories.
///
/// # Arguments
///
/// - `&Path` - Directory or file path to inspect.
/// - `&mut Vec<PathBuf>` - Output accumulator of Rust source paths.
///
/// # Returns
///
/// - `io::Result<()>` - Traversal result.
fn collect_rs_files(path: &Path, out: &mut Vec<PathBuf>) -> io::Result<()> {
    if path.is_file() {
        if path.extension().and_then(OsStr::to_str) == Some(EUV_FMT_RS_EXT) {
            out.push(path.to_path_buf());
        }
        return Ok(());
    }
    if !path.is_dir() {
        return Ok(());
    }
    for entry_res in fs::read_dir(path)? {
        let entry: DirEntry = entry_res?;
        let child: PathBuf = entry.path();
        if child.is_dir() {
            let name: String = entry.file_name().to_string_lossy().into_owned();
            if name.starts_with('.') || EUV_FMT_SKIP_DIRS.contains(&name.as_str()) {
                continue;
            }
            collect_rs_files(&child, out)?;
        } else if child.extension().and_then(OsStr::to_str) == Some(EUV_FMT_RS_EXT) {
            out.push(child);
        }
    }
    Ok(())
}

/// Appends `count` ASCII space bytes into `out` via bulk slicing of `INDENT_SPACES_128`.
///
/// # Arguments
///
/// - `&mut String` - Destination output buffer.
/// - `usize` - Number of space characters to append.
fn push_spaces(out: &mut String, mut count: usize) {
    while count >= INDENT_SPACES_128.len() {
        out.push_str(INDENT_SPACES_128);
        count -= INDENT_SPACES_128.len();
    }
    if count > 0 {
        out.push_str(&INDENT_SPACES_128[..count]);
    }
}

/// Formats the contents inside `{ ... }` of an `euv` macro invocation into reusable buffer `out`.
///
/// # Arguments
///
/// - `&str` - Raw interior slice between `{` and `}`.
/// - `usize` - Base indentation in spaces of the macro call line.
/// - `&mut String` - Reusable formatted block output buffer.
/// - `&mut Vec<usize>` - Reusable delimiter indentation stack.
fn format_macro_brace_body_into(
    inner: &str,
    base_indent: usize,
    out: &mut String,
    indent_stack: &mut Vec<usize>,
) {
    out.clear();
    if !inner.contains('\n') {
        let trimmed: &str = inner.trim();
        if trimmed.is_empty() {
            out.push_str("{}");
        } else {
            out.reserve(trimmed.len() + 4);
            out.push_str("{ ");
            out.push_str(trimmed);
            out.push_str(" }");
        }
        return;
    }
    out.reserve(inner.len() + 64);
    out.push_str("{\n");
    indent_stack.clear();
    let mut visual_depth: usize = 1;
    let mut in_block_comment: bool = false;
    let mut raw_hashes: Option<usize> = None;
    for raw_line in inner.lines() {
        if let Some(hashes) = raw_hashes {
            out.push_str(raw_line);
            out.push('\n');
            raw_hashes = scan_raw_string_end(raw_line, hashes);
            continue;
        }
        let trimmed: &str = raw_line.trim();
        if trimmed.is_empty() {
            continue;
        }
        let (leading_closes, trailing_closes, line_opens, next_comment, next_raw): (
            usize,
            usize,
            usize,
            bool,
            Option<usize>,
        ) = scan_line_delimiter_counts(trimmed, in_block_comment);
        in_block_comment = next_comment;
        raw_hashes = next_raw;
        for _ in 0..leading_closes {
            if let Some(delta) = indent_stack.pop() {
                visual_depth -= delta;
            }
        }
        let indent_spaces: usize = base_indent + visual_depth * EUV_FMT_INDENT_SPACES;
        push_spaces(out, indent_spaces);
        out.push_str(trimmed);
        out.push('\n');
        for _ in 0..trailing_closes {
            if let Some(delta) = indent_stack.pop() {
                visual_depth -= delta;
            }
        }
        for open_idx in 0..line_opens {
            let delta: usize = usize::from(open_idx == 0);
            indent_stack.push(delta);
            visual_depth += delta;
        }
    }
    push_spaces(out, base_indent);
    out.push('}');
}

/// Scans a single trimmed line into stack-counted delimiter deltas outside strings and comments.
///
/// # Arguments
///
/// - `&str` - Trimmed source line.
/// - `bool` - Whether a `/* ... */` block comment was already open on entry.
///
/// # Returns
///
/// - `(usize, usize, usize, bool, Option<usize>)` - `(leading_closes, trailing_closes, net_opens, next_in_block_comment, unclosed_raw_string_hashes)`.
fn scan_line_delimiter_counts(
    line: &str,
    mut in_block_comment: bool,
) -> (usize, usize, usize, bool, Option<usize>) {
    let bytes: &[u8] = line.as_bytes();
    let len: usize = bytes.len();
    let mut i: usize = 0;
    let mut leading_closes: usize = 0;
    let mut trailing_closes: usize = 0;
    let mut line_opens: usize = 0;
    let mut seen_open: bool = false;
    while i < len {
        if in_block_comment {
            if bytes[i] == BYTE_STAR && i + 1 < len && bytes[i + 1] == BYTE_SLASH {
                in_block_comment = false;
                i += 2;
            } else {
                i += 1;
            }
            continue;
        }
        if bytes[i] == BYTE_SLASH && i + 1 < len && bytes[i + 1] == BYTE_SLASH {
            break;
        }
        if bytes[i] == BYTE_SLASH && i + 1 < len && bytes[i + 1] == BYTE_STAR {
            in_block_comment = true;
            i += 2;
            continue;
        }
        if bytes[i] == BYTE_LOWER_R
            && let Some((end_pos, hashes)) = parse_raw_string_on_line(bytes, i)
        {
            match end_pos {
                Some(next_i) => {
                    i = next_i;
                    continue;
                }
                None => {
                    return (
                        leading_closes,
                        trailing_closes,
                        line_opens,
                        in_block_comment,
                        Some(hashes),
                    );
                }
            }
        }
        if bytes[i] == BYTE_DQUOTE {
            i += 1;
            while i < len {
                if bytes[i] == BYTE_BACKSLASH && i + 1 < len {
                    i += 2;
                    continue;
                }
                if bytes[i] == BYTE_DQUOTE {
                    i += 1;
                    break;
                }
                i += 1;
            }
            continue;
        }
        if bytes[i] == BYTE_SQUOTE
            && let Some(next_i) = skip_char_literal(bytes, i)
        {
            i = next_i;
            continue;
        }
        match bytes[i] {
            BYTE_LBRACE | BYTE_LPAREN | BYTE_LBRACKET => {
                seen_open = true;
                line_opens += 1;
            }
            BYTE_RBRACE | BYTE_RPAREN | BYTE_RBRACKET => {
                if !seen_open {
                    leading_closes += 1;
                } else if line_opens > 0 {
                    line_opens -= 1;
                } else {
                    trailing_closes += 1;
                }
            }
            _ => {}
        }
        i += 1;
    }
    (
        leading_closes,
        trailing_closes,
        line_opens,
        in_block_comment,
        None,
    )
}

/// Checks if a multi-line raw string literal closes on `line`.
///
/// # Arguments
///
/// - `&str` - Raw line inside an active raw string literal.
/// - `usize` - Number of `#` delimiters required after `"`.
///
/// # Returns
///
/// - `Option<usize>` - `None` if closed on this line, or `Some(hashes)` if still open.
fn scan_raw_string_end(line: &str, hashes: usize) -> Option<usize> {
    let bytes: &[u8] = line.as_bytes();
    let len: usize = bytes.len();
    let mut i: usize = 0;
    while i < len {
        if bytes[i] == BYTE_DQUOTE {
            let mut h: usize = 0;
            while i + 1 + h < len && bytes[i + 1 + h] == BYTE_HASH && h < hashes {
                h += 1;
            }
            if h == hashes {
                return None;
            }
        }
        i += 1;
    }
    Some(hashes)
}

/// Attempts to parse a Rust raw string starting at `bytes[start] == b'r'` within a single line.
///
/// # Arguments
///
/// - `&[u8]` - Line bytes.
/// - `usize` - Index of `'r'`.
///
/// # Returns
///
/// - `Option<(Option<usize>, usize)>` - `Some((end_index_if_closed_on_line, hash_count))` if a raw string starts here.
fn parse_raw_string_on_line(bytes: &[u8], start: usize) -> Option<(Option<usize>, usize)> {
    let len: usize = bytes.len();
    if start > 0 && is_ident_continue(bytes[start - 1]) {
        return None;
    }
    let mut i: usize = start + 1;
    let mut hashes: usize = 0;
    while i < len && bytes[i] == BYTE_HASH {
        hashes += 1;
        i += 1;
    }
    if i >= len || bytes[i] != BYTE_DQUOTE {
        return None;
    }
    i += 1;
    while i < len {
        if bytes[i] == BYTE_DQUOTE {
            let mut h: usize = 0;
            while i + 1 + h < len && bytes[i + 1 + h] == BYTE_HASH && h < hashes {
                h += 1;
            }
            if h == hashes {
                return Some((Some(i + 1 + hashes), hashes));
            }
        }
        i += 1;
    }
    Some((None, hashes))
}

/// Skips a string literal, raw string literal, char literal, or comment starting at `idx`.
///
/// # Arguments
///
/// - `&[u8]` - Full Rust source bytes.
/// - `usize` - Current byte offset.
///
/// # Returns
///
/// - `Option<usize>` - Byte offset immediately after the skipped literal or comment.
fn skip_string_or_comment(bytes: &[u8], idx: usize) -> Option<usize> {
    let len: usize = bytes.len();
    if bytes[idx] == BYTE_SLASH && idx + 1 < len && bytes[idx + 1] == BYTE_SLASH {
        let mut i: usize = idx + 2;
        while i < len && bytes[i] != BYTE_NEWLINE {
            i += 1;
        }
        return Some(i);
    }
    if bytes[idx] == BYTE_SLASH && idx + 1 < len && bytes[idx + 1] == BYTE_STAR {
        let mut i: usize = idx + 2;
        let mut nest: usize = 1;
        while i + 1 < len && nest > 0 {
            if bytes[i] == BYTE_SLASH && bytes[i + 1] == BYTE_STAR {
                nest += 1;
                i += 2;
            } else if bytes[i] == BYTE_STAR && bytes[i + 1] == BYTE_SLASH {
                nest -= 1;
                i += 2;
            } else {
                i += 1;
            }
        }
        return Some(i);
    }
    if bytes[idx] == BYTE_LOWER_R && (idx == 0 || !is_ident_continue(bytes[idx - 1])) {
        let mut i: usize = idx + 1;
        let mut hashes: usize = 0;
        while i < len && bytes[i] == BYTE_HASH {
            hashes += 1;
            i += 1;
        }
        if i < len && bytes[i] == BYTE_DQUOTE {
            i += 1;
            while i < len {
                if bytes[i] == BYTE_DQUOTE {
                    let mut h: usize = 0;
                    while i + 1 + h < len && bytes[i + 1 + h] == BYTE_HASH && h < hashes {
                        h += 1;
                    }
                    if h == hashes {
                        return Some(i + 1 + hashes);
                    }
                }
                i += 1;
            }
            return Some(len);
        }
    }
    if bytes[idx] == BYTE_DQUOTE {
        let mut i: usize = idx + 1;
        while i < len {
            if bytes[i] == BYTE_BACKSLASH && i + 1 < len {
                i += 2;
                continue;
            }
            if bytes[i] == BYTE_DQUOTE {
                return Some(i + 1);
            }
            i += 1;
        }
        return Some(len);
    }
    if bytes[idx] == BYTE_SQUOTE {
        return skip_char_literal(bytes, idx);
    }
    None
}

/// Skips a Rust character literal (`'a'`, `'\n'`, `'\''`) without confusing lifetimes (`'static`).
///
/// # Arguments
///
/// - `&[u8]` - Source bytes.
/// - `usize` - Offset of opening `'`.
///
/// # Returns
///
/// - `Option<usize>` - Byte offset immediately after closing `'` if a char literal was matched.
fn skip_char_literal(bytes: &[u8], idx: usize) -> Option<usize> {
    let len: usize = bytes.len();
    if idx + 2 >= len {
        return None;
    }
    if bytes[idx + 1] == BYTE_BACKSLASH {
        let mut i: usize = idx + 2;
        while i < len && i <= idx + 10 {
            if bytes[i] == BYTE_SQUOTE {
                return Some(i + 1);
            }
            i += 1;
        }
        return None;
    }
    if bytes[idx + 2] == BYTE_SQUOTE {
        return Some(idx + 3);
    }
    None
}

/// Finds the matching closing `}` for an opening `{` at `open_idx`, skipping nested braces, strings, and comments.
///
/// # Arguments
///
/// - `&[u8]` - Source bytes.
/// - `usize` - Byte offset of the opening `{`.
///
/// # Returns
///
/// - `Option<usize>` - Byte offset of the matching `}` if balanced.
fn find_matching_brace(bytes: &[u8], open_idx: usize) -> Option<usize> {
    let len: usize = bytes.len();
    let mut i: usize = open_idx + 1;
    let mut depth: usize = 1;
    while i < len {
        if let Some(next_i) = skip_string_or_comment(bytes, i) {
            i = next_i;
            continue;
        }
        if bytes[i] == BYTE_LBRACE {
            depth += 1;
        } else if bytes[i] == BYTE_RBRACE {
            depth -= 1;
            if depth == 0 {
                return Some(i);
            }
        }
        i += 1;
    }
    None
}

/// Computes the leading space count of the line containing byte offset `pos`.
///
/// # Arguments
///
/// - `&str` - Source text.
/// - `usize` - Byte offset within the line.
///
/// # Returns
///
/// - `usize` - Leading space indentation count.
fn line_leading_spaces(source: &str, pos: usize) -> usize {
    let bytes: &[u8] = source.as_bytes();
    let mut line_start: usize = pos;
    while line_start > 0 && bytes[line_start - 1] != BYTE_NEWLINE {
        line_start -= 1;
    }
    let mut spaces: usize = 0;
    while line_start + spaces < bytes.len() && bytes[line_start + spaces] == BYTE_SPACE {
        spaces += 1;
    }
    spaces
}

/// Returns `true` when `b` can begin a Rust identifier.
///
/// # Arguments
///
/// - `u8` - ASCII byte.
///
/// # Returns
///
/// - `bool` - Whether `b` is alphabetic or `_`.
fn is_ident_start(b: u8) -> bool {
    b.is_ascii_alphabetic() || b == BYTE_UNDERSCORE
}

/// Returns `true` when `b` can continue a Rust identifier.
///
/// # Arguments
///
/// - `u8` - ASCII byte.
///
/// # Returns
///
/// - `bool` - Whether `b` is alphanumeric or `_`.
fn is_ident_continue(b: u8) -> bool {
    b.is_ascii_alphanumeric() || b == BYTE_UNDERSCORE
}
