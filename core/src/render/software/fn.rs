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

/// Computes a deterministic 64-bit hash key for caching `TextLayoutResult` entries across frames.
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
    let mut hash: u64 = FNV1A_OFFSET_BASIS;
    for byte in text.as_bytes().iter().copied() {
        hash ^= byte as u64;
        hash = hash.wrapping_mul(FNV1A_PRIME);
    }
    let fields: [u32; 4] = [
        style.get_font_size().to_bits(),
        style.get_line_height().to_bits(),
        style.get_letter_spacing().to_bits(),
        (style.get_weight() as u32)
            | ((style.get_align() as u32) << 8)
            | ((style.get_family() as u32) << 16)
            | ((style.get_overflow() as u32) << 20)
            | (u32::from(style.get_italic()) << 24),
    ];
    for word in fields.into_iter() {
        hash ^= word as u64;
        hash = hash.wrapping_mul(FNV1A_PRIME);
    }
    hash
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
    let has_border: bool = border_w > 0.0;
    let inner_hw: f32 = (half_w - border_w).max(0.0);
    let inner_hh: f32 = (half_h - border_w).max(0.0);
    let inner_r: f32 = (corner_r - border_w).max(0.0);
    let mut outer_hits: f32 = 0.0;
    let mut inner_hits: f32 = 0.0;
    for (row_idx, &oy) in grid.iter().enumerate() {
        let jx: f32 = if row_idx % 2 == 0 { -0.0625 } else { 0.0625 };
        let sy: f32 = py + oy;
        for (col_idx, &ox) in grid.iter().enumerate() {
            let jy: f32 = if col_idx % 2 == 0 { 0.0625 } else { -0.0625 };
            let sx: f32 = px + ox + jx;
            let sample_y: f32 = sy + jy;
            let d_out: f32 = rounded_box_sdf(sx, sample_y, half_w, half_h, corner_r);
            if d_out <= 0.0 {
                outer_hits += 1.0;
                if has_border {
                    if inner_hw > 0.0
                        && inner_hh > 0.0
                        && rounded_box_sdf(sx, sample_y, inner_hw, inner_hh, inner_r) <= 0.0
                    {
                        inner_hits += 1.0;
                    }
                } else {
                    inner_hits += 1.0;
                }
            }
        }
    }
    (outer_hits * 0.0625, inner_hits * 0.0625)
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
#[must_use]
pub fn circle_supersampled_coverage(dx: f32, dy: f32, radius: f32) -> f32 {
    let grid: [f32; 4] = [-0.375, -0.125, 0.125, 0.375];
    let r2: f32 = radius * radius;
    let mut hits: f32 = 0.0;
    for (row_idx, &oy) in grid.iter().enumerate() {
        let jx: f32 = if row_idx % 2 == 0 { -0.0625 } else { 0.0625 };
        let sy: f32 = dy + oy;
        for (col_idx, &ox) in grid.iter().enumerate() {
            let jy: f32 = if col_idx % 2 == 0 { 0.0625 } else { -0.0625 };
            let sx: f32 = dx + ox + jx;
            let sample_y: f32 = sy + jy;
            if sx * sx + sample_y * sample_y <= r2 {
                hits += 1.0;
            }
        }
    }
    hits * 0.0625
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
    let layouts: &[TextLayoutResult] =
        unsafe { from_raw_parts(job.layouts_ptr as *const TextLayoutResult, job.layouts_len) };
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
            &mut band, commands, prepped, layouts, atlas, job.scale, job.aa,
        );
    }
    if local_shaded > 0 {
        let _: usize = job.total_shaded.fetch_add(local_shaded, Ordering::AcqRel);
    }
}
