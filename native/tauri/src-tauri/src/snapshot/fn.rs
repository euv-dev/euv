use super::*;

/// Checks every static path before reusing a previously published snapshot.
///
/// # Arguments
/// - `&Path` - Candidate active snapshot directory.
/// - `&[String]` - Required static file inventory.
///
/// # Returns
/// - `bool` - Whether the entry page and all required resources are nonempty files.
pub(crate) async fn complete_snapshot(directory: &Path, paths: &[String]) -> bool {
    for path in once(INDEX_FILE_NAME).chain(paths.iter().map(String::as_str)) {
        if !metadata(directory.join(path))
            .await
            .is_ok_and(|entry: Metadata| entry.is_file() && entry.len() > 0)
        {
            return false;
        }
    }
    true
}

/// Lists independently navigable documents absent from the root resource graph.
///
/// Theme entry points are selected from WASM rather than HTML links. Discover
/// their dependencies from the remote documents instead of pinning downloads to
/// stale filenames or unrelated development output in the local build directory.
///
/// # Arguments
/// - `&[(&str, &[u8])]` - The complete embedded web output inventory.
///
/// # Returns
/// - `Vec<String>` - Root and theme documents whose dependencies must be fetched.
pub(crate) fn snapshot_paths(files: &[(&str, &[u8])]) -> Vec<String> {
    files
        .iter()
        .filter(|(path, _): &&(&str, &[u8])| {
            *path == INDEX_FILE_NAME
                || (path.starts_with(SNAPSHOT_THEMES_PREFIX) && path.ends_with(HTML_EXTENSION))
        })
        .map(|(path, _): &(&str, &[u8])| (*path).to_string())
        .collect()
}

/// Rebases a web deployment's document base onto its local document directory.
///
/// Theme pages must retain their own directory rather than resolving their WASM
/// against the root page or an HTTP deployment prefix absent from the snapshot.
///
/// # Arguments
/// - `&str` - The original HTML document.
///
/// # Returns
/// - `String` - HTML whose base tags resolve relative to the served document.
pub(crate) fn local_document(html: &str) -> String {
    let lower: String = html.to_ascii_lowercase();
    let mut result: String = String::with_capacity(html.len());
    let mut cursor: usize = 0;
    while let Some(relative) = lower[cursor..].find("<base") {
        let start: usize = cursor + relative;
        let boundary: Option<u8> = lower.as_bytes().get(start + 5).copied();
        if !boundary.is_some_and(|byte: u8| byte.is_ascii_whitespace() || byte == b'>') {
            result.push_str(&html[cursor..start + 5]);
            cursor = start + 5;
            continue;
        }
        let Some(end): Option<usize> = lower[start..].find('>') else {
            break;
        };
        result.push_str(&html[cursor..start]);
        result.push_str("<base href=\"./\" />");
        cursor = start + end + 1;
    }
    result.push_str(&html[cursor..]);
    result
}
