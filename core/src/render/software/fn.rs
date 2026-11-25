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

/// Converts a signed distance into smooth anti-aliased alpha coverage in `[0.0, 1.0]`.
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
            (0.5 - distance).clamp(0.0, 1.0)
        }
    }
}
