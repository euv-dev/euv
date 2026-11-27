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
