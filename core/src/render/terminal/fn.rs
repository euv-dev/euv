use super::*;

/// Returns `true` when `ch` is a zero-width combining mark, format control, or variation selector.
///
/// # Arguments
///
/// - `char` - Unicode scalar value to classify.
///
/// # Returns
///
/// - `bool` - `true` when the scalar occupies zero terminal columns.
#[must_use]
pub const fn is_zero_width_combining(ch: char) -> bool {
    let code: u32 = ch as u32;
    matches!(
        code,
        0x0300..=0x036F
            | 0x0483..=0x0489
            | 0x0591..=0x05BD
            | 0x05BF
            | 0x05C1..=0x05C2
            | 0x05C4..=0x05C5
            | 0x05C7
            | 0x0610..=0x061A
            | 0x064B..=0x065F
            | 0x0670
            | 0x06D6..=0x06DC
            | 0x06DF..=0x06E4
            | 0x06E7..=0x06E8
            | 0x06EA..=0x06ED
            | 0x1AB0..=0x1AFF
            | 0x1DC0..=0x1DFF
            | 0x200B..=0x200F
            | 0x202A..=0x202E
            | 0x2060..=0x206F
            | 0x20D0..=0x20FF
            | 0xFE00..=0xFE0F
            | 0xFE20..=0xFE2F
            | 0xFEFF
            | 0xE0100..=0xE01EF
    )
}

/// Returns `true` when `ch` occupies two terminal columns (CJK ideographs, Hangul, fullwidth forms, and wide emoji).
///
/// # Arguments
///
/// - `char` - Unicode scalar value to classify.
///
/// # Returns
///
/// - `bool` - `true` when the scalar renders as double-width in standard terminals.
#[must_use]
pub const fn is_wide_char(ch: char) -> bool {
    let code: u32 = ch as u32;
    matches!(
        code,
        0x1100..=0x115F
            | 0x2329..=0x232A
            | 0x2E80..=0x303E
            | 0x3041..=0x33FF
            | 0x3400..=0x4DBF
            | 0x4E00..=0xA4CF
            | 0xA960..=0xA97F
            | 0xAC00..=0xD7A3
            | 0xF900..=0xFAFF
            | 0xFE10..=0xFE19
            | 0xFE30..=0xFE6F
            | 0xFF01..=0xFF60
            | 0xFFE0..=0xFFE6
            | 0x1F300..=0x1FAFF
            | 0x20000..=0x3FFFD
    )
}

/// Computes the East-Asian / Unicode terminal cell display width of `ch` (`0`, `1`, or `2`).
///
/// # Arguments
///
/// - `char` - Unicode scalar value to measure.
///
/// # Returns
///
/// - `u8` - Display column width (`0` for control/combining marks, `2` for wide CJK/fullwidth/emoji, `1` otherwise).
#[must_use]
pub const fn terminal_char_width(ch: char) -> u8 {
    let code: u32 = ch as u32;
    if code <= 0x1F || (code >= 0x7F && code <= 0x9F) || is_zero_width_combining(ch) {
        0
    } else if is_wide_char(ch) {
        2
    } else {
        1
    }
}

/// Computes total terminal column width of a UTF-8 string across all scalar values.
///
/// # Arguments
///
/// - `&str` - UTF-8 text slice to measure.
///
/// # Returns
///
/// - `usize` - Sum of terminal display column widths.
#[must_use]
pub fn terminal_str_width(text: &str) -> usize {
    let mut total: usize = 0;
    for ch in text.chars() {
        total = total.saturating_add(terminal_char_width(ch) as usize);
    }
    total
}

/// Appends an ANSI cursor-positioning CSI sequence (`ESC[{row+1};{col+1}H`) into `out`.
///
/// # Arguments
///
/// - `&mut String` - Destination ANSI output buffer.
/// - `u16` - Zero-based column coordinate.
/// - `u16` - Zero-based row coordinate.
pub(crate) fn append_cursor_move(out: &mut String, col: u16, row: u16) {
    let row_one: u32 = (row as u32) + 1;
    let col_one: u32 = (col as u32) + 1;
    out.push_str(&format!("{ANSI_CSI_PREFIX}{row_one};{col_one}H"));
}

/// Appends an ANSI SGR styling sequence for foreground, background, bold, reverse, and underline flags into `out`.
///
/// # Arguments
///
/// - `&mut String` - Destination ANSI output buffer.
/// - `Option<Color>` - Optional foreground 24-bit color.
/// - `Option<Color>` - Optional background 24-bit color.
/// - `bool` - Bold flag.
/// - `bool` - Reverse-video flag.
/// - `bool` - Underline flag.
pub(crate) fn append_sgr_style(
    out: &mut String,
    fg: Option<Color>,
    bg: Option<Color>,
    bold: bool,
    reverse: bool,
    underline: bool,
) {
    out.push_str(ANSI_CSI_PREFIX);
    out.push('0');
    if bold {
        out.push_str(ANSI_BOLD_CODE);
    }
    if reverse {
        out.push_str(ANSI_REVERSE_CODE);
    }
    if underline {
        out.push_str(ANSI_UNDERLINE_CODE);
    }
    if let Some(color) = fg {
        let r: u8 = color.get_r();
        let g: u8 = color.get_g();
        let b: u8 = color.get_b();
        out.push_str(&format!("{ANSI_FG_TRUECOLOR_PREFIX}{r};{g};{b}"));
    }
    if let Some(color) = bg {
        let r: u8 = color.get_r();
        let g: u8 = color.get_g();
        let b: u8 = color.get_b();
        out.push_str(&format!("{ANSI_BG_TRUECOLOR_PREFIX}{r};{g};{b}"));
    }
    out.push('m');
}
