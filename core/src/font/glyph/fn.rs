use super::*;

/// Builds the 128-entry compile-time proportional advance ratio lookup table for ASCII bytes `0..128`.
///
/// # Returns
///
/// - `[f32; 128]` - Proportional width ratio relative to font size for every ASCII codepoint.
#[must_use]
pub(crate) const fn build_ascii_advance_lut() -> [f32; 128] {
    let mut table: [f32; 128] = [PROPORTIONAL_ADVANCE_RATIO; 128];
    let groups: [(&[u8], f32); 9] = [
        (b"ilj!.,:;'", 0.29),
        (b"Ifrt()[]/", 0.37),
        (b"-\"", 0.40),
        (b"JLsczvx", 0.52),
        (b"mw", 0.84),
        (b"MW", 0.90),
        (b"OQGDHNUABCKRVXY", 0.72),
        (b"EFPSTZ", 0.64),
        (b"0123456789", 0.58),
    ];
    let mut g_idx: usize = 0;
    while g_idx < groups.len() {
        let (bytes, ratio): (&[u8], f32) = groups[g_idx];
        let mut idx: usize = 0;
        while idx < bytes.len() {
            table[bytes[idx] as usize] = ratio;
            idx += 1;
        }
        g_idx += 1;
    }
    table[0x7C] = 0.29;
    table[b' ' as usize] = 0.30;
    table
}

/// Looks up the raw 5x7 bitmap rows for a Unicode character.
///
/// # Arguments
///
/// - `char` - Character to query.
///
/// # Returns
///
/// - `[u8; BITMAP_GLYPH_HEIGHT]` - Seven 5-bit row bitmasks.
#[inline(always)]
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
#[inline(always)]
#[must_use]
pub fn measure_char_advance(ch: char, style: TextStyle) -> f32 {
    let code: u32 = ch as u32;
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
    let base_ratio: f32 = if code < 128 {
        if is_mono {
            MONOSPACE_ADVANCE_RATIO
        } else {
            ASCII_ADVANCE_LUT[code as usize]
        }
    } else if is_wide_unicode(ch) {
        1.04
    } else if is_mono {
        MONOSPACE_ADVANCE_RATIO
    } else {
        0.76
    };
    style.get_font_size() * base_ratio * bold_factor + style.get_letter_spacing()
}

