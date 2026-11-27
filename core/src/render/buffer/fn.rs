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
