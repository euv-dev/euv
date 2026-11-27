use super::*;

/// Computes a compile-time approximation of `exp(-x)` for non-negative `x` using range-reduced Taylor series.
///
/// # Arguments
///
/// - `f32` - Non-negative exponent magnitude `x`.
///
/// # Returns
///
/// - `f32` - Accurate `exp(-x)` value in `(0.0, 1.0]`.
#[must_use]
pub(crate) const fn const_exp_neg_f32(x: f32) -> f32 {
    if x <= 0.0 {
        return 1.0;
    }
    let reduced: f32 = -x * 0.0625;
    let mut sum: f32 = 1.0;
    let mut term: f32 = 1.0;
    let mut k: usize = 1;
    while k <= 10 {
        term = term * reduced / (k as f32);
        sum += term;
        k += 1;
    }
    let p2: f32 = sum * sum;
    let p4: f32 = p2 * p2;
    let p8: f32 = p4 * p4;
    p8 * p8
}

/// Builds the 513-entry compile-time analytical Gaussian `0.5 * erfc(u / sqrt(2))` lookup table for `u` in `[-3.2, +3.2]`.
///
/// # Returns
///
/// - `[f32; 513]` - Precomputed Gaussian falloff samples.
#[must_use]
pub(crate) const fn build_gaussian_erfc_lut() -> [f32; 513] {
    let mut table: [f32; 513] = [0.0; 513];
    let step: f32 = (GAUSSIAN_LUT_RANGE * 2.0) / (GAUSSIAN_LUT_SEGMENTS as f32);
    let mut idx: usize = 0;
    while idx <= GAUSSIAN_LUT_SEGMENTS {
        let u: f32 = -GAUSSIAN_LUT_RANGE + (idx as f32) * step;
        let x: f32 = u / SQRT_2;
        let abs_x: f32 = if x < 0.0 { -x } else { x };
        let t: f32 = 1.0 / (1.0 + 0.327_591_1 * abs_x);
        let poly: f32 = t
            * (0.254_829_6
                + t * (-0.284_496_72 + t * (1.421_413_7 + t * (-1.453_152 + t * 1.061_405_4))));
        let erfc_abs: f32 = poly * const_exp_neg_f32(abs_x * abs_x);
        let val: f32 = if x >= 0.0 {
            0.5 * erfc_abs
        } else {
            1.0 - 0.5 * erfc_abs
        };
        table[idx] = if val < 0.0 {
            0.0
        } else if val > 1.0 {
            1.0
        } else {
            val
        };
        idx += 1;
    }
    table
}

/// Evaluates the true 2D analytical Gaussian shadow falloff via a 513-entry L1-cached linear-interpolated `erfc` LUT (zero per-pixel `exp()`).
///
/// # Arguments
///
/// - `f32` - Signed distance from the shadow caster box in physical pixels.
/// - `f32` - Inverse Gaussian standard deviation `1.0 / sigma.max(0.5)`.
///
/// # Returns
///
/// - `f32` - Smooth $C^\infty$ shadow opacity multiplier in `[0.0, 1.0]`.
#[must_use]
pub fn fast_gaussian_shadow_falloff(dist: f32, inv_sigma: f32) -> f32 {
    let u: f32 = dist * inv_sigma;
    if u >= GAUSSIAN_LUT_RANGE {
        return 0.0;
    }
    if u <= -GAUSSIAN_LUT_RANGE {
        return 1.0;
    }
    let pos: f32 = (u + GAUSSIAN_LUT_RANGE) * GAUSSIAN_LUT_STEP_INV;
    let idx: usize = (pos as usize).min(GAUSSIAN_LUT_SEGMENTS - 1);
    let frac: f32 = pos - (idx as f32);
    let a: f32 = GAUSSIAN_ERFC_LUT[idx];
    let b: f32 = GAUSSIAN_ERFC_LUT[idx + 1];
    a + (b - a) * frac
}

/// Computes a deterministic 64-bit FxHash key for caching `TextLayoutResult` entries across frames.
///
/// # Arguments
///
/// - `&str` - Source text content.
/// - `TextStyle` - Typography metrics affecting line wrapping.
///
/// # Returns
///
/// - `u64` - Deterministic layout cache key hash.
#[must_use]
pub(crate) fn hash_text_layout_key(text: &str, style: TextStyle) -> u64 {
    let bytes: &[u8] = text.as_bytes();
    let mut hash: u64 = (FNV1A_OFFSET_BASIS ^ (bytes.len() as u64)).wrapping_mul(FX_HASH_K64);
    let (chunks, tail): (&[[u8; 8]], &[u8]) = bytes.as_chunks::<8>();
    for chunk in chunks.iter() {
        let word: u64 = u64::from_le_bytes(*chunk);
        hash = (hash.rotate_left(5) ^ word).wrapping_mul(FX_HASH_K64);
    }
    let mut tail_word: u64 = 0;
    for (idx, byte) in tail.iter().copied().enumerate() {
        tail_word |= (byte as u64) << (idx * 8);
    }
    if !tail.is_empty() {
        hash = (hash.rotate_left(5) ^ tail_word).wrapping_mul(FX_HASH_K64);
    }
    let style_flags: u32 = (style.get_weight() as u32)
        | ((style.get_align() as u32) << 8)
        | ((style.get_family() as u32) << 16)
        | ((style.get_overflow() as u32) << 20)
        | (u32::from(style.get_italic()) << 24);
    let w0: u64 = (style.get_font_size().to_bits() as u64)
        | ((style.get_line_height().to_bits() as u64) << 32);
    let w1: u64 = (style.get_letter_spacing().to_bits() as u64) | ((style_flags as u64) << 32);
    hash = (hash.rotate_left(5) ^ w0).wrapping_mul(FX_HASH_K64);
    (hash.rotate_left(5) ^ w1).wrapping_mul(FX_HASH_K64)
}

/// Evaluates Interleaved Gradient Noise (IGN) in `[-0.5, 0.5]` to dither 8-bit shadows and gradients without visible banding.
///
/// # Arguments
///
/// - `u32` - Physical pixel X coordinate.
/// - `u32` - Physical pixel Y coordinate.
///
/// # Returns
///
/// - `f32` - Centered high-frequency dither value in `[-0.5, 0.5]`.
#[must_use]
pub fn interleaved_gradient_noise(x: u32, y: u32) -> f32 {
    let fx: f32 = x as f32;
    let fy: f32 = y as f32;
    let n: f32 = (52.982_918 * (fx * 0.067_110_56 + fy * 0.005_837_15).fract()).fract();
    n - 0.5
}

/// Evaluates the true 2D analytical Gaussian shadow falloff `0.5 * erfc(dist / (sigma * sqrt(2)))`.
///
/// # Arguments
///
/// - `f32` - Signed distance from the shadow caster box in physical pixels.
/// - `f32` - Gaussian standard deviation `sigma` in physical pixels.
///
/// # Returns
///
/// - `f32` - Smooth $C^\infty$ shadow opacity multiplier in `[0.0, 1.0]`.
#[must_use]
pub fn gaussian_shadow_falloff(dist: f32, sigma: f32) -> f32 {
    let safe_sigma: f32 = sigma.max(0.5);
    let x: f32 = dist / (safe_sigma * SQRT_2);
    let abs_x: f32 = x.abs();
    let t: f32 = 1.0 / (1.0 + 0.327_591_1 * abs_x);
    let poly: f32 = t
        * (0.254_829_6
            + t * (-0.284_496_72 + t * (1.421_413_7 + t * (-1.453_152 + t * 1.061_405_4))));
    let erfc_abs: f32 = poly * (-abs_x * abs_x).exp();
    let falloff: f32 = if x >= 0.0 {
        0.5 * erfc_abs
    } else {
        1.0 - 0.5 * erfc_abs
    };
    falloff.clamp(0.0, 1.0)
}

/// Selects the per-quadrant corner radius for point `(px, py)` relative to box center.
///
/// # Arguments
///
/// - `CornerRadius` - Four-corner radii in physical pixels.
/// - `f32` - Relative X coordinate.
/// - `f32` - Relative Y coordinate.
///
/// # Returns
///
/// - `f32` - Quadrant corner radius.
#[must_use]
pub fn quadrant_corner_radius(radius: CornerRadius, px: f32, py: f32) -> f32 {
    if px < 0.0 {
        if py < 0.0 {
            radius.get_top_left()
        } else {
            radius.get_bottom_left()
        }
    } else if py < 0.0 {
        radius.get_top_right()
    } else {
        radius.get_bottom_right()
    }
}

/// Evaluates the signed distance from point `(px, py)` to a rounded rectangle box.
///
/// # Arguments
///
/// - `f32` - Sample X coordinate relative to box center.
/// - `f32` - Sample Y coordinate relative to box center.
/// - `f32` - Half box width.
/// - `f32` - Half box height.
/// - `f32` - Effective corner radius for this quadrant.
///
/// # Returns
///
/// - `f32` - Signed distance (`<= 0.0` inside, `> 0.0` outside).
#[must_use]
pub fn rounded_box_sdf(px: f32, py: f32, half_w: f32, half_h: f32, radius: f32) -> f32 {
    let r: f32 = radius.min(half_w).min(half_h).max(0.0);
    let qx: f32 = px.abs() - half_w + r;
    let qy: f32 = py.abs() - half_h + r;
    let outside: f32 = (qx.max(0.0) * qx.max(0.0) + qy.max(0.0) * qy.max(0.0)).sqrt();
    let inside: f32 = qx.max(qy).min(0.0);
    outside + inside - r
}

