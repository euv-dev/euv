use super::*;

/// Computes a compile-time IEEE-754 `f32` square root via Newton-Raphson iteration for LUT generation.
///
/// # Arguments
///
/// - `f32` - Non-negative input value.
///
/// # Returns
///
/// - `f32` - Square root of `value`.
#[must_use]
pub(crate) const fn const_sqrt_f32(value: f32) -> f32 {
    if value <= 0.0 {
        return 0.0;
    }
    let mut guess: f32 = if value > 1.0 { value * 0.5 } else { 0.5 };
    let mut iter: usize = 0;
    while iter < 14 {
        guess = 0.5 * (guess + value / guess);
        iter += 1;
    }
    guess
}

/// Builds the 256-entry compile-time sRGB byte to gamma-2.0 linear-light lookup table.
///
/// # Returns
///
/// - `[f32; 256]` - Normalized linear-light luminance for each `u8` channel value.
#[must_use]
pub(crate) const fn build_srgb_to_lin_lut() -> [f32; 256] {
    let mut table: [f32; 256] = [0.0; 256];
    let inv_255: f32 = 1.0 / 255.0;
    let mut idx: usize = 0;
    while idx < 256 {
        let norm: f32 = (idx as f32) * inv_255;
        table[idx] = norm * norm;
        idx += 1;
    }
    table
}

/// Builds the 4096-entry compile-time linear-light to Q12.4 fixed-point sRGB lookup table (1/16 LSB precision for sub-LSB dithering).
///
/// # Returns
///
/// - `[u16; 4096]` - Fixed-point `sRGB * 16` values covering linear `[0.0, 1.0]`.
#[must_use]
pub(crate) const fn build_lin_to_srgb_q4_lut() -> [u16; 4096] {
    let mut table: [u16; 4096] = [0; 4096];
    let inv_4095: f32 = 1.0 / 4095.0;
    let mut idx: usize = 0;
    while idx < 4096 {
        let lin: f32 = (idx as f32) * inv_4095;
        let scaled: f32 = const_sqrt_f32(lin) * 255.0 * 16.0 + 0.5;
        let clamped: f32 = if scaled < 0.0 {
            0.0
        } else if scaled > 4080.0 {
            4080.0
        } else {
            scaled
        };
        table[idx] = clamped as u16;
        idx += 1;
    }
    table
}

/// Precomputes bit-exact per-channel `[u8; 256]` sRGB linear-light source-over lookup tables for constant translucent `color`.
///
/// # Arguments
///
/// - `Color` - Foreground translucent color (`0 < a < 255`).
///
/// # Returns
///
/// - `([u8; 256], [u8; 256], [u8; 256], [u8; 256])` - Lookup tables `(lut_r, lut_g, lut_b, lut_a)` mapping destination channel bytes to blended output bytes.
#[must_use]
pub(crate) fn build_linear_blend_luts(
    color: Color,
) -> ([u8; 256], [u8; 256], [u8; 256], [u8; 256]) {
    let sa: u8 = color.get_a();
    let a: f32 = (sa as f32) * (1.0 / 255.0);
    let inv_a: f32 = 1.0 - a;
    let sr_a: f32 = SRGB_TO_LIN_LUT[color.get_r() as usize] * a;
    let sg_a: f32 = SRGB_TO_LIN_LUT[color.get_g() as usize] * a;
    let sb_a: f32 = SRGB_TO_LIN_LUT[color.get_b() as usize] * a;
    let sa_u32: u32 = sa as u32;
    let inv_a_u32: u32 = 255 - sa_u32;
    let same_rg: bool = color.get_r() == color.get_g();
    let same_rb: bool = color.get_r() == color.get_b();
    let mut lut_r: [u8; 256] = [0; 256];
    let mut lut_g: [u8; 256] = [0; 256];
    let mut lut_b: [u8; 256] = [0; 256];
    let mut lut_a: [u8; 256] = [0; 256];
    for d in 0..256 {
        let lin_inv: f32 = SRGB_TO_LIN_LUT[d] * inv_a;
        let ir: usize = ((sr_a + lin_inv) * 4095.0).clamp(0.0, 4095.0) as usize;
        let out_r: u8 = ((LIN_TO_SRGB_Q4_LUT[ir] + 8) >> 4).min(255) as u8;
        lut_r[d] = out_r;
        lut_g[d] = if same_rg {
            out_r
        } else {
            let ig: usize = ((sg_a + lin_inv) * 4095.0).clamp(0.0, 4095.0) as usize;
            ((LIN_TO_SRGB_Q4_LUT[ig] + 8) >> 4).min(255) as u8
        };
        lut_b[d] = if same_rb {
            out_r
        } else {
            let ib: usize = ((sb_a + lin_inv) * 4095.0).clamp(0.0, 4095.0) as usize;
            ((LIN_TO_SRGB_Q4_LUT[ib] + 8) >> 4).min(255) as u8
        };
        let va: u32 = (d as u32) * inv_a_u32 + 128;
        lut_a[d] = (sa_u32 + ((va + (va >> 8)) >> 8)).min(255) as u8;
    }
    (lut_r, lut_g, lut_b, lut_a)
}

