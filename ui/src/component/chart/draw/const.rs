/// Subdivision segments used for Catmull-Rom smooth line curves per control span.
pub(crate) const DRAW_CURVE_SEGMENTS: usize = 10;

/// Subdivision segments used for quadratic and cubic Bézier arcs.
pub(crate) const DRAW_BEZIER_SEGMENTS: usize = 18;

/// Maximum arc segments per full 360-degree sweep for wedges, gauges, and sunbursts.
pub(crate) const DRAW_ARC_FULL_STEPS: usize = 64;

/// Default dash length in logical pixels for dashed reference lines.
pub(crate) const DRAW_DASH_LEN: f32 = 5.0;

/// Default gap length in logical pixels for dashed reference lines.
pub(crate) const DRAW_GAP_LEN: f32 = 4.0;

/// Relative arrowhead barb spread angle in radians (~26 degrees).
pub(crate) const DRAW_ARROW_ANGLE_RAD: f32 = 0.45;