/// Converts a signed distance into exact pixel-width anti-aliased coverage in `[0.0, 1.0]` with zero straight-edge bleed.
///
/// # Arguments
///
/// - `f32` - Signed distance in physical pixels.
/// - `AntialiasingMode` - Active anti-aliasing policy.
///
/// # Returns
///
/// - `f32` - Pixel coverage in `[0.0, 1.0]`.
#[must_use]
pub fn sdf_coverage(distance: f32, aa: AntialiasingMode) -> f32 {
    match aa {
        AntialiasingMode::None => {
            if distance <= 0.0 {
                1.0
            } else {
                0.0
            }
        }
        AntialiasingMode::AnalyticalSdf => (0.5 - distance).clamp(0.0, 1.0),
        AntialiasingMode::Msaa4x => {
            let offsets: [f32; 4] = [-0.375, -0.125, 0.125, 0.375];
            let mut sum: f32 = 0.0;
            for off in offsets.into_iter() {
                sum += (0.5 - (distance + off)).clamp(0.0, 1.0);
            }
            sum * 0.25
        }
        AntialiasingMode::Ssaa2x => (0.5 - distance / 0.82).clamp(0.0, 1.0),
    }
}

/// Evaluates 4x4 rotated-grid supersampled coverage for a rounded box and concentric inner border at pixel `(px, py)`.
///
/// # Arguments
///
/// - `f32` - Pixel center X relative to box center.
/// - `f32` - Pixel center Y relative to box center.
/// - `f32` - Outer half-width in physical pixels.
/// - `f32` - Outer half-height in physical pixels.
/// - `f32` - Quadrant outer corner radius in physical pixels.
/// - `f32` - Concentric border thickness in physical pixels (`0.0` for borderless fill).
///
/// # Returns
///
/// - `(f32, f32)` - Tuple `(outer_coverage, inner_fill_coverage)` in `[0.0, 1.0]`.
#[inline(always)]
#[must_use]
pub fn rounded_box_supersampled_coverage(
    px: f32,
    py: f32,
    half_w: f32,
    half_h: f32,
    corner_r: f32,
    border_w: f32,
) -> (f32, f32) {
    let grid: [f32; 4] = [-0.375, -0.125, 0.125, 0.375];
    let outer_r: f32 = corner_r.min(half_w).min(half_h).max(0.0);
    let outer_cx: f32 = half_w - outer_r;
    let outer_cy: f32 = half_h - outer_r;
    let outer_r2: f32 = outer_r * outer_r;
    let has_border: bool = border_w > 0.0;
    if !has_border {
        let mut outer_hits: u32 = 0;
        for (row_idx, &oy) in grid.iter().enumerate() {
            let jx: f32 = if row_idx & 1 == 0 { -0.0625 } else { 0.0625 };
            let base_x: f32 = px + jx;
            let base_y: f32 = py + oy;
            for (col_idx, &ox) in grid.iter().enumerate() {
                let jy: f32 = if col_idx & 1 == 0 { 0.0625 } else { -0.0625 };
                let qx: f32 = ((base_x + ox).abs() - outer_cx).max(0.0);
                let qy: f32 = ((base_y + jy).abs() - outer_cy).max(0.0);
                outer_hits += u32::from(qx * qx + qy * qy <= outer_r2);
            }
        }
        let cov: f32 = (outer_hits as f32) * 0.0625;
        return (cov, cov);
    }
    let inner_hw: f32 = (half_w - border_w).max(0.0);
    let inner_hh: f32 = (half_h - border_w).max(0.0);
    let has_inner: bool = inner_hw > 0.0 && inner_hh > 0.0;
    let inner_r: f32 = (corner_r - border_w).min(inner_hw).min(inner_hh).max(0.0);
    let inner_cx: f32 = inner_hw - inner_r;
    let inner_cy: f32 = inner_hh - inner_r;
    let inner_r2: f32 = inner_r * inner_r;
    let mut outer_hits: u32 = 0;
    let mut inner_hits: u32 = 0;
    for (row_idx, &oy) in grid.iter().enumerate() {
        let jx: f32 = if row_idx & 1 == 0 { -0.0625 } else { 0.0625 };
        let base_x: f32 = px + jx;
        let base_y: f32 = py + oy;
        for (col_idx, &ox) in grid.iter().enumerate() {
            let jy: f32 = if col_idx & 1 == 0 { 0.0625 } else { -0.0625 };
            let abs_x: f32 = (base_x + ox).abs();
            let abs_y: f32 = (base_y + jy).abs();
            let qx: f32 = (abs_x - outer_cx).max(0.0);
            let qy: f32 = (abs_y - outer_cy).max(0.0);
            if qx * qx + qy * qy <= outer_r2 {
                outer_hits += 1;
                if has_inner {
                    let iqx: f32 = (abs_x - inner_cx).max(0.0);
                    let iqy: f32 = (abs_y - inner_cy).max(0.0);
                    inner_hits += u32::from(iqx * iqx + iqy * iqy <= inner_r2);
                }
            }
        }
    }
    ((outer_hits as f32) * 0.0625, (inner_hits as f32) * 0.0625)
}

/// Evaluates 4x4 rotated-grid supersampled coverage for a filled circle at pixel `(dx, dy)` relative to center.
///
/// # Arguments
///
/// - `f32` - Pixel center X relative to circle center.
/// - `f32` - Pixel center Y relative to circle center.
/// - `f32` - Circle radius in physical pixels.
///
/// # Returns
///
/// - `f32` - Subpixel coverage in `[0.0, 1.0]`.
#[inline(always)]
#[must_use]
pub fn circle_supersampled_coverage(dx: f32, dy: f32, radius: f32) -> f32 {
    let d2: f32 = dx * dx + dy * dy;
    let out_lim: f32 = radius + 0.625;
    if d2 > out_lim * out_lim {
        return 0.0;
    }
    if radius > 0.625 {
        let in_lim: f32 = radius - 0.625;
        if d2 <= in_lim * in_lim {
            return 1.0;
        }
    }
    let grid: [f32; 4] = [-0.375, -0.125, 0.125, 0.375];
    let r2: f32 = radius * radius;
    let mut hits: u32 = 0;
    for (row_idx, &oy) in grid.iter().enumerate() {
        let jx: f32 = if row_idx & 1 == 0 { -0.0625 } else { 0.0625 };
        let base_x: f32 = dx + jx;
        let sy: f32 = dy + oy;
        for (col_idx, &ox) in grid.iter().enumerate() {
            let jy: f32 = if col_idx & 1 == 0 { 0.0625 } else { -0.0625 };
            let sx: f32 = base_x + ox;
            let sample_y: f32 = sy + jy;
            hits += u32::from(sx * sx + sample_y * sample_y <= r2);
        }
    }
    (hits as f32) * 0.0625
}

/// Evaluates 4x4 rotated-grid supersampled coverage for a triangle at physical pixel `(px, py)`.
///
/// # Arguments
///
/// - `f32` - Pixel center X coordinate.
/// - `f32` - Pixel center Y coordinate.
/// - `Point` - First triangle vertex.
/// - `Point` - Second triangle vertex.
/// - `Point` - Third triangle vertex.
///
/// # Returns
///
/// - `f32` - Subpixel coverage in `[0.0, 1.0]`.
#[must_use]
pub fn triangle_supersampled_coverage(px: f32, py: f32, p0: Point, p1: Point, p2: Point) -> f32 {
    let area: f32 = (p1.get_x() - p0.get_x()) * (p2.get_y() - p0.get_y())
        - (p1.get_y() - p0.get_y()) * (p2.get_x() - p0.get_x());
    if area.abs() < 1e-5 {
        return 0.0;
    }
    let sign: f32 = if area > 0.0 { 1.0 } else { -1.0 };
    let grid: [f32; 4] = [-0.375, -0.125, 0.125, 0.375];
    let mut hits: f32 = 0.0;
    for (row_idx, &oy) in grid.iter().enumerate() {
        let jx: f32 = if row_idx % 2 == 0 { -0.0625 } else { 0.0625 };
        let sy: f32 = py + oy;
        for (col_idx, &ox) in grid.iter().enumerate() {
            let jy: f32 = if col_idx % 2 == 0 { 0.0625 } else { -0.0625 };
            let sx: f32 = px + ox + jx;
            let sample_y: f32 = sy + jy;
            let e0: f32 = ((p1.get_x() - p0.get_x()) * (sample_y - p0.get_y())
                - (p1.get_y() - p0.get_y()) * (sx - p0.get_x()))
                * sign;
            let e1: f32 = ((p2.get_x() - p1.get_x()) * (sample_y - p1.get_y())
                - (p2.get_y() - p1.get_y()) * (sx - p1.get_x()))
                * sign;
            let e2: f32 = ((p0.get_x() - p2.get_x()) * (sample_y - p2.get_y())
                - (p0.get_y() - p2.get_y()) * (sx - p2.get_x()))
                * sign;
            if e0 >= 0.0 && e1 >= 0.0 && e2 >= 0.0 {
                hits += 1.0;
            }
        }
    }
    hits * 0.0625
}

/// Snaps logical rectangle bounds to crisp physical pixel edges across fractional DPI scales (`1.25x`, `1.5x`, `2.625x`, `3x`).
///
/// # Arguments
///
/// - `Rect` - Logical rectangle.
/// - `f32` - Active physical scale factor.
///
/// # Returns
///
/// - `Rect` - Edge-aligned physical rectangle.
#[must_use]
pub fn snap_physical_rect(rect: Rect, scale: f32) -> Rect {
    let x0: f32 = (rect.get_x() * scale).round();
    let y0: f32 = (rect.get_y() * scale).round();
    let x1: f32 = (rect.right() * scale).round();
    let y1: f32 = (rect.bottom() * scale).round();
    Rect::new(x0, y0, (x1 - x0).max(0.0), (y1 - y0).max(0.0))
}

