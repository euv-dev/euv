use super::*;

/// Sets the opening HTML element's language without changing other attributes.
///
/// # Arguments
///
/// - `&str` - The generated HTML document.
/// - `&str` - The active locale's BCP-47 language tag.
///
/// # Returns
///
/// - `Option<String>` - The localized document, or `None` without an opening HTML element.
pub(crate) fn localize_document_html(html: &str, language: &str) -> Option<String> {
    let lower: String = html.to_ascii_lowercase();
    let start: usize = lower.find(DOCUMENT_HTML_TAG)? + DOCUMENT_HTML_TAG.len();
    let bytes: &[u8] = html.as_bytes();
    if !bytes
        .get(start)
        .is_some_and(|byte: &u8| byte.is_ascii_whitespace() || *byte == DOCUMENT_TAG_END)
    {
        return None;
    }
    let mut cursor: usize = start;
    loop {
        while bytes.get(cursor).is_some_and(u8::is_ascii_whitespace) {
            cursor += 1;
        }
        let name_start: usize = cursor;
        match *bytes.get(cursor)? {
            DOCUMENT_TAG_END | DOCUMENT_TAG_SLASH => {
                return Some(format!(
                    r#"{} lang="{}"{}"#,
                    &html[..cursor],
                    language,
                    &html[cursor..]
                ));
            }
            _ => {}
        }
        while bytes.get(cursor).is_some_and(|byte: &u8| {
            !byte.is_ascii_whitespace()
                && ![
                    DOCUMENT_ATTRIBUTE_EQUALS,
                    DOCUMENT_TAG_END,
                    DOCUMENT_TAG_SLASH,
                ]
                .contains(byte)
        }) {
            cursor += 1;
        }
        if cursor == name_start {
            return None;
        }
        let name_end: usize = cursor;
        while bytes.get(cursor).is_some_and(u8::is_ascii_whitespace) {
            cursor += 1;
        }
        if bytes.get(cursor) == Some(&DOCUMENT_ATTRIBUTE_EQUALS) {
            cursor += 1;
            while bytes.get(cursor).is_some_and(u8::is_ascii_whitespace) {
                cursor += 1;
            }
            let quote: u8 = *bytes.get(cursor)?;
            if [DOCUMENT_QUOTE_DOUBLE, DOCUMENT_QUOTE_SINGLE].contains(&quote) {
                cursor += 1;
                cursor += bytes
                    .get(cursor..)?
                    .iter()
                    .position(|byte: &u8| *byte == quote)?
                    + 1;
            } else {
                while bytes.get(cursor).is_some_and(|byte: &u8| {
                    !byte.is_ascii_whitespace() && *byte != DOCUMENT_TAG_END
                }) {
                    cursor += 1;
                }
            }
        }
        if html[name_start..name_end].eq_ignore_ascii_case(DOCUMENT_HTML_LANGUAGE_ATTRIBUTE) {
            return Some(format!(
                r#"{}lang="{}"{}"#,
                &html[..name_start],
                language,
                &html[cursor..]
            ));
        }
    }
}

/// Writes the active document language into the index and its static-host fallback.
///
/// # Arguments
///
/// - `&Path` - The completed locale bundle's output directory.
///
/// # Returns
///
/// - `Result<(), String>` - Success once both localized HTML files are written.
pub(crate) fn localize_document_output(out_dir: &Path) -> Result<(), String> {
    let index_html: PathBuf = out_dir.join(DOCUMENT_INDEX_FILE_NAME);
    if !index_html.is_file() {
        return Ok(());
    }
    let language_path: PathBuf = out_dir
        .join(DOCUMENT_DEPLOY_DIR)
        .join(DOCUMENT_LANGUAGE_FILE_NAME);
    let language: String = fs::read_to_string(&language_path).map_err(|error: io::Error| {
        format!(
            "read document language {}: {error}",
            language_path.display()
        )
    })?;
    let html: String = fs::read_to_string(&index_html)
        .map_err(|error: io::Error| format!("read index.html: {error}"))?;
    let localized: String = localize_document_html(&html, language.trim())
        .ok_or_else(|| DOCUMENT_MISSING_HTML_ERROR.to_string())?;
    fs::write(&index_html, localized)
        .map_err(|error: io::Error| format!("localize index.html: {error}"))?;
    fs::copy(&index_html, out_dir.join(DOCUMENT_NOT_FOUND_FILE_NAME))
        .map_err(|error: io::Error| format!("copy 404.html: {error}"))?;
    Ok(())
}