/// Precomputes a 256-entry Q12.4 sRGB linear-light source-over table for one constant shadow channel and opacity.
///
/// # Arguments
///
/// - `u8` - Source shadow sRGB channel byte.
/// - `f32` - Clamped continuous shadow opacity in `[0.0, 1.0]`.
///
/// # Returns
///
/// - `[u16; 256]` - Q12.4 (`sRGB * 16`, in `0..=4080`) output table indexed by destination sRGB byte.
#[must_use]
pub(crate) fn build_shadow_q4_lut_channel(src_ch: u8, alpha: f32) -> [u16; 256] {
    let inv_a: f32 = 1.0 - alpha;
    let src_lin_a: f32 = SRGB_TO_LIN_LUT[src_ch as usize] * alpha;
    let mut table: [u16; 256] = [0; 256];
    for (d, slot) in table.iter_mut().enumerate() {
        let lin_idx: usize =
            ((src_lin_a + SRGB_TO_LIN_LUT[d] * inv_a) * 4095.0).clamp(0.0, 4095.0) as usize;
        *slot = LIN_TO_SRGB_Q4_LUT[lin_idx];
    }
    table
}

/// Composites a clipped glyph coverage row onto a destination RGBA8 span with opaque specialization and 8-byte zero-alpha fast-skip.
///
/// # Arguments
///
/// - `&mut [u8]` - Destination RGBA8 byte slice covering `coverage.len()` pixels.
/// - `&[u8]` - Clipped glyph alpha coverage bytes.
/// - `Color` - Foreground text RGBA color (`color.get_a() > 0`).
///
/// # Returns
///
/// - `usize` - Number of non-zero coverage pixels shaded.
#[inline(always)]
pub(crate) fn blend_glyph_row_span(dst_span: &mut [u8], coverage: &[u8], color: Color) -> usize {
    let color_a: u32 = color.get_a() as u32;
    let sr_u8: u8 = color.get_r();
    let sg_u8: u8 = color.get_g();
    let sb_u8: u8 = color.get_b();
    let sr: u32 = sr_u8 as u32;
    let sg: u32 = sg_u8 as u32;
    let sb: u32 = sb_u8 as u32;
    let dst_px: &mut [[u8; 4]] = dst_span.as_chunks_mut::<4>().0;
    let len: usize = dst_px.len().min(coverage.len());
    let (dst_blocks, dst_tail): (&mut [[[u8; 4]; 8]], &mut [[u8; 4]]) =
        dst_px[..len].as_chunks_mut::<8>();
    let (cov_blocks, cov_tail): (&[[u8; 8]], &[u8]) = coverage[..len].as_chunks::<8>();
    let mut shaded: usize = 0;
    if color_a == 255 {
        let opaque_px: [u8; 4] = [sr_u8, sg_u8, sb_u8, 255];
        let opaque_block: [[u8; 4]; 8] = [opaque_px; 8];
        for (d_blk, c_blk) in dst_blocks.iter_mut().zip(cov_blocks.iter()) {
            let word: u64 = u64::from_ne_bytes(*c_blk);
            if word == 0 {
                continue;
            }
            if word == u64::MAX {
                *d_blk = opaque_block;
                shaded += 8;
                continue;
            }
            for (px, alpha_byte) in d_blk.iter_mut().zip(c_blk.iter().copied()) {
                if alpha_byte == 0 {
                    continue;
                }
                if alpha_byte == 255 {
                    *px = opaque_px;
                } else {
                    let sa_u32: u32 = alpha_byte as u32;
                    let inv_a: u32 = 255 - sa_u32;
                    let vr: u32 = sr * sa_u32 + (px[0] as u32) * inv_a + 128;
                    let vg: u32 = sg * sa_u32 + (px[1] as u32) * inv_a + 128;
                    let vb: u32 = sb * sa_u32 + (px[2] as u32) * inv_a + 128;
                    let va: u32 = (px[3] as u32) * inv_a + 128;
                    px[0] = ((vr + (vr >> 8)) >> 8) as u8;
                    px[1] = ((vg + (vg >> 8)) >> 8) as u8;
                    px[2] = ((vb + (vb >> 8)) >> 8) as u8;
                    px[3] = (sa_u32 + ((va + (va >> 8)) >> 8)).min(255) as u8;
                }
                shaded += 1;
            }
        }
        for (px, alpha_byte) in dst_tail.iter_mut().zip(cov_tail.iter().copied()) {
            if alpha_byte == 0 {
                continue;
            }
            if alpha_byte == 255 {
                *px = opaque_px;
            } else {
                let sa_u32: u32 = alpha_byte as u32;
                let inv_a: u32 = 255 - sa_u32;
                let vr: u32 = sr * sa_u32 + (px[0] as u32) * inv_a + 128;
                let vg: u32 = sg * sa_u32 + (px[1] as u32) * inv_a + 128;
                let vb: u32 = sb * sa_u32 + (px[2] as u32) * inv_a + 128;
                let va: u32 = (px[3] as u32) * inv_a + 128;
                px[0] = ((vr + (vr >> 8)) >> 8) as u8;
                px[1] = ((vg + (vg >> 8)) >> 8) as u8;
                px[2] = ((vb + (vb >> 8)) >> 8) as u8;
                px[3] = (sa_u32 + ((va + (va >> 8)) >> 8)).min(255) as u8;
            }
            shaded += 1;
        }
        return shaded;
    }
    for (d_blk, c_blk) in dst_blocks.iter_mut().zip(cov_blocks.iter()) {
        if u64::from_ne_bytes(*c_blk) == 0 {
            continue;
        }
        for (px, alpha_byte) in d_blk.iter_mut().zip(c_blk.iter().copied()) {
            if alpha_byte == 0 {
                continue;
            }
            let prod: u32 = color_a * (alpha_byte as u32) + 128;
            let sa_u32: u32 = (prod + (prod >> 8)) >> 8;
            if sa_u32 > 0 {
                let inv_a: u32 = 255 - sa_u32;
                let vr: u32 = sr * sa_u32 + (px[0] as u32) * inv_a + 128;
                let vg: u32 = sg * sa_u32 + (px[1] as u32) * inv_a + 128;
                let vb: u32 = sb * sa_u32 + (px[2] as u32) * inv_a + 128;
                let va: u32 = (px[3] as u32) * inv_a + 128;
                px[0] = ((vr + (vr >> 8)) >> 8) as u8;
                px[1] = ((vg + (vg >> 8)) >> 8) as u8;
                px[2] = ((vb + (vb >> 8)) >> 8) as u8;
                px[3] = (sa_u32 + ((va + (va >> 8)) >> 8)).min(255) as u8;
            }
            shaded += 1;
        }
    }
    for (px, alpha_byte) in dst_tail.iter_mut().zip(cov_tail.iter().copied()) {
        if alpha_byte == 0 {
            continue;
        }
        let prod: u32 = color_a * (alpha_byte as u32) + 128;
        let sa_u32: u32 = (prod + (prod >> 8)) >> 8;
        if sa_u32 > 0 {
            let inv_a: u32 = 255 - sa_u32;
            let vr: u32 = sr * sa_u32 + (px[0] as u32) * inv_a + 128;
            let vg: u32 = sg * sa_u32 + (px[1] as u32) * inv_a + 128;
            let vb: u32 = sb * sa_u32 + (px[2] as u32) * inv_a + 128;
            let va: u32 = (px[3] as u32) * inv_a + 128;
            px[0] = ((vr + (vr >> 8)) >> 8) as u8;
            px[1] = ((vg + (vg >> 8)) >> 8) as u8;
            px[2] = ((vb + (vb >> 8)) >> 8) as u8;
            px[3] = (sa_u32 + ((va + (va >> 8)) >> 8)).min(255) as u8;
        }
        shaded += 1;
    }
    shaded
}