/// Composites a rounded rectangle's fill and concentric border cleanly into a single RGBA sample with zero seam bleed.
///
/// # Arguments
///
/// - `Color` - Interior fill color.
/// - `Color` - Concentric border stroke color.
/// - `f32` - Outer shape smoothstep coverage in `[0.0, 1.0]`.
/// - `f32` - Inner fill smoothstep coverage in `[0.0, 1.0]`.
///
/// # Returns
///
/// - `Color` - Premultiplication-free blended RGBA pixel sample ready for framebuffer alpha blending.
#[must_use]
pub fn composite_fill_and_border(
    fill: Color,
    border: Color,
    outer_cov: f32,
    inner_cov: f32,
) -> Color {
    let border_cov: f32 = (outer_cov - inner_cov).clamp(0.0, 1.0);
    let fill_a: f32 = ((fill.get_a() as f32) / 255.0) * inner_cov;
    let border_a: f32 = ((border.get_a() as f32) / 255.0) * border_cov;
    let total_a: f32 = (fill_a + border_a).clamp(0.0, 1.0);
    if total_a <= 0.0001 {
        return COLOR_TRANSPARENT;
    }
    let inv_a: f32 = 1.0 / total_a;
    let fr: f32 = SRGB_TO_LIN_LUT[fill.get_r() as usize];
    let fg: f32 = SRGB_TO_LIN_LUT[fill.get_g() as usize];
    let fb: f32 = SRGB_TO_LIN_LUT[fill.get_b() as usize];
    let br: f32 = SRGB_TO_LIN_LUT[border.get_r() as usize];
    let bg: f32 = SRGB_TO_LIN_LUT[border.get_g() as usize];
    let bb: f32 = SRGB_TO_LIN_LUT[border.get_b() as usize];
    let ir: usize = (((fr * fill_a + br * border_a) * inv_a) * 4095.0).clamp(0.0, 4095.0) as usize;
    let ig: usize = (((fg * fill_a + bg * border_a) * inv_a) * 4095.0).clamp(0.0, 4095.0) as usize;
    let ib: usize = (((fb * fill_a + bb * border_a) * inv_a) * 4095.0).clamp(0.0, 4095.0) as usize;
    let r: u8 = ((LIN_TO_SRGB_Q4_LUT[ir] + 8) >> 4).min(255) as u8;
    let g: u8 = ((LIN_TO_SRGB_Q4_LUT[ig] + 8) >> 4).min(255) as u8;
    let b: u8 = ((LIN_TO_SRGB_Q4_LUT[ib] + 8) >> 4).min(255) as u8;
    let a: u8 = (total_a * 255.0).round().clamp(0.0, 255.0) as u8;
    Color::new(r, g, b, a)
}

/// Composites rounded-rectangle fill and border using precomputed linear-light RGB and normalized alphas.
///
/// # Arguments
///
/// - `[f32; 4]` - Precomputed fill `(lin_r, lin_g, lin_b, alpha_norm)`.
/// - `[f32; 4]` - Precomputed border `(lin_r, lin_g, lin_b, alpha_norm)`.
/// - `f32` - Outer shape coverage in `[0.0, 1.0]`.
/// - `f32` - Inner fill coverage in `[0.0, 1.0]`.
///
/// # Returns
///
/// - `Color` - Bit-exact blended RGBA sample matching `composite_fill_and_border`.
#[inline(always)]
#[must_use]
pub(crate) fn composite_fill_and_border_precomputed(
    fill_lin: [f32; 4],
    border_lin: [f32; 4],
    outer_cov: f32,
    inner_cov: f32,
) -> Color {
    let border_cov: f32 = (outer_cov - inner_cov).clamp(0.0, 1.0);
    let fill_a: f32 = fill_lin[3] * inner_cov;
    let border_a: f32 = border_lin[3] * border_cov;
    let total_a: f32 = (fill_a + border_a).clamp(0.0, 1.0);
    if total_a <= 0.0001 {
        return COLOR_TRANSPARENT;
    }
    let inv_a: f32 = 1.0 / total_a;
    let ir: usize = (((fill_lin[0] * fill_a + border_lin[0] * border_a) * inv_a) * 4095.0)
        .clamp(0.0, 4095.0) as usize;
    let ig: usize = (((fill_lin[1] * fill_a + border_lin[1] * border_a) * inv_a) * 4095.0)
        .clamp(0.0, 4095.0) as usize;
    let ib: usize = (((fill_lin[2] * fill_a + border_lin[2] * border_a) * inv_a) * 4095.0)
        .clamp(0.0, 4095.0) as usize;
    let r: u8 = ((LIN_TO_SRGB_Q4_LUT[ir] + 8) >> 4).min(255) as u8;
    let g: u8 = ((LIN_TO_SRGB_Q4_LUT[ig] + 8) >> 4).min(255) as u8;
    let b: u8 = ((LIN_TO_SRGB_Q4_LUT[ib] + 8) >> 4).min(255) as u8;
    let a: u8 = (total_a * 255.0).round().clamp(0.0, 255.0) as u8;
    Color::new(r, g, b, a)
}

/// Blends an undithered RGBA `Color` directly into a mutable 4-byte pixel slot using linear-light compositing.
///
/// # Arguments
///
/// - `&mut [u8]` - Destination framebuffer row slice.
/// - `u32` - Pixel X coordinate within `row`.
/// - `Color` - Foreground color.
/// - `[f32; 3]` - Precomputed linear RGB `(lin_r, lin_g, lin_b)` for `color`.
#[inline(always)]
pub(crate) fn blend_row_pixel_linear_precomputed(
    row: &mut [u8],
    x: u32,
    color: Color,
    lin_rgb: [f32; 3],
) {
    let sa: u8 = color.get_a();
    let off: usize = (x as usize) * 4;
    if sa == 0 || off + 4 > row.len() {
        return;
    }
    if sa == 255 {
        row[off] = color.get_r();
        row[off + 1] = color.get_g();
        row[off + 2] = color.get_b();
        row[off + 3] = 255;
        return;
    }
    let a: f32 = (sa as f32) * (1.0 / 255.0);
    let inv_a: f32 = 1.0 - a;
    let dr: f32 = SRGB_TO_LIN_LUT[row[off] as usize];
    let dg: f32 = SRGB_TO_LIN_LUT[row[off + 1] as usize];
    let db: f32 = SRGB_TO_LIN_LUT[row[off + 2] as usize];
    let ir: usize = ((lin_rgb[0] * a + dr * inv_a) * 4095.0).clamp(0.0, 4095.0) as usize;
    let ig: usize = ((lin_rgb[1] * a + dg * inv_a) * 4095.0).clamp(0.0, 4095.0) as usize;
    let ib: usize = ((lin_rgb[2] * a + db * inv_a) * 4095.0).clamp(0.0, 4095.0) as usize;
    row[off] = ((LIN_TO_SRGB_Q4_LUT[ir] + 8) >> 4).min(255) as u8;
    row[off + 1] = ((LIN_TO_SRGB_Q4_LUT[ig] + 8) >> 4).min(255) as u8;
    row[off + 2] = ((LIN_TO_SRGB_Q4_LUT[ib] + 8) >> 4).min(255) as u8;
    let sa_u32: u32 = sa as u32;
    let va: u32 = (row[off + 3] as u32) * (255 - sa_u32) + 128;
    row[off + 3] = (sa_u32 + ((va + (va >> 8)) >> 8)).min(255) as u8;
}

/// Blends an undithered RGB color with continuous opacity `alpha_f` directly into a mutable row slice.
///
/// # Arguments
///
/// - `&mut [u8]` - Destination framebuffer row slice.
/// - `u32` - Pixel X coordinate within `row`.
/// - `Color` - Foreground RGB tint.
/// - `[f32; 3]` - Precomputed linear RGB `(lin_r, lin_g, lin_b)` for `color`.
/// - `f32` - Continuous opacity in `[0.0, 1.0]`.
#[inline(always)]
pub(crate) fn blend_row_pixel_f32_precomputed(
    row: &mut [u8],
    x: u32,
    color: Color,
    lin_rgb: [f32; 3],
    alpha_f: f32,
) {
    let off: usize = (x as usize) * 4;
    if alpha_f <= 0.0002 || off + 4 > row.len() {
        return;
    }
    let a: f32 = alpha_f.clamp(0.0, 1.0);
    if a >= 0.9995 {
        row[off] = color.get_r();
        row[off + 1] = color.get_g();
        row[off + 2] = color.get_b();
        row[off + 3] = 255;
        return;
    }
    let inv_a: f32 = 1.0 - a;
    let dr: f32 = SRGB_TO_LIN_LUT[row[off] as usize];
    let dg: f32 = SRGB_TO_LIN_LUT[row[off + 1] as usize];
    let db: f32 = SRGB_TO_LIN_LUT[row[off + 2] as usize];
    let ir: usize = ((lin_rgb[0] * a + dr * inv_a) * 4095.0).clamp(0.0, 4095.0) as usize;
    let ig: usize = ((lin_rgb[1] * a + dg * inv_a) * 4095.0).clamp(0.0, 4095.0) as usize;
    let ib: usize = ((lin_rgb[2] * a + db * inv_a) * 4095.0).clamp(0.0, 4095.0) as usize;
    row[off] = ((LIN_TO_SRGB_Q4_LUT[ir] + 8) >> 4).min(255) as u8;
    row[off + 1] = ((LIN_TO_SRGB_Q4_LUT[ig] + 8) >> 4).min(255) as u8;
    row[off + 2] = ((LIN_TO_SRGB_Q4_LUT[ib] + 8) >> 4).min(255) as u8;
    let sa_u32: u32 = (a * 255.0) as u32;
    let va: u32 = (row[off + 3] as u32) * (255 - sa_u32) + 128;
    row[off + 3] = (sa_u32 + ((va + (va >> 8)) >> 8)).min(255) as u8;
}

