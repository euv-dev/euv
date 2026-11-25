use super::*;

/// Computes the Levenshtein edit distance between two ASCII property names.
///
/// # Arguments
///
/// - `&str` - First string slice.
/// - `&str` - Second string slice.
///
/// # Returns
///
/// - `usize` - Minimum single-character edits required to transform `a` into `b`.
pub(crate) fn edit_distance(a: &str, b: &str) -> usize {
    let a_bytes: &[u8] = a.as_bytes();
    let b_bytes: &[u8] = b.as_bytes();
    let m: usize = a_bytes.len();
    let n: usize = b_bytes.len();
    if m == 0 {
        return n;
    }
    if n == 0 {
        return m;
    }
    let mut prev: Vec<usize> = (0..=n).collect();
    let mut curr: Vec<usize> = vec![0; n + 1];
    for i in 1..=m {
        curr[0] = i;
        for j in 1..=n {
            let cost: usize = if a_bytes[i - 1] == b_bytes[j - 1] {
                0
            } else {
                1
            };
            let del: usize = prev[j] + 1;
            let ins: usize = curr[j - 1] + 1;
            let sub: usize = prev[j - 1] + cost;
            curr[j] = del.min(ins).min(sub);
        }
        prev.copy_from_slice(&curr);
    }
    prev[n]
}

/// Finds the closest known property candidate to `input` for friendly compiler suggestions.
///
/// # Arguments
///
/// - `&str` - Unknown identifier entered by the user.
/// - `&[&'a str]` - Exhaustive list of valid property names.
///
/// # Returns
///
/// - `Option<&'a str>` - Closest candidate within edit distance 2 (or prefix match), if any.
pub(crate) fn suggest_closest<'a>(input: &str, candidates: &[&'a str]) -> Option<&'a str> {
    let mut best: Option<(&'a str, usize)> = None;
    for candidate in candidates.iter().copied() {
        let dist: usize = edit_distance(input, candidate);
        let is_prefix: bool =
            input.len() >= 2 && (candidate.starts_with(input) || input.starts_with(candidate));
        if dist <= 2 || is_prefix {
            match best {
                Some((_, best_dist)) if dist >= best_dist => {}
                _ => {
                    best = Some((candidate, dist));
                }
            }
        }
    }
    best.map(|(name, _): (&'a str, usize)| name)
}

/// Builds a span-accurate `syn::Error` for an unknown macro property with a typo suggestion and valid list.
///
/// # Arguments
///
/// - `&Ident` - Offending property identifier carrying its exact source span.
/// - `&str` - Macro context label (e.g. `"view! element \`row\`"` or `"style!"`).
/// - `&[&str]` - List of supported property names.
///
/// # Returns
///
/// - `Error` - Actionable compiler error pointing directly at `ident`.
pub(crate) fn unknown_prop_error(ident: &Ident, scope: &str, candidates: &[&str]) -> Error {
    let name: String = ident.to_string();
    let supported: String = candidates.join(", ");
    let msg: String = match suggest_closest(&name, candidates) {
        Some(suggestion) => format!(
            "unknown property `{name}` in {scope} — did you mean `{suggestion}`? Supported properties: {supported}"
        ),
        None => format!("unknown property `{name}` in {scope}. Supported properties: {supported}"),
    };
    Error::new(ident.span(), msg)
}