/// Fills an RGBA8 byte span with solid `rgba` using `slice::fill` for uniform-byte colors and 32-byte wide stores otherwise.
///
/// # Arguments
///
/// - `&mut [u8]` - Target RGBA8 byte slice whose length is a multiple of 4.
/// - `[u8; 4]` - Solid RGBA8 pixel bytes to broadcast across `span`.
#[inline(always)]
pub(crate) fn fill_rgba_span_32(span: &mut [u8], rgba: [u8; 4]) {
    if rgba[0] == rgba[1] && rgba[0] == rgba[2] && rgba[0] == rgba[3] {
        span.fill(rgba[0]);
        return;
    }
    let wide32: [u8; 32] = [
        rgba[0], rgba[1], rgba[2], rgba[3], rgba[0], rgba[1], rgba[2], rgba[3], rgba[0], rgba[1],
        rgba[2], rgba[3], rgba[0], rgba[1], rgba[2], rgba[3], rgba[0], rgba[1], rgba[2], rgba[3],
        rgba[0], rgba[1], rgba[2], rgba[3], rgba[0], rgba[1], rgba[2], rgba[3], rgba[0], rgba[1],
        rgba[2], rgba[3],
    ];
    let (blocks, tail): (&mut [[u8; 32]], &mut [u8]) = span.as_chunks_mut::<32>();
    for chunk in blocks.iter_mut() {
        *chunk = wide32;
    }
    for px in tail.as_chunks_mut::<4>().0.iter_mut() {
        *px = rgba;
    }
}

