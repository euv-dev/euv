use super::*;

/// Maps each wrapped `TextLine` back to its inclusive-start, exclusive-end character indices inside `text`.
///
/// # Arguments
///
/// - `&str` - Original source UTF-8 text.
/// - `&TextLayoutResult` - Wrapped layout produced by `DeterministicTextMeasurer`.
///
/// # Returns
///
/// - `Vec<(usize, usize, f32)>` - Ordered `(start_char, end_char, line_width)` tuples for each line.
#[must_use]
pub(crate) fn map_text_lines_to_char_ranges(
    text: &str,
    layout: &TextLayoutResult,
) -> Vec<(usize, usize, f32)> {
    let chars: Vec<char> = text.chars().collect();
    let total: usize = chars.len();
    let lines: &[TextLine] = layout.get_lines().as_slice();
    let mut ranges: Vec<(usize, usize, f32)> = Vec::with_capacity(lines.len());
    let mut cursor: usize = 0;
    for (line_idx, line) in lines.iter().enumerate() {
        let line_str: &str = line.get_text().as_str();
        let first_line_char: Option<char> = line_str.chars().next();
        if line_idx > 0 && cursor < total {
            if chars[cursor] == '\n' {
                cursor += 1;
            } else {
                let mut peek: usize = cursor;
                while peek < total && chars[peek] == ' ' {
                    peek += 1;
                }
                if peek < total && chars[peek] == '\n' {
                    cursor = peek + 1;
                } else if first_line_char != Some(' ') {
                    cursor = peek;
                }
            }
        }
        let start_char: usize = cursor.min(total);
        let line_char_len: usize = line_str.chars().count();
        let end_char: usize = start_char.saturating_add(line_char_len).min(total);
        cursor = end_char;
        ranges.push((start_char, end_char, line.get_width()));
    }
    ranges
}

/// Computes the aligned horizontal starting coordinate for a laid-out line inside `rect`.
///
/// # Arguments
///
/// - `Rect` - Logical text content bounding rectangle.
/// - `f32` - Measured logical width of the line.
/// - `TextAlign` - Horizontal text alignment mode.
///
/// # Returns
///
/// - `f32` - Logical X origin where the line's first glyph starts.
#[must_use]
pub(crate) fn line_aligned_origin_x(rect: Rect, line_width: f32, align: TextAlign) -> f32 {
    match align {
        TextAlign::Left => rect.get_x(),
        TextAlign::Center => rect.get_x() + ((rect.get_width() - line_width) * 0.5).max(0.0),
        TextAlign::Right => rect.get_x() + (rect.get_width() - line_width).max(0.0),
    }
}

/// Hit-tests logical pointer coordinates `point` against `text` inside `rect` to return the nearest character boundary index.
///
/// # Arguments
///
/// - `&str` - Source UTF-8 text string.
/// - `TextStyle` - Typography style used for measurement and alignment.
/// - `Rect` - Screen-space logical text content bounding box.
/// - `Point` - Screen-space logical pointer coordinates.
///
/// # Returns
///
/// - `usize` - Nearest character index in `0..=text.chars().count()`.
#[must_use]
pub(crate) fn hit_test_text_char(text: &str, style: TextStyle, rect: Rect, point: Point) -> usize {
    let total_chars: usize = text.chars().count();
    if total_chars == 0 {
        return 0;
    }
    let max_w: f32 = rect.get_width().max(1.0);
    let layout: TextLayoutResult = DeterministicTextMeasurer.measure(text, style, Some(max_w));
    let lines: &[TextLine] = layout.get_lines().as_slice();
    if lines.is_empty() {
        return 0;
    }
    let line_h: f32 = layout.get_line_height_px().max(1.0);
    let ranges: Vec<(usize, usize, f32)> = map_text_lines_to_char_ranges(text, &layout);
    let raw_line: isize = ((point.get_y() - rect.get_y()) / line_h).floor() as isize;
    if raw_line < 0 {
        return 0;
    }
    let max_line: usize = lines.len().saturating_sub(1);
    if (raw_line as usize) > max_line {
        return total_chars;
    }
    let line_idx: usize = raw_line as usize;
    let (start_char, end_char, line_w): (usize, usize, f32) = ranges[line_idx];
    let line_str: &str = lines[line_idx].get_text().as_str();
    let origin_x: f32 = line_aligned_origin_x(rect, line_w, style.get_align());
    if point.get_x() <= origin_x {
        return start_char;
    }
    let mut pen_x: f32 = origin_x;
    for (col_idx, ch) in line_str.chars().enumerate() {
        let adv: f32 = measure_char_advance(ch, style).max(1.0);
        if point.get_x() < pen_x + adv * 0.5 {
            return start_char.saturating_add(col_idx).min(total_chars);
        }
        pen_x += adv;
    }
    end_char.min(total_chars)
}

