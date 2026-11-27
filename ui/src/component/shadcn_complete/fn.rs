/// Returns `true` when pre-lowercased `haystack` contains the pre-lowercased `needle_lower` byte slice.
///
/// # Arguments
///
/// - `&[u8]` - Pre-lowercased haystack bytes (valid UTF-8 segments optionally separated by `0xFF`).
/// - `&[u8]` - Pre-lowercased UTF-8 needle bytes.
///
/// # Returns
///
/// - `bool` - `true` when `needle_lower` occurs contiguously inside `haystack`.
#[must_use]
pub(crate) fn contains_ascii_lowered_needle(haystack: &[u8], needle_lower: &[u8]) -> bool {
    let n_len: usize = needle_lower.len();
    if n_len == 0 {
        return true;
    }
    if n_len > haystack.len() {
        return false;
    }
    let first: u8 = needle_lower[0];
    let tail: &[u8] = &needle_lower[1..];
    let limit: usize = haystack.len() - n_len;
    let mut pos: usize = 0;
    while pos <= limit {
        if haystack[pos] == first && haystack[pos + 1..pos + n_len] == *tail {
            return true;
        }
        pos += 1;
    }
    false
}

/// Returns `true` when `haystack` contains `needle` using zero-allocation ASCII case-insensitive matching.
///
/// # Arguments
///
/// - `&str` - Source UTF-8 haystack text.
/// - `&str` - UTF-8 search query substring.
///
/// # Returns
///
/// - `bool` - `true` when `needle` occurs inside `haystack` ignoring ASCII letter case.
#[must_use]
pub(crate) fn contains_ascii_ignore_case(haystack: &str, needle: &str) -> bool {
    let h: &[u8] = haystack.as_bytes();
    let n: &[u8] = needle.as_bytes();
    let n_len: usize = n.len();
    if n_len == 0 {
        return true;
    }
    if n_len > h.len() {
        return false;
    }
    let first: u8 = n[0].to_ascii_lowercase();
    let tail: &[u8] = &n[1..];
    let limit: usize = h.len() - n_len;
    let mut pos: usize = 0;
    while pos <= limit {
        if h[pos].to_ascii_lowercase() == first
            && h[pos + 1..pos + n_len].eq_ignore_ascii_case(tail)
        {
            return true;
        }
        pos += 1;
    }
    false
}