/// Applies a slice of precomputed 1D edge samples directly onto mutable scanline `row`.
///
/// # Arguments
///
/// - `&mut [u8]` - Destination framebuffer row slice.
/// - `&[EdgeBlendSample]` - Precomputed horizontal edge strip samples.
#[inline(always)]
pub(crate) fn apply_edge_strip_row(row: &mut [u8], samples: &[EdgeBlendSample]) {
    let row_len: usize = row.len();
    for sample in samples.iter() {
        let off: usize = (sample.x as usize) * 4;
        if off + 4 > row_len {
            continue;
        }
        if sample.is_opaque {
            row[off] = sample.rgba[0];
            row[off + 1] = sample.rgba[1];
            row[off + 2] = sample.rgba[2];
            row[off + 3] = 255;
        } else {
            let dr: f32 = SRGB_TO_LIN_LUT[row[off] as usize];
            let dg: f32 = SRGB_TO_LIN_LUT[row[off + 1] as usize];
            let db: f32 = SRGB_TO_LIN_LUT[row[off + 2] as usize];
            let ir: usize =
                ((sample.sr_a + dr * sample.inv_a) * 4095.0).clamp(0.0, 4095.0) as usize;
            let ig: usize =
                ((sample.sg_a + dg * sample.inv_a) * 4095.0).clamp(0.0, 4095.0) as usize;
            let ib: usize =
                ((sample.sb_a + db * sample.inv_a) * 4095.0).clamp(0.0, 4095.0) as usize;
            row[off] = ((LIN_TO_SRGB_Q4_LUT[ir] + 8) >> 4).min(255) as u8;
            row[off + 1] = ((LIN_TO_SRGB_Q4_LUT[ig] + 8) >> 4).min(255) as u8;
            row[off + 2] = ((LIN_TO_SRGB_Q4_LUT[ib] + 8) >> 4).min(255) as u8;
            let va: u32 = (row[off + 3] as u32) * (255 - sample.sa_u32) + 128;
            row[off + 3] = (sample.sa_u32 + ((va + (va >> 8)) >> 8)).min(255) as u8;
        }
    }
}

/// Tests whether a physical visible rectangle overlaps any rounded-corner cutout square of `(clip_rect, radius)`.
///
/// # Arguments
///
/// - `Rect` - Candidate command visible bounds in physical pixels.
/// - `Rect` - Physical rounded clip rectangle.
/// - `CornerRadius` - Physical clip corner radii.
///
/// # Returns
///
/// - `bool` - True if at least one rounded corner box overlaps `vis`.
#[must_use]
pub(crate) fn rect_intersects_rounded_corners(
    vis: Rect,
    clip_rect: Rect,
    radius: CornerRadius,
) -> bool {
    let half_w: f32 = clip_rect.get_width() * 0.5;
    let half_h: f32 = clip_rect.get_height() * 0.5;
    let max_r: f32 = half_w.min(half_h).max(0.0);
    if max_r <= 0.25 {
        return false;
    }
    let x0: f32 = clip_rect.get_x();
    let y0: f32 = clip_rect.get_y();
    let x1: f32 = clip_rect.right();
    let y1: f32 = clip_rect.bottom();
    let r_tl: f32 = radius.get_top_left().min(max_r);
    if r_tl > 0.25
        && Rect::new(x0, y0, r_tl + 1.0, r_tl + 1.0)
            .intersect(vis)
            .is_some()
    {
        return true;
    }
    let r_tr: f32 = radius.get_top_right().min(max_r);
    if r_tr > 0.25
        && Rect::new(x1 - r_tr - 1.0, y0, r_tr + 1.0, r_tr + 1.0)
            .intersect(vis)
            .is_some()
    {
        return true;
    }
    let r_br: f32 = radius.get_bottom_right().min(max_r);
    if r_br > 0.25
        && Rect::new(x1 - r_br - 1.0, y1 - r_br - 1.0, r_br + 1.0, r_br + 1.0)
            .intersect(vis)
            .is_some()
    {
        return true;
    }
    let r_bl: f32 = radius.get_bottom_left().min(max_r);
    r_bl > 0.25
        && Rect::new(x0, y1 - r_bl - 1.0, r_bl + 1.0, r_bl + 1.0)
            .intersect(vis)
            .is_some()
}

/// Appends up to four pairwise-disjoint rounded-corner pixel boxes clipped to `(clip, fb_w, band_y0..band_y1)`.
///
/// # Arguments
///
/// - `RoundedClipMask` - Physical rounded clip descriptor.
/// - `Rect` - Active band-intersected clip bounds.
/// - `(u32, u32, u32)` - Tuple `(fb_w, band_y0, band_y1)`.
/// - `&mut Vec<(u32, u32, u32, u32)>` - Output list of `(x0, y0, x1, y1)` corner pixel boxes.
pub(crate) fn collect_rounded_clip_boxes(
    mask: RoundedClipMask,
    clip: Rect,
    band_bounds: (u32, u32, u32),
    out: &mut Vec<(u32, u32, u32, u32)>,
) {
    let (fb_w, band_y0, band_y1): (u32, u32, u32) = band_bounds;
    let clip_x0: u32 = (clip.get_x().floor().max(0.0) as u32).min(fb_w);
    let clip_y0: u32 = (clip.get_y().floor().max(0.0) as u32).clamp(band_y0, band_y1);
    let clip_x1: u32 = (clip.right().ceil().max(0.0) as u32).min(fb_w);
    let clip_y1: u32 = (clip.bottom().ceil().max(0.0) as u32).clamp(band_y0, band_y1);
    if clip_x0 >= clip_x1 || clip_y0 >= clip_y1 {
        return;
    }
    let rect: Rect = mask.get_rect();
    let radius: CornerRadius = mask.get_radius();
    let half_w: f32 = rect.get_width() * 0.5;
    let half_h: f32 = rect.get_height() * 0.5;
    let max_r: f32 = half_w.min(half_h).max(0.0);
    if max_r <= 0.25 {
        return;
    }
    let mid_x: u32 = ((rect.get_x() + half_w).round().max(0.0) as u32).min(fb_w);
    let mid_y: u32 = (rect.get_y() + half_h).round().max(0.0) as u32;
    let rx0_base: u32 = (rect.get_x().floor().max(0.0) as u32).min(fb_w);
    let ry0_base: u32 = rect.get_y().floor().max(0.0) as u32;
    let rx1_base: u32 = (rect.right().ceil().max(0.0) as u32).min(fb_w);
    let ry1_base: u32 = rect.bottom().ceil().max(0.0) as u32;
    let r_tl: f32 = radius.get_top_left().min(max_r);
    if r_tl > 0.25 {
        let bx0: u32 = rx0_base.max(clip_x0);
        let bx1: u32 =
            (((rect.get_x() + r_tl + 1.0).ceil().max(0.0) as u32).min(mid_x)).min(clip_x1);
        let by0: u32 = ry0_base.max(clip_y0);
        let by1: u32 =
            (((rect.get_y() + r_tl + 1.0).ceil().max(0.0) as u32).min(mid_y)).min(clip_y1);
        if bx0 < bx1 && by0 < by1 {
            out.push((bx0, by0, bx1, by1));
        }
    }
    let r_tr: f32 = radius.get_top_right().min(max_r);
    if r_tr > 0.25 {
        let bx0: u32 =
            (((rect.right() - r_tr - 1.0).floor().max(0.0) as u32).max(mid_x)).max(clip_x0);
        let bx1: u32 = rx1_base.min(clip_x1);
        let by0: u32 = ry0_base.max(clip_y0);
        let by1: u32 =
            (((rect.get_y() + r_tr + 1.0).ceil().max(0.0) as u32).min(mid_y)).min(clip_y1);
        if bx0 < bx1 && by0 < by1 {
            out.push((bx0, by0, bx1, by1));
        }
    }
    let r_br: f32 = radius.get_bottom_right().min(max_r);
    if r_br > 0.25 {
        let bx0: u32 =
            (((rect.right() - r_br - 1.0).floor().max(0.0) as u32).max(mid_x)).max(clip_x0);
        let bx1: u32 = rx1_base.min(clip_x1);
        let by0: u32 =
            (((rect.bottom() - r_br - 1.0).floor().max(0.0) as u32).max(mid_y)).max(clip_y0);
        let by1: u32 = ry1_base.min(clip_y1);
        if bx0 < bx1 && by0 < by1 {
            out.push((bx0, by0, bx1, by1));
        }
    }
    let r_bl: f32 = radius.get_bottom_left().min(max_r);
    if r_bl > 0.25 {
        let bx0: u32 = rx0_base.max(clip_x0);
        let bx1: u32 =
            (((rect.get_x() + r_bl + 1.0).ceil().max(0.0) as u32).min(mid_x)).min(clip_x1);
        let by0: u32 =
            (((rect.bottom() - r_bl - 1.0).floor().max(0.0) as u32).max(mid_y)).max(clip_y0);
        let by1: u32 = ry1_base.min(clip_y1);
        if bx0 < bx1 && by0 < by1 {
            out.push((bx0, by0, bx1, by1));
        }
    }
}