/// Computes visual highlight rectangles covering the ordered character range `range` within laid-out `text`.
///
/// # Arguments
///
/// - `&str` - Source UTF-8 text string.
/// - `TextStyle` - Typography style matching the rendered `TextRun`.
/// - `Rect` - Logical text content bounding box.
/// - `(usize, usize)` - Ordered `(start_char, end_char)` selection span.
///
/// # Returns
///
/// - `Vec<Rect>` - Per-line logical highlight rectangles in top-to-bottom order.
#[must_use]
pub(crate) fn compute_text_selection_rects(
    text: &str,
    style: TextStyle,
    rect: Rect,
    range: (usize, usize),
) -> Vec<Rect> {
    let (sel_start, sel_end): (usize, usize) = range;
    if sel_start >= sel_end || text.is_empty() {
        return Vec::new();
    }
    let max_w: f32 = rect.get_width().max(1.0);
    let layout: TextLayoutResult = DeterministicTextMeasurer.measure(text, style, Some(max_w));
    let lines: &[TextLine] = layout.get_lines().as_slice();
    if lines.is_empty() {
        return Vec::new();
    }
    let line_h: f32 = layout.get_line_height_px().max(1.0);
    let ranges: Vec<(usize, usize, f32)> = map_text_lines_to_char_ranges(text, &layout);
    let mut out: Vec<Rect> = Vec::new();
    for (line_idx, &(line_start, line_end, line_w)) in ranges.iter().enumerate() {
        let ov_start: usize = sel_start.max(line_start);
        let ov_end: usize = sel_end.min(line_end);
        let origin_x: f32 = line_aligned_origin_x(rect, line_w, style.get_align());
        let line_y: f32 = rect.get_y() + (line_idx as f32) * line_h;
        if ov_start < ov_end {
            let rel_start: usize = ov_start.saturating_sub(line_start);
            let rel_end: usize = ov_end.saturating_sub(line_start);
            let mut x0: f32 = origin_x;
            let mut x1: f32 = origin_x;
            for (col_idx, ch) in lines[line_idx].get_text().chars().enumerate() {
                let adv: f32 = measure_char_advance(ch, style).max(1.0);
                if col_idx < rel_start {
                    x0 += adv;
                }
                if col_idx < rel_end {
                    x1 += adv;
                } else {
                    break;
                }
            }
            let width: f32 = (x1 - x0).max(2.0);
            out.push(Rect::new(x0, line_y, width, line_h));
        } else if line_start == line_end && sel_start <= line_start && sel_end > line_end {
            let empty_w: f32 = (style.get_font_size() * 0.35).max(4.0);
            out.push(Rect::new(origin_x, line_y, empty_w, line_h));
        }
    }
    out
}

/// Classifies a Unicode character into word, whitespace, or punctuation group for double-click word selection.
///
/// # Arguments
///
/// - `char` - Character to classify.
///
/// # Returns
///
/// - `u8` - Class tag (`0` = word/identifier, `1` = whitespace, `2` = punctuation).
#[must_use]
fn selection_char_class(ch: char) -> u8 {
    if ch.is_alphanumeric() || ch == '_' {
        0
    } else if ch.is_whitespace() {
        1
    } else {
        2
    }
}

/// Computes the inclusive-start, exclusive-end word character range enclosing `char_idx` for double-click selection.
///
/// # Arguments
///
/// - `&str` - Source UTF-8 text string.
/// - `usize` - Clicked character index.
///
/// # Returns
///
/// - `(usize, usize)` - Ordered `(start_char, end_char)` range spanning the enclosing word or token.
#[must_use]
pub(crate) fn word_bounds_at_char(text: &str, char_idx: usize) -> (usize, usize) {
    let chars: Vec<char> = text.chars().collect();
    let len: usize = chars.len();
    if len == 0 {
        return (0, 0);
    }
    let anchor: usize = if char_idx >= len {
        len.saturating_sub(1)
    } else {
        char_idx
    };
    let cls: u8 = selection_char_class(chars[anchor]);
    let mut start: usize = anchor;
    while start > 0 && selection_char_class(chars[start - 1]) == cls {
        start -= 1;
    }
    let mut end: usize = anchor + 1;
    while end < len && selection_char_class(chars[end]) == cls {
        end += 1;
    }
    (start, end)
}
