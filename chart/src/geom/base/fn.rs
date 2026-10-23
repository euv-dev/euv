use super::*;

/// Converts an elliptical arc into a sequence of cubic Bézier segments.
///
/// The arc is expressed in the SVG convention: angles are measured
/// clockwise from the positive x-axis because the y-axis points down, and
/// `sweep` is the signed angular extent of the arc.
///
/// The caller receives the segments through `emit` so the conversion can
/// feed either the path builder or a mesh accumulator without allocating
/// an intermediate vector.
///
/// # Arguments
///
/// - `Ellipse` - A `Ellipse` parameter.
/// - `f64` - A 64-bit float (`f64`).
/// - `usize` - A non-negative integer (`usize`).
/// - `F` - A generic type parameter.
pub(crate) fn arc_to_cubic<F>(
    ellipse: Ellipse,
    start_angle: f64,
    sweep: f64,
    segments: usize,
    mut emit: F,
) where
    F: FnMut(f64, f64, f64, f64, f64, f64),
{
    let Ellipse {
        center_x,
        center_y,
        radius_x,
        radius_y,
    } = ellipse;
    let segments: usize = segments.max(1);
    let step: f64 = sweep / segments as f64;
    let kappa: f64 = ARC_BEZIER_KAPPA * (step / 4.0).tan();

    let mut angle: f64 = start_angle;
    let mut current_x: f64 = center_x + radius_x * angle.cos();
    let mut current_y: f64 = center_y + radius_y * angle.sin();

    for index in 0..segments {
        let next_angle: f64 = angle + step;
        let next_x: f64 = center_x + radius_x * next_angle.cos();
        let next_y: f64 = center_y + radius_y * next_angle.sin();

        let control1_x: f64 = current_x + kappa * radius_x * -angle.sin();
        let control1_y: f64 = current_y + kappa * radius_y * angle.cos();
        let control2_x: f64 = next_x - kappa * radius_x * -next_angle.sin();
        let control2_y: f64 = next_y - kappa * radius_y * next_angle.cos();

        emit(
            control1_x, control1_y, control2_x, control2_y, next_x, next_y,
        );

        current_x = next_x;
        current_y = next_y;
        angle = next_angle;

        if index + 1 < segments {
            // The next segment continues from where this one ended.
            let _: f64 = current_x;
        }
    }
}

/// Returns the number of Bézier segments needed to approximate an arc of
/// the given angular sweep within the given tolerance.
///
/// The count grows with the square root of the sweep, which is the
/// standard result for uniform Bézier subdivision: halving the angular
/// error by halving the segment length.
///
/// # Arguments
///
/// - `f64` - A 64-bit float (`f64`).
///
/// # Returns
///
/// - `usize` - A non-negative integer.
pub(crate) fn arc_segment_count(sweep: f64, radius: f64, tolerance: f64) -> usize {
    if radius <= 0.0 || tolerance <= 0.0 {
        return 1;
    }
    let sweep: f64 = sweep.abs();
    let max_angle: f64 = (4.0 / 3.0 * (tolerance / radius).tan()).atan();
    if max_angle <= f64::EPSILON {
        return MAX_ARC_SEGMENTS;
    }
    let count: usize = (sweep / max_angle).ceil() as usize;
    count.clamp(1, MAX_ARC_SEGMENTS)
}

/// Computes an initial estimate for the number of line segments needed to
/// approximate a cubic Bézier within the given tolerance.
///
/// The estimate is a starting point, not a guarantee: the tessellator
/// re-checks flatness after splitting and subdivides further when the
/// estimate turns out to be optimistic. The bound used here is the
/// second-difference form, whose deviation from the true curve is at most
/// `max(|P0 − 2·P1 + P2|, |P1 − 2·P2 + P3|) / 8`.
///
/// # Arguments
///
/// - `[(f64, f64); 4]` - A `[(f64, f64); 4]` parameter.
/// - `f64` - A 64-bit float (`f64`).
///
/// # Returns
///
/// - `usize` - A non-negative integer.
pub(crate) fn cubic_segment_count(points: [(f64, f64); 4], tolerance: f64) -> usize {
    let [(x0, y0), (x1, y1), (x2, y2), (x3, y3)] = points;
    let ax: f64 = (x0 - 2.0 * x1 + x2).abs();
    let ay: f64 = (y0 - 2.0 * y1 + y2).abs();
    let bx: f64 = (x1 - 2.0 * x2 + x3).abs();
    let by: f64 = (y1 - 2.0 * y2 + y3).abs();
    let deviation: f64 = ax.max(ay).max(bx).max(by) / 8.0;
    if deviation <= tolerance {
        1
    } else {
        let count: f64 = (deviation / tolerance).sqrt().ceil();
        (count as usize).clamp(1, 64)
    }
}