/// Evaluates anti-aliased rounded-clip mask coverage in `[0.0, 1.0]` for physical pixel `(x, y)`.
///
/// # Arguments
///
/// - `u32` - Physical pixel X coordinate.
/// - `u32` - Physical pixel Y coordinate.
/// - `RoundedClipMask` - Physical rounded clip descriptor.
/// - `AntialiasingMode` - Active anti-aliasing policy.
///
/// # Returns
///
/// - `f32` - Mask coverage in `[0.0, 1.0]` (`1.0` inside, `0.0` outside).
#[must_use]
pub(crate) fn rounded_clip_coverage(
    x: u32,
    y: u32,
    mask: RoundedClipMask,
    aa: AntialiasingMode,
) -> f32 {
    let rect: Rect = mask.get_rect();
    let half_w: f32 = rect.get_width() * 0.5;
    let half_h: f32 = rect.get_height() * 0.5;
    let px: f32 = (x as f32) + 0.5 - (rect.get_x() + half_w);
    let py: f32 = (y as f32) + 0.5 - (rect.get_y() + half_h);
    let corner_r: f32 = quadrant_corner_radius(mask.get_radius(), px, py)
        .min(half_w)
        .min(half_h)
        .max(0.0);
    if corner_r <= 0.0 {
        return 1.0;
    }
    let qx: f32 = px.abs() - (half_w - corner_r);
    let qy: f32 = py.abs() - (half_h - corner_r);
    if qx <= 0.0 || qy <= 0.0 {
        let dist: f32 = qx.max(qy) - corner_r;
        if dist <= -1.0 {
            return 1.0;
        }
        if dist >= 1.0 {
            return 0.0;
        }
        if aa == AntialiasingMode::None {
            return if dist <= 0.0 { 1.0 } else { 0.0 };
        }
    } else {
        let d_sq: f32 = qx * qx + qy * qy;
        let r_in: f32 = corner_r - 1.0;
        if r_in >= 0.0 && d_sq <= r_in * r_in {
            return 1.0;
        }
        let r_out: f32 = corner_r + 1.0;
        if d_sq >= r_out * r_out {
            return 0.0;
        }
        if aa == AntialiasingMode::None {
            return if d_sq <= corner_r * corner_r {
                1.0
            } else {
                0.0
            };
        }
    }
    let (cov, _): (f32, f32) =
        rounded_box_supersampled_coverage(px, py, half_w, half_h, corner_r, 0.0);
    cov
}

/// Blends a single RGBA8 pixel inside `row` back toward `saved_rgba` using linear-light clip coverage `cov`.
///
/// # Arguments
///
/// - `&mut [u8]` - Destination 4-byte pixel slice inside a framebuffer scanline.
/// - `[u8; 4]` - Pre-command saved RGBA8 pixel bytes.
/// - `f32` - Clip mask coverage in `[0.0, 1.0]` (`1.0` keeps current pixel, `0.0` restores `saved_rgba`).
#[inline(always)]
pub(crate) fn lerp_pixel_slice_from_saved_linear(dst: &mut [u8], saved_rgba: [u8; 4], cov: f32) {
    let cur: [u8; 4] = [dst[0], dst[1], dst[2], dst[3]];
    if cur == saved_rgba {
        return;
    }
    let c: f32 = cov.clamp(0.0, 1.0);
    let inv_c: f32 = 1.0 - c;
    let nr: f32 = SRGB_TO_LIN_LUT[cur[0] as usize];
    let ng: f32 = SRGB_TO_LIN_LUT[cur[1] as usize];
    let nb: f32 = SRGB_TO_LIN_LUT[cur[2] as usize];
    let or: f32 = SRGB_TO_LIN_LUT[saved_rgba[0] as usize];
    let og: f32 = SRGB_TO_LIN_LUT[saved_rgba[1] as usize];
    let ob: f32 = SRGB_TO_LIN_LUT[saved_rgba[2] as usize];
    let ir: usize = ((nr * c + or * inv_c) * 4095.0).clamp(0.0, 4095.0) as usize;
    let ig: usize = ((ng * c + og * inv_c) * 4095.0).clamp(0.0, 4095.0) as usize;
    let ib: usize = ((nb * c + ob * inv_c) * 4095.0).clamp(0.0, 4095.0) as usize;
    dst[0] = ((LIN_TO_SRGB_Q4_LUT[ir] + 8) >> 4).min(255) as u8;
    dst[1] = ((LIN_TO_SRGB_Q4_LUT[ig] + 8) >> 4).min(255) as u8;
    dst[2] = ((LIN_TO_SRGB_Q4_LUT[ib] + 8) >> 4).min(255) as u8;
    let na: f32 = (cur[3] as f32) * c + (saved_rgba[3] as f32) * inv_c;
    dst[3] = na.round().clamp(0.0, 255.0) as u8;
}

/// Evaluates rounded-clip coverage at `(x, y)` using `corner_masks` when cached, falling back to `rounded_clip_coverage`.
///
/// # Arguments
///
/// - `u32` - Physical pixel X coordinate.
/// - `u32` - Physical pixel Y coordinate.
/// - `RoundedClipMask` - Physical rounded clip descriptor.
/// - `&[CachedCornerMask]` - Read-only cached quadrant corner masks.
/// - `AntialiasingMode` - Active anti-aliasing policy.
///
/// # Returns
///
/// - `f32` - Exact coverage in `[0.0, 1.0]`.
#[inline(always)]
#[must_use]
pub(crate) fn rounded_clip_coverage_cached(
    x: u32,
    y: u32,
    mask: RoundedClipMask,
    corner_masks: &[CachedCornerMask],
    aa: AntialiasingMode,
) -> f32 {
    let rect: Rect = mask.get_rect();
    let half_w: f32 = rect.get_width() * 0.5;
    let half_h: f32 = rect.get_height() * 0.5;
    let px: f32 = (x as f32) + 0.5 - (rect.get_x() + half_w);
    let py: f32 = (y as f32) + 0.5 - (rect.get_y() + half_h);
    let quad: usize = if px < 0.0 {
        if py < 0.0 { 0 } else { 3 }
    } else if py < 0.0 {
        1
    } else {
        2
    };
    let mask_id: u32 = mask.get_corner_mask_ids()[quad];
    if mask_id != u32::MAX
        && let Some(cm) = corner_masks.get(mask_id as usize)
    {
        let dim_i: i32 = cm.dim as i32;
        let origin_x: i32 = if quad == 0 || quad == 3 {
            rect.get_x().round() as i32
        } else {
            (rect.right().round() as i32) - dim_i
        };
        let origin_y: i32 = if quad == 0 || quad == 1 {
            rect.get_y().round() as i32
        } else {
            (rect.bottom().round() as i32) - dim_i
        };
        let mx: i32 = (x as i32) - origin_x;
        let my: i32 = (y as i32) - origin_y;
        if mx >= 0 && mx < dim_i && my >= 0 && my < dim_i {
            let cov_u8: u8 = cm.cov[(my as usize) * (cm.dim as usize) + (mx as usize)];
            return (cov_u8 as f32) * 0.0625;
        }
        return 1.0;
    }
    rounded_clip_coverage(x, y, mask, aa)
}

/// Blits a contiguous 1:1 RGBA8 source scanline onto `dst_span`, using `copy_from_slice` when all pixels are opaque.
///
/// # Arguments
///
/// - `&[u8]` - Contiguous RGBA8 source pixel slice.
/// - `&mut [u8]` - Contiguous RGBA8 destination pixel slice of equal length.
#[inline(always)]
pub(crate) fn blit_1to1_rgba_span(src_span: &[u8], dst_span: &mut [u8]) {
    let len: usize = src_span.len().min(dst_span.len());
    let src_s: &[u8] = &src_span[..len];
    let dst_s: &mut [u8] = &mut dst_span[..len];
    let src_chunks: &[[u8; 4]] = src_s.as_chunks::<4>().0;
    if src_chunks.iter().all(|px: &[u8; 4]| px[3] == 255) {
        dst_s.copy_from_slice(src_s);
        return;
    }
    let dst_chunks: &mut [[u8; 4]] = dst_s.as_chunks_mut::<4>().0;
    for (sp, dp) in src_chunks.iter().zip(dst_chunks.iter_mut()) {
        let sa: u32 = sp[3] as u32;
        if sa == 0 {
            continue;
        }
        if sa == 255 {
            *dp = *sp;
            continue;
        }
        let inv_sa: u32 = 255 - sa;
        let da: u32 = dp[3] as u32;
        if da == 255 {
            dp[0] = (((sp[0] as u32) * sa + (dp[0] as u32) * inv_sa) / 255).min(255) as u8;
            dp[1] = (((sp[1] as u32) * sa + (dp[1] as u32) * inv_sa) / 255).min(255) as u8;
            dp[2] = (((sp[2] as u32) * sa + (dp[2] as u32) * inv_sa) / 255).min(255) as u8;
            dp[3] = 255;
        } else {
            let out_a: u32 = sa + ((da * inv_sa + 127) / 255);
            dp[0] = ((((sp[0] as u32) * sa) + (((dp[0] as u32) * da * inv_sa + 127) / 255)) / out_a)
                .min(255) as u8;
            dp[1] = ((((sp[1] as u32) * sa) + (((dp[1] as u32) * da * inv_sa + 127) / 255)) / out_a)
                .min(255) as u8;
            dp[2] = ((((sp[2] as u32) * sa) + (((dp[2] as u32) * da * inv_sa + 127) / 255)) / out_a)
                .min(255) as u8;
            dp[3] = out_a.min(255) as u8;
        }
    }
}

