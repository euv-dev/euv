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
