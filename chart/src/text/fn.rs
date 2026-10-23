use super::*;

/// Splits a rich-text string into its plain and styled runs.
///
/// ECharts writes rich text as `{styleName|content}`; a run without a
/// style name is plain text. The content itself may contain braces, so
/// the scan tracks the nesting depth rather than splitting on the first
/// closing brace.
///
/// # Arguments
///
/// - `&str` - Shared reference to a `str`.
///
/// # Returns
///
/// - `Vec<(Option<String>, String)>` - A `Vec<(Option<String>, String)>` value.
pub(crate) fn split_rich_text(text: &str) -> Vec<(Option<String>, String)> {
    let mut runs: Vec<(Option<String>, String)> = Vec::new();
    let mut current: String = String::new();
    let mut current_style: Option<String> = None;
    let mut characters: Peekable<Chars<'_>> = text.chars().peekable();

    while let Some(character) = characters.next() {
        if character != '{' {
            if current_style.is_none() {
                current.push(character);
            }
            continue;
        }
        // A run starts: read the style name up to the first bar.
        let mut name: String = String::new();
        let mut found_bar: bool = false;
        for next in characters.by_ref() {
            if next == '|' {
                found_bar = true;
                break;
            }
            if next == '}' {
                break;
            }
            name.push(next);
        }
        if !found_bar {
            // Not a style run after all: keep the braces as literal text.
            current.push('{');
            current.push_str(&name);
            continue;
        }
        // Read the content up to the matching closing brace.
        let mut content: String = String::new();
        let mut depth: usize = 1usize;
        for next in characters.by_ref() {
            if next == '{' {
                depth += 1;
            } else if next == '}' {
                depth -= 1;
                if depth == 0 {
                    break;
                }
            }
            content.push(next);
        }
        if !current.is_empty() {
            runs.push((current_style.clone(), take(&mut current)));
        }
        if current_style.is_none() {
            current_style = Some(name);
        }
        current.push_str(&content);
    }

    if !current.is_empty() {
        runs.push((current_style, current));
    }
    runs
}

/// Extracts the font size, in pixels, from a CSS `font` shorthand.
///
/// The measurer needs the size to compute the line height, and the
/// browser's `measure_text` does not report it.
///
/// # Arguments
///
/// - `&str` - Shared reference to a `str`.
///
/// # Returns
///
/// - `f64` - A 64-bit float.
pub(crate) fn font_size_of(font: &str) -> f64 {
    let parts: SplitWhitespace<'_> = font.split_whitespace();
    for part in parts {
        if part.ends_with("px")
            && let Ok(size) = part[..part.len() - 2].parse::<f64>()
        {
            return size;
        }
    }
    12.0
}

/// Estimates the metrics of a run of text.
///
/// The width is the average advance of a Latin character times the font
/// size, which is the estimate a browser's own layout uses before the
/// font is loaded.
///
/// # Arguments
///
/// - `&str` - Shared reference to a `str`.
/// - `f64` - A 64-bit float (`f64`).
///
/// # Returns
///
/// - `TextMetrics` - A `TextMetrics` value.
pub(crate) fn approximate_metrics(text: &str, size: f64) -> TextMetrics {
    let width: f64 = text.chars().count() as f64 * size * 0.6;
    TextMetrics {
        width,
        height: size * 1.2,
        ascent: size * 0.8,
        descent: size * 0.2,
        line_height: size * 1.2,
    }
}