/// Blends a single RGBA8 source pixel with anti-aliased coverage `cov` directly into destination 4-byte slice `dp`.
///
/// # Arguments
///
/// - `&mut [u8]` - Destination 4-byte pixel slice.
/// - `[u8; 4]` - Source RGBA8 pixel bytes.
/// - `f32` - Shape coverage multiplier in `(0.0, 1.0]`.
#[inline(always)]
pub(crate) fn blend_rgba_pixel_with_cov(dp: &mut [u8], src_rgba: [u8; 4], cov: f32) {
    let sa: u32 = if cov >= 0.9995 {
        src_rgba[3] as u32
    } else {
        (((src_rgba[3] as f32) * cov).round().clamp(0.0, 255.0)) as u32
    };
    if sa == 0 {
        return;
    }
    if sa == 255 {
        dp[0] = src_rgba[0];
        dp[1] = src_rgba[1];
        dp[2] = src_rgba[2];
        dp[3] = 255;
        return;
    }
    let inv_sa: u32 = 255 - sa;
    let da: u32 = dp[3] as u32;
    if da == 255 {
        dp[0] = (((src_rgba[0] as u32) * sa + (dp[0] as u32) * inv_sa) / 255).min(255) as u8;
        dp[1] = (((src_rgba[1] as u32) * sa + (dp[1] as u32) * inv_sa) / 255).min(255) as u8;
        dp[2] = (((src_rgba[2] as u32) * sa + (dp[2] as u32) * inv_sa) / 255).min(255) as u8;
        dp[3] = 255;
    } else {
        let out_a: u32 = sa + ((da * inv_sa + 127) / 255);
        dp[0] = ((((src_rgba[0] as u32) * sa) + (((dp[0] as u32) * da * inv_sa + 127) / 255))
            / out_a)
            .min(255) as u8;
        dp[1] = ((((src_rgba[1] as u32) * sa) + (((dp[1] as u32) * da * inv_sa + 127) / 255))
            / out_a)
            .min(255) as u8;
        dp[2] = ((((src_rgba[2] as u32) * sa) + (((dp[2] as u32) * da * inv_sa + 127) / 255))
            / out_a)
            .min(255) as u8;
        dp[3] = out_a.min(255) as u8;
    }
}

/// Samples a single RGBA8 pixel using precomputed 8.8 fixed-point bilinear row offsets and horizontal LUT entry.
///
/// # Arguments
///
/// - `&[u8]` - Source RGBA8 pixel buffer.
/// - `(usize, usize, u32, u32)` - Vertical row parameters `(row0_px, row1_px, wy_256, iwy_256)`.
/// - `(usize, usize, u32)` - Horizontal LUT sample `(sx0, sx1, wx_256)`.
///
/// # Returns
///
/// - `[u8; 4]` - Bilinearly filtered RGBA8 pixel.
#[inline(always)]
#[must_use]
pub(crate) fn sample_bilinear_fixed(
    src_bytes: &[u8],
    y_params: (usize, usize, u32, u32),
    x_entry: (usize, usize, u32),
) -> [u8; 4] {
    let (row0, row1, wy, iwy): (usize, usize, u32, u32) = y_params;
    let (sx0, sx1, wx): (usize, usize, u32) = x_entry;
    let iwx: u32 = 256 - wx;
    if wy == 0 {
        let i0: usize = (row0 + sx0) * 4;
        let i1: usize = (row0 + sx1) * 4;
        if i1 + 4 > src_bytes.len() {
            return [0, 0, 0, 0];
        }
        return [
            (((src_bytes[i0] as u32) * iwx + (src_bytes[i1] as u32) * wx + 128) >> 8) as u8,
            (((src_bytes[i0 + 1] as u32) * iwx + (src_bytes[i1 + 1] as u32) * wx + 128) >> 8) as u8,
            (((src_bytes[i0 + 2] as u32) * iwx + (src_bytes[i1 + 2] as u32) * wx + 128) >> 8) as u8,
            (((src_bytes[i0 + 3] as u32) * iwx + (src_bytes[i1 + 3] as u32) * wx + 128) >> 8) as u8,
        ];
    }
    let i00: usize = (row0 + sx0) * 4;
    let i10: usize = (row0 + sx1) * 4;
    let i01: usize = (row1 + sx0) * 4;
    let i11: usize = (row1 + sx1) * 4;
    if i11 + 4 > src_bytes.len() {
        return [0, 0, 0, 0];
    }
    let w00: u32 = iwx * iwy;
    let w10: u32 = wx * iwy;
    let w01: u32 = iwx * wy;
    let w11: u32 = wx * wy;
    [
        (((src_bytes[i00] as u32) * w00
            + (src_bytes[i10] as u32) * w10
            + (src_bytes[i01] as u32) * w01
            + (src_bytes[i11] as u32) * w11
            + 32768)
            >> 16) as u8,
        (((src_bytes[i00 + 1] as u32) * w00
            + (src_bytes[i10 + 1] as u32) * w10
            + (src_bytes[i01 + 1] as u32) * w01
            + (src_bytes[i11 + 1] as u32) * w11
            + 32768)
            >> 16) as u8,
        (((src_bytes[i00 + 2] as u32) * w00
            + (src_bytes[i10 + 2] as u32) * w10
            + (src_bytes[i01 + 2] as u32) * w01
            + (src_bytes[i11 + 2] as u32) * w11
            + 32768)
            >> 16) as u8,
        (((src_bytes[i00 + 3] as u32) * w00
            + (src_bytes[i10 + 3] as u32) * w10
            + (src_bytes[i01 + 3] as u32) * w01
            + (src_bytes[i11 + 3] as u32) * w11
            + 32768)
            >> 16) as u8,
    ]
}

/// Blends a single gradient pixel sample with coverage `cov` and bit-exact IGN dithering into scanline `row`.
///
/// # Arguments
///
/// - `&mut [u8]` - Mutable destination framebuffer scanline slice.
/// - `u32` - Physical pixel X coordinate within `row`.
/// - `f32` - Precomputed vertical IGN phase `(y as f32) * 0.005_837_15`.
/// - `Color` - Interpolated gradient stop color at `(x, y)`.
/// - `f32` - Coverage multiplier in `(0.0, 1.0]`.
#[inline(always)]
pub(crate) fn blend_gradient_row_pixel(
    row: &mut [u8],
    x: u32,
    y_term: f32,
    sample: Color,
    cov: f32,
) {
    let off: usize = (x as usize) * 4;
    let alpha_f: f32 = ((sample.get_a() as f32) * (1.0 / 255.0)) * cov;
    if alpha_f <= 0.0002 || off + 4 > row.len() {
        return;
    }
    let a: f32 = alpha_f.clamp(0.0, 1.0);
    if a >= 0.9995 {
        row[off] = sample.get_r();
        row[off + 1] = sample.get_g();
        row[off + 2] = sample.get_b();
        row[off + 3] = 255;
        return;
    }
    let inv_a: f32 = 1.0 - a;
    let sr: f32 = SRGB_TO_LIN_LUT[sample.get_r() as usize];
    let sg: f32 = SRGB_TO_LIN_LUT[sample.get_g() as usize];
    let sb: f32 = SRGB_TO_LIN_LUT[sample.get_b() as usize];
    let dr: f32 = SRGB_TO_LIN_LUT[row[off] as usize];
    let dg: f32 = SRGB_TO_LIN_LUT[row[off + 1] as usize];
    let db: f32 = SRGB_TO_LIN_LUT[row[off + 2] as usize];
    let ir: usize = ((sr * a + dr * inv_a) * 4095.0).clamp(0.0, 4095.0) as usize;
    let ig: usize = ((sg * a + dg * inv_a) * 4095.0).clamp(0.0, 4095.0) as usize;
    let ib: usize = ((sb * a + db * inv_a) * 4095.0).clamp(0.0, 4095.0) as usize;
    let n: f32 = (52.982_918 * ((x as f32) * 0.067_110_56 + y_term).fract()).fract();
    let dq: i32 = ((n - 0.5) * 16.0) as i32 + 8;
    row[off] = (((LIN_TO_SRGB_Q4_LUT[ir] as i32) + dq) >> 4).clamp(0, 255) as u8;
    row[off + 1] = (((LIN_TO_SRGB_Q4_LUT[ig] as i32) + dq) >> 4).clamp(0, 255) as u8;
    row[off + 2] = (((LIN_TO_SRGB_Q4_LUT[ib] as i32) + dq) >> 4).clamp(0, 255) as u8;
    let sa_u32: u32 = (a * 255.0) as u32;
    let va: u32 = (row[off + 3] as u32) * (255 - sa_u32) + 128;
    row[off + 3] = (sa_u32 + ((va + (va >> 8)) >> 8)).min(255) as u8;
}

/// Evaluates anti-aliased coverage for a rounded-rectangle gradient corner/edge pixel without redundant SDF work.
///
/// # Arguments
///
/// - `(f32, f32, f32, f32)` - Tuple `(px, py, half_w, half_h)` relative to box center.
/// - `CornerRadius` - Physical per-quadrant corner radii.
/// - `AntialiasingMode` - Active anti-aliasing policy.
///
/// # Returns
///
/// - `f32` - Pixel coverage in `[0.0, 1.0]`.
#[inline(always)]
#[must_use]
pub(crate) fn gradient_rounded_pixel_coverage(
    geom: (f32, f32, f32, f32),
    radius: CornerRadius,
    aa: AntialiasingMode,
) -> f32 {
    let (px, py, half_w, half_h): (f32, f32, f32, f32) = geom;
    let corner_r: f32 = quadrant_corner_radius(radius, px, py)
        .min(half_w)
        .min(half_h)
        .max(0.0);
    let dist: f32 = rounded_box_sdf(px, py, half_w, half_h, corner_r);
    if dist > 0.85 {
        return 0.0;
    }
    if dist < -0.85 {
        return 1.0;
    }
    let in_corner: bool = aa != AntialiasingMode::None
        && corner_r > 0.5
        && px.abs() > (half_w - corner_r - 1.5)
        && py.abs() > (half_h - corner_r - 1.5);
    if in_corner {
        rounded_box_supersampled_coverage(px, py, half_w, half_h, corner_r, 0.0).0
    } else {
        sdf_coverage(dist, aa)
    }
}