/// Composites a pre-clipped RGBA8 byte span using precomputed bit-exact 256-entry linear-light channel lookup tables.
///
/// # Arguments
///
/// - `&mut [u8]` - Pre-clipped destination RGBA8 byte slice.
/// - `&([u8; 256], [u8; 256], [u8; 256], [u8; 256])` - Precomputed per-channel lookup tables `(lut_r, lut_g, lut_b, lut_a)`.
#[inline(always)]
pub(crate) fn fill_span_linear_luts_slice(
    span: &mut [u8],
    luts: &([u8; 256], [u8; 256], [u8; 256], [u8; 256]),
) {
    let (lut_r, lut_g, lut_b, lut_a): &([u8; 256], [u8; 256], [u8; 256], [u8; 256]) = luts;
    for px in span.as_chunks_mut::<4>().0.iter_mut() {
        px[0] = lut_r[px[0] as usize];
        px[1] = lut_g[px[1] as usize];
        px[2] = lut_b[px[2] as usize];
        px[3] = lut_a[px[3] as usize];
    }
}

/// Fills or composites a pre-clipped RGBA8 byte span with `color` in gamma-correct linear light.
///
/// # Arguments
///
/// - `&mut [u8]` - Pre-clipped destination RGBA8 byte slice.
/// - `Color` - Foreground RGBA color.
#[inline(always)]
pub(crate) fn fill_span_linear_slice(span: &mut [u8], color: Color) {
    let sa: u8 = color.get_a();
    if sa == 0 {
        return;
    }
    if sa == 255 {
        fill_rgba_span_32(span, [color.get_r(), color.get_g(), color.get_b(), 255]);
        return;
    }
    if span.len() >= 128 * 4 {
        let luts: ([u8; 256], [u8; 256], [u8; 256], [u8; 256]) = build_linear_blend_luts(color);
        fill_span_linear_luts_slice(span, &luts);
        return;
    }
    let a: f32 = (sa as f32) * (1.0 / 255.0);
    let inv_a: f32 = 1.0 - a;
    let sr_a: f32 = SRGB_TO_LIN_LUT[color.get_r() as usize] * a;
    let sg_a: f32 = SRGB_TO_LIN_LUT[color.get_g() as usize] * a;
    let sb_a: f32 = SRGB_TO_LIN_LUT[color.get_b() as usize] * a;
    let sa_u32: u32 = sa as u32;
    let inv_a_u32: u32 = 255 - sa_u32;
    for px in span.as_chunks_mut::<4>().0.iter_mut() {
        let dr: f32 = SRGB_TO_LIN_LUT[px[0] as usize];
        let dg: f32 = SRGB_TO_LIN_LUT[px[1] as usize];
        let db: f32 = SRGB_TO_LIN_LUT[px[2] as usize];
        let ir: usize = ((sr_a + dr * inv_a) * 4095.0).clamp(0.0, 4095.0) as usize;
        let ig: usize = ((sg_a + dg * inv_a) * 4095.0).clamp(0.0, 4095.0) as usize;
        let ib: usize = ((sb_a + db * inv_a) * 4095.0).clamp(0.0, 4095.0) as usize;
        let va: u32 = (px[3] as u32) * inv_a_u32 + 128;
        px[0] = ((LIN_TO_SRGB_Q4_LUT[ir] + 8) >> 4).min(255) as u8;
        px[1] = ((LIN_TO_SRGB_Q4_LUT[ig] + 8) >> 4).min(255) as u8;
        px[2] = ((LIN_TO_SRGB_Q4_LUT[ib] + 8) >> 4).min(255) as u8;
        px[3] = (sa_u32 + ((va + (va >> 8)) >> 8)).min(255) as u8;
    }
}

/// Folds one 32-byte block (`4 x u64` little-endian words) into the 4 independent FNV-1a hash lanes.
///
/// # Arguments
///
/// - `&mut [u64; 4]` - Mutable 4-lane FNV-1a hash accumulators.
/// - `&[u8; 32]` - 32-byte input pixel block.
#[inline(always)]
pub(crate) fn fold_fnv1a_32(lanes: &mut [u64; 4], blk: &[u8; 32]) {
    let (words, _): (&[[u8; 8]], &[u8]) = blk.as_chunks::<8>();
    lanes[0] = (lanes[0] ^ u64::from_le_bytes(words[0])).wrapping_mul(FNV1A_PRIME);
    lanes[1] = (lanes[1] ^ u64::from_le_bytes(words[1])).wrapping_mul(FNV1A_PRIME);
    lanes[2] = (lanes[2] ^ u64::from_le_bytes(words[2])).wrapping_mul(FNV1A_PRIME);
    lanes[3] = (lanes[3] ^ u64::from_le_bytes(words[3])).wrapping_mul(FNV1A_PRIME);
}
