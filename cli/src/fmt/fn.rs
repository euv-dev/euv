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
    let bytes: &[u8] = source.as_bytes();
    let len: usize = bytes.len();
    let mut out: String = String::with_capacity(len + 64);
    let mut idx: usize = 0;
    let mut count: usize = 0;
    while idx < len {
        if let Some(next_idx) = skip_string_or_comment(bytes, idx) {
            out.push_str(&source[idx..next_idx]);
            idx = next_idx;
            continue;
        }
        if is_ident_start(bytes[idx]) && (idx == 0 || !is_ident_continue(bytes[idx - 1])) {
            let ident_start: usize = idx;
            let mut ident_end: usize = idx + 1;
            while ident_end < len && is_ident_continue(bytes[ident_end]) {
                ident_end += 1;
            }
            let ident: &str = &source[ident_start..ident_end];
            if EUV_FMT_MACRO_NAMES.contains(&ident) && ident_end < len && bytes[ident_end] == b'!' {
                let mut probe: usize = ident_end + 1;
                while probe < len && (bytes[probe] == b' ' || bytes[probe] == b'\t') {
                    probe += 1;
                }
                if probe < len
                    && bytes[probe] == b'{'
                    && let Some(close_brace) = find_matching_brace(bytes, probe)
                {
                    let inner: &str = &source[(probe + 1)..close_brace];
                    let base_indent: usize = line_leading_spaces(source, ident_start);
                    let formatted_block: String = format_macro_brace_body(inner, base_indent);
                    out.push_str(ident);
                    out.push_str("! ");
                    out.push_str(&formatted_block);
                    if formatted_block != source[probe..=close_brace]
                        || &source[(ident_end + 1)..probe] != " "
                    {
                        count += 1;
                    }
                    idx = close_brace + 1;
                    continue;
                }
            }
            out.push_str(&source[ident_start..ident_end]);
            idx = ident_end;
            continue;
        }
        let ch_len: usize = source[idx..]
            .chars()
            .next()
            .map(char::len_utf8)
            .unwrap_or(1);
        out.push_str(&source[idx..(idx + ch_len)]);
        idx += ch_len;
    }
    (out, count)
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
        let (formatted, macro_hits): (String, usize) = format_euv_macros_in_source(&original);
        if formatted != original {
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

/// Formats the contents inside `{ ... }` of an `euv` macro invocation with exact 4-space nested indentation.
///
/// # Arguments
///
/// - `&str` - Raw interior slice between `{` and `}`.
/// - `usize` - Base indentation in spaces of the macro call line.
///
/// # Returns
///
/// - `String` - Cleanly formatted `{ ... }` block including outer braces.
fn format_macro_brace_body(inner: &str, base_indent: usize) -> String {
    if !inner.contains('\n') {
        let trimmed: &str = inner.trim();
        if trimmed.is_empty() {
            return "{}".to_string();
        }
        return format!("{{ {trimmed} }}");
    }
    let mut out: String = String::with_capacity(inner.len() + 64);
    out.push_str("{\n");
    let mut indent_stack: Vec<usize> = Vec::new();
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
        let (events, next_comment, next_raw): (Vec<bool>, bool, Option<usize>) =
            scan_line_delimiter_events(trimmed, in_block_comment);
        in_block_comment = next_comment;
        raw_hashes = next_raw;
        let leading_closes: usize = events
            .iter()
            .take_while(|is_open: &&bool| !**is_open)
            .count();
        for _ in 0..leading_closes {
            let _: Option<usize> = indent_stack.pop();
        }
        let visual_depth: usize = 1 + indent_stack.iter().copied().sum::<usize>();
        let indent_spaces: usize = base_indent + visual_depth * EUV_FMT_INDENT_SPACES;
        for _ in 0..indent_spaces {
            out.push(' ');
        }
        out.push_str(trimmed);
        out.push('\n');
        let mut line_opens: usize = 0;
        for is_open in events.into_iter().skip(leading_closes) {
            if is_open {
                line_opens += 1;
            } else if line_opens > 0 {
                line_opens -= 1;
            } else {
                let _: Option<usize> = indent_stack.pop();
            }
        }
        for open_idx in 0..line_opens {
            indent_stack.push(usize::from(open_idx == 0));
        }
    }
    for _ in 0..base_indent {
        out.push(' ');
    }
    out.push('}');
    out
}