/// Computes tight non-cutout bounding span `[out0, out1)` inside `[start, end)` excluding `[cut0, cut1)`.
///
/// # Arguments
///
/// - `u32` - Inclusive span start X.
/// - `u32` - Exclusive span end X.
/// - `u32` - Inclusive cutout start X.
/// - `u32` - Exclusive cutout end X.
///
/// # Returns
///
/// - `(u32, u32)` - Tight bounding active interval `(out0, out1)` (`out0 >= out1` when empty).
#[inline(always)]
#[must_use]
pub(crate) fn active_span_bounds(start: u32, end: u32, cut0: u32, cut1: u32) -> (u32, u32) {
    if start >= end {
        return (end, end);
    }
    if cut0 >= cut1 || cut1 <= start || cut0 >= end {
        return (start, end);
    }
    if cut0 <= start && cut1 >= end {
        return (end, end);
    }
    if cut0 <= start {
        return (cut1, end);
    }
    if cut1 >= end {
        return (start, cut0);
    }
    (start, end)
}

/// Evaluates bit-exact rounded-box signed distance for a shadow corner pixel with sqrt-free non-arc branches and early cutoff reject.
///
/// # Arguments
///
/// - `f32` - Absolute horizontal offset `|px|` from caster center.
/// - `f32` - Precomputed row vertical corner coordinate `|py| - half_h + r`.
/// - `f32` - Precomputed squared positive vertical offset `qy.max(0.0)^2`.
/// - `(f32, f32, f32, f32)` - Tuple `(half_w, r, cutoff, cutoff_r_safe_sq)`.
///
/// # Returns
///
/// - `f32` - Exact signed distance (or `cutoff` when conservatively outside cutoff).
#[inline(always)]
#[must_use]
pub(crate) fn shadow_corner_dist_fast(
    px_abs: f32,
    qy: f32,
    qy_pos_sq: f32,
    geom: (f32, f32, f32, f32),
) -> f32 {
    let (half_w, r, cutoff, cutoff_r_safe_sq): (f32, f32, f32, f32) = geom;
    let qx: f32 = px_abs - half_w + r;
    if qx > 0.0 {
        if qy > 0.0 {
            let d_sq: f32 = qx * qx + qy_pos_sq;
            if d_sq >= cutoff_r_safe_sq {
                return cutoff;
            }
            d_sq.sqrt() - r
        } else {
            qx - r
        }
    } else if qy > 0.0 {
        qy - r
    } else {
        qx.max(qy) - r
    }
}

/// Blends a single shadow pixel into `row` at `x` using hoisted linear tint RGB and bit-exact IGN dithering.
///
/// # Arguments
///
/// - `&mut [u8]` - Mutable target framebuffer scanline slice.
/// - `u32` - Physical pixel X coordinate within `row`.
/// - `f32` - Precomputed vertical IGN term `(y as f32) * 0.005_837_15`.
/// - `[u8; 3]` - Shadow tint sRGB bytes `[r, g, b]`.
/// - `[f32; 3]` - Hoisted linear shadow tint `[sr, sg, sb]`.
/// - `f32` - Continuous shadow opacity in `(0.0005, 1.0]`.
#[inline(always)]
pub(crate) fn blend_shadow_row_pixel(
    row: &mut [u8],
    x: u32,
    y_term: f32,
    color_rgb: [u8; 3],
    lin_rgb: [f32; 3],
    alpha_f: f32,
) {
    let off: usize = (x as usize) * 4;
    if off + 4 > row.len() {
        return;
    }
    let a: f32 = alpha_f.clamp(0.0, 1.0);
    if a >= 0.9995 {
        row[off] = color_rgb[0];
        row[off + 1] = color_rgb[1];
        row[off + 2] = color_rgb[2];
        row[off + 3] = 255;
        return;
    }
    let inv_a: f32 = 1.0 - a;
    let dr: f32 = SRGB_TO_LIN_LUT[row[off] as usize];
    let dg: f32 = SRGB_TO_LIN_LUT[row[off + 1] as usize];
    let db: f32 = SRGB_TO_LIN_LUT[row[off + 2] as usize];
    let ir: usize = ((lin_rgb[0] * a + dr * inv_a) * 4095.0).clamp(0.0, 4095.0) as usize;
    let ig: usize = ((lin_rgb[1] * a + dg * inv_a) * 4095.0).clamp(0.0, 4095.0) as usize;
    let ib: usize = ((lin_rgb[2] * a + db * inv_a) * 4095.0).clamp(0.0, 4095.0) as usize;
    let n: f32 = (52.982_918 * ((x as f32) * 0.067_110_56 + y_term).fract()).fract();
    let dq: i32 = ((n - 0.5) * 16.0) as i32 + 8;
    row[off] = (((LIN_TO_SRGB_Q4_LUT[ir] as i32) + dq) >> 4).clamp(0, 255) as u8;
    row[off + 1] = (((LIN_TO_SRGB_Q4_LUT[ig] as i32) + dq) >> 4).clamp(0, 255) as u8;
    row[off + 2] = (((LIN_TO_SRGB_Q4_LUT[ib] as i32) + dq) >> 4).clamp(0, 255) as u8;
    let sa_u32: u32 = (a * 255.0) as u32;
    let va: u32 = (row[off + 3] as u32) * (255 - sa_u32) + 128;
    row[off + 3] = (sa_u32 + ((va + (va >> 8)) >> 8)).min(255) as u8;
}

/// Blends one symmetrically mirrored corner shadow pixel using shared precomputed parameters and a 1-pixel destination cache.
///
/// # Arguments
///
/// - `&mut [u8]` - Mutable target framebuffer scanline slice.
/// - `u32` - Physical pixel X coordinate within `row`.
/// - `f32` - Precomputed vertical IGN term `(y as f32) * 0.005_837_15`.
/// - `ShadowStripParams` - Precomputed linear blend factors for this corner sample.
/// - `&mut ShadowPixelCache` - Single-pixel destination Q4 cache shared across mirrored quadrants.
#[inline(always)]
pub(crate) fn blend_shadow_mirrored_pixel(
    row: &mut [u8],
    x: u32,
    y_term: f32,
    params: ShadowStripParams,
    cache: &mut ShadowPixelCache,
) {
    let off: usize = (x as usize) * 4;
    if off + 4 > row.len() {
        return;
    }
    let dst_u32: u32 = u32::from_ne_bytes([row[off], row[off + 1], row[off + 2], row[off + 3]]);
    if !cache.valid || dst_u32 != cache.dst_u32 {
        let dr: f32 = SRGB_TO_LIN_LUT[row[off] as usize];
        let dg: f32 = SRGB_TO_LIN_LUT[row[off + 1] as usize];
        let db: f32 = SRGB_TO_LIN_LUT[row[off + 2] as usize];
        let ir: usize = ((params.sr_a + dr * params.inv_a) * 4095.0).clamp(0.0, 4095.0) as usize;
        let ig: usize = ((params.sg_a + dg * params.inv_a) * 4095.0).clamp(0.0, 4095.0) as usize;
        let ib: usize = ((params.sb_a + db * params.inv_a) * 4095.0).clamp(0.0, 4095.0) as usize;
        let va: u32 = (row[off + 3] as u32) * params.inv_a_u32 + 128;
        cache.dst_u32 = dst_u32;
        cache.q4_r = LIN_TO_SRGB_Q4_LUT[ir] as i32;
        cache.q4_g = LIN_TO_SRGB_Q4_LUT[ig] as i32;
        cache.q4_b = LIN_TO_SRGB_Q4_LUT[ib] as i32;
        cache.out_a = (params.sa_u32 + ((va + (va >> 8)) >> 8)).min(255) as u8;
        cache.valid = true;
    }
    let n: f32 = (52.982_918 * ((x as f32) * 0.067_110_56 + y_term).fract()).fract();
    let dq: i32 = ((n - 0.5) * 16.0) as i32 + 8;
    row[off] = ((cache.q4_r + dq) >> 4).clamp(0, 255) as u8;
    row[off + 1] = ((cache.q4_g + dq) >> 4).clamp(0, 255) as u8;
    row[off + 2] = ((cache.q4_b + dq) >> 4).clamp(0, 255) as u8;
    row[off + 3] = cache.out_a;
}

