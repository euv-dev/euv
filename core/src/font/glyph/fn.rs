use super::*;

/// Looks up the raw 5x7 bitmap rows for a Unicode character.
///
/// # Arguments
///
/// - `char` - Character to query.
///
/// # Returns
///
/// - `[u8; BITMAP_GLYPH_HEIGHT]` - Seven 5-bit row bitmasks.
#[must_use]
pub fn glyph_bitmap_rows(ch: char) -> [u8; BITMAP_GLYPH_HEIGHT] {
    let code: u32 = ch as u32;
    if (ASCII_FIRST_PRINTABLE..=ASCII_LAST_PRINTABLE).contains(&code) {
        ASCII_FONT_5X7[(code - ASCII_FIRST_PRINTABLE) as usize]
    } else {
        FALLBACK_GLYPH_ROWS
    }
}

/// Computes deterministic logical advance width for a character under `style`.
///
/// # Arguments
///
/// - `char` - Character to measure.
/// - `TextStyle` - Active typography style.
///
/// # Returns
///
/// - `f32` - Advance width in logical pixels.
#[must_use]
pub fn measure_char_advance(ch: char, style: TextStyle) -> f32 {
    let is_mono: bool = style.get_family() == FontFamily::Monospace;
    let bold_factor: f32 = if is_mono {
        1.0
    } else if style.get_weight().is_bold() {
        1.04
    } else if matches!(style.get_weight(), FontWeight::Medium) {
        1.02
    } else {
        1.0
    };
    let base_ratio: f32 = match style.get_family() {
        FontFamily::Monospace => {
            if is_wide_unicode(ch) {
                1.04
            } else {
                MONOSPACE_ADVANCE_RATIO
            }
        }
        FontFamily::Sans | FontFamily::Serif => {
            if is_wide_unicode(ch) {
                1.04
            } else if (ch as u32) > 0x7F {
                0.76
            } else {
                match ch {
                    'i' | 'l' | 'j' | '!' | '.' | ',' | ':' | ';' | '|' | '\'' => 0.29,
                    'I' | 'f' | 'r' | 't' | '(' | ')' | '[' | ']' | '/' => 0.37,
                    ' ' => 0.30,
                    '-' | '"' => 0.40,
                    'J' | 'L' | 's' | 'c' | 'z' | 'v' | 'x' => 0.52,
                    'm' | 'w' => 0.84,
                    'M' | 'W' => 0.90,
                    'O' | 'Q' | 'G' | 'D' | 'H' | 'N' | 'U' | 'A' | 'B' | 'C' | 'K' | 'R' | 'V'
                    | 'X' | 'Y' => 0.72,
                    'E' | 'F' | 'P' | 'S' | 'T' | 'Z' => 0.64,
                    '0'..='9' => 0.58,
                    _ => PROPORTIONAL_ADVANCE_RATIO,
                }
            }
        }
    };
    style.get_font_size() * base_ratio * bold_factor + style.get_letter_spacing()
}

/// Rasterizes a glyph into a smooth alpha-coverage bitmap at the requested key size.
///
/// # Arguments
///
/// - `GlyphKey` - Quantized glyph descriptor.
///
/// # Returns
///
/// - `RasterizedGlyph` - Coverage bitmap and pixel metrics.
#[must_use]
pub fn rasterize_glyph(key: GlyphKey) -> RasterizedGlyph {
    let code: u32 = key.get_codepoint() as u32;
    if code > ASCII_LAST_PRINTABLE {
        return rasterize_unicode_deterministic(key);
    }
    let size_px: f32 = (key.get_size_tenths() as f32) * 0.1;
    let mut prop_style: TextStyle = TextStyle::default();
    prop_style
        .set_font_size(size_px)
        .set_family(if key.get_monospace() {
            FontFamily::Monospace
        } else {
            FontFamily::Sans
        })
        .set_weight(if key.get_bold() {
            FontWeight::Bold
        } else {
            FontWeight::Normal
        });
    let target_adv: f32 = measure_char_advance(key.get_codepoint(), prop_style);
    let out_h: u32 = (size_px * 0.86).round().max(7.0) as u32;
    let advance: u32 = target_adv.ceil().max(4.0) as u32;
    let out_w: u32 = (target_adv * 0.86).round().max(3.0).min(advance as f32) as u32;
    let rows: [u8; BITMAP_GLYPH_HEIGHT] = glyph_bitmap_rows(key.get_codepoint());
    let bold: bool = key.get_bold();
    let italic: bool = key.get_italic();
    let total_pixels: usize = (out_w as usize) * (out_h as usize);
    let mut coverage: Vec<u8> = vec![0; total_pixels];
    let src_w: f32 = BITMAP_GLYPH_WIDTH as f32;
    let src_h: f32 = BITMAP_GLYPH_HEIGHT as f32;
    let dst_w: f32 = out_w as f32;
    let dst_h: f32 = out_h as f32;
    let sub_offsets: [(f32, f32); 4] = [(0.25, 0.25), (0.75, 0.25), (0.25, 0.75), (0.75, 0.75)];
    for py in 0..out_h {
        let row_off: usize = (py as usize) * (out_w as usize);
        let italic_shift: f32 = if italic {
            (0.5 - (py as f32) / dst_h.max(1.0)) * 0.85
        } else {
            0.0
        };
        for px in 0..out_w {
            let mut hits: u16 = 0;
            for (ox, oy) in sub_offsets.into_iter() {
                let sy: f32 = ((py as f32) + oy) * src_h / dst_h;
                let sx: f32 =
                    (((px as f32) + ox) * src_w / dst_w - italic_shift).clamp(0.0, src_w - 0.01);
                let row_idx: usize = (sy.floor() as usize).min(BITMAP_GLYPH_HEIGHT - 1);
                let col_idx: usize = (sx.floor() as usize).min(BITMAP_GLYPH_WIDTH - 1);
                let row_bits: u8 = rows[row_idx];
                let bit_pos: usize = BITMAP_GLYPH_WIDTH - 1 - col_idx;
                let hit: bool = ((row_bits >> bit_pos) & 1) != 0;
                let bold_hit: bool = if bold && col_idx > 0 {
                    let prev_bit: usize = BITMAP_GLYPH_WIDTH - col_idx;
                    ((row_bits >> prev_bit) & 1) != 0
                } else {
                    false
                };
                if hit || bold_hit {
                    hits += 1;
                }
            }
            if hits > 0 {
                coverage[row_off + (px as usize)] = ((hits * 255) / 4) as u8;
            }
        }
    }
    RasterizedGlyph::new(out_w, out_h, advance, coverage)
}