/// Scans a single trimmed line for ordered delimiter events (`true` for `{`/`(`/ `[`, `false` for `}`/`)`/`]`) outside strings and comments.
///
/// # Arguments
///
/// - `&str` - Trimmed source line.
/// - `bool` - Whether a `/* ... */` block comment was already open on entry.
///
/// # Returns
///
/// - `(Vec<bool>, bool, Option<usize>)` - `(ordered_events, next_in_block_comment, unclosed_raw_string_hashes)`.
fn scan_line_delimiter_events(
    line: &str,
    mut in_block_comment: bool,
) -> (Vec<bool>, bool, Option<usize>) {
    let bytes: &[u8] = line.as_bytes();
    let len: usize = bytes.len();
    let mut i: usize = 0;
    let mut events: Vec<bool> = Vec::new();
    while i < len {
        if in_block_comment {
            if bytes[i] == b'*' && i + 1 < len && bytes[i + 1] == b'/' {
                in_block_comment = false;
                i += 2;
            } else {
                i += 1;
            }
            continue;
        }
        if bytes[i] == b'/' && i + 1 < len && bytes[i + 1] == b'/' {
            break;
        }
        if bytes[i] == b'/' && i + 1 < len && bytes[i + 1] == b'*' {
            in_block_comment = true;
            i += 2;
            continue;
        }
        if bytes[i] == b'r'
            && let Some((end_pos, hashes)) = parse_raw_string_on_line(bytes, i)
        {
            match end_pos {
                Some(next_i) => {
                    i = next_i;
                    continue;
                }
                None => return (events, in_block_comment, Some(hashes)),
            }
        }
        if bytes[i] == b'"' {
            i += 1;
            while i < len {
                if bytes[i] == b'\\' && i + 1 < len {
                    i += 2;
                    continue;
                }
                if bytes[i] == b'"' {
                    i += 1;
                    break;
                }
                i += 1;
            }
            continue;
        }
        if bytes[i] == b'\''
            && let Some(next_i) = skip_char_literal(bytes, i)
        {
            i = next_i;
            continue;
        }
        match bytes[i] {
            b'{' | b'(' | b'[' => events.push(true),
            b'}' | b')' | b']' => events.push(false),
            _ => {}
        }
        i += 1;
    }
    (events, in_block_comment, None)
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
        if bytes[i] == b'"' {
            let mut h: usize = 0;
            while i + 1 + h < len && bytes[i + 1 + h] == b'#' && h < hashes {
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
    while i < len && bytes[i] == b'#' {
        hashes += 1;
        i += 1;
    }
    if i >= len || bytes[i] != b'"' {
        return None;
    }
    i += 1;
    while i < len {
        if bytes[i] == b'"' {
            let mut h: usize = 0;
            while i + 1 + h < len && bytes[i + 1 + h] == b'#' && h < hashes {
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
    if bytes[idx] == b'/' && idx + 1 < len && bytes[idx + 1] == b'/' {
        let mut i: usize = idx + 2;
        while i < len && bytes[i] != b'\n' {
            i += 1;
        }
        return Some(i);
    }
    if bytes[idx] == b'/' && idx + 1 < len && bytes[idx + 1] == b'*' {
        let mut i: usize = idx + 2;
        let mut nest: usize = 1;
        while i + 1 < len && nest > 0 {
            if bytes[i] == b'/' && bytes[i + 1] == b'*' {
                nest += 1;
                i += 2;
            } else if bytes[i] == b'*' && bytes[i + 1] == b'/' {
                nest -= 1;
                i += 2;
            } else {
                i += 1;
            }
        }
        return Some(i);
    }
    if bytes[idx] == b'r' && (idx == 0 || !is_ident_continue(bytes[idx - 1])) {
        let mut i: usize = idx + 1;
        let mut hashes: usize = 0;
        while i < len && bytes[i] == b'#' {
            hashes += 1;
            i += 1;
        }
        if i < len && bytes[i] == b'"' {
            i += 1;
            while i < len {
                if bytes[i] == b'"' {
                    let mut h: usize = 0;
                    while i + 1 + h < len && bytes[i + 1 + h] == b'#' && h < hashes {
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
    if bytes[idx] == b'"' {
        let mut i: usize = idx + 1;
        while i < len {
            if bytes[i] == b'\\' && i + 1 < len {
                i += 2;
                continue;
            }
            if bytes[i] == b'"' {
                return Some(i + 1);
            }
            i += 1;
        }
        return Some(len);
    }
    if bytes[idx] == b'\'' {
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
    if bytes[idx + 1] == b'\\' {
        let mut i: usize = idx + 2;
        while i < len && i <= idx + 10 {
            if bytes[i] == b'\'' {
                return Some(i + 1);
            }
            i += 1;
        }
        return None;
    }
    if bytes[idx + 2] == b'\'' {
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
        if bytes[i] == b'{' {
            depth += 1;
        } else if bytes[i] == b'}' {
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
    while line_start > 0 && bytes[line_start - 1] != b'\n' {
        line_start -= 1;
    }
    let mut spaces: usize = 0;
    while line_start + spaces < bytes.len() && bytes[line_start + spaces] == b' ' {
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
    b.is_ascii_alphabetic() || b == b'_'
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
    b.is_ascii_alphanumeric() || b == b'_'
}