/// Precomputes 1D horizontal Gaussian shadow falloff samples for vertical side strip `[x_start, x_end)`.
///
/// # Arguments
///
/// - `(u32, u32)` - Inclusive/exclusive horizontal pixel span `(x_start, x_end)`.
/// - `&ShadowRenderCtx` - Shared shadow geometry and hoisted linear tint constants.
/// - `&mut Vec<ShadowStripHot>` - Output hot per-column cache vector (cleared and populated).
/// - `&mut Vec<ShadowStripParams>` - Output cold blend parameter vector (cleared and populated).
///
/// # Returns
///
/// - `(u32, u32)` - Trimmed active horizontal span `(act0, act1)` matching the populated vectors (`act0 >= act1` when empty).
#[must_use]
pub(crate) fn build_shadow_vertical_strip(
    x_span: (u32, u32),
    ctx: &ShadowRenderCtx,
    hot_out: &mut Vec<ShadowStripHot>,
    params_out: &mut Vec<ShadowStripParams>,
) -> (u32, u32) {
    hot_out.clear();
    params_out.clear();
    let (x_start, x_end): (u32, u32) = x_span;
    if x_start >= x_end {
        return (x_end, x_end);
    }
    let mut first_active: Option<u32> = None;
    let mut last_active_end: u32 = x_start;
    for x in x_start..x_end {
        let dist: f32 = ((x as f32) + 0.5 - ctx.cx).abs() - ctx.half_w;
        let alpha_f: f32 = if dist < ctx.cutoff {
            ctx.base_alpha * fast_gaussian_shadow_falloff(dist, ctx.inv_sigma)
        } else {
            0.0
        };
        if alpha_f <= 0.0005 {
            if first_active.is_some() {
                hot_out.push(ShadowStripHot::default());
                params_out.push(ShadowStripParams::default());
            }
            continue;
        }
        if first_active.is_none() {
            first_active = Some(x);
        }
        last_active_end = x + 1;
        let a: f32 = alpha_f.clamp(0.0, 1.0);
        let ign_x: f32 = (x as f32) * 0.067_110_56;
        if a >= 0.9995 {
            hot_out.push(ShadowStripHot {
                ign_x,
                cached_dst: 0,
                q4_r: 0,
                q4_g: 0,
                q4_b: 0,
                out_a: 255,
                state: 3,
            });
            params_out.push(ShadowStripParams::default());
        } else {
            let inv_a: f32 = 1.0 - a;
            let sa_u32: u32 = (a * 255.0) as u32;
            hot_out.push(ShadowStripHot {
                ign_x,
                cached_dst: 0,
                q4_r: 0,
                q4_g: 0,
                q4_b: 0,
                out_a: 0,
                state: 1,
            });
            params_out.push(ShadowStripParams {
                sr_a: ctx.lin_rgb[0] * a,
                sg_a: ctx.lin_rgb[1] * a,
                sb_a: ctx.lin_rgb[2] * a,
                inv_a,
                sa_u32,
                inv_a_u32: 255 - sa_u32,
            });
        }
    }
    let Some(act0) = first_active else {
        return (x_end, x_end);
    };
    let active_len: usize = (last_active_end - act0) as usize;
    hot_out.truncate(active_len);
    params_out.truncate(active_len);
    (act0, last_active_end)
}

/// Applies precomputed 1D horizontal shadow strip samples across `[span_x0, span_x1)` onto scanline `row`.
///
/// # Arguments
///
/// - `&mut [u8]` - Mutable destination framebuffer scanline slice.
/// - `(u32, u32, u32)` - Tuple `(span_x0, span_x1, strip_origin_x)`.
/// - `f32` - Precomputed vertical IGN term `(y as f32) * 0.005_837_15`.
/// - `[u8; 3]` - Shadow tint sRGB bytes `[r, g, b]`.
/// - `&mut [ShadowStripHot]` - Mutable hot per-column destination cache slice.
/// - `&[ShadowStripParams]` - Cold per-column linear blend parameter slice.
///
/// # Returns
///
/// - `usize` - Number of shaded pixels written within `[span_x0, span_x1)`.
#[inline(always)]
#[must_use]
pub(crate) fn apply_shadow_strip_span(
    row: &mut [u8],
    span_coords: (u32, u32, u32),
    y_term: f32,
    color_rgb: [u8; 3],
    hot: &mut [ShadowStripHot],
    params: &[ShadowStripParams],
) -> usize {
    let (span_x0, span_x1, strip_origin_x): (u32, u32, u32) = span_coords;
    if span_x0 >= span_x1 {
        return 0;
    }
    let start_idx: usize = (span_x0 - strip_origin_x) as usize;
    let count: usize = (span_x1 - span_x0) as usize;
    let byte_start: usize = (span_x0 as usize) * 4;
    let byte_end: usize = (span_x1 as usize) * 4;
    if byte_end > row.len() || start_idx + count > hot.len() || start_idx + count > params.len() {
        return 0;
    }
    let dst_chunks: &mut [[u8; 4]] = row[byte_start..byte_end].as_chunks_mut::<4>().0;
    let hot_slice: &mut [ShadowStripHot] = &mut hot[start_idx..start_idx + count];
    let params_slice: &[ShadowStripParams] = &params[start_idx..start_idx + count];
    let mut shaded: usize = 0;
    for (idx, (px, h)) in dst_chunks.iter_mut().zip(hot_slice.iter_mut()).enumerate() {
        let state: u8 = h.state;
        if state == 0 {
            continue;
        }
        if state == 3 {
            px[0] = color_rgb[0];
            px[1] = color_rgb[1];
            px[2] = color_rgb[2];
            px[3] = 255;
            shaded += 1;
            continue;
        }
        let dst_u32: u32 = u32::from_ne_bytes(*px);
        if state != 2 || dst_u32 != h.cached_dst {
            let p: ShadowStripParams = params_slice[idx];
            let dr: f32 = SRGB_TO_LIN_LUT[px[0] as usize];
            let dg: f32 = SRGB_TO_LIN_LUT[px[1] as usize];
            let db: f32 = SRGB_TO_LIN_LUT[px[2] as usize];
            let ir: usize = ((p.sr_a + dr * p.inv_a) * 4095.0).clamp(0.0, 4095.0) as usize;
            let ig: usize = ((p.sg_a + dg * p.inv_a) * 4095.0).clamp(0.0, 4095.0) as usize;
            let ib: usize = ((p.sb_a + db * p.inv_a) * 4095.0).clamp(0.0, 4095.0) as usize;
            let va: u32 = (px[3] as u32) * p.inv_a_u32 + 128;
            h.cached_dst = dst_u32;
            h.q4_r = LIN_TO_SRGB_Q4_LUT[ir] as i16;
            h.q4_g = LIN_TO_SRGB_Q4_LUT[ig] as i16;
            h.q4_b = LIN_TO_SRGB_Q4_LUT[ib] as i16;
            h.out_a = (p.sa_u32 + ((va + (va >> 8)) >> 8)).min(255) as u8;
            h.state = 2;
        }
        let n: f32 = (52.982_918 * (h.ign_x + y_term).fract()).fract();
        let dq: i32 = ((n - 0.5) * 16.0) as i32 + 8;
        px[0] = (((h.q4_r as i32) + dq) >> 4).clamp(0, 255) as u8;
        px[1] = (((h.q4_g as i32) + dq) >> 4).clamp(0, 255) as u8;
        px[2] = (((h.q4_b as i32) + dq) >> 4).clamp(0, 255) as u8;
        px[3] = h.out_a;
        shaded += 1;
    }
    shaded
}

/// Returns the process-wide persistent horizontal-band rasterizer worker pool, lazily spawning warm threads once.
///
/// # Returns
///
/// - `&'static BandWorkerPool` - Shared persistent band worker pool reference.
#[cfg(not(target_arch = "wasm32"))]
#[must_use]
pub(crate) fn global_band_worker_pool() -> &'static BandWorkerPool {
    BAND_WORKER_POOL.get_or_init(BandWorkerPool::new)
}

/// Executes dynamic work-stealing horizontal band rasterization for `job` until all strips are claimed.
///
/// # Arguments
///
/// - `&BandJob` - Active frame dispatch descriptor pointing to caller-pinned slices.
#[cfg(not(target_arch = "wasm32"))]
pub(crate) fn run_band_job_loop(job: &BandJob) {
    let tasks: &[BandTaskSlice] =
        unsafe { from_raw_parts(job.tasks_ptr as *const BandTaskSlice, job.tasks_len) };
    let commands: &[DrawCommand] =
        unsafe { from_raw_parts(job.commands_ptr as *const DrawCommand, job.commands_len) };
    let prepped: &[PreppedCommand] =
        unsafe { from_raw_parts(job.prepped_ptr as *const PreppedCommand, job.prepped_len) };
    let positioned_glyphs: &[PositionedGlyph] =
        unsafe { from_raw_parts(job.glyphs_ptr as *const PositionedGlyph, job.glyphs_len) };
    let caret_rects: &[Rect] =
        unsafe { from_raw_parts(job.caret_rects_ptr as *const Rect, job.caret_rects_len) };
    let rounded_clips: &[RoundedClipMask] = unsafe {
        from_raw_parts(
            job.rounded_clips_ptr as *const RoundedClipMask,
            job.rounded_clips_len,
        )
    };
    let corner_masks: &[CachedCornerMask] = unsafe {
        from_raw_parts(
            job.corner_masks_ptr as *const CachedCornerMask,
            job.corner_masks_len,
        )
    };
    let atlas: &GlyphAtlas = unsafe { &*(job.atlas_ptr as *const GlyphAtlas) };
    let mut local_shaded: usize = 0;
    loop {
        let idx: usize = job.next_idx.fetch_add(1, Ordering::AcqRel);
        if idx >= tasks.len() {
            break;
        }
        let task: BandTaskSlice = tasks[idx];
        let pixels: &mut [u8] =
            unsafe { from_raw_parts_mut(task.get_pixels_ptr() as *mut u8, task.get_pixels_len()) };
        if task.get_base_src_ptr() != 0 {
            let src_slice: &[u8] = unsafe {
                from_raw_parts(task.get_base_src_ptr() as *const u8, task.get_pixels_len())
            };
            pixels.copy_from_slice(src_slice);
        }
        let mut band: PixelBand<'_> = PixelBand {
            pixels,
            width: task.get_width(),
            y_offset: task.get_y_start(),
            height: task.get_height(),
        };
        local_shaded += SoftwareRenderer::render_band(
            &mut band,
            commands,
            prepped,
            (positioned_glyphs, caret_rects, rounded_clips, corner_masks),
            atlas,
            (job.scale, job.aa),
        );
    }
    if local_shaded > 0 {
        let _: usize = job.total_shaded.fetch_add(local_shaded, Ordering::AcqRel);
    }
}
