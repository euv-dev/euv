use super::*;

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

/// Converts a signed distance into Hermite smoothstep anti-aliased alpha coverage in `[0.0, 1.0]`.
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
        AntialiasingMode::AnalyticalSdf | AntialiasingMode::Msaa4x | AntialiasingMode::Ssaa2x => {
            let t: f32 = (0.5 - distance / 1.15).clamp(0.0, 1.0);
            t * t * (3.0 - 2.0 * t)
        }
    }
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
    let r: u8 = (((fill.get_r() as f32) * fill_a + (border.get_r() as f32) * border_a) * inv_a)
        .round()
        .clamp(0.0, 255.0) as u8;
    let g: u8 = (((fill.get_g() as f32) * fill_a + (border.get_g() as f32) * border_a) * inv_a)
        .round()
        .clamp(0.0, 255.0) as u8;
    let b: u8 = (((fill.get_b() as f32) * fill_a + (border.get_b() as f32) * border_a) * inv_a)
        .round()
        .clamp(0.0, 255.0) as u8;
    let a: u8 = (total_a * 255.0).round().clamp(0.0, 255.0) as u8;
    Color::new(r, g, b, a)
}