/// Applies optional horizontal bold stroke dilation to a 5-bit glyph bitmap row mask.
///
/// # Arguments
///
/// - `u8` - Source 5-bit row mask (`bit 4` is column 0, `bit 0` is column 4).
/// - `bool` - Whether bold stroke thickening is active.
///
/// # Returns
///
/// - `u8` - Effective 5-bit row mask encoding `hit || bold_hit` for every column.
#[inline(always)]
#[must_use]
fn dilate_row_mask(row_bits: u8, bold: bool) -> u8 {
    if bold {
        row_bits | ((row_bits >> 1) & 0x0F)
    } else {
        row_bits
    }
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
    let out_w_usize: usize = out_w as usize;
    let total_pixels: usize = out_w_usize * (out_h as usize);
    let mut coverage: Vec<u8> = vec![0; total_pixels];
    if rows == [0; BITMAP_GLYPH_HEIGHT] {
        return RasterizedGlyph::new(out_w, out_h, advance, coverage);
    }
    let bold: bool = key.get_bold();
    let italic: bool = key.get_italic();
    let src_w: f32 = BITMAP_GLYPH_WIDTH as f32;
    let src_h: f32 = BITMAP_GLYPH_HEIGHT as f32;
    let dst_w: f32 = out_w as f32;
    let dst_h: f32 = out_h as f32;
    let max_sx: f32 = src_w - 0.01;
    let max_row: usize = BITMAP_GLYPH_HEIGHT - 1;
    let max_col: usize = BITMAP_GLYPH_WIDTH - 1;
    if !italic && out_w_usize <= GLYPH_STACK_COL_CACHE {
        let mut shift_l_buf: [u8; GLYPH_STACK_COL_CACHE] = [0; GLYPH_STACK_COL_CACHE];
        let mut shift_r_buf: [u8; GLYPH_STACK_COL_CACHE] = [0; GLYPH_STACK_COL_CACHE];
        for px in 0..out_w_usize {
            let pxf: f32 = px as f32;
            let sx_l: f32 = ((pxf + 0.25) * src_w / dst_w).clamp(0.0, max_sx);
            let sx_r: f32 = ((pxf + 0.75) * src_w / dst_w).clamp(0.0, max_sx);
            let col_l: usize = (sx_l.floor() as usize).min(max_col);
            let col_r: usize = (sx_r.floor() as usize).min(max_col);
            shift_l_buf[px] = (max_col - col_l) as u8;
            shift_r_buf[px] = (max_col - col_r) as u8;
        }
        for py in 0..out_h {
            let pyf: f32 = py as f32;
            let row_top_idx: usize = (((pyf + 0.25) * src_h / dst_h).floor() as usize).min(max_row);
            let row_bot_idx: usize = (((pyf + 0.75) * src_h / dst_h).floor() as usize).min(max_row);
            let eff_top: u8 = dilate_row_mask(rows[row_top_idx], bold);
            let eff_bot: u8 = dilate_row_mask(rows[row_bot_idx], bold);
            if eff_top == 0 && eff_bot == 0 {
                continue;
            }
            let row_off: usize = (py as usize) * out_w_usize;
            let row_dst: &mut [u8] = &mut coverage[row_off..row_off + out_w_usize];
            for (px, dst_px) in row_dst.iter_mut().enumerate() {
                let sl: u8 = shift_l_buf[px];
                let sr: u8 = shift_r_buf[px];
                let hits: u8 = ((eff_top >> sl) & 1)
                    + ((eff_top >> sr) & 1)
                    + ((eff_bot >> sl) & 1)
                    + ((eff_bot >> sr) & 1);
                if hits > 0 {
                    *dst_px = SUBPIXEL_ALPHA_LUT[hits as usize];
                }
            }
        }
        return RasterizedGlyph::new(out_w, out_h, advance, coverage);
    }
    let inv_dst_h_italic: f32 = 1.0 / dst_h.max(1.0);
    for py in 0..out_h {
        let pyf: f32 = py as f32;
        let row_top_idx: usize = (((pyf + 0.25) * src_h / dst_h).floor() as usize).min(max_row);
        let row_bot_idx: usize = (((pyf + 0.75) * src_h / dst_h).floor() as usize).min(max_row);
        let eff_top: u8 = dilate_row_mask(rows[row_top_idx], bold);
        let eff_bot: u8 = dilate_row_mask(rows[row_bot_idx], bold);
        if eff_top == 0 && eff_bot == 0 {
            continue;
        }
        let italic_shift: f32 = if italic {
            (0.5 - pyf * inv_dst_h_italic) * 0.85
        } else {
            0.0
        };
        let row_off: usize = (py as usize) * out_w_usize;
        let row_dst: &mut [u8] = &mut coverage[row_off..row_off + out_w_usize];
        for (px, dst_px) in row_dst.iter_mut().enumerate() {
            let pxf: f32 = px as f32;
            let sx_l: f32 = ((pxf + 0.25) * src_w / dst_w - italic_shift).clamp(0.0, max_sx);
            let sx_r: f32 = ((pxf + 0.75) * src_w / dst_w - italic_shift).clamp(0.0, max_sx);
            let sl: u8 = (max_col - (sx_l.floor() as usize).min(max_col)) as u8;
            let sr: u8 = (max_col - (sx_r.floor() as usize).min(max_col)) as u8;
            let hits: u8 = ((eff_top >> sl) & 1)
                + ((eff_top >> sr) & 1)
                + ((eff_bot >> sl) & 1)
                + ((eff_bot >> sr) & 1);
            if hits > 0 {
                *dst_px = SUBPIXEL_ALPHA_LUT[hits as usize];
            }
        }
    }
    RasterizedGlyph::new(out_w, out_h, advance, coverage)
}
